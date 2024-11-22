use air_r_syntax::RSyntaxKind;
use tree_sitter::{Node, TreeCursor};

#[derive(Debug, PartialEq)]
pub enum NodeType {
    Program,
    FunctionDefinition,
    Parameters,
    Parameter,
    IfStatement,
    ForStatement,
    WhileStatement,
    RepeatStatement,
    BracedExpression,
    ParenthesizedExpression,
    Call,
    Subset,
    Subset2,
    Arguments,
    Argument,
    UnaryOperator(UnaryOperatorType),
    BinaryOperator(BinaryOperatorType),
    ExtractOperator(ExtractOperatorType),
    NamespaceOperator(NamespaceOperatorType),
    Integer,
    Complex,
    Float,
    String,
    StringContent,
    EscapeSequence,
    Identifier,
    DotDotI,
    Dots,
    Return,
    Next,
    Break,
    True,
    False,
    Null,
    Inf,
    Nan,
    Na(NaType),
    Comment,
    Comma,
    Error,
    Anonymous(String),
}

fn node_type(x: &Node) -> NodeType {
    match x.kind() {
        "program" => NodeType::Program,
        "function_definition" => NodeType::FunctionDefinition,
        "parameters" => NodeType::Parameters,
        "parameter" => NodeType::Parameter,
        "if_statement" => NodeType::IfStatement,
        "for_statement" => NodeType::ForStatement,
        "while_statement" => NodeType::WhileStatement,
        "repeat_statement" => NodeType::RepeatStatement,
        "braced_expression" => NodeType::BracedExpression,
        "parenthesized_expression" => NodeType::ParenthesizedExpression,
        "call" => NodeType::Call,
        "subset" => NodeType::Subset,
        "subset2" => NodeType::Subset2,
        "arguments" => NodeType::Arguments,
        "argument" => NodeType::Argument,
        "unary_operator" => NodeType::UnaryOperator(unary_operator_type(x)),
        "binary_operator" => NodeType::BinaryOperator(binary_operator_type(x)),
        "extract_operator" => NodeType::ExtractOperator(extract_operator_type(x)),
        "namespace_operator" => NodeType::NamespaceOperator(namespace_operator_type(x)),
        "integer" => NodeType::Integer,
        "complex" => NodeType::Complex,
        "float" => NodeType::Float,
        "string" => NodeType::String,
        "string_content" => NodeType::StringContent,
        "escape_sequence" => NodeType::EscapeSequence,
        "identifier" => NodeType::Identifier,
        "dot_dot_i" => NodeType::DotDotI,
        "dots" => NodeType::Dots,
        "return" => NodeType::Return,
        "next" => NodeType::Next,
        "break" => NodeType::Break,
        "true" => NodeType::True,
        "false" => NodeType::False,
        "null" => NodeType::Null,
        "inf" => NodeType::Inf,
        "nan" => NodeType::Nan,
        "na" => NodeType::Na(na_type(x)),
        "comment" => NodeType::Comment,
        "comma" => NodeType::Comma,
        "ERROR" => NodeType::Error,
        anonymous => NodeType::Anonymous(anonymous.to_string()),
    }
}

#[derive(Debug, PartialEq)]
pub enum UnaryOperatorType {
    /// `?`
    Help,
    /// `~`
    Tilde,
    /// `!`
    Not,
    /// `+`
    Plus,
    /// `-`
    Minus,
}

fn unary_operator_type(x: &Node) -> UnaryOperatorType {
    let x = x.child_by_field_name("operator").unwrap();

    match x.kind() {
        "?" => UnaryOperatorType::Help,
        "~" => UnaryOperatorType::Tilde,
        "!" => UnaryOperatorType::Not,
        "+" => UnaryOperatorType::Plus,
        "-" => UnaryOperatorType::Minus,
        _ => panic!("Unknown `unary_operator` kind {}.", x.kind()),
    }
}

#[derive(Debug, PartialEq)]
pub enum BinaryOperatorType {
    /// `?`
    Help,
    /// `~`
    Tilde,
    /// `<-`
    LeftAssignment,
    /// `<<-`
    LeftSuperAssignment,
    /// `:=`
    WalrusAssignment,
    /// `->`
    RightAssignment,
    /// `->>`
    RightSuperAssignment,
    /// `=`
    EqualsAssignment,
    /// `|`
    Or,
    /// `&`
    And,
    /// `||`
    Or2,
    /// `&&`
    And2,
    /// `<`
    LessThan,
    /// `<=`
    LessThanOrEqualTo,
    /// `>`
    GreaterThan,
    /// `>=`
    GreaterThanOrEqualTo,
    /// `==`
    Equal,
    /// `!=`
    NotEqual,
    /// `+`
    Plus,
    /// `-`
    Minus,
    /// `*`
    Multiply,
    /// `/`
    Divide,
    /// `^` or `**`
    Exponentiate,
    /// Infix operators, like `%>%`
    Special,
    /// `|>`
    Pipe,
    /// `:`
    Colon,
}

fn binary_operator_type(x: &Node) -> BinaryOperatorType {
    let x = x.child_by_field_name("operator").unwrap();

    match x.kind() {
        "?" => BinaryOperatorType::Help,
        "~" => BinaryOperatorType::Tilde,
        "<-" => BinaryOperatorType::LeftAssignment,
        "<<-" => BinaryOperatorType::LeftSuperAssignment,
        ":=" => BinaryOperatorType::WalrusAssignment,
        "->" => BinaryOperatorType::RightAssignment,
        "->>" => BinaryOperatorType::RightSuperAssignment,
        "=" => BinaryOperatorType::EqualsAssignment,
        "|" => BinaryOperatorType::Or,
        "&" => BinaryOperatorType::And,
        "||" => BinaryOperatorType::Or2,
        "&&" => BinaryOperatorType::And2,
        "<" => BinaryOperatorType::LessThan,
        "<=" => BinaryOperatorType::LessThanOrEqualTo,
        ">" => BinaryOperatorType::GreaterThan,
        ">=" => BinaryOperatorType::GreaterThanOrEqualTo,
        "==" => BinaryOperatorType::Equal,
        "!=" => BinaryOperatorType::NotEqual,
        "+" => BinaryOperatorType::Plus,
        "-" => BinaryOperatorType::Minus,
        "*" => BinaryOperatorType::Multiply,
        "/" => BinaryOperatorType::Divide,
        "^" => BinaryOperatorType::Exponentiate,
        "**" => BinaryOperatorType::Exponentiate,
        "special" => BinaryOperatorType::Special,
        "|>" => BinaryOperatorType::Pipe,
        ":" => BinaryOperatorType::Colon,
        _ => panic!("Unknown `binary_operator` kind {}.", x.kind()),
    }
}

#[derive(Debug, PartialEq)]
pub enum ExtractOperatorType {
    /// `$`
    Dollar,
    /// `@`
    At,
}

fn extract_operator_type(x: &Node) -> ExtractOperatorType {
    let x = x.child_by_field_name("operator").unwrap();

    match x.kind() {
        "$" => ExtractOperatorType::Dollar,
        "@" => ExtractOperatorType::At,
        _ => panic!("Unknown `extract_operator` kind {}.", x.kind()),
    }
}

#[derive(Debug, PartialEq)]
pub enum NamespaceOperatorType {
    /// `::`
    External,
    /// `:::`
    Internal,
}

fn namespace_operator_type(x: &Node) -> NamespaceOperatorType {
    let x = x.child_by_field_name("operator").unwrap();

    match x.kind() {
        "::" => NamespaceOperatorType::External,
        ":::" => NamespaceOperatorType::Internal,
        _ => panic!("Unknown `namespace_operator` kind {}.", x.kind()),
    }
}

#[derive(Debug, PartialEq)]
pub enum NaType {
    /// `NA`
    Logical,
    /// `NA_integer_`
    Integer,
    /// `NA_real_`
    Double,
    /// `NA_complex_`
    Complex,
    /// `NA_character_`
    Character,
}

fn na_type(x: &Node) -> NaType {
    let x = x.child(0).unwrap();

    match x.kind() {
        "NA" => NaType::Logical,
        "NA_integer_" => NaType::Integer,
        "NA_real_" => NaType::Double,
        "NA_complex_" => NaType::Complex,
        "NA_character_" => NaType::Character,
        _ => panic!("Unknown `na` kind {}.", x.kind()),
    }
}

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

    pub fn peek(&self) -> &Option<WalkEvent<Node<'tree>>> {
        &self.next
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
        "parameter" => parameter_syntax_kind(x),
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
        "argument" => argument_syntax_kind(x),
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
        "dots" => RSyntaxKind::DOTS,
        "dot_dot_i" => RSyntaxKind::R_DOT_DOT_I,
        "comment" => RSyntaxKind::COMMENT,
        kind => unreachable!("Not implemented: '{kind}'."),
    }
}

/// Determine the specific `RSyntaxKind` of a `"parameter"` node
///
/// A parameter can be one of 3 kinds:
/// - `function(x)` = R_IDENTIFIER_PARAMETER
/// - `function(x = 5)` = R_DEFAULT_PARAMETER
/// - `function(...)` = R_DOTS_PARAMETER
///
/// The tree-sitter grammar doesn't tell us which this is, but
/// we can figure it out from the node structure.
fn parameter_syntax_kind(x: &Node) -> RSyntaxKind {
    // `name` is a mandatory field on all 3 variants
    let name = x.child_by_field_name("name").unwrap();

    if name.kind() == "dots" {
        // Clearly `...`
        return RSyntaxKind::R_DOTS_PARAMETER;
    }

    let mut cursor = x.walk();

    // If a child is an anonymous `=`, must be default parameter
    for child in x.children(&mut cursor) {
        if child.is_named() {
            continue;
        }
        if child.kind() != "=" {
            continue;
        }
        return RSyntaxKind::R_DEFAULT_PARAMETER;
    }

    RSyntaxKind::R_IDENTIFIER_PARAMETER
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

// Disambiguate the 3 main types of argument kinds.
// Holes don't actually show up in the tree-sitter tree.
fn argument_syntax_kind(x: &Node) -> RSyntaxKind {
    // Required field on `argument` for `R_NAMED_ARGUMENT` case
    if x.child_by_field_name("name").is_some() {
        return RSyntaxKind::R_NAMED_ARGUMENT;
    }

    // Required field on `argument` for `R_DOTS` ad `R_UNNAMED_ARGUMENT` cases
    let value = x.child_by_field_name("value").unwrap();

    match value.kind() {
        "dots" => RSyntaxKind::R_DOTS_ARGUMENT,
        _ => RSyntaxKind::R_UNNAMED_ARGUMENT,
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

pub fn node_has_error_or_missing(node: &Node) -> bool {
    // According to the docs, `node.has_error()` should return `true`
    // if `node` is itself an error, or if it contains any errors, but that
    // doesn't seem to be the case for terminal ERROR nodes.
    // https://github.com/tree-sitter/tree-sitter/issues/3623
    node.is_error() || node.has_error()
}
