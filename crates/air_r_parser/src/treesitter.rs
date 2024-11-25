use air_r_syntax::RSyntaxKind;
use tree_sitter::{Node, TreeCursor};

/// `WalkEvent` describes tree walking process.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum WalkEvent<T> {
    /// Fired before traversing the node.
    Enter(T),
    /// Fired after the node is traversed.
    Leave(T),
}

impl<T> WalkEvent<T> {
    pub fn map<F: FnOnce(T) -> U, U>(self, f: F) -> WalkEvent<U> {
        match self {
            WalkEvent::Enter(it) => WalkEvent::Enter(f(it)),
            WalkEvent::Leave(it) => WalkEvent::Leave(f(it)),
        }
    }
}

// TODO: Assign iterator to rowan
pub struct Preorder<'tree> {
    cursor: TreeCursor<'tree>,
    next: Option<WalkEvent<Node<'tree>>>,
}

impl<'tree> Preorder<'tree> {
    fn new(node: Node) -> Preorder {
        let cursor = node.walk();
        let next = Some(WalkEvent::Enter(node));
        Preorder { cursor, next }
    }

    /// Peek at the upcoming node without advancing to it
    ///
    /// NOTE: Effectively free, since we track this anyways
    pub fn peek(&self) -> &Option<WalkEvent<Node<'tree>>> {
        &self.next
    }

    /// Peek at the upcoming node's field name
    ///
    /// NOTE: The `self.cursor` is kept in sync with `self.next` and is always
    /// pointing to the upcoming node, so using the cursor to extract the field
    /// name is always valid.
    pub fn peek_field_name(&self) -> Option<&'static str> {
        self.cursor.field_name()
    }

    pub fn skip_subtree(&mut self) {
        let next = self.next.take();
        self.next = next.as_ref().and_then(|next| {
            Some(match next {
                WalkEvent::Enter(_first_child) => match self.cursor.goto_parent() {
                    true => WalkEvent::Leave(self.cursor.node()),
                    false => return None,
                },
                WalkEvent::Leave(parent) => WalkEvent::Leave(*parent),
            })
        });
    }
}

impl<'tree> Iterator for Preorder<'tree> {
    type Item = WalkEvent<Node<'tree>>;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.next.take();
        self.next = next.as_ref().and_then(|next| {
            Some(match next {
                WalkEvent::Enter(node) => match self.cursor.goto_first_child() {
                    true => WalkEvent::Enter(self.cursor.node()),
                    false => WalkEvent::Leave(*node),
                },
                WalkEvent::Leave(node) => match self.cursor.goto_next_sibling() {
                    true => WalkEvent::Enter(self.cursor.node()),
                    false => match self.cursor.goto_parent() {
                        true => WalkEvent::Leave(self.cursor.node()),
                        false => return None,
                    },
                },
            })
        });
        next
    }
}

fn node_syntax_kind(x: &Node) -> RSyntaxKind {
    match x.kind() {
        "program" => RSyntaxKind::R_ROOT,
        "unary_operator" => RSyntaxKind::R_UNARY_EXPRESSION,
        "binary_operator" => RSyntaxKind::R_BINARY_EXPRESSION,
        "extract_operator" => RSyntaxKind::R_EXTRACT_EXPRESSION,
        "namespace_operator" => RSyntaxKind::R_NAMESPACE_EXPRESSION,
        "function_definition" => RSyntaxKind::R_FUNCTION_DEFINITION,
        "parameters" => RSyntaxKind::R_PARAMETERS,
        "parameter" => RSyntaxKind::R_PARAMETER,
        "if_statement" => RSyntaxKind::R_IF_STATEMENT,
        "for_statement" => RSyntaxKind::R_FOR_STATEMENT,
        "while_statement" => RSyntaxKind::R_WHILE_STATEMENT,
        "repeat_statement" => RSyntaxKind::R_REPEAT_STATEMENT,
        "braced_expression" => RSyntaxKind::R_BRACED_EXPRESSIONS,
        "parenthesized_expression" => RSyntaxKind::R_PARENTHESIZED_EXPRESSION,
        "call" => RSyntaxKind::R_CALL,
        "subset" => RSyntaxKind::R_SUBSET,
        "subset2" => RSyntaxKind::R_SUBSET2,
        "arguments" => arguments_syntax_kind(x),
        "argument" => RSyntaxKind::R_ARGUMENT,
        "identifier" => RSyntaxKind::R_IDENTIFIER,
        "integer" => RSyntaxKind::R_INTEGER_VALUE,
        "float" => RSyntaxKind::R_DOUBLE_VALUE,
        "complex" => RSyntaxKind::R_COMPLEX_VALUE,
        "string" => RSyntaxKind::R_STRING_VALUE,
        "return" => RSyntaxKind::R_RETURN_EXPRESSION,
        "next" => RSyntaxKind::R_NEXT_EXPRESSION,
        "break" => RSyntaxKind::R_BREAK_EXPRESSION,
        "true" => RSyntaxKind::R_TRUE_EXPRESSION,
        "false" => RSyntaxKind::R_FALSE_EXPRESSION,
        "null" => RSyntaxKind::R_NULL_EXPRESSION,
        "inf" => RSyntaxKind::R_INF_EXPRESSION,
        "nan" => RSyntaxKind::R_NAN_EXPRESSION,
        "na" => RSyntaxKind::R_NA_EXPRESSION,
        "NA" => RSyntaxKind::NA_LOGICAL_KW,
        "NA_integer_" => RSyntaxKind::NA_INTEGER_KW,
        "NA_real_" => RSyntaxKind::NA_DOUBLE_KW,
        "NA_complex_" => RSyntaxKind::NA_COMPLEX_KW,
        "NA_character_" => RSyntaxKind::NA_CHARACTER_KW,
        "{" => RSyntaxKind::L_CURLY,
        "}" => RSyntaxKind::R_CURLY,
        "[" => RSyntaxKind::L_BRACK,
        "]" => RSyntaxKind::R_BRACK,
        "[[" => RSyntaxKind::L_BRACK2,
        "]]" => RSyntaxKind::R_BRACK2,
        "(" => RSyntaxKind::L_PAREN,
        ")" => RSyntaxKind::R_PAREN,
        "?" => RSyntaxKind::WAT,
        "~" => RSyntaxKind::TILDE,
        "<-" => RSyntaxKind::ASSIGN,
        "<<-" => RSyntaxKind::SUPER_ASSIGN,
        ":=" => RSyntaxKind::WALRUS,
        "->" => RSyntaxKind::ASSIGN_RIGHT,
        "->>" => RSyntaxKind::SUPER_ASSIGN_RIGHT,
        "=" => RSyntaxKind::EQUAL,
        "|" => RSyntaxKind::OR,
        "&" => RSyntaxKind::AND,
        "||" => RSyntaxKind::OR2,
        "&&" => RSyntaxKind::AND2,
        "<" => RSyntaxKind::LESS_THAN,
        "<=" => RSyntaxKind::LESS_THAN_OR_EQUAL_TO,
        ">" => RSyntaxKind::GREATER_THAN,
        ">=" => RSyntaxKind::GREATER_THAN_OR_EQUAL_TO,
        "==" => RSyntaxKind::EQUAL2,
        "!=" => RSyntaxKind::NOT_EQUAL,
        "+" => RSyntaxKind::PLUS,
        "-" => RSyntaxKind::MINUS,
        "*" => RSyntaxKind::MULTIPLY,
        "/" => RSyntaxKind::DIVIDE,
        "^" => RSyntaxKind::EXPONENTIATE,
        "**" => RSyntaxKind::EXPONENTIATE2,
        "|>" => RSyntaxKind::PIPE,
        "special" => RSyntaxKind::SPECIAL,
        ":" => RSyntaxKind::COLON,
        "::" => RSyntaxKind::COLON2,
        ":::" => RSyntaxKind::COLON3,
        "$" => RSyntaxKind::DOLLAR,
        "@" => RSyntaxKind::AT,
        "!" => RSyntaxKind::BANG,
        "function" => RSyntaxKind::FUNCTION_KW,
        "\\" => RSyntaxKind::BACKSLASH,
        "if" => RSyntaxKind::IF_KW,
        "else" => RSyntaxKind::ELSE_KW,
        "for" => RSyntaxKind::FOR_KW,
        "in" => RSyntaxKind::IN_KW,
        "while" => RSyntaxKind::WHILE_KW,
        "repeat" => RSyntaxKind::REPEAT_KW,
        "comma" => RSyntaxKind::COMMA,
        "dots" => RSyntaxKind::R_DOTS,
        "dot_dot_i" => RSyntaxKind::R_DOT_DOT_I,
        "comment" => RSyntaxKind::COMMENT,
        kind => unreachable!("Not implemented: '{kind}'."),
    }
}

// Disambiguate the 3 types of argument groups
fn arguments_syntax_kind(x: &Node) -> RSyntaxKind {
    let open = x.child_by_field_name("open").unwrap();

    match open.kind() {
        "(" => RSyntaxKind::R_CALL_ARGUMENTS,
        "[" => RSyntaxKind::R_SUBSET_ARGUMENTS,
        "[[" => RSyntaxKind::R_SUBSET2_ARGUMENTS,
        _ => unreachable!("Unknown arguments `open` token: {}", open.kind()),
    }
}

pub trait NodeTypeExt: Sized {
    fn syntax_kind(&self) -> RSyntaxKind;
    fn preorder(&self) -> Preorder;
}

impl NodeTypeExt for Node<'_> {
    fn syntax_kind(&self) -> RSyntaxKind {
        node_syntax_kind(self)
    }

    fn preorder(&self) -> Preorder {
        Preorder::new(*self)
    }
}
