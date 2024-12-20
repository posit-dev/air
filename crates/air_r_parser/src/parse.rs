use std::marker::PhantomData;

use air_r_syntax::RLanguage;
use air_r_syntax::RRoot;
use air_r_syntax::RSyntaxKind;
use air_r_syntax::RSyntaxNode;
use biome_parser::event::Event;
use biome_parser::prelude::ParseDiagnostic;
use biome_parser::prelude::Trivia;
use biome_parser::AnyParse;
use biome_rowan::AstNode;
use biome_rowan::NodeCache;
use biome_rowan::TextRange;
use biome_rowan::TextSize;
use biome_rowan::TriviaPieceKind;
use biome_unicode_table::Dispatch;
use tree_sitter::Tree;

use crate::treesitter::NodeTypeExt;
use crate::treesitter::Preorder;
use crate::treesitter::WalkEvent;
use crate::ParseError;
use crate::RLosslessTreeSink;
use crate::RParserOptions;

/// A utility struct for managing the result of a parser job
#[derive(Debug, Clone)]
pub struct Parse<T> {
    root: RSyntaxNode,
    errors: Vec<ParseError>,
    _ty: PhantomData<T>,
}

impl<T> Parse<T> {
    pub fn new(root: RSyntaxNode, errors: Vec<ParseError>) -> Parse<T> {
        Parse {
            root,
            errors,
            _ty: PhantomData,
        }
    }

    pub fn cast<N: AstNode<Language = RLanguage>>(self) -> Option<Parse<N>> {
        if N::can_cast(self.syntax().kind()) {
            Some(Parse::new(self.root, self.errors))
        } else {
            None
        }
    }

    /// The syntax node represented by this Parse result
    pub fn syntax(&self) -> RSyntaxNode {
        self.root.clone()
    }

    /// Get the errors which occurred when parsing
    pub fn errors(&self) -> &[ParseError] {
        self.errors.as_slice()
    }

    /// Get the errors which occurred when parsing
    pub fn into_errors(self) -> Vec<ParseError> {
        self.errors
    }

    /// Returns [true] if the parser encountered some errors during the parsing.
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
}

impl<T: AstNode<Language = RLanguage>> Parse<T> {
    /// Convert this parse result into a typed AST node.
    ///
    /// # Panics
    /// Panics if the node represented by this parse result mismatches.
    pub fn tree(&self) -> T {
        self.try_tree().unwrap_or_else(|| {
            panic!(
                "Expected tree to be a {} but root is:\n{:#?}",
                std::any::type_name::<T>(),
                self.syntax()
            )
        })
    }

    /// Try to convert this parse's untyped syntax node into an AST node.
    pub fn try_tree(&self) -> Option<T> {
        T::cast(self.syntax())
    }

    /// Convert this parse into a result
    pub fn into_result(self) -> Result<T, Vec<ParseError>> {
        if !self.has_errors() {
            Ok(self.tree())
        } else {
            Err(self.errors)
        }
    }
}

impl<T> From<Parse<T>> for AnyParse {
    fn from(parse: Parse<T>) -> Self {
        let root = parse.syntax();
        let errors = parse.into_errors();
        let diagnostics = errors
            .into_iter()
            .map(ParseError::into_diagnostic)
            .collect();
        Self::new(
            // SAFETY: the parser should always return a root node
            root.as_send().unwrap(),
            diagnostics,
        )
    }
}

pub fn parse(text: &str, options: RParserOptions) -> Parse<RRoot> {
    let mut cache = NodeCache::default();
    parse_r_with_cache(text, options, &mut cache)
}

pub fn parse_r_with_cache(
    text: &str,
    options: RParserOptions,
    cache: &mut NodeCache,
) -> Parse<RRoot> {
    tracing::debug_span!("parse").in_scope(move || {
        let (events, tokens, errors) = parse_text(text, options);

        // We've determined that passing diagnostics through does nothing.
        // They go into the tree-sink but come right back out. We think they
        // are a holdover from rust-analyzer that can be removed now. The real
        // errors are in `errors`.
        let _diagnostics = vec![];

        let mut tree_sink = RLosslessTreeSink::with_cache(text, &tokens, cache);
        biome_parser::event::process(&mut tree_sink, events, _diagnostics);
        let (green, _diagnostics) = tree_sink.finish();

        Parse::new(green, errors)
    })
}

pub fn parse_text(
    text: &str,
    _options: RParserOptions,
) -> (Vec<Event<RSyntaxKind>>, Vec<Trivia>, Vec<ParseError>) {
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_r::LANGUAGE.into())
        .unwrap();

    let ast = parser.parse(text, None).unwrap();

    if ast.root_node().has_error() {
        // TODO: In the long term we want an error resiliant parser.
        // This would probably only be able to happen if we swap out tree sitter
        // for a hand written recursive descent pratt parser using the Biome infra.
        return parse_failure();
    }

    parse_tree(ast, text)
}

fn parse_failure() -> (Vec<Event<RSyntaxKind>>, Vec<Trivia>, Vec<ParseError>) {
    // Must provide a root node on failures, otherwise `tree_sink.finish()` fails
    let events = vec![
        Event::Start {
            kind: RSyntaxKind::R_ROOT,
            forward_parent: None,
        },
        Event::Finish,
    ];

    // No trivia
    let trivia = vec![];

    // Generate a single diagnostic, wrap it in our error type
    let span: Option<TextRange> = None;
    let diagnostic = ParseDiagnostic::new("Failed to parse", span);
    let error = ParseError::from(diagnostic);
    let errors = vec![error];

    (events, trivia, errors)
}

fn parse_tree(ast: Tree, text: &str) -> (Vec<Event<RSyntaxKind>>, Vec<Trivia>, Vec<ParseError>) {
    let mut walker = RWalk::new(text);

    let root = ast.root_node();
    let mut iter = root.preorder();
    walker.walk(&mut iter);

    walker.parse.drain()
}

/// Given an ast with absolutely no ERROR or MISSING nodes, let's walk that tree
/// and collect our `trivia` and `events`.
struct RWalk<'src> {
    text: &'src str,
    parse: RParse,
    last_end: TextSize,
    between_two_tokens: bool,
}

impl<'src> RWalk<'src> {
    fn new(text: &'src str) -> Self {
        Self {
            text,
            parse: RParse::new(),
            last_end: TextSize::from(0),
            // Starts between the start of file and the first token
            between_two_tokens: false,
        }
    }

    fn walk(&mut self, iter: &mut Preorder) {
        while let Some(event) = iter.next() {
            match event {
                WalkEvent::Enter(node) => self.handle_enter(node, node.syntax_kind(), iter),
                WalkEvent::Leave(node) => self.handle_leave(node, node.syntax_kind()),
            }
        }
    }

    /// Walk only the upcoming node, including its subtree
    ///
    /// If the `next()` event is an `Enter`, `walk_next()` walks the
    /// `node` returned in the `Enter` event until hitting the `Leave` event
    /// of that `node`.
    fn walk_next(&mut self, iter: &mut Preorder) {
        let end = match iter.peek() {
            Some(end) => match end {
                WalkEvent::Enter(end) => *end,
                WalkEvent::Leave(_) => return,
            },
            None => return,
        };

        while let Some(event) = iter.next() {
            match event {
                WalkEvent::Enter(node) => self.handle_enter(node, node.syntax_kind(), iter),
                WalkEvent::Leave(node) => {
                    self.handle_leave(node, node.syntax_kind());
                    if node == end {
                        break;
                    }
                }
            }
        }
    }

    fn handle_enter(&mut self, node: tree_sitter::Node, kind: RSyntaxKind, iter: &mut Preorder) {
        match kind {
            RSyntaxKind::R_ROOT => self.handle_root_enter(),

            RSyntaxKind::R_UNARY_EXPRESSION
            | RSyntaxKind::R_BINARY_EXPRESSION
            | RSyntaxKind::R_FUNCTION_DEFINITION
            | RSyntaxKind::R_FOR_STATEMENT
            | RSyntaxKind::R_WHILE_STATEMENT
            | RSyntaxKind::R_REPEAT_STATEMENT
            | RSyntaxKind::R_PARENTHESIZED_EXPRESSION
            | RSyntaxKind::R_EXTRACT_EXPRESSION
            | RSyntaxKind::R_NAMESPACE_EXPRESSION
            | RSyntaxKind::R_NA_EXPRESSION => self.handle_node_enter(kind),

            RSyntaxKind::R_CALL => self.handle_call_like_enter(
                RSyntaxKind::R_CALL,
                RSyntaxKind::R_CALL_ARGUMENTS,
                node,
                iter,
            ),
            RSyntaxKind::R_SUBSET => self.handle_call_like_enter(
                RSyntaxKind::R_SUBSET,
                RSyntaxKind::R_SUBSET_ARGUMENTS,
                node,
                iter,
            ),
            RSyntaxKind::R_SUBSET2 => self.handle_call_like_enter(
                RSyntaxKind::R_SUBSET2,
                RSyntaxKind::R_SUBSET2_ARGUMENTS,
                node,
                iter,
            ),
            RSyntaxKind::R_PARAMETERS => self.handle_parameters_enter(node, iter),
            RSyntaxKind::R_PARAMETER => self.handle_parameter_enter(node, iter),
            RSyntaxKind::R_IF_STATEMENT => self.handle_if_statement_enter(node, iter),
            RSyntaxKind::R_ARGUMENT => self.handle_argument_enter(node, iter),
            RSyntaxKind::R_BRACED_EXPRESSIONS => self.handle_braced_expressions_enter(node, iter),

            // Literals / wrapped keywords
            RSyntaxKind::R_DOUBLE_VALUE
            | RSyntaxKind::R_IDENTIFIER
            | RSyntaxKind::R_DOT_DOT_I
            | RSyntaxKind::R_DOTS
            | RSyntaxKind::R_RETURN_EXPRESSION
            | RSyntaxKind::R_NEXT_EXPRESSION
            | RSyntaxKind::R_BREAK_EXPRESSION
            | RSyntaxKind::R_TRUE_EXPRESSION
            | RSyntaxKind::R_FALSE_EXPRESSION
            | RSyntaxKind::R_NULL_EXPRESSION
            | RSyntaxKind::R_INF_EXPRESSION
            | RSyntaxKind::R_NAN_EXPRESSION => self.handle_value_enter(kind),

            RSyntaxKind::R_INTEGER_VALUE => self.handle_integer_value_enter(iter),
            RSyntaxKind::R_COMPLEX_VALUE => self.handle_complex_value_enter(iter),
            RSyntaxKind::R_STRING_VALUE => self.handle_string_value_enter(iter),

            // Tokens are no-ops on `Enter`, handled on `Leave`
            RSyntaxKind::SEMICOLON
            | RSyntaxKind::COMMA
            | RSyntaxKind::TILDE
            | RSyntaxKind::ASSIGN
            | RSyntaxKind::SUPER_ASSIGN
            | RSyntaxKind::WALRUS
            | RSyntaxKind::ASSIGN_RIGHT
            | RSyntaxKind::SUPER_ASSIGN_RIGHT
            | RSyntaxKind::EQUAL
            | RSyntaxKind::OR
            | RSyntaxKind::AND
            | RSyntaxKind::OR2
            | RSyntaxKind::AND2
            | RSyntaxKind::LESS_THAN
            | RSyntaxKind::LESS_THAN_OR_EQUAL_TO
            | RSyntaxKind::GREATER_THAN
            | RSyntaxKind::GREATER_THAN_OR_EQUAL_TO
            | RSyntaxKind::EQUAL2
            | RSyntaxKind::NOT_EQUAL
            | RSyntaxKind::PLUS
            | RSyntaxKind::MINUS
            | RSyntaxKind::MULTIPLY
            | RSyntaxKind::DIVIDE
            | RSyntaxKind::EXPONENTIATE
            | RSyntaxKind::EXPONENTIATE2
            | RSyntaxKind::PIPE
            | RSyntaxKind::SPECIAL
            | RSyntaxKind::COLON
            | RSyntaxKind::COLON2
            | RSyntaxKind::COLON3
            | RSyntaxKind::DOLLAR
            | RSyntaxKind::AT
            | RSyntaxKind::BANG
            | RSyntaxKind::WAT
            | RSyntaxKind::BACKSLASH
            | RSyntaxKind::FUNCTION_KW
            | RSyntaxKind::FOR_KW
            | RSyntaxKind::IN_KW
            | RSyntaxKind::WHILE_KW
            | RSyntaxKind::REPEAT_KW
            | RSyntaxKind::IF_KW
            | RSyntaxKind::ELSE_KW
            | RSyntaxKind::NA_LOGICAL_KW
            | RSyntaxKind::NA_INTEGER_KW
            | RSyntaxKind::NA_DOUBLE_KW
            | RSyntaxKind::NA_COMPLEX_KW
            | RSyntaxKind::NA_CHARACTER_KW
            | RSyntaxKind::L_PAREN
            | RSyntaxKind::R_PAREN
            | RSyntaxKind::L_BRACK
            | RSyntaxKind::R_BRACK
            | RSyntaxKind::L_BRACK2
            | RSyntaxKind::R_BRACK2
            | RSyntaxKind::L_CURLY
            | RSyntaxKind::R_CURLY => (),

            // Comments
            RSyntaxKind::COMMENT => self.handle_comment_enter(),

            // Unreachable
            RSyntaxKind::R_ELSE_CLAUSE
            | RSyntaxKind::R_CALL_ARGUMENTS
            | RSyntaxKind::R_SUBSET_ARGUMENTS
            | RSyntaxKind::R_SUBSET2_ARGUMENTS
            | RSyntaxKind::R_ARGUMENT_NAME_CLAUSE
            | RSyntaxKind::R_PARAMETER_LIST
            | RSyntaxKind::R_ARGUMENT_LIST
            | RSyntaxKind::R_EXPRESSION_LIST
            | RSyntaxKind::R_PARAMETER_DEFAULT
            | RSyntaxKind::EOF
            | RSyntaxKind::UNICODE_BOM
            | RSyntaxKind::DOTS
            | RSyntaxKind::R_INTEGER_LITERAL
            | RSyntaxKind::R_DOUBLE_LITERAL
            | RSyntaxKind::R_COMPLEX_LITERAL
            | RSyntaxKind::R_STRING_LITERAL
            | RSyntaxKind::NEWLINE
            | RSyntaxKind::WHITESPACE
            | RSyntaxKind::IDENT
            | RSyntaxKind::DOTDOTI
            | RSyntaxKind::RETURN_KW
            | RSyntaxKind::NEXT_KW
            | RSyntaxKind::BREAK_KW
            | RSyntaxKind::TRUE_KW
            | RSyntaxKind::FALSE_KW
            | RSyntaxKind::NULL_KW
            | RSyntaxKind::INF_KW
            | RSyntaxKind::NAN_KW
            | RSyntaxKind::R_BOGUS
            | RSyntaxKind::R_BOGUS_VALUE
            | RSyntaxKind::R_BOGUS_EXPRESSION
            | RSyntaxKind::TOMBSTONE
            | RSyntaxKind::__LAST => unreachable!("{kind:?}"),
        }
    }

    fn handle_leave(&mut self, node: tree_sitter::Node, kind: RSyntaxKind) {
        match kind {
            RSyntaxKind::R_ROOT => self.handle_root_leave(node),

            RSyntaxKind::R_UNARY_EXPRESSION
            | RSyntaxKind::R_BINARY_EXPRESSION
            | RSyntaxKind::R_FUNCTION_DEFINITION
            | RSyntaxKind::R_FOR_STATEMENT
            | RSyntaxKind::R_WHILE_STATEMENT
            | RSyntaxKind::R_REPEAT_STATEMENT
            | RSyntaxKind::R_PARENTHESIZED_EXPRESSION
            | RSyntaxKind::R_EXTRACT_EXPRESSION
            | RSyntaxKind::R_NAMESPACE_EXPRESSION
            | RSyntaxKind::R_NA_EXPRESSION => self.handle_node_leave(kind),

            RSyntaxKind::R_CALL => self.handle_call_like_leave(kind),
            RSyntaxKind::R_SUBSET => self.handle_call_like_leave(kind),
            RSyntaxKind::R_SUBSET2 => self.handle_call_like_leave(kind),
            RSyntaxKind::R_PARAMETERS => self.handle_parameters_leave(),
            RSyntaxKind::R_PARAMETER => self.handle_parameter_leave(),
            RSyntaxKind::R_IF_STATEMENT => self.handle_if_statement_leave(),
            RSyntaxKind::R_ARGUMENT => self.handle_argument_leave(),
            RSyntaxKind::R_BRACED_EXPRESSIONS => self.handle_braced_expressions_leave(),

            // Literals / wrapped keywords
            RSyntaxKind::R_DOUBLE_VALUE => {
                self.handle_value_leave(node, kind, RSyntaxKind::R_DOUBLE_LITERAL)
            }
            RSyntaxKind::R_IDENTIFIER => self.handle_value_leave(node, kind, RSyntaxKind::IDENT),
            RSyntaxKind::R_DOTS => self.handle_value_leave(node, kind, RSyntaxKind::DOTS),
            RSyntaxKind::R_DOT_DOT_I => self.handle_value_leave(node, kind, RSyntaxKind::DOTDOTI),
            RSyntaxKind::R_RETURN_EXPRESSION => {
                self.handle_value_leave(node, kind, RSyntaxKind::RETURN_KW)
            }
            RSyntaxKind::R_NEXT_EXPRESSION => {
                self.handle_value_leave(node, kind, RSyntaxKind::NEXT_KW)
            }
            RSyntaxKind::R_BREAK_EXPRESSION => {
                self.handle_value_leave(node, kind, RSyntaxKind::BREAK_KW)
            }
            RSyntaxKind::R_TRUE_EXPRESSION => {
                self.handle_value_leave(node, kind, RSyntaxKind::TRUE_KW)
            }
            RSyntaxKind::R_FALSE_EXPRESSION => {
                self.handle_value_leave(node, kind, RSyntaxKind::FALSE_KW)
            }
            RSyntaxKind::R_NULL_EXPRESSION => {
                self.handle_value_leave(node, kind, RSyntaxKind::NULL_KW)
            }
            RSyntaxKind::R_INF_EXPRESSION => {
                self.handle_value_leave(node, kind, RSyntaxKind::INF_KW)
            }
            RSyntaxKind::R_NAN_EXPRESSION => {
                self.handle_value_leave(node, kind, RSyntaxKind::NAN_KW)
            }
            RSyntaxKind::R_INTEGER_VALUE => self.handle_integer_value_leave(node),
            RSyntaxKind::R_COMPLEX_VALUE => self.handle_complex_value_leave(node),
            RSyntaxKind::R_STRING_VALUE => self.handle_string_value_leave(node),

            // Tokens
            RSyntaxKind::SEMICOLON
            | RSyntaxKind::COMMA
            | RSyntaxKind::TILDE
            | RSyntaxKind::ASSIGN
            | RSyntaxKind::SUPER_ASSIGN
            | RSyntaxKind::WALRUS
            | RSyntaxKind::ASSIGN_RIGHT
            | RSyntaxKind::SUPER_ASSIGN_RIGHT
            | RSyntaxKind::EQUAL
            | RSyntaxKind::OR
            | RSyntaxKind::AND
            | RSyntaxKind::OR2
            | RSyntaxKind::AND2
            | RSyntaxKind::LESS_THAN
            | RSyntaxKind::LESS_THAN_OR_EQUAL_TO
            | RSyntaxKind::GREATER_THAN
            | RSyntaxKind::GREATER_THAN_OR_EQUAL_TO
            | RSyntaxKind::EQUAL2
            | RSyntaxKind::NOT_EQUAL
            | RSyntaxKind::PLUS
            | RSyntaxKind::MINUS
            | RSyntaxKind::MULTIPLY
            | RSyntaxKind::DIVIDE
            | RSyntaxKind::EXPONENTIATE
            | RSyntaxKind::EXPONENTIATE2
            | RSyntaxKind::PIPE
            | RSyntaxKind::SPECIAL
            | RSyntaxKind::COLON
            | RSyntaxKind::COLON2
            | RSyntaxKind::COLON3
            | RSyntaxKind::DOLLAR
            | RSyntaxKind::AT
            | RSyntaxKind::BANG
            | RSyntaxKind::WAT
            | RSyntaxKind::BACKSLASH
            | RSyntaxKind::FUNCTION_KW
            | RSyntaxKind::FOR_KW
            | RSyntaxKind::IN_KW
            | RSyntaxKind::WHILE_KW
            | RSyntaxKind::REPEAT_KW
            | RSyntaxKind::IF_KW
            | RSyntaxKind::ELSE_KW
            | RSyntaxKind::NA_LOGICAL_KW
            | RSyntaxKind::NA_INTEGER_KW
            | RSyntaxKind::NA_DOUBLE_KW
            | RSyntaxKind::NA_COMPLEX_KW
            | RSyntaxKind::NA_CHARACTER_KW
            | RSyntaxKind::L_PAREN
            | RSyntaxKind::R_PAREN
            | RSyntaxKind::L_BRACK
            | RSyntaxKind::R_BRACK
            | RSyntaxKind::L_BRACK2
            | RSyntaxKind::R_BRACK2
            | RSyntaxKind::L_CURLY
            | RSyntaxKind::R_CURLY => self.handle_token(node, kind),

            // Comments
            RSyntaxKind::COMMENT => self.handle_comment_leave(node),

            // Unreachable directly
            RSyntaxKind::R_ELSE_CLAUSE
            | RSyntaxKind::R_CALL_ARGUMENTS
            | RSyntaxKind::R_SUBSET_ARGUMENTS
            | RSyntaxKind::R_SUBSET2_ARGUMENTS
            | RSyntaxKind::R_ARGUMENT_NAME_CLAUSE
            | RSyntaxKind::R_PARAMETER_LIST
            | RSyntaxKind::R_ARGUMENT_LIST
            | RSyntaxKind::R_EXPRESSION_LIST
            | RSyntaxKind::R_PARAMETER_DEFAULT
            | RSyntaxKind::EOF
            | RSyntaxKind::UNICODE_BOM
            | RSyntaxKind::DOTS
            | RSyntaxKind::DOTDOTI
            | RSyntaxKind::R_INTEGER_LITERAL
            | RSyntaxKind::R_DOUBLE_LITERAL
            | RSyntaxKind::R_COMPLEX_LITERAL
            | RSyntaxKind::R_STRING_LITERAL
            | RSyntaxKind::NEWLINE
            | RSyntaxKind::WHITESPACE
            | RSyntaxKind::IDENT
            | RSyntaxKind::RETURN_KW
            | RSyntaxKind::NEXT_KW
            | RSyntaxKind::BREAK_KW
            | RSyntaxKind::TRUE_KW
            | RSyntaxKind::FALSE_KW
            | RSyntaxKind::NULL_KW
            | RSyntaxKind::INF_KW
            | RSyntaxKind::NAN_KW
            | RSyntaxKind::R_BOGUS
            | RSyntaxKind::R_BOGUS_VALUE
            | RSyntaxKind::R_BOGUS_EXPRESSION
            | RSyntaxKind::TOMBSTONE
            | RSyntaxKind::__LAST => unreachable!("{kind:?}"),
        }
    }

    fn handle_node_enter(&mut self, kind: RSyntaxKind) {
        self.parse.start(kind);
    }

    // `_kind` is nice to see at call sites as it makes the code more self-expanatory
    fn handle_node_leave(&mut self, _kind: RSyntaxKind) {
        self.parse.finish();
    }

    fn handle_value_enter(&mut self, kind: RSyntaxKind) {
        self.handle_node_enter(kind);
    }

    fn handle_value_leave(
        &mut self,
        node: tree_sitter::Node,
        kind: RSyntaxKind,
        literal_kind: RSyntaxKind,
    ) {
        // Push the token for the literal
        self.handle_token(node, literal_kind);

        // Then close the node
        self.handle_node_leave(kind);
    }

    fn handle_root_enter(&mut self) {
        // Start the overarching root
        self.handle_node_enter(RSyntaxKind::R_ROOT);

        // TODO: Handle optional BOM?

        // Root contains a list of `expressions`
        self.handle_node_enter(RSyntaxKind::R_EXPRESSION_LIST);
    }

    fn handle_root_leave(&mut self, node: tree_sitter::Node) {
        // Finish expression list
        self.handle_node_leave(RSyntaxKind::R_EXPRESSION_LIST);

        // No longer between two tokens.
        // Now between last token and EOF.
        self.between_two_tokens = false;

        // TODO!: Don't unwrap()
        let this_end = TextSize::try_from(node.end_byte()).unwrap();
        let gap = &self.text[usize::from(self.last_end)..usize::from(this_end)];

        // Derive trivia between last token and end of document.
        // It is always leading trivia of the `EOF` token,
        // which `TreeSink` adds for us.
        self.parse
            .derive_trivia(gap, self.last_end, self.between_two_tokens);

        self.handle_node_leave(RSyntaxKind::R_ROOT);
    }

    fn handle_token(&mut self, node: tree_sitter::Node, kind: RSyntaxKind) {
        // TODO!: Don't unwrap()
        let this_start = TextSize::try_from(node.start_byte()).unwrap();
        let this_end = TextSize::try_from(node.end_byte()).unwrap();
        let gap = &self.text[usize::from(self.last_end)..usize::from(this_start)];

        self.parse
            .derive_trivia(gap, self.last_end, self.between_two_tokens);

        self.parse.token(kind, this_end);

        self.last_end = this_end;
        self.between_two_tokens = true;
    }

    fn handle_comment_enter(&mut self) {
        // Nothing, handled on `Leave`
    }

    fn handle_comment_leave(&mut self, node: tree_sitter::Node) {
        let this_start = TextSize::try_from(node.start_byte()).unwrap();
        let this_end = TextSize::try_from(node.end_byte()).unwrap();
        let gap = &self.text[usize::from(self.last_end)..usize::from(this_start)];

        let mut trailing = self.between_two_tokens;

        if gap.contains('\n') {
            // If the gap has a newline this is a leading comment
            trailing = false;
            self.parse
                .derive_trivia(gap, self.last_end, self.between_two_tokens);
        } else {
            // Otherwise we're just after a token and this is a trailing comment,
            // unless we are at the beginning of the document, in which case
            // the whitespace and comment are leading.
            //
            // We also make sure we don't add an empty whitespace trivia.
            if this_start != self.last_end {
                self.parse.trivia.push(Trivia::new(
                    TriviaPieceKind::Whitespace,
                    TextRange::new(self.last_end, this_start),
                    trailing,
                ));
            }
        }

        // Comments are "single line" even if they are consecutive
        self.parse.trivia.push(Trivia::new(
            TriviaPieceKind::SingleLineComment,
            TextRange::new(this_start, this_end),
            trailing,
        ));

        self.last_end = this_end;
    }

    fn handle_parameters_enter(&mut self, node: tree_sitter::Node, iter: &mut Preorder) {
        self.handle_node_enter(RSyntaxKind::R_PARAMETERS);

        // Regardless of whether or not we see an `R_PARAMETER`, we have to
        // open and close the required `R_PARAMETER_LIST`
        while let Some(event) = iter.peek() {
            match event {
                WalkEvent::Enter(next) => match next.syntax_kind() {
                    RSyntaxKind::L_PAREN => {
                        self.walk_next(iter);
                        self.handle_node_enter(RSyntaxKind::R_PARAMETER_LIST);
                    }
                    RSyntaxKind::R_PAREN => {
                        self.handle_node_leave(RSyntaxKind::R_PARAMETER_LIST);
                        self.walk_next(iter);
                    }
                    RSyntaxKind::R_PARAMETER | RSyntaxKind::COMMA | RSyntaxKind::COMMENT => {
                        self.walk_next(iter)
                    }
                    kind => unreachable!("{kind:?}"),
                },
                WalkEvent::Leave(next) => {
                    if node != *next {
                        panic!("Expected next `Leave` event to be for `node`.");
                    }
                    break;
                }
            }
        }
    }

    fn handle_parameters_leave(&mut self) {
        self.handle_node_leave(RSyntaxKind::R_PARAMETERS);
    }

    fn handle_parameter_enter(&mut self, node: tree_sitter::Node, iter: &mut Preorder) {
        self.handle_node_enter(RSyntaxKind::R_PARAMETER);

        // Seeing an `=` causes us to open an `R_PARAMETER_DEFAULT` node which
        // we push the `=` keyword under, along with the remaining `value` for
        // the default, and any comments that appear there. We then close the
        // `R_PARAMETER_DEFAULT` on the way out.
        let mut used_equal = false;

        while let Some(event) = iter.peek() {
            match event {
                WalkEvent::Enter(next) => match next.syntax_kind() {
                    RSyntaxKind::EQUAL => {
                        used_equal = true;
                        self.handle_node_enter(RSyntaxKind::R_PARAMETER_DEFAULT);
                        self.walk_next(iter);
                    }
                    // Main loop handles everything else
                    _ => self.walk_next(iter),
                },
                WalkEvent::Leave(next) => {
                    if node != *next {
                        panic!("Expected next `Leave` event to be for `node`.");
                    }
                    break;
                }
            }
        }

        if used_equal {
            self.handle_node_leave(RSyntaxKind::R_PARAMETER_DEFAULT);
        }
    }

    fn handle_parameter_leave(&mut self) {
        self.handle_node_leave(RSyntaxKind::R_PARAMETER);
    }

    fn handle_integer_value_enter(&mut self, iter: &mut Preorder) {
        // Skip subtree, we don't want to separate the literal from the `L`.
        // Can't have comments between the value and the `L`.
        iter.skip_subtree();
        self.handle_value_enter(RSyntaxKind::R_INTEGER_VALUE);
    }

    fn handle_integer_value_leave(&mut self, node: tree_sitter::Node) {
        self.handle_value_leave(
            node,
            RSyntaxKind::R_INTEGER_VALUE,
            RSyntaxKind::R_INTEGER_LITERAL,
        );
    }

    fn handle_complex_value_enter(&mut self, iter: &mut Preorder) {
        // Skip subtree, we don't want to separate the literal from the `i`.
        // Can't have comments between the value and the `i`.
        iter.skip_subtree();
        self.handle_value_enter(RSyntaxKind::R_COMPLEX_VALUE);
    }

    fn handle_complex_value_leave(&mut self, node: tree_sitter::Node) {
        self.handle_value_leave(
            node,
            RSyntaxKind::R_COMPLEX_VALUE,
            RSyntaxKind::R_COMPLEX_LITERAL,
        );
    }

    fn handle_string_value_enter(&mut self, iter: &mut Preorder) {
        // Skip subtree, we currently don't separate string types.
        // Can't have comments in a string subtree.
        iter.skip_subtree();
        self.handle_value_enter(RSyntaxKind::R_STRING_VALUE);
    }

    fn handle_string_value_leave(&mut self, node: tree_sitter::Node) {
        self.handle_value_leave(
            node,
            RSyntaxKind::R_STRING_VALUE,
            RSyntaxKind::R_STRING_LITERAL,
        );
    }

    fn handle_if_statement_enter(&mut self, node: tree_sitter::Node, iter: &mut Preorder) {
        self.handle_node_enter(RSyntaxKind::R_IF_STATEMENT);

        // Seeing an `else` causes us to open an `R_ELSE_CLAUSE` node which
        // we push the `else` keyword under, along with the `alternative`,
        // and any comments that appear there. We then close the `R_ELSE_CLAUSE`
        // on the way out.
        let mut used_else = false;

        while let Some(event) = iter.peek() {
            match event {
                WalkEvent::Enter(next) => match next.syntax_kind() {
                    RSyntaxKind::IF_KW => self.walk_next(iter),
                    RSyntaxKind::L_PAREN => self.walk_next(iter),
                    RSyntaxKind::R_PAREN => self.walk_next(iter),
                    RSyntaxKind::ELSE_KW => {
                        used_else = true;
                        self.handle_node_enter(RSyntaxKind::R_ELSE_CLAUSE);
                        self.walk_next(iter);
                    }
                    RSyntaxKind::COMMENT => self.walk_next(iter),
                    // i.e. the `condition`, `consequence`, and `alternative`
                    _ => self.walk_next(iter),
                },
                WalkEvent::Leave(next) => {
                    if node != *next {
                        panic!("Expected next `Leave` event to be for `node`.");
                    }
                    break;
                }
            }
        }

        if used_else {
            self.handle_node_leave(RSyntaxKind::R_ELSE_CLAUSE);
        }
    }

    fn handle_if_statement_leave(&mut self) {
        self.handle_node_leave(RSyntaxKind::R_IF_STATEMENT);
    }

    fn handle_braced_expressions_enter(&mut self, node: tree_sitter::Node, iter: &mut Preorder) {
        self.handle_node_enter(RSyntaxKind::R_BRACED_EXPRESSIONS);

        while let Some(event) = iter.peek() {
            match event {
                WalkEvent::Enter(next) => match next.syntax_kind() {
                    RSyntaxKind::L_CURLY => {
                        self.walk_next(iter);
                        self.handle_node_enter(RSyntaxKind::R_EXPRESSION_LIST);
                    }
                    RSyntaxKind::R_CURLY => {
                        self.handle_node_leave(RSyntaxKind::R_EXPRESSION_LIST);
                        self.walk_next(iter);
                    }
                    RSyntaxKind::COMMENT => self.walk_next(iter),
                    _ => self.walk_next(iter),
                },
                WalkEvent::Leave(next) => {
                    if node != *next {
                        panic!("Expected next `Leave` event to be for `node`.");
                    }
                    break;
                }
            }
        }
    }

    fn handle_braced_expressions_leave(&mut self) {
        self.handle_node_leave(RSyntaxKind::R_BRACED_EXPRESSIONS);
    }

    fn handle_call_like_enter(
        &mut self,
        kind: RSyntaxKind,
        arguments_kind: RSyntaxKind,
        node: tree_sitter::Node,
        iter: &mut Preorder,
    ) {
        self.handle_node_enter(kind);

        while let Some(event) = iter.peek() {
            match event {
                WalkEvent::Enter(_) => match iter.peek_field_name() {
                    Some("arguments") => self.handle_arguments(arguments_kind, iter),
                    // `"function"` field and comments
                    _ => self.walk_next(iter),
                },
                WalkEvent::Leave(next) => {
                    if node != *next {
                        panic!("Expected next `Leave` event to be for `node`.");
                    }
                    break;
                }
            }
        }
    }

    fn handle_call_like_leave(&mut self, kind: RSyntaxKind) {
        self.handle_node_leave(kind);
    }

    fn handle_arguments(&mut self, kind: RSyntaxKind, iter: &mut Preorder) {
        // `Enter` event for arguments
        let event = iter.next().unwrap();
        let WalkEvent::Enter(node) = event else {
            panic!("Expected to `Enter` arguments");
        };

        self.handle_arguments_enter(kind, node, iter);

        // `Leave` event for arguments
        let event = iter.next().unwrap();
        assert_eq!(
            event,
            WalkEvent::Leave(node),
            "Expected to `Leave` arguments"
        );

        self.handle_arguments_leave(kind);
    }

    fn handle_arguments_enter(
        &mut self,
        kind: RSyntaxKind,
        node: tree_sitter::Node,
        iter: &mut Preorder,
    ) {
        let (open, close) = match kind {
            RSyntaxKind::R_CALL_ARGUMENTS => (RSyntaxKind::L_PAREN, RSyntaxKind::R_PAREN),
            RSyntaxKind::R_SUBSET_ARGUMENTS => (RSyntaxKind::L_BRACK, RSyntaxKind::R_BRACK),
            RSyntaxKind::R_SUBSET2_ARGUMENTS => (RSyntaxKind::L_BRACK2, RSyntaxKind::R_BRACK2),
            _ => unreachable!("Found unexpected kind '{kind:?}'."),
        };

        self.handle_node_enter(kind);

        let mut last_kind = RSyntaxKind::R_BOGUS;

        while let Some(event) = iter.peek() {
            match event {
                WalkEvent::Enter(next) => match next.syntax_kind() {
                    kind if kind == open => {
                        self.walk_next(iter);
                        self.handle_node_enter(RSyntaxKind::R_ARGUMENT_LIST);
                        last_kind = open;
                    }
                    kind if kind == close => {
                        self.handle_hole_before_close(last_kind);
                        self.handle_node_leave(RSyntaxKind::R_ARGUMENT_LIST);
                        self.walk_next(iter);
                        last_kind = close;
                    }
                    RSyntaxKind::COMMA => {
                        self.handle_hole_before_comma(last_kind, open);
                        self.walk_next(iter);
                        last_kind = RSyntaxKind::COMMA;
                    }
                    RSyntaxKind::R_ARGUMENT => {
                        self.walk_next(iter);
                        last_kind = RSyntaxKind::R_ARGUMENT;
                    }
                    RSyntaxKind::COMMENT => {
                        self.walk_next(iter);
                        // Not setting `last_kind` here!
                    }
                    kind => unreachable!("Found {kind:?} in arguments"),
                },
                WalkEvent::Leave(next) => {
                    if node != *next {
                        panic!("Expected next `Leave` event to be for `node`.");
                    }
                    break;
                }
            }
        }
    }

    fn handle_arguments_leave(&mut self, kind: RSyntaxKind) {
        self.handle_node_leave(kind);
    }

    fn handle_argument_enter(&mut self, node: tree_sitter::Node, iter: &mut Preorder) {
        self.handle_node_enter(RSyntaxKind::R_ARGUMENT);

        // A complication with arguments is that tree-sitter-r doesn't differentiate
        // between named and unnamed arguments at the node level. Named arguments
        // have a `"name"` field node (identifier/string/dots/dotdoti) followed
        // by an `=` sign. If we see that `"name"` field, we open the name clause
        // node and close it after we see the `=`.
        let mut seen_equal = false;

        while let Some(event) = iter.peek() {
            match event {
                WalkEvent::Enter(next) => match next.syntax_kind() {
                    RSyntaxKind::R_IDENTIFIER
                    | RSyntaxKind::R_STRING_VALUE
                    | RSyntaxKind::R_DOTS
                    | RSyntaxKind::R_DOT_DOT_I => {
                        if !seen_equal {
                            if let Some(next_field_name) = iter.peek_field_name() {
                                if next_field_name == "name" {
                                    self.handle_node_enter(RSyntaxKind::R_ARGUMENT_NAME_CLAUSE)
                                }
                            }
                        }
                        self.walk_next(iter);
                    }
                    RSyntaxKind::EQUAL => {
                        seen_equal = true;
                        self.walk_next(iter);
                        self.handle_node_leave(RSyntaxKind::R_ARGUMENT_NAME_CLAUSE);
                    }
                    // Comments, values, etc
                    _ => self.walk_next(iter),
                },
                WalkEvent::Leave(next) => {
                    if node != *next {
                        panic!("Expected next `Leave` event to be for `node`.");
                    }
                    break;
                }
            }
        }
    }

    fn handle_argument_leave(&mut self) {
        self.handle_node_leave(RSyntaxKind::R_ARGUMENT);
    }

    /// Is there a hole before this `)`, `]`, or `]]`?
    ///
    /// ```r
    /// fn(,<here>)
    ///
    /// fn(
    ///   x,
    ///   # comment
    ///   <here>
    /// )
    /// ```
    fn handle_hole_before_close(&mut self, last_kind: RSyntaxKind) {
        if last_kind == RSyntaxKind::COMMA {
            self.handle_node_enter(RSyntaxKind::R_ARGUMENT);
            self.handle_node_leave(RSyntaxKind::R_ARGUMENT);
        }
    }

    /// Is there a hole before this `,`?
    ///
    /// ```r
    /// fn(<here>,)
    ///
    /// fn(x, <here>,)
    ///
    /// fn(
    ///   x,
    ///   # comment
    ///   <here>,
    /// )
    /// ```
    fn handle_hole_before_comma(&mut self, last_kind: RSyntaxKind, open: RSyntaxKind) {
        if last_kind == RSyntaxKind::COMMA || last_kind == open {
            self.handle_node_enter(RSyntaxKind::R_ARGUMENT);
            self.handle_node_leave(RSyntaxKind::R_ARGUMENT);
        }
    }
}

struct RParse {
    events: Vec<Event<RSyntaxKind>>,
    trivia: Vec<Trivia>,
    errors: Vec<ParseError>,
}

impl RParse {
    fn new() -> Self {
        Self {
            events: Vec::new(),
            trivia: Vec::new(),
            errors: Vec::new(),
        }
    }

    fn start(&mut self, kind: RSyntaxKind) {
        self.push_event(Event::Start {
            kind,
            forward_parent: None,
        });
    }

    fn token(&mut self, kind: RSyntaxKind, end: TextSize) {
        self.push_event(Event::Token { kind, end });
    }

    fn finish(&mut self) {
        self.push_event(Event::Finish);
    }

    fn push_event(&mut self, event: Event<RSyntaxKind>) {
        self.events.push(event);
    }

    fn push_trivia(&mut self, trivia: Trivia) {
        self.trivia.push(trivia);
    }

    fn drain(self) -> (Vec<Event<RSyntaxKind>>, Vec<Trivia>, Vec<ParseError>) {
        (self.events, self.trivia, self.errors)
    }

    // TODO!: Need to handle comments too. It will be like `derive_trivia()`
    // but whitespace after the final token on a line but before a trailing
    // comment is also considered trailing trivia (I think the trick to
    // recognize is that any whitespace before a comment is considered trailing
    // until you see your first newline)

    /// Given:
    /// - A slice of `text` starting at byte `start`
    /// - Which only contains whitespace or newlines
    /// - And represents a "gap" between two tokens
    ///
    /// Derive the implied stream of trivia that exists in that gap
    ///
    /// SAFETY: `last_end <= next_start`
    /// SAFETY: `last_end` and `next_start` must define a range within `text`
    fn derive_trivia(&mut self, text: &str, mut start: TextSize, between_two_tokens: bool) {
        let mut iter = text.as_bytes().iter().peekable();
        let mut end = start;

        // - Between the start of file and the first token, all trivia is leading
        //   (it leads the first token), so we skip this.
        // - Between the last token and the end of file, all trivia is leading
        //   (it leads the EOF token that `TreeSink` adds), so we skip this.
        // - Between two tokens, all trivia is leading unless we see a newline,
        //   which this branch handles specially.
        if between_two_tokens {
            let mut trailing = false;

            // All whitespace between two tokens is leading until we hit the
            // first `\r`, `\r\n`, or `\n`, at which point the whitespace is
            // considered trailing of the last token, and the newline and
            // everything after it is considered leading of the next token.
            // A lone `\r` not attached to an `\n` should not happen in a
            // well-formed file (unless inside a string token), so we just
            // treat it as a `\r\n` line ending.
            while let Some(byte) = iter.peek() {
                if let b'\r' | b'\n' = byte {
                    // We found a newline, so all trivia up to this point is
                    // trailing to the last token. Don't advance the iterator so
                    // that this newline may be processed as leading trivia.
                    trailing = true;

                    // Break and fallthrough
                    break;
                }
                end += TextSize::from(1);
                let _ = iter.next();
            }

            if start != end {
                let range = TextRange::new(start, end);
                self.push_trivia(Trivia::new(TriviaPieceKind::Whitespace, range, trailing));
                start = end;
            }

            // Fallthrough so that our current byte can be processed as leading
            // trivia
        }

        // Now push all leading trivia
        let trailing = false;

        while let Some(byte) = iter.next() {
            end += TextSize::from(1);

            if Self::is_whitespace(*byte) {
                // Finish out stream of whitespace
                while iter.next_if(|byte| Self::is_whitespace(**byte)).is_some() {
                    end += TextSize::from(1);
                }
                let range = TextRange::new(start, end);
                self.push_trivia(Trivia::new(TriviaPieceKind::Whitespace, range, trailing));
                start = end;
                continue;
            }

            if let b'\r' = byte {
                match iter.next_if(|byte| **byte == b'\n') {
                    Some(_) => {
                        // Finish out `\r\n`
                        end += TextSize::from(1);
                        let range = TextRange::new(start, end);
                        self.push_trivia(Trivia::new(TriviaPieceKind::Newline, range, trailing));
                        start = end;
                    }
                    None => {
                        // Finish out `\r`
                        let range = TextRange::new(start, end);
                        self.push_trivia(Trivia::new(TriviaPieceKind::Newline, range, trailing));
                        start = end;
                    }
                }
                continue;
            }

            if let b'\n' = byte {
                // Finish out `\n`
                let range = TextRange::new(start, end);
                self.push_trivia(Trivia::new(TriviaPieceKind::Newline, range, trailing));
                start = end;
                continue;
            }

            unreachable!("Detected non trivia character!");
        }
    }

    fn is_whitespace(byte: u8) -> bool {
        // `WHS` maps newlines as "whitespace" but we handle that specially
        match biome_unicode_table::lookup_byte(byte) {
            Dispatch::WHS => byte != b'\r' && byte != b'\n',
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    enum Pos {
        Leading,
        Trailing,
    }

    fn trivia(text: &str) -> Vec<Trivia> {
        let (_events, trivia, _errors) = parse_text(text, RParserOptions::default());
        trivia
    }

    fn ws(start: u32, end: u32, position: Pos) -> Trivia {
        Trivia::new(
            TriviaPieceKind::Whitespace,
            TextRange::new(TextSize::from(start), TextSize::from(end)),
            matches!(position, Pos::Trailing),
        )
    }

    fn nl(start: u32, end: u32) -> Trivia {
        Trivia::new(
            TriviaPieceKind::Newline,
            TextRange::new(TextSize::from(start), TextSize::from(end)),
            false,
        )
    }

    fn cmt(start: u32, end: u32, position: Pos) -> Trivia {
        Trivia::new(
            TriviaPieceKind::SingleLineComment,
            TextRange::new(TextSize::from(start), TextSize::from(end)),
            matches!(position, Pos::Trailing),
        )
    }

    // TODO: It would be great if `biome_parser::token_source::Trivia`
    // implemented `PartialEq`, maybe we should ask for that.
    fn assert_eq_trivia(lhs: Vec<Trivia>, rhs: Vec<Trivia>) {
        assert_eq!(lhs.len(), rhs.len());

        for (i, (lhs, rhs)) in lhs.iter().zip(rhs.iter()).enumerate() {
            let message = format!("In event {i} with:\nlhs {lhs:?}\nrhs {rhs:?}");
            assert_eq!(lhs.kind(), rhs.kind(), "{message}");
            assert_eq!(lhs.text_range(), rhs.text_range(), "{message}");
            assert_eq!(lhs.trailing(), rhs.trailing(), "{message}");
        }
    }

    // TODO: It would be great if `biome_parser::token_source::Trivia`
    // implemented `PartialEq`, maybe we should ask for that.
    fn assert_eq_events(lhs: Vec<Event<RSyntaxKind>>, rhs: Vec<Event<RSyntaxKind>>) {
        assert_eq!(lhs.len(), rhs.len(), "With:\nlhs {lhs:?}\nrhs {rhs:?}");

        for (i, (lhs, rhs)) in lhs.iter().zip(rhs.iter()).enumerate() {
            let message = format!("In event {i} with:\nlhs {lhs:?}\nrhs {rhs:?}");

            match lhs {
                Event::Start {
                    kind: lhs_kind,
                    forward_parent: lhs_forward_parent,
                } => match rhs {
                    Event::Start {
                        kind: rhs_kind,
                        forward_parent: rhs_forward_parent,
                    } => {
                        assert_eq!(lhs_kind, rhs_kind, "{message}");
                        assert_eq!(lhs_forward_parent, rhs_forward_parent, "{message}");
                    }
                    Event::Finish => panic!("{message}"),
                    Event::Token { .. } => panic!("{message}"),
                },
                Event::Token {
                    kind: lhs_kind,
                    end: lhs_end,
                } => match rhs {
                    Event::Token {
                        kind: rhs_kind,
                        end: rhs_end,
                    } => {
                        assert_eq!(lhs_kind, rhs_kind, "{message}");
                        assert_eq!(lhs_end, rhs_end, "{message}");
                    }
                    Event::Start { .. } => panic!("{message}"),
                    Event::Finish => panic!("{message}"),
                },
                Event::Finish => match rhs {
                    Event::Finish => (),
                    Event::Start { .. } => panic!("{message}"),
                    Event::Token { .. } => panic!("{message}"),
                },
            }
        }
    }

    #[test]
    fn test_parse_trivia_smoke_test() {
        assert_eq_trivia(
            trivia("1 + 1"),
            vec![ws(1, 2, Pos::Leading), ws(3, 4, Pos::Leading)],
        );
    }

    #[test]
    fn test_parse_trivia_tab_test() {
        assert_eq_trivia(
            trivia("1\t+\t\n\t1"),
            vec![
                ws(1, 2, Pos::Leading),
                ws(3, 4, Pos::Trailing),
                nl(4, 5),
                ws(5, 6, Pos::Leading),
            ],
        );
    }

    #[test]
    fn test_parse_trivia_trailing_test() {
        assert_eq_trivia(
            trivia("1 + \n1"),
            vec![ws(1, 2, Pos::Leading), ws(3, 4, Pos::Trailing), nl(4, 5)],
        );
    }

    #[test]
    fn test_parse_trivia_trailing_trivia_test() {
        // Note that trivia between the last token and `EOF` is always
        // leading and will be attached to an `EOF` token by `TreeSink`.
        assert_eq_trivia(
            trivia("1  \n "),
            vec![ws(1, 3, Pos::Leading), nl(3, 4), ws(4, 5, Pos::Leading)],
        );
    }

    #[test]
    fn test_parse_trivia_trailing_crlf_test() {
        assert_eq_trivia(
            trivia("1 + \r\n1"),
            vec![ws(1, 2, Pos::Leading), ws(3, 4, Pos::Trailing), nl(4, 6)],
        );
    }

    #[test]
    fn test_parse_trivia_before_first_token() {
        assert_eq_trivia(trivia("  \n1"), vec![ws(0, 2, Pos::Leading), nl(2, 3)]);
    }

    #[test]
    fn test_parse_trivia_comment_test() {
        assert_eq_trivia(
            trivia("1 #"),
            vec![ws(1, 2, Pos::Trailing), cmt(2, 3, Pos::Trailing)],
        );
    }

    #[test]
    fn test_parse_trivia_comment_nothing_else_test() {
        assert_eq_trivia(trivia("#"), vec![cmt(0, 1, Pos::Leading)]);
    }

    #[test]
    fn test_parse_trivia_comment_end_of_document_test() {
        assert_eq_trivia(trivia("1\n#"), vec![nl(1, 2), cmt(2, 3, Pos::Leading)]);
    }

    #[test]
    fn test_parse_trivia_whitespace_between_comments_test() {
        let text = "
1 #
#
2
"
        .trim();
        assert_eq_trivia(
            trivia(text),
            vec![
                ws(1, 2, Pos::Trailing),
                cmt(2, 3, Pos::Trailing),
                nl(3, 4),
                cmt(4, 5, Pos::Leading),
                nl(5, 6),
            ],
        );
    }

    #[test]
    fn test_parse_trivia_comment_beginning_of_document_test() {
        assert_eq_trivia(trivia("#\n1"), vec![cmt(0, 1, Pos::Leading), nl(1, 2)]);
    }

    #[test]
    fn test_parse_trivia_comment_beginning_of_document_with_whitespace_test() {
        assert_eq_trivia(
            trivia(" \n \n#"),
            vec![
                ws(0, 1, Pos::Leading),
                nl(1, 2),
                ws(2, 3, Pos::Leading),
                nl(3, 4),
                cmt(4, 5, Pos::Leading),
            ],
        );
    }

    #[test]
    fn test_parse_smoke_test() {
        let (events, trivia, _errors) = parse_text("1+1", RParserOptions::default());

        let expect = vec![
            Event::Start {
                kind: RSyntaxKind::R_ROOT,
                forward_parent: None,
            },
            Event::Start {
                kind: RSyntaxKind::R_EXPRESSION_LIST,
                forward_parent: None,
            },
            Event::Start {
                kind: RSyntaxKind::R_BINARY_EXPRESSION,
                forward_parent: None,
            },
            Event::Start {
                kind: RSyntaxKind::R_DOUBLE_VALUE,
                forward_parent: None,
            },
            Event::Token {
                kind: RSyntaxKind::R_DOUBLE_LITERAL,
                end: TextSize::from(1),
            },
            Event::Finish,
            Event::Token {
                kind: RSyntaxKind::PLUS,
                end: TextSize::from(2),
            },
            Event::Start {
                kind: RSyntaxKind::R_DOUBLE_VALUE,
                forward_parent: None,
            },
            Event::Token {
                kind: RSyntaxKind::R_DOUBLE_LITERAL,
                end: TextSize::from(3),
            },
            Event::Finish,
            Event::Finish,
            Event::Finish,
            Event::Finish,
        ];

        assert_eq_events(events, expect);
        assert!(trivia.is_empty());
    }

    #[test]
    fn test_parse_function_definition() {
        let (events, trivia, _errors) = parse_text("function() 1", RParserOptions::default());

        let expect = vec![
            Event::Start {
                kind: RSyntaxKind::R_ROOT,
                forward_parent: None,
            },
            Event::Start {
                kind: RSyntaxKind::R_EXPRESSION_LIST,
                forward_parent: None,
            },
            Event::Start {
                kind: RSyntaxKind::R_FUNCTION_DEFINITION,
                forward_parent: None,
            },
            Event::Token {
                kind: RSyntaxKind::FUNCTION_KW,
                end: TextSize::from(8),
            },
            Event::Start {
                kind: RSyntaxKind::R_PARAMETERS,
                forward_parent: None,
            },
            Event::Token {
                kind: RSyntaxKind::L_PAREN,
                end: TextSize::from(9),
            },
            Event::Start {
                kind: RSyntaxKind::R_PARAMETER_LIST,
                forward_parent: None,
            },
            Event::Finish, // R_PARAMETER_LIST
            Event::Token {
                kind: RSyntaxKind::R_PAREN,
                end: TextSize::from(10),
            },
            Event::Finish, // R_PARAMETERS
            Event::Start {
                kind: RSyntaxKind::R_DOUBLE_VALUE,
                forward_parent: None,
            },
            Event::Token {
                kind: RSyntaxKind::R_DOUBLE_LITERAL,
                end: TextSize::from(12),
            },
            Event::Finish, // R_DOUBLE_VALUE
            Event::Finish, // R_FUNCTION_DEFINITION
            Event::Finish, // R_EXPRESSION_LIST
            Event::Finish, // R_ROOT
        ];
        assert_eq_events(events, expect);

        let expect = vec![ws(10, 11, Pos::Leading)];
        assert_eq_trivia(trivia, expect);
    }

    #[test]
    fn test_parse_call() {
        let (events, trivia, _errors) = parse_text("fn()", RParserOptions::default());

        let expect = vec![
            Event::Start {
                kind: RSyntaxKind::R_ROOT,
                forward_parent: None,
            },
            Event::Start {
                kind: RSyntaxKind::R_EXPRESSION_LIST,
                forward_parent: None,
            },
            Event::Start {
                kind: RSyntaxKind::R_CALL,
                forward_parent: None,
            },
            Event::Start {
                kind: RSyntaxKind::R_IDENTIFIER,
                forward_parent: None,
            },
            Event::Token {
                kind: RSyntaxKind::IDENT,
                end: TextSize::from(2),
            },
            Event::Finish, // R_IDENTIFIER
            Event::Start {
                kind: RSyntaxKind::R_CALL_ARGUMENTS,
                forward_parent: None,
            },
            Event::Token {
                kind: RSyntaxKind::L_PAREN,
                end: TextSize::from(3),
            },
            Event::Start {
                kind: RSyntaxKind::R_ARGUMENT_LIST,
                forward_parent: None,
            },
            Event::Finish, // R_ARGUMENT_LIST
            Event::Token {
                kind: RSyntaxKind::R_PAREN,
                end: TextSize::from(4),
            },
            Event::Finish, // R_CALL_ARGUMENTS
            Event::Finish, // R_CALL
            Event::Finish, // R_EXPRESSION_LIST
            Event::Finish, // R_ROOT
        ];
        assert_eq_events(events, expect);

        let expect = vec![];
        assert_eq_trivia(trivia, expect);
    }
}
