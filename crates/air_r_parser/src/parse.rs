use air_r_syntax::RSyntaxKind;
use biome_parser::event::Event;
use biome_parser::prelude::ParseDiagnostic;
use biome_parser::prelude::Trivia;
use biome_parser::AnyParse;
use biome_rowan::NodeCache;
use biome_rowan::TextRange;
use biome_rowan::TextSize;
use biome_rowan::TriviaPieceKind;
use biome_unicode_table::Dispatch;
use tree_sitter::Tree;

use crate::treesitter::NodeTypeExt;
use crate::treesitter::Preorder;
use crate::treesitter::WalkEvent;
use crate::RLosslessTreeSink;
use crate::RParserOptions;

// TODO(r): These should really return an intermediate `Parse` type which
// can `.into()` an `AnyParse`, see `biome_js_parser`'s `Parse` type
pub fn parse(text: &str, options: RParserOptions) -> AnyParse {
    let mut cache = NodeCache::default();
    parse_r_with_cache(text, options, &mut cache)
}

pub fn parse_r_with_cache(text: &str, options: RParserOptions, cache: &mut NodeCache) -> AnyParse {
    tracing::debug_span!("parse").in_scope(move || {
        let (events, tokens, errors) = parse_text(text, options);
        let mut tree_sink = RLosslessTreeSink::with_cache(text, &tokens, cache);
        biome_parser::event::process(&mut tree_sink, events, errors);
        let (green, parse_errors) = tree_sink.finish();
        AnyParse::new(green.as_send().unwrap(), parse_errors)
    })
}

pub fn parse_text(
    text: &str,
    _options: RParserOptions,
) -> (Vec<Event<RSyntaxKind>>, Vec<Trivia>, Vec<ParseDiagnostic>) {
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

fn parse_failure() -> (Vec<Event<RSyntaxKind>>, Vec<Trivia>, Vec<ParseDiagnostic>) {
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

    // Generate a single diagnostic
    let span: Option<TextRange> = None;
    let error = ParseDiagnostic::new("Tree-sitter failed", span);
    let errors = vec![error];

    (events, trivia, errors)
}

fn parse_tree(
    ast: Tree,
    text: &str,
) -> (Vec<Event<RSyntaxKind>>, Vec<Trivia>, Vec<ParseDiagnostic>) {
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

    fn handle_enter(&mut self, node: tree_sitter::Node, kind: RSyntaxKind, iter: &mut Preorder) {
        match kind {
            RSyntaxKind::R_ROOT => self.handle_root_enter(),

            RSyntaxKind::R_UNARY_EXPRESSION
            | RSyntaxKind::R_BINARY_EXPRESSION
            | RSyntaxKind::R_FUNCTION_DEFINITION
            | RSyntaxKind::R_FOR_STATEMENT
            | RSyntaxKind::R_WHILE_STATEMENT
            | RSyntaxKind::R_REPEAT_STATEMENT
            | RSyntaxKind::R_CALL
            | RSyntaxKind::R_UNNAMED_ARGUMENT
            | RSyntaxKind::R_PARENTHESIZED_EXPRESSION => self.handle_node_enter(kind),

            RSyntaxKind::R_PARAMETERS => self.handle_parameters_enter(node, iter),
            RSyntaxKind::R_DOTS_PARAMETER => self.handle_dots_parameter_enter(iter),
            RSyntaxKind::R_IDENTIFIER_PARAMETER => self.handle_identifier_parameter_enter(iter),
            RSyntaxKind::R_DEFAULT_PARAMETER => self.handle_default_parameter_enter(node, iter),
            RSyntaxKind::R_IF_STATEMENT => self.handle_if_statement_enter(node, iter),
            RSyntaxKind::R_CALL_ARGUMENTS => self.handle_call_arguments_enter(node, iter),
            RSyntaxKind::R_NAMED_ARGUMENT => self.handle_named_argument_enter(node, iter),
            RSyntaxKind::R_DOTS_ARGUMENT => self.handle_dots_argument_enter(node, iter),
            RSyntaxKind::R_BRACED_EXPRESSIONS => self.handle_braced_expressions_enter(node, iter),

            // Literals / wrapped keywords
            RSyntaxKind::R_DOUBLE_VALUE
            | RSyntaxKind::R_LOGICAL_VALUE
            | RSyntaxKind::R_NULL_VALUE
            | RSyntaxKind::R_IDENTIFIER
            | RSyntaxKind::R_DOT_DOT_I
            | RSyntaxKind::R_RETURN_EXPRESSION
            | RSyntaxKind::R_NEXT_EXPRESSION
            | RSyntaxKind::R_BREAK_EXPRESSION
            | RSyntaxKind::R_TRUE_EXPRESSION
            | RSyntaxKind::R_FALSE_EXPRESSION
            | RSyntaxKind::R_NULL_EXPRESSION
            | RSyntaxKind::R_INF_EXPRESSION
            | RSyntaxKind::R_NAN_EXPRESSION => self.handle_value_enter(kind),

            RSyntaxKind::R_NA_EXPRESSION => self.handle_na_value_enter(iter),
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
            | RSyntaxKind::L_PAREN
            | RSyntaxKind::R_PAREN
            | RSyntaxKind::L_CURLY
            | RSyntaxKind::R_CURLY => (),

            // Comments
            RSyntaxKind::COMMENT => self.handle_comment_enter(),

            // Unreachable
            RSyntaxKind::R_DOTS
            | RSyntaxKind::R_ELSE_CLAUSE
            | RSyntaxKind::R_HOLE_ARGUMENT
            | RSyntaxKind::R_PARAMETER_LIST
            | RSyntaxKind::R_ARGUMENT_LIST
            | RSyntaxKind::R_EXPRESSION_LIST
            | RSyntaxKind::EOF
            | RSyntaxKind::UNICODE_BOM
            | RSyntaxKind::L_BRACK
            | RSyntaxKind::R_BRACK
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
            | RSyntaxKind::NA_LOGICAL_KW
            | RSyntaxKind::NA_INTEGER_KW
            | RSyntaxKind::NA_DOUBLE_KW
            | RSyntaxKind::NA_COMPLEX_KW
            | RSyntaxKind::NA_CHARACTER_KW
            | RSyntaxKind::R_BOGUS
            | RSyntaxKind::R_BOGUS_VALUE
            | RSyntaxKind::R_BOGUS_EXPRESSION
            | RSyntaxKind::R_BOGUS_PARAMETER
            | RSyntaxKind::R_BOGUS_ARGUMENT
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
            | RSyntaxKind::R_UNNAMED_ARGUMENT
            | RSyntaxKind::R_PARENTHESIZED_EXPRESSION
            | RSyntaxKind::R_CALL => self.handle_node_leave(),

            RSyntaxKind::R_PARAMETERS => self.handle_parameters_leave(),
            RSyntaxKind::R_DOTS_PARAMETER => self.handle_dots_parameter_leave(node),
            RSyntaxKind::R_IDENTIFIER_PARAMETER => self.handle_identifier_parameter_leave(node),
            RSyntaxKind::R_DEFAULT_PARAMETER => self.handle_default_parameter_leave(),
            RSyntaxKind::R_IF_STATEMENT => self.handle_if_statement_leave(),
            RSyntaxKind::R_CALL_ARGUMENTS => self.handle_call_arguments_leave(),
            RSyntaxKind::R_NAMED_ARGUMENT => self.handle_named_argument_leave(),
            RSyntaxKind::R_DOTS_ARGUMENT => self.handle_dots_argument_leave(),
            RSyntaxKind::R_BRACED_EXPRESSIONS => self.handle_braced_expressions_leave(),

            // Literals / wrapped keywords
            RSyntaxKind::R_DOUBLE_VALUE => {
                self.handle_value_leave(node, RSyntaxKind::R_DOUBLE_LITERAL)
            }
            RSyntaxKind::R_IDENTIFIER => self.handle_value_leave(node, RSyntaxKind::IDENT),
            RSyntaxKind::R_DOT_DOT_I => self.handle_value_leave(node, RSyntaxKind::DOTDOTI),
            RSyntaxKind::R_RETURN_EXPRESSION => {
                self.handle_value_leave(node, RSyntaxKind::RETURN_KW)
            }
            RSyntaxKind::R_NEXT_EXPRESSION => self.handle_value_leave(node, RSyntaxKind::NEXT_KW),
            RSyntaxKind::R_BREAK_EXPRESSION => self.handle_value_leave(node, RSyntaxKind::BREAK_KW),
            RSyntaxKind::R_TRUE_EXPRESSION => self.handle_value_leave(node, RSyntaxKind::TRUE_KW),
            RSyntaxKind::R_FALSE_EXPRESSION => self.handle_value_leave(node, RSyntaxKind::FALSE_KW),
            RSyntaxKind::R_NULL_EXPRESSION => self.handle_value_leave(node, RSyntaxKind::NULL_KW),
            RSyntaxKind::R_INF_EXPRESSION => self.handle_value_leave(node, RSyntaxKind::INF_KW),
            RSyntaxKind::R_NAN_EXPRESSION => self.handle_value_leave(node, RSyntaxKind::NAN_KW),
            RSyntaxKind::R_NA_EXPRESSION => self.handle_na_value_leave(node),
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
            | RSyntaxKind::L_PAREN
            | RSyntaxKind::R_PAREN
            | RSyntaxKind::L_CURLY
            | RSyntaxKind::R_CURLY => self.handle_token(node, kind),

            // Comments
            RSyntaxKind::COMMENT => self.handle_comment_leave(node),

            // Unreachable directly
            RSyntaxKind::R_DOTS
            | RSyntaxKind::R_ELSE_CLAUSE
            | RSyntaxKind::R_HOLE_ARGUMENT
            | RSyntaxKind::R_PARAMETER_LIST
            | RSyntaxKind::R_ARGUMENT_LIST
            | RSyntaxKind::R_EXPRESSION_LIST
            | RSyntaxKind::EOF
            | RSyntaxKind::UNICODE_BOM
            | RSyntaxKind::L_BRACK
            | RSyntaxKind::R_BRACK
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
            | RSyntaxKind::NA_LOGICAL_KW
            | RSyntaxKind::NA_INTEGER_KW
            | RSyntaxKind::NA_DOUBLE_KW
            | RSyntaxKind::NA_COMPLEX_KW
            | RSyntaxKind::NA_CHARACTER_KW
            | RSyntaxKind::R_BOGUS
            | RSyntaxKind::R_BOGUS_VALUE
            | RSyntaxKind::R_BOGUS_EXPRESSION
            | RSyntaxKind::R_BOGUS_PARAMETER
            | RSyntaxKind::R_BOGUS_ARGUMENT
            | RSyntaxKind::TOMBSTONE
            | RSyntaxKind::__LAST => unreachable!("{kind:?}"),
        }
    }

    fn handle_node_enter(&mut self, kind: RSyntaxKind) {
        self.parse.start(kind);
    }

    fn handle_node_leave(&mut self) {
        self.parse.finish();
    }

    fn handle_value_enter(&mut self, kind: RSyntaxKind) {
        self.handle_node_enter(kind);
    }

    fn handle_value_leave(&mut self, node: tree_sitter::Node, literal_kind: RSyntaxKind) {
        // Push the token for the literal
        self.handle_token(node, literal_kind);

        // Then close the node
        self.handle_node_leave();
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
        self.handle_node_leave();

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

        self.handle_node_leave();
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
        // We handle all children directly
        iter.skip_subtree();

        self.handle_node_enter(RSyntaxKind::R_PARAMETERS);

        let mut cursor = node.walk();

        // Regardless of whether or not we see an `R_PARAMETER`, we have to
        // open and close the required `R_PARAMETER_LIST`
        for child in node.children(&mut cursor) {
            let mut child_iter = child.preorder();

            match child.syntax_kind() {
                RSyntaxKind::L_PAREN => {
                    self.walk(&mut child_iter);
                    self.handle_node_enter(RSyntaxKind::R_PARAMETER_LIST);
                }
                RSyntaxKind::R_PAREN => {
                    self.handle_node_leave();
                    self.walk(&mut child_iter);
                }
                RSyntaxKind::R_DOTS_PARAMETER => self.walk(&mut child_iter),
                RSyntaxKind::R_IDENTIFIER_PARAMETER => self.walk(&mut child_iter),
                RSyntaxKind::R_DEFAULT_PARAMETER => self.walk(&mut child_iter),
                RSyntaxKind::COMMA => self.walk(&mut child_iter),
                RSyntaxKind::COMMENT => self.walk(&mut child_iter),
                kind => unreachable!("{kind:?}"),
            }
        }
    }

    fn handle_parameters_leave(&mut self) {
        self.handle_node_leave();
    }

    fn handle_dots_parameter_enter(&mut self, iter: &mut Preorder) {
        // Stop at TS `"parameter"`, don't recurse into single `"dots"` child,
        // we know what this is.
        iter.skip_subtree();

        self.handle_node_enter(RSyntaxKind::R_DOTS_PARAMETER);
    }

    fn handle_dots_parameter_leave(&mut self, node: tree_sitter::Node) {
        self.handle_token(node, RSyntaxKind::DOTS);

        self.handle_node_leave();
    }

    fn handle_identifier_parameter_enter(&mut self, iter: &mut Preorder) {
        // Stop at TS `"parameter"`, don't recurse into single `"identifier"` child,
        // we know what this is.
        iter.skip_subtree();

        self.handle_node_enter(RSyntaxKind::R_IDENTIFIER_PARAMETER);
    }

    fn handle_identifier_parameter_leave(&mut self, node: tree_sitter::Node) {
        self.handle_token(node, RSyntaxKind::IDENT);

        self.handle_node_leave();
    }

    fn handle_default_parameter_enter(&mut self, node: tree_sitter::Node, iter: &mut Preorder) {
        // Skip subtree, we will handle it
        iter.skip_subtree();

        self.handle_node_enter(RSyntaxKind::R_DEFAULT_PARAMETER);

        let mut cursor = node.walk();

        for child in node.children(&mut cursor) {
            match child.syntax_kind() {
                RSyntaxKind::R_IDENTIFIER => {
                    // Push a simple `IDENT` instead
                    self.handle_token(child, RSyntaxKind::IDENT);
                }
                _ => {
                    // `=`, and RHS of default parameter (i.e. any R expression)
                    // are handled in the main loop
                    self.walk(&mut child.preorder())
                }
            }
        }
    }

    fn handle_default_parameter_leave(&mut self) {
        self.handle_node_leave();
    }

    fn handle_na_value_enter(&mut self, iter: &mut Preorder) {
        // Skip subtree, we don't want to enter the 1 required child that
        // contains the anonymous token for the na variant. It's handled on
        // the way out.
        iter.skip_subtree();

        self.handle_value_enter(RSyntaxKind::R_NA_EXPRESSION);
    }

    fn handle_na_value_leave(&mut self, node: tree_sitter::Node) {
        // Safety: `na` nodes have exactly 1 required child, and the `kind`
        // tells you what variant of missing it is. Can't have comments inside
        // here.
        let node = node.child(0).unwrap();

        let kind = match node.kind() {
            "NA" => RSyntaxKind::NA_LOGICAL_KW,
            "NA_integer_" => RSyntaxKind::NA_INTEGER_KW,
            "NA_real_" => RSyntaxKind::NA_DOUBLE_KW,
            "NA_complex_" => RSyntaxKind::NA_COMPLEX_KW,
            "NA_character_" => RSyntaxKind::NA_CHARACTER_KW,
            kind => unreachable!("Unexpected kind '{kind}'"),
        };

        self.handle_value_leave(node, kind);
    }

    fn handle_integer_value_enter(&mut self, iter: &mut Preorder) {
        // Skip subtree, we don't want to separate the literal from the `L`.
        // Can't have comments between the value and the `L`.
        iter.skip_subtree();
        self.handle_value_enter(RSyntaxKind::R_INTEGER_VALUE);
    }

    fn handle_integer_value_leave(&mut self, node: tree_sitter::Node) {
        self.handle_value_leave(node, RSyntaxKind::R_INTEGER_LITERAL);
    }

    fn handle_complex_value_enter(&mut self, iter: &mut Preorder) {
        // Skip subtree, we don't want to separate the literal from the `i`.
        // Can't have comments between the value and the `i`.
        iter.skip_subtree();
        self.handle_value_enter(RSyntaxKind::R_COMPLEX_VALUE);
    }

    fn handle_complex_value_leave(&mut self, node: tree_sitter::Node) {
        self.handle_value_leave(node, RSyntaxKind::R_COMPLEX_LITERAL);
    }

    fn handle_string_value_enter(&mut self, iter: &mut Preorder) {
        // Skip subtree, we currently don't separate string types.
        // Can't have comments in a string subtree.
        iter.skip_subtree();
        self.handle_value_enter(RSyntaxKind::R_STRING_VALUE);
    }

    fn handle_string_value_leave(&mut self, node: tree_sitter::Node) {
        self.handle_value_leave(node, RSyntaxKind::R_STRING_LITERAL);
    }

    fn handle_if_statement_enter(&mut self, node: tree_sitter::Node, iter: &mut Preorder) {
        // We handle all children directly
        iter.skip_subtree();

        self.handle_node_enter(RSyntaxKind::R_IF_STATEMENT);

        let mut used_else = false;
        let mut cursor = node.walk();

        // Seeing an `else` causes us to open an `R_ELSE_CLAUSE` node which
        // we push the `else` keyword under, along with the `alternative`,
        // and any comments that appear there. We then close the `R_ELSE_CLAUSE`
        // on the way out.
        for child in node.children(&mut cursor) {
            let mut child_iter = child.preorder();

            match child.syntax_kind() {
                RSyntaxKind::IF_KW => self.walk(&mut child_iter),
                RSyntaxKind::L_PAREN => self.walk(&mut child_iter),
                RSyntaxKind::R_PAREN => self.walk(&mut child_iter),
                RSyntaxKind::ELSE_KW => {
                    used_else = true;
                    self.handle_node_enter(RSyntaxKind::R_ELSE_CLAUSE);
                    self.walk(&mut child_iter);
                }
                RSyntaxKind::COMMENT => self.walk(&mut child_iter),
                // i.e. the `condition`, `consequence`, and `alternative`
                _ => self.walk(&mut child_iter),
            }
        }

        if used_else {
            // Leaving `R_ELSE_CLAUSE`
            self.handle_node_leave();
        }
    }

    fn handle_if_statement_leave(&mut self) {
        self.handle_node_leave();
    }

    fn handle_braced_expressions_enter(&mut self, node: tree_sitter::Node, iter: &mut Preorder) {
        // We handle all children directly
        iter.skip_subtree();

        self.handle_node_enter(RSyntaxKind::R_BRACED_EXPRESSIONS);

        let mut cursor = node.walk();

        for child in node.children(&mut cursor) {
            let mut child_iter = child.preorder();

            match child.syntax_kind() {
                RSyntaxKind::L_CURLY => {
                    self.walk(&mut child_iter);
                    self.handle_node_enter(RSyntaxKind::R_EXPRESSION_LIST);
                }
                RSyntaxKind::R_CURLY => {
                    self.handle_node_leave();
                    self.walk(&mut child_iter);
                }
                RSyntaxKind::COMMENT => self.walk(&mut child_iter),
                _ => self.walk(&mut child_iter),
            }
        }
    }

    fn handle_braced_expressions_leave(&mut self) {
        self.handle_node_leave();
    }

    fn handle_call_arguments_enter(&mut self, node: tree_sitter::Node, iter: &mut Preorder) {
        // We handle all children directly
        iter.skip_subtree();

        self.handle_node_enter(RSyntaxKind::R_CALL_ARGUMENTS);

        let mut cursor = node.walk();

        for child in node.children(&mut cursor) {
            let mut child_iter = child.preorder();

            match child.syntax_kind() {
                RSyntaxKind::L_PAREN => {
                    self.walk(&mut child_iter);
                    self.handle_node_enter(RSyntaxKind::R_ARGUMENT_LIST);
                }
                RSyntaxKind::R_PAREN => {
                    self.handle_hole_before_r_paren(child);
                    // Leave `R_ARGUMENT_LIST`
                    self.handle_node_leave();
                    self.walk(&mut child_iter);
                }
                RSyntaxKind::COMMA => {
                    self.handle_hole_before_comma(child);
                    self.walk(&mut child_iter);
                }
                RSyntaxKind::R_DOTS_ARGUMENT => self.walk(&mut child_iter),
                RSyntaxKind::R_NAMED_ARGUMENT => self.walk(&mut child_iter),
                RSyntaxKind::R_UNNAMED_ARGUMENT => self.walk(&mut child_iter),
                RSyntaxKind::COMMENT => self.walk(&mut child_iter),
                kind => unreachable!("Found {kind:?} in arguments"),
            }
        }
    }

    fn handle_call_arguments_leave(&mut self) {
        self.handle_node_leave();
    }

    /// Is there a hole before this `)`?
    ///
    /// Ignores comments
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
    fn handle_hole_before_r_paren(&mut self, mut node: tree_sitter::Node) {
        while let Some(previous) = node.prev_sibling() {
            match previous.syntax_kind() {
                RSyntaxKind::COMMENT => (),
                RSyntaxKind::COMMA => {
                    self.handle_node_enter(RSyntaxKind::R_HOLE_ARGUMENT);
                    self.handle_node_leave();
                    break;
                }
                _ => break,
            }
            node = previous;
        }
    }

    /// Is there a hole before this `,`?
    ///
    /// Ignores comments
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
    fn handle_hole_before_comma(&mut self, mut node: tree_sitter::Node) {
        while let Some(previous) = node.prev_sibling() {
            match previous.syntax_kind() {
                RSyntaxKind::COMMENT => (),
                RSyntaxKind::COMMA | RSyntaxKind::L_PAREN => {
                    self.handle_node_enter(RSyntaxKind::R_HOLE_ARGUMENT);
                    self.handle_node_leave();
                    break;
                }
                _ => break,
            }
            node = previous;
        }
    }

    fn handle_dots_argument_enter(&mut self, node: tree_sitter::Node, iter: &mut Preorder) {
        iter.skip_subtree();
        self.handle_node_enter(RSyntaxKind::R_DOTS_ARGUMENT);
        self.handle_token(node, RSyntaxKind::DOTS);
    }

    fn handle_dots_argument_leave(&mut self) {
        self.handle_node_leave();
    }

    fn handle_named_argument_enter(&mut self, node: tree_sitter::Node, iter: &mut Preorder) {
        iter.skip_subtree();

        self.handle_node_enter(RSyntaxKind::R_NAMED_ARGUMENT);

        let mut cursor = node.walk();

        for child in node.children(&mut cursor) {
            let mut child_iter = child.preorder();

            match child.syntax_kind() {
                RSyntaxKind::DOTS => {
                    // Promote to `R_DOTS` node
                    self.handle_node_enter(RSyntaxKind::R_DOTS);
                    self.handle_token(child, RSyntaxKind::DOTS);
                    self.handle_node_leave();
                }
                _ => self.walk(&mut child_iter),
            }
        }
    }

    fn handle_named_argument_leave(&mut self) {
        self.handle_node_leave();
    }
}

struct RParse {
    events: Vec<Event<RSyntaxKind>>,
    trivia: Vec<Trivia>,
    errors: Vec<ParseDiagnostic>,
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

    fn drain(self) -> (Vec<Event<RSyntaxKind>>, Vec<Trivia>, Vec<ParseDiagnostic>) {
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
