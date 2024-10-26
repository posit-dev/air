use biome_r_syntax::RSyntaxKind;
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
#[derive(Debug, Copy, Clone)]
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
// TODO: Switch to `TreeCursor` instead of `Node` because we're currently doing
// `Node::parent()` which requires a full traversal
pub struct Preorder<'tree> {
    start: Node<'tree>,
    next: Option<WalkEvent<Node<'tree>>>,
    skip_subtree: bool,
}

impl<'tree> Preorder<'tree> {
    fn new(start: Node) -> Preorder {
        let next = Some(WalkEvent::Enter(start.clone()));
        Preorder {
            start,
            next,
            skip_subtree: false,
        }
    }

    pub fn skip_subtree(&mut self) {
        self.skip_subtree = true;
    }

    #[cold]
    fn do_skip(&mut self) {
        self.next = self.next.take().map(|next| match next {
            WalkEvent::Enter(first_child) => WalkEvent::Leave(first_child.parent().unwrap()),
            WalkEvent::Leave(parent) => WalkEvent::Leave(parent),
        })
    }
}

impl<'tree> Iterator for Preorder<'tree> {
    type Item = WalkEvent<Node<'tree>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.skip_subtree {
            self.do_skip();
            self.skip_subtree = false;
        }
        let next = self.next.take();
        self.next = next.as_ref().and_then(|next| {
            Some(match next {
                WalkEvent::Enter(node) => match node.child(0) {
                    Some(child) => WalkEvent::Enter(child),
                    None => WalkEvent::Leave(node.clone()),
                },
                WalkEvent::Leave(node) => {
                    if node == &self.start {
                        return None;
                    }
                    match node.next_sibling() {
                        Some(sibling) => WalkEvent::Enter(sibling),
                        None => WalkEvent::Leave(node.parent()?),
                    }
                }
            })
        });
        next
    }
}

fn node_syntax_kind(x: &Node) -> RSyntaxKind {
    match x.kind() {
        "program" => RSyntaxKind::R_ROOT,
        "binary_operator" => RSyntaxKind::R_BINARY_EXPRESSION,
        "function_definition" => RSyntaxKind::R_FUNCTION_DEFINITION,
        "parameters" => RSyntaxKind::R_PARAMETERS,
        "parameter" => parameter_syntax_kind(x),
        "identifier" => RSyntaxKind::R_IDENTIFIER,
        "integer" => RSyntaxKind::R_INTEGER_VALUE,
        "float" => RSyntaxKind::R_DOUBLE_VALUE,
        "string" => RSyntaxKind::R_STRING_VALUE,
        "true" => RSyntaxKind::R_LOGICAL_VALUE,
        "false" => RSyntaxKind::R_LOGICAL_VALUE,
        "null" => RSyntaxKind::R_NULL_VALUE,
        "{" => RSyntaxKind::L_CURLY,
        "}" => RSyntaxKind::R_CURLY,
        "[" => RSyntaxKind::L_BRACK,
        "]" => RSyntaxKind::R_BRACK,
        "(" => RSyntaxKind::L_PAREN,
        ")" => RSyntaxKind::R_PAREN,
        "+" => RSyntaxKind::PLUS,
        "=" => equal_syntax_kind(x),
        "function" => RSyntaxKind::FUNCTION_KW,
        "comma" => RSyntaxKind::COMMA,
        "comment" => RSyntaxKind::COMMENT,
        kind => unreachable!("Not implemented: '{kind}'."),
    }
}

fn equal_syntax_kind(x: &Node) -> RSyntaxKind {
    if x.is_named() {
        unreachable!("Not implemented: named `=`.");
    } else {
        RSyntaxKind::EQUAL
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

pub trait NodeTypeExt: Sized {
    fn syntax_kind(&self) -> RSyntaxKind;
    fn preorder(&self) -> Preorder;
}

impl NodeTypeExt for Node<'_> {
    fn syntax_kind(&self) -> RSyntaxKind {
        node_syntax_kind(self)
    }

    fn preorder(&self) -> Preorder {
        Preorder::new(self.clone())
    }
}

pub fn node_has_error_or_missing(node: &Node) -> bool {
    // According to the docs, `node.has_error()` should return `true`
    // if `node` is itself an error, or if it contains any errors, but that
    // doesn't seem to be the case for terminal ERROR nodes.
    // https://github.com/tree-sitter/tree-sitter/issues/3623
    node.is_error() || node.has_error()
}
