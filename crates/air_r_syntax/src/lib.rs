#[macro_use]
mod generated;
mod file_source;
pub mod string_ext;
mod syntax_node;

pub use self::generated::*;
pub use biome_rowan::{TextLen, TextRange, TextSize, TokenAtOffset, TriviaPieceKind, WalkEvent};
pub use file_source::RFileSource;
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
            RSyntaxKind::R_BOGUS
            | RSyntaxKind::R_BOGUS_VALUE
            | RSyntaxKind::R_BOGUS_EXPRESSION
            | RSyntaxKind::R_BOGUS_PARAMETER => true,

            RSyntaxKind::TOMBSTONE
            | RSyntaxKind::EOF
            | RSyntaxKind::UNICODE_BOM
            | RSyntaxKind::SEMICOLON
            | RSyntaxKind::COMMA
            | RSyntaxKind::L_CURLY
            | RSyntaxKind::R_CURLY
            | RSyntaxKind::L_BRACK
            | RSyntaxKind::R_BRACK
            | RSyntaxKind::L_PAREN
            | RSyntaxKind::R_PAREN
            | RSyntaxKind::PLUS
            | RSyntaxKind::EQUAL
            | RSyntaxKind::DOTS
            | RSyntaxKind::FUNCTION_KW
            | RSyntaxKind::FOR_KW
            | RSyntaxKind::IN_KW
            | RSyntaxKind::R_INTEGER_LITERAL
            | RSyntaxKind::R_DOUBLE_LITERAL
            | RSyntaxKind::R_STRING_LITERAL
            | RSyntaxKind::R_LOGICAL_LITERAL
            | RSyntaxKind::R_NULL_LITERAL
            | RSyntaxKind::NEWLINE
            | RSyntaxKind::WHITESPACE
            | RSyntaxKind::IDENT
            | RSyntaxKind::COMMENT
            | RSyntaxKind::R_ROOT
            | RSyntaxKind::R_IDENTIFIER
            | RSyntaxKind::R_BINARY_EXPRESSION
            | RSyntaxKind::R_FUNCTION_DEFINITION
            | RSyntaxKind::R_PARAMETERS
            | RSyntaxKind::R_PARAMETER_LIST
            | RSyntaxKind::R_DOTS_PARAMETER
            | RSyntaxKind::R_IDENTIFIER_PARAMETER
            | RSyntaxKind::R_DEFAULT_PARAMETER
            | RSyntaxKind::R_FOR_STATEMENT
            | RSyntaxKind::R_EXPRESSION_LIST
            | RSyntaxKind::R_INTEGER_VALUE
            | RSyntaxKind::R_DOUBLE_VALUE
            | RSyntaxKind::R_STRING_VALUE
            | RSyntaxKind::R_LOGICAL_VALUE
            | RSyntaxKind::R_NULL_VALUE
            | RSyntaxKind::__LAST => false,
        }
    }

    fn to_bogus(&self) -> Self {
        match self {
            // Bogus value
            RSyntaxKind::R_INTEGER_VALUE => RSyntaxKind::R_BOGUS_VALUE,
            RSyntaxKind::R_DOUBLE_VALUE => RSyntaxKind::R_BOGUS_VALUE,
            RSyntaxKind::R_STRING_VALUE => RSyntaxKind::R_BOGUS_VALUE,
            RSyntaxKind::R_LOGICAL_VALUE => RSyntaxKind::R_BOGUS_VALUE,
            RSyntaxKind::R_NULL_VALUE => RSyntaxKind::R_BOGUS_VALUE,
            RSyntaxKind::R_BOGUS_VALUE => RSyntaxKind::R_BOGUS_VALUE,

            // Bogus parameter
            RSyntaxKind::R_DOTS_PARAMETER => RSyntaxKind::R_BOGUS_PARAMETER,
            RSyntaxKind::R_IDENTIFIER_PARAMETER => RSyntaxKind::R_BOGUS_PARAMETER,
            RSyntaxKind::R_DEFAULT_PARAMETER => RSyntaxKind::R_BOGUS_PARAMETER,
            RSyntaxKind::R_BOGUS_PARAMETER => RSyntaxKind::R_BOGUS_PARAMETER,

            // Bogus expression
            RSyntaxKind::R_BINARY_EXPRESSION => RSyntaxKind::R_BOGUS_EXPRESSION,
            RSyntaxKind::R_BOGUS_EXPRESSION => RSyntaxKind::R_BOGUS_EXPRESSION,

            // Bogus
            RSyntaxKind::TOMBSTONE => RSyntaxKind::R_BOGUS,
            RSyntaxKind::EOF => RSyntaxKind::R_BOGUS,
            RSyntaxKind::UNICODE_BOM => RSyntaxKind::R_BOGUS,
            RSyntaxKind::SEMICOLON => RSyntaxKind::R_BOGUS,
            RSyntaxKind::COMMA => RSyntaxKind::R_BOGUS,
            RSyntaxKind::L_CURLY => RSyntaxKind::R_BOGUS,
            RSyntaxKind::R_CURLY => RSyntaxKind::R_BOGUS,
            RSyntaxKind::L_BRACK => RSyntaxKind::R_BOGUS,
            RSyntaxKind::R_BRACK => RSyntaxKind::R_BOGUS,
            RSyntaxKind::L_PAREN => RSyntaxKind::R_BOGUS,
            RSyntaxKind::R_PAREN => RSyntaxKind::R_BOGUS,
            RSyntaxKind::PLUS => RSyntaxKind::R_BOGUS,
            RSyntaxKind::EQUAL => RSyntaxKind::R_BOGUS,
            RSyntaxKind::DOTS => RSyntaxKind::R_BOGUS,
            RSyntaxKind::FUNCTION_KW => RSyntaxKind::R_BOGUS,
            RSyntaxKind::FOR_KW => RSyntaxKind::R_BOGUS,
            RSyntaxKind::IN_KW => RSyntaxKind::R_BOGUS,
            RSyntaxKind::R_INTEGER_LITERAL => RSyntaxKind::R_BOGUS,
            RSyntaxKind::R_DOUBLE_LITERAL => RSyntaxKind::R_BOGUS,
            RSyntaxKind::R_STRING_LITERAL => RSyntaxKind::R_BOGUS,
            RSyntaxKind::R_LOGICAL_LITERAL => RSyntaxKind::R_BOGUS,
            RSyntaxKind::R_NULL_LITERAL => RSyntaxKind::R_BOGUS,
            RSyntaxKind::NEWLINE => RSyntaxKind::R_BOGUS,
            RSyntaxKind::WHITESPACE => RSyntaxKind::R_BOGUS,
            RSyntaxKind::IDENT => RSyntaxKind::R_BOGUS,
            RSyntaxKind::COMMENT => RSyntaxKind::R_BOGUS,
            RSyntaxKind::R_ROOT => RSyntaxKind::R_BOGUS,
            RSyntaxKind::R_IDENTIFIER => RSyntaxKind::R_BOGUS,
            RSyntaxKind::R_FUNCTION_DEFINITION => RSyntaxKind::R_BOGUS,
            RSyntaxKind::R_PARAMETERS => RSyntaxKind::R_BOGUS,
            RSyntaxKind::R_PARAMETER_LIST => RSyntaxKind::R_BOGUS,
            RSyntaxKind::R_FOR_STATEMENT => RSyntaxKind::R_BOGUS,
            RSyntaxKind::R_EXPRESSION_LIST => RSyntaxKind::R_BOGUS,
            RSyntaxKind::__LAST => RSyntaxKind::R_BOGUS,
            RSyntaxKind::R_BOGUS => RSyntaxKind::R_BOGUS,
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
