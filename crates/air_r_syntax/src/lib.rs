#[macro_use]
mod generated;
pub mod argument_ext;
pub mod string_ext;
mod syntax_node;

pub use self::generated::*;
pub use biome_rowan::TextLen;
pub use biome_rowan::TextRange;
pub use biome_rowan::TextSize;
pub use biome_rowan::TokenAtOffset;
pub use biome_rowan::TokenText;
pub use biome_rowan::TriviaPieceKind;
pub use biome_rowan::WalkEvent;
pub use syntax_node::*;

use biome_rowan::RawSyntaxKind;
use biome_rowan::SyntaxKind;

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
            RSyntaxKind::R_BOGUS
            | RSyntaxKind::R_BOGUS_VALUE
            | RSyntaxKind::R_BOGUS_EXPRESSION
            | RSyntaxKind::R_BOGUS_SELECTOR
            | RSyntaxKind::R_BOGUS_PARAMETER_NAME
            | RSyntaxKind::R_BOGUS_FOR_VARIABLE => true,

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
            | RSyntaxKind::R_SINGLE_QUOTED_STRING_LITERAL
            | RSyntaxKind::R_DOUBLE_QUOTED_STRING_LITERAL
            | RSyntaxKind::R_RAW_STRING_LITERAL
            | RSyntaxKind::NEWLINE
            | RSyntaxKind::WHITESPACE
            | RSyntaxKind::IDENT
            | RSyntaxKind::COMMENT
            | RSyntaxKind::BACKSLASH
            | RSyntaxKind::ERROR_TOKEN
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
            | RSyntaxKind::R_BRACED_STATEMENTS
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
            | RSyntaxKind::R_STATEMENT_LIST
            | RSyntaxKind::R_EXPRESSION_STATEMENT
            | RSyntaxKind::R_EMPTY_STATEMENT
            | RSyntaxKind::R_INTEGER_VALUE
            | RSyntaxKind::R_DOUBLE_VALUE
            | RSyntaxKind::R_COMPLEX_VALUE
            | RSyntaxKind::R_STRING_VALUE
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

            // Bogus selector
            RSyntaxKind::R_BOGUS_SELECTOR => RSyntaxKind::R_BOGUS_SELECTOR,

            // Bogus parameter name
            RSyntaxKind::R_BOGUS_PARAMETER_NAME => RSyntaxKind::R_BOGUS_PARAMETER_NAME,

            // Bogus for variable
            RSyntaxKind::R_BOGUS_FOR_VARIABLE => RSyntaxKind::R_BOGUS_FOR_VARIABLE,

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
            | RSyntaxKind::R_SINGLE_QUOTED_STRING_LITERAL
            | RSyntaxKind::R_DOUBLE_QUOTED_STRING_LITERAL
            | RSyntaxKind::R_RAW_STRING_LITERAL
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
            | RSyntaxKind::ERROR_TOKEN
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
            | RSyntaxKind::R_BRACED_STATEMENTS
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
            | RSyntaxKind::R_STATEMENT_LIST
            | RSyntaxKind::R_EXPRESSION_STATEMENT
            | RSyntaxKind::R_EMPTY_STATEMENT
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
