#[macro_use]
mod generated;
pub mod argument_ext;
pub mod call_ext;
pub mod string_ext;
mod syntax_node;

pub use self::generated::*;
pub use biome_rowan::{TextLen, TextRange, TextSize, TokenAtOffset, TriviaPieceKind, WalkEvent};
pub use syntax_node::*;

use biome_rowan::{RawSyntaxKind, SyntaxKind, TokenText};

impl From<u16> for RSyntaxKind {
    fn from(d: u16) -> RSyntaxKind {
        assert!(d <= (RSyntaxKind::__LAST as u16));
        unsafe { std::mem::transmute::<u16, RSyntaxKind>(d) }
    }
}

impl From<RSyntaxKind> for u16 {
    fn from(k: RSyntaxKind) -> u16 {
        k as u16
    }
}

impl biome_rowan::SyntaxKind for RSyntaxKind {
    const TOMBSTONE: Self = RSyntaxKind::TOMBSTONE;
    const EOF: Self = RSyntaxKind::EOF;

    fn is_bogus(&self) -> bool {
        match self {
            RSyntaxKind::R_BOGUS | RSyntaxKind::R_BOGUS_VALUE | RSyntaxKind::R_BOGUS_EXPRESSION => {
                true
            }

            RSyntaxKind::TOMBSTONE
            | RSyntaxKind::EOF
            | RSyntaxKind::UNICODE_BOM
            | RSyntaxKind::SEMICOLON
            | RSyntaxKind::COMMA
            | RSyntaxKind::L_CURLY
            | RSyntaxKind::R_CURLY
            | RSyntaxKind::L_BRACK
            | RSyntaxKind::R_BRACK
            | RSyntaxKind::L_BRACK2
            | RSyntaxKind::R_BRACK2
            | RSyntaxKind::L_PAREN
            | RSyntaxKind::R_PAREN
            | RSyntaxKind::DOTS
            | RSyntaxKind::DOTDOTI
            | RSyntaxKind::FUNCTION_KW
            | RSyntaxKind::FOR_KW
            | RSyntaxKind::IN_KW
            | RSyntaxKind::WHILE_KW
            | RSyntaxKind::REPEAT_KW
            | RSyntaxKind::IF_KW
            | RSyntaxKind::ELSE_KW
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
            | RSyntaxKind::R_INTEGER_LITERAL
            | RSyntaxKind::R_DOUBLE_LITERAL
            | RSyntaxKind::R_COMPLEX_LITERAL
            | RSyntaxKind::R_STRING_LITERAL
            | RSyntaxKind::NEWLINE
            | RSyntaxKind::WHITESPACE
            | RSyntaxKind::IDENT
            | RSyntaxKind::COMMENT
            | RSyntaxKind::BACKSLASH
            | RSyntaxKind::R_ROOT
            | RSyntaxKind::R_DOTS
            | RSyntaxKind::R_DOT_DOT_I
            | RSyntaxKind::R_IDENTIFIER
            | RSyntaxKind::R_UNARY_EXPRESSION
            | RSyntaxKind::R_BINARY_EXPRESSION
            | RSyntaxKind::R_EXTRACT_EXPRESSION
            | RSyntaxKind::R_NAMESPACE_EXPRESSION
            | RSyntaxKind::R_FUNCTION_DEFINITION
            | RSyntaxKind::R_PARAMETERS
            | RSyntaxKind::R_PARAMETER_LIST
            | RSyntaxKind::R_PARAMETER
            | RSyntaxKind::R_PARAMETER_DEFAULT
            | RSyntaxKind::R_IF_STATEMENT
            | RSyntaxKind::R_ELSE_CLAUSE
            | RSyntaxKind::R_FOR_STATEMENT
            | RSyntaxKind::R_WHILE_STATEMENT
            | RSyntaxKind::R_REPEAT_STATEMENT
            | RSyntaxKind::R_BRACED_EXPRESSIONS
            | RSyntaxKind::R_PARENTHESIZED_EXPRESSION
            | RSyntaxKind::R_CALL
            | RSyntaxKind::R_CALL_ARGUMENTS
            | RSyntaxKind::R_SUBSET
            | RSyntaxKind::R_SUBSET_ARGUMENTS
            | RSyntaxKind::R_SUBSET2
            | RSyntaxKind::R_SUBSET2_ARGUMENTS
            | RSyntaxKind::R_ARGUMENT_LIST
            | RSyntaxKind::R_ARGUMENT
            | RSyntaxKind::R_ARGUMENT_NAME_CLAUSE
            | RSyntaxKind::R_EXPRESSION_LIST
            | RSyntaxKind::R_INTEGER_VALUE
            | RSyntaxKind::R_DOUBLE_VALUE
            | RSyntaxKind::R_COMPLEX_VALUE
            | RSyntaxKind::R_STRING_VALUE
            | RSyntaxKind::R_RETURN_EXPRESSION
            | RSyntaxKind::R_NEXT_EXPRESSION
            | RSyntaxKind::R_BREAK_EXPRESSION
            | RSyntaxKind::R_TRUE_EXPRESSION
            | RSyntaxKind::R_FALSE_EXPRESSION
            | RSyntaxKind::R_NULL_EXPRESSION
            | RSyntaxKind::R_INF_EXPRESSION
            | RSyntaxKind::R_NAN_EXPRESSION
            | RSyntaxKind::R_NA_EXPRESSION
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
            | RSyntaxKind::__LAST => false,
        }
    }

    fn to_bogus(&self) -> Self {
        match self {
            // Bogus value
            RSyntaxKind::R_INTEGER_VALUE
            | RSyntaxKind::R_DOUBLE_VALUE
            | RSyntaxKind::R_COMPLEX_VALUE
            | RSyntaxKind::R_STRING_VALUE
            | RSyntaxKind::R_BOGUS_VALUE => RSyntaxKind::R_BOGUS_VALUE,

            // Bogus expression
            RSyntaxKind::R_UNARY_EXPRESSION
            | RSyntaxKind::R_BINARY_EXPRESSION
            | RSyntaxKind::R_EXTRACT_EXPRESSION
            | RSyntaxKind::R_NAMESPACE_EXPRESSION
            | RSyntaxKind::R_BOGUS_EXPRESSION => RSyntaxKind::R_BOGUS_EXPRESSION,

            // Bogus
            RSyntaxKind::TOMBSTONE
            | RSyntaxKind::EOF
            | RSyntaxKind::UNICODE_BOM
            | RSyntaxKind::SEMICOLON
            | RSyntaxKind::COMMA
            | RSyntaxKind::L_CURLY
            | RSyntaxKind::R_CURLY
            | RSyntaxKind::L_BRACK
            | RSyntaxKind::R_BRACK
            | RSyntaxKind::L_BRACK2
            | RSyntaxKind::R_BRACK2
            | RSyntaxKind::L_PAREN
            | RSyntaxKind::R_PAREN
            | RSyntaxKind::DOTS
            | RSyntaxKind::DOTDOTI
            | RSyntaxKind::FUNCTION_KW
            | RSyntaxKind::FOR_KW
            | RSyntaxKind::IN_KW
            | RSyntaxKind::WHILE_KW
            | RSyntaxKind::REPEAT_KW
            | RSyntaxKind::IF_KW
            | RSyntaxKind::ELSE_KW
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
            | RSyntaxKind::R_INTEGER_LITERAL
            | RSyntaxKind::R_DOUBLE_LITERAL
            | RSyntaxKind::R_COMPLEX_LITERAL
            | RSyntaxKind::R_STRING_LITERAL
            | RSyntaxKind::R_RETURN_EXPRESSION
            | RSyntaxKind::R_NEXT_EXPRESSION
            | RSyntaxKind::R_BREAK_EXPRESSION
            | RSyntaxKind::R_TRUE_EXPRESSION
            | RSyntaxKind::R_FALSE_EXPRESSION
            | RSyntaxKind::R_NULL_EXPRESSION
            | RSyntaxKind::R_INF_EXPRESSION
            | RSyntaxKind::R_NAN_EXPRESSION
            | RSyntaxKind::R_NA_EXPRESSION
            | RSyntaxKind::NEWLINE
            | RSyntaxKind::WHITESPACE
            | RSyntaxKind::IDENT
            | RSyntaxKind::COMMENT
            | RSyntaxKind::BACKSLASH
            | RSyntaxKind::R_ROOT
            | RSyntaxKind::R_IDENTIFIER
            | RSyntaxKind::R_DOTS
            | RSyntaxKind::R_DOT_DOT_I
            | RSyntaxKind::R_FUNCTION_DEFINITION
            | RSyntaxKind::R_PARAMETERS
            | RSyntaxKind::R_PARAMETER_LIST
            | RSyntaxKind::R_PARAMETER
            | RSyntaxKind::R_PARAMETER_DEFAULT
            | RSyntaxKind::R_IF_STATEMENT
            | RSyntaxKind::R_ELSE_CLAUSE
            | RSyntaxKind::R_FOR_STATEMENT
            | RSyntaxKind::R_WHILE_STATEMENT
            | RSyntaxKind::R_REPEAT_STATEMENT
            | RSyntaxKind::R_BRACED_EXPRESSIONS
            | RSyntaxKind::R_PARENTHESIZED_EXPRESSION
            | RSyntaxKind::R_CALL
            | RSyntaxKind::R_CALL_ARGUMENTS
            | RSyntaxKind::R_SUBSET
            | RSyntaxKind::R_SUBSET_ARGUMENTS
            | RSyntaxKind::R_SUBSET2
            | RSyntaxKind::R_SUBSET2_ARGUMENTS
            | RSyntaxKind::R_ARGUMENT_LIST
            | RSyntaxKind::R_ARGUMENT
            | RSyntaxKind::R_ARGUMENT_NAME_CLAUSE
            | RSyntaxKind::R_EXPRESSION_LIST
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
            | RSyntaxKind::__LAST
            | RSyntaxKind::R_BOGUS => RSyntaxKind::R_BOGUS,
        }
    }

    #[inline]
    fn to_raw(&self) -> RawSyntaxKind {
        RawSyntaxKind(*self as u16)
    }

    #[inline]
    fn from_raw(raw: RawSyntaxKind) -> Self {
        Self::from(raw.0)
    }

    fn is_root(&self) -> bool {
        matches!(self, RSyntaxKind::R_ROOT)
    }

    fn is_list(&self) -> bool {
        RSyntaxKind::is_list(*self)
    }

    fn is_trivia(self) -> bool {
        matches!(
            self,
            RSyntaxKind::NEWLINE | RSyntaxKind::WHITESPACE | RSyntaxKind::COMMENT
        )
    }

    fn to_string(&self) -> Option<&'static str> {
        RSyntaxKind::to_string(self)
    }
}

impl TryFrom<RSyntaxKind> for TriviaPieceKind {
    type Error = ();

    fn try_from(value: RSyntaxKind) -> Result<Self, Self::Error> {
        if value.is_trivia() {
            match value {
                RSyntaxKind::NEWLINE => Ok(TriviaPieceKind::Newline),
                RSyntaxKind::WHITESPACE => Ok(TriviaPieceKind::Whitespace),
                RSyntaxKind::COMMENT => Ok(TriviaPieceKind::SingleLineComment),
                _ => unreachable!("Not Trivia"),
            }
        } else {
            Err(())
        }
    }
}

/// See [tree-sitter-r](https://github.com/r-lib/tree-sitter-r/blob/main/grammar.js)
#[derive(Debug, Eq, Ord, PartialOrd, PartialEq, Copy, Clone, Hash)]
pub enum OperatorPrecedence {
    Help = 0,
    LeftAssignment = 1,
    EqualsAssignment = 2,
    RightAssignment = 3,
    Tilde = 4,
    LogicalOr = 5,
    LogicalAnd = 6,
    UnaryNot = 7,
    Relational = 8,
    Additive = 9,
    Multiplicative = 10,
    Special = 11,
    Colon = 12,
    UnaryPlusMinus = 13,
    Exponential = 14,
    Extract = 15,
    Namespace = 16,
}

impl OperatorPrecedence {
    /// Returns the precedence for a binary operator token or [None] if the token isn't a binary operator
    ///
    /// We don't treat namespace operators `::` and `:::` or extraction operators
    /// `$` and `@` as binary.
    pub fn try_from_binary_operator(kind: RSyntaxKind) -> Option<OperatorPrecedence> {
        Some(match kind {
            T![?] => OperatorPrecedence::Help,
            T![<-] => OperatorPrecedence::LeftAssignment,
            T![<<-] => OperatorPrecedence::LeftAssignment,
            T![:=] => OperatorPrecedence::LeftAssignment,
            T![=] => OperatorPrecedence::EqualsAssignment,
            T![->] => OperatorPrecedence::RightAssignment,
            T![->>] => OperatorPrecedence::RightAssignment,
            T![~] => OperatorPrecedence::Tilde,
            T![|] => OperatorPrecedence::LogicalOr,
            T![||] => OperatorPrecedence::LogicalOr,
            T![&] => OperatorPrecedence::LogicalAnd,
            T![&&] => OperatorPrecedence::LogicalAnd,
            T![==] => OperatorPrecedence::Relational,
            T![>] => OperatorPrecedence::Relational,
            T![>=] => OperatorPrecedence::Relational,
            T![<] => OperatorPrecedence::Relational,
            T![<=] => OperatorPrecedence::Relational,
            T![+] => OperatorPrecedence::Additive,
            T![-] => OperatorPrecedence::Additive,
            T![*] => OperatorPrecedence::Multiplicative,
            T![/] => OperatorPrecedence::Multiplicative,
            T![|>] => OperatorPrecedence::Special,
            RSyntaxKind::SPECIAL => OperatorPrecedence::Special,
            T![:] => OperatorPrecedence::Colon,
            T![^] => OperatorPrecedence::Exponential,
            T![**] => OperatorPrecedence::Exponential,
            _ => return None,
        })
    }
}

/// Text of `token`, excluding all trivia and removing quotes if `token` is a string literal.
pub fn inner_string_text(token: &RSyntaxToken) -> TokenText {
    let mut text = token.token_text_trimmed();
    if token.kind() == RSyntaxKind::R_STRING_VALUE {
        // remove string delimiters
        // SAFETY: string literal token have a delimiters at the start and the end of the string
        let range = TextRange::new(1.into(), text.len() - TextSize::from(1));
        text = text.slice(range);
    }
    text
}
