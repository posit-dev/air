//! Generated file, do not edit by hand, see `xtask/codegen`

#![allow(clippy::all)]
#![allow(bad_style, missing_docs, unreachable_pub)]
#[doc = r" The kind of syntax node, e.g. `IDENT`, `FUNCTION_KW`, or `FOR_STMT`."]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[repr(u16)]
pub enum RSyntaxKind {
    #[doc(hidden)]
    TOMBSTONE,
    #[doc = r" Marks the end of the file. May have trivia attached"]
    EOF,
    #[doc = r" Any Unicode BOM character that may be present at the start of"]
    #[doc = r" a file."]
    UNICODE_BOM,
    SEMICOLON,
    COMMA,
    L_CURLY,
    R_CURLY,
    L_BRACK,
    R_BRACK,
    L_PAREN,
    R_PAREN,
    PLUS,
    EQUAL,
    DOTS,
    FUNCTION_KW,
    FOR_KW,
    IN_KW,
    IF_KW,
    ELSE_KW,
    R_INTEGER_LITERAL,
    R_DOUBLE_LITERAL,
    R_COMPLEX_LITERAL,
    R_STRING_LITERAL,
    R_LOGICAL_LITERAL,
    R_NULL_LITERAL,
    NEWLINE,
    WHITESPACE,
    IDENT,
    COMMENT,
    BACKSLASH,
    R_ROOT,
    R_IDENTIFIER,
    R_BINARY_EXPRESSION,
    R_FUNCTION_DEFINITION,
    R_PARAMETERS,
    R_PARAMETER_LIST,
    R_IDENTIFIER_PARAMETER,
    R_DOTS_PARAMETER,
    R_DEFAULT_PARAMETER,
    R_IF_STATEMENT,
    R_ELSE_CLAUSE,
    R_FOR_STATEMENT,
    R_EXPRESSION_LIST,
    R_INTEGER_VALUE,
    R_DOUBLE_VALUE,
    R_COMPLEX_VALUE,
    R_STRING_VALUE,
    R_LOGICAL_VALUE,
    R_NULL_VALUE,
    R_BOGUS,
    R_BOGUS_VALUE,
    R_BOGUS_EXPRESSION,
    R_BOGUS_PARAMETER,
    #[doc(hidden)]
    __LAST,
}
use self::RSyntaxKind::*;
impl RSyntaxKind {
    pub const fn is_punct(self) -> bool {
        match self {
            SEMICOLON | COMMA | L_CURLY | R_CURLY | L_BRACK | R_BRACK | L_PAREN | R_PAREN
            | PLUS | EQUAL | DOTS => true,
            _ => false,
        }
    }
    pub const fn is_literal(self) -> bool {
        match self {
            R_INTEGER_LITERAL | R_DOUBLE_LITERAL | R_COMPLEX_LITERAL | R_STRING_LITERAL
            | R_LOGICAL_LITERAL | R_NULL_LITERAL => true,
            _ => false,
        }
    }
    pub const fn is_list(self) -> bool {
        match self {
            R_PARAMETER_LIST | R_EXPRESSION_LIST => true,
            _ => false,
        }
    }
    pub fn from_keyword(ident: &str) -> Option<RSyntaxKind> {
        let kw = match ident {
            "function" => FUNCTION_KW,
            "for" => FOR_KW,
            "in" => IN_KW,
            "if" => IF_KW,
            "else" => ELSE_KW,
            _ => return None,
        };
        Some(kw)
    }
    pub const fn to_string(&self) -> Option<&'static str> {
        let tok = match self {
            SEMICOLON => ";",
            COMMA => ",",
            L_CURLY => "{",
            R_CURLY => "}",
            L_BRACK => "[",
            R_BRACK => "]",
            L_PAREN => "(",
            R_PAREN => ")",
            PLUS => "+",
            EQUAL => "=",
            DOTS => "...",
            FUNCTION_KW => "function",
            FOR_KW => "for",
            IN_KW => "in",
            IF_KW => "if",
            ELSE_KW => "else",
            R_STRING_VALUE => "string value",
            _ => return None,
        };
        Some(tok)
    }
}
#[doc = r" Utility macro for creating a SyntaxKind through simple macro syntax"]
#[macro_export]
macro_rules ! T { [;] => { $ crate :: RSyntaxKind :: SEMICOLON } ; [,] => { $ crate :: RSyntaxKind :: COMMA } ; ['{'] => { $ crate :: RSyntaxKind :: L_CURLY } ; ['}'] => { $ crate :: RSyntaxKind :: R_CURLY } ; ['['] => { $ crate :: RSyntaxKind :: L_BRACK } ; [']'] => { $ crate :: RSyntaxKind :: R_BRACK } ; ['('] => { $ crate :: RSyntaxKind :: L_PAREN } ; [')'] => { $ crate :: RSyntaxKind :: R_PAREN } ; [+] => { $ crate :: RSyntaxKind :: PLUS } ; [=] => { $ crate :: RSyntaxKind :: EQUAL } ; [...] => { $ crate :: RSyntaxKind :: DOTS } ; [function] => { $ crate :: RSyntaxKind :: FUNCTION_KW } ; [for] => { $ crate :: RSyntaxKind :: FOR_KW } ; [in] => { $ crate :: RSyntaxKind :: IN_KW } ; [if] => { $ crate :: RSyntaxKind :: IF_KW } ; [else] => { $ crate :: RSyntaxKind :: ELSE_KW } ; [ident] => { $ crate :: RSyntaxKind :: IDENT } ; [EOF] => { $ crate :: RSyntaxKind :: EOF } ; [UNICODE_BOM] => { $ crate :: RSyntaxKind :: UNICODE_BOM } ; [#] => { $ crate :: RSyntaxKind :: HASH } ; }
