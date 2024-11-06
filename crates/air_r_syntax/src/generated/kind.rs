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
    MINUS,
    TILDE,
    BANG,
    WAT,
    EQUAL,
    DOTS,
    FUNCTION_KW,
    FOR_KW,
    IN_KW,
    WHILE_KW,
    REPEAT_KW,
    IF_KW,
    ELSE_KW,
    RETURN_KW,
    NEXT_KW,
    BREAK_KW,
    TRUE_KW,
    FALSE_KW,
    NULL_KW,
    INF_KW,
    NAN_KW,
    NA_LOGICAL_KW,
    NA_INTEGER_KW,
    NA_DOUBLE_KW,
    NA_COMPLEX_KW,
    NA_CHARACTER_KW,
    R_INTEGER_LITERAL,
    R_DOUBLE_LITERAL,
    R_COMPLEX_LITERAL,
    R_STRING_LITERAL,
    NEWLINE,
    WHITESPACE,
    IDENT,
    COMMENT,
    BACKSLASH,
    R_ROOT,
    R_IDENTIFIER,
    R_DOTS,
    R_UNARY_EXPRESSION,
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
    R_WHILE_STATEMENT,
    R_REPEAT_STATEMENT,
    R_BRACED_EXPRESSIONS,
    R_PARENTHESIZED_EXPRESSION,
    R_CALL,
    R_CALL_ARGUMENTS,
    R_ARGUMENT_LIST,
    R_NAMED_ARGUMENT,
    R_UNNAMED_ARGUMENT,
    R_DOTS_ARGUMENT,
    R_HOLE_ARGUMENT,
    R_EXPRESSION_LIST,
    R_INTEGER_VALUE,
    R_DOUBLE_VALUE,
    R_COMPLEX_VALUE,
    R_STRING_VALUE,
    R_RETURN_EXPRESSION,
    R_NEXT_EXPRESSION,
    R_BREAK_EXPRESSION,
    R_TRUE_EXPRESSION,
    R_FALSE_EXPRESSION,
    R_NULL_EXPRESSION,
    R_INF_EXPRESSION,
    R_NAN_EXPRESSION,
    R_NA_EXPRESSION,
    R_BOGUS,
    R_BOGUS_VALUE,
    R_BOGUS_EXPRESSION,
    R_BOGUS_PARAMETER,
    R_BOGUS_ARGUMENT,
    #[doc(hidden)]
    __LAST,
}
use self::RSyntaxKind::*;
impl RSyntaxKind {
    pub const fn is_punct(self) -> bool {
        match self {
            SEMICOLON | COMMA | L_CURLY | R_CURLY | L_BRACK | R_BRACK | L_PAREN | R_PAREN
            | PLUS | MINUS | TILDE | BANG | WAT | EQUAL | DOTS => true,
            _ => false,
        }
    }
    pub const fn is_literal(self) -> bool {
        match self {
            R_INTEGER_LITERAL | R_DOUBLE_LITERAL | R_COMPLEX_LITERAL | R_STRING_LITERAL => true,
            _ => false,
        }
    }
    pub const fn is_list(self) -> bool {
        match self {
            R_PARAMETER_LIST | R_ARGUMENT_LIST | R_EXPRESSION_LIST => true,
            _ => false,
        }
    }
    pub fn from_keyword(ident: &str) -> Option<RSyntaxKind> {
        let kw = match ident {
            "function" => FUNCTION_KW,
            "for" => FOR_KW,
            "in" => IN_KW,
            "while" => WHILE_KW,
            "repeat" => REPEAT_KW,
            "if" => IF_KW,
            "else" => ELSE_KW,
            "return" => RETURN_KW,
            "next" => NEXT_KW,
            "break" => BREAK_KW,
            "true" => TRUE_KW,
            "false" => FALSE_KW,
            "null" => NULL_KW,
            "inf" => INF_KW,
            "nan" => NAN_KW,
            "na_logical" => NA_LOGICAL_KW,
            "na_integer" => NA_INTEGER_KW,
            "na_double" => NA_DOUBLE_KW,
            "na_complex" => NA_COMPLEX_KW,
            "na_character" => NA_CHARACTER_KW,
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
            MINUS => "-",
            TILDE => "~",
            BANG => "!",
            WAT => "?",
            EQUAL => "=",
            DOTS => "...",
            FUNCTION_KW => "function",
            FOR_KW => "for",
            IN_KW => "in",
            WHILE_KW => "while",
            REPEAT_KW => "repeat",
            IF_KW => "if",
            ELSE_KW => "else",
            RETURN_KW => "return",
            NEXT_KW => "next",
            BREAK_KW => "break",
            TRUE_KW => "true",
            FALSE_KW => "false",
            NULL_KW => "null",
            INF_KW => "inf",
            NAN_KW => "nan",
            NA_LOGICAL_KW => "na_logical",
            NA_INTEGER_KW => "na_integer",
            NA_DOUBLE_KW => "na_double",
            NA_COMPLEX_KW => "na_complex",
            NA_CHARACTER_KW => "na_character",
            R_STRING_VALUE => "string value",
            _ => return None,
        };
        Some(tok)
    }
}
#[doc = r" Utility macro for creating a SyntaxKind through simple macro syntax"]
#[macro_export]
macro_rules ! T { [;] => { $ crate :: RSyntaxKind :: SEMICOLON } ; [,] => { $ crate :: RSyntaxKind :: COMMA } ; ['{'] => { $ crate :: RSyntaxKind :: L_CURLY } ; ['}'] => { $ crate :: RSyntaxKind :: R_CURLY } ; ['['] => { $ crate :: RSyntaxKind :: L_BRACK } ; [']'] => { $ crate :: RSyntaxKind :: R_BRACK } ; ['('] => { $ crate :: RSyntaxKind :: L_PAREN } ; [')'] => { $ crate :: RSyntaxKind :: R_PAREN } ; [+] => { $ crate :: RSyntaxKind :: PLUS } ; [-] => { $ crate :: RSyntaxKind :: MINUS } ; [~] => { $ crate :: RSyntaxKind :: TILDE } ; [!] => { $ crate :: RSyntaxKind :: BANG } ; [?] => { $ crate :: RSyntaxKind :: WAT } ; [=] => { $ crate :: RSyntaxKind :: EQUAL } ; [...] => { $ crate :: RSyntaxKind :: DOTS } ; [function] => { $ crate :: RSyntaxKind :: FUNCTION_KW } ; [for] => { $ crate :: RSyntaxKind :: FOR_KW } ; [in] => { $ crate :: RSyntaxKind :: IN_KW } ; [while] => { $ crate :: RSyntaxKind :: WHILE_KW } ; [repeat] => { $ crate :: RSyntaxKind :: REPEAT_KW } ; [if] => { $ crate :: RSyntaxKind :: IF_KW } ; [else] => { $ crate :: RSyntaxKind :: ELSE_KW } ; [return] => { $ crate :: RSyntaxKind :: RETURN_KW } ; [next] => { $ crate :: RSyntaxKind :: NEXT_KW } ; [break] => { $ crate :: RSyntaxKind :: BREAK_KW } ; [true] => { $ crate :: RSyntaxKind :: TRUE_KW } ; [false] => { $ crate :: RSyntaxKind :: FALSE_KW } ; [null] => { $ crate :: RSyntaxKind :: NULL_KW } ; [inf] => { $ crate :: RSyntaxKind :: INF_KW } ; [nan] => { $ crate :: RSyntaxKind :: NAN_KW } ; [na_logical] => { $ crate :: RSyntaxKind :: NA_LOGICAL_KW } ; [na_integer] => { $ crate :: RSyntaxKind :: NA_INTEGER_KW } ; [na_double] => { $ crate :: RSyntaxKind :: NA_DOUBLE_KW } ; [na_complex] => { $ crate :: RSyntaxKind :: NA_COMPLEX_KW } ; [na_character] => { $ crate :: RSyntaxKind :: NA_CHARACTER_KW } ; [ident] => { $ crate :: RSyntaxKind :: IDENT } ; [EOF] => { $ crate :: RSyntaxKind :: EOF } ; [UNICODE_BOM] => { $ crate :: RSyntaxKind :: UNICODE_BOM } ; [#] => { $ crate :: RSyntaxKind :: HASH } ; }
