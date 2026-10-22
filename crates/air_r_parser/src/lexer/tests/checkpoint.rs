use super::*;

#[test]
fn rewind_restores_the_current_token() {
    let mut lexer = RLexer::from_str("a + b");

    assert_eq!(lexer.next_token(RLexContext::Regular), IDENT);
    let checkpoint = lexer.checkpoint();

    assert_eq!(lexer.next_token(RLexContext::Regular), WHITESPACE);
    assert_eq!(lexer.next_token(RLexContext::Regular), PLUS);

    lexer.rewind(checkpoint);
    assert_eq!(lexer.current(), IDENT);
    assert_eq!(lexer.current_range(), TextRange::new(0.into(), 1.into()));
    assert_eq!(lexer.next_token(RLexContext::Regular), WHITESPACE);
}

#[test]
fn rewind_restores_the_sticky_line_break() {
    let mut lexer = RLexer::from_str("1\n2");

    assert_eq!(lexer.next_token(RLexContext::Regular), R_DOUBLE_LITERAL);
    assert_eq!(lexer.next_token(RLexContext::Regular), NEWLINE);
    assert!(lexer.after_line_break);

    // Checkpointed with the break still pending, and consumed straight after
    let checkpoint = lexer.checkpoint();
    assert_eq!(lexer.next_token(RLexContext::Regular), R_DOUBLE_LITERAL);
    assert!(lexer.has_preceding_line_break());
    assert!(!lexer.after_line_break);

    lexer.rewind(checkpoint);
    assert!(lexer.after_line_break);
    assert_eq!(lexer.next_token(RLexContext::Regular), R_DOUBLE_LITERAL);
    assert!(lexer.has_preceding_line_break());
}

#[test]
fn rewind_restores_the_current_flags() {
    let mut lexer = RLexer::from_str("1\n2 3");

    assert_eq!(lexer.next_token(RLexContext::Regular), R_DOUBLE_LITERAL);
    assert_eq!(lexer.next_token(RLexContext::Regular), NEWLINE);
    assert_eq!(lexer.next_token(RLexContext::Regular), R_DOUBLE_LITERAL);
    let checkpoint = lexer.checkpoint();

    assert_eq!(lexer.next_token(RLexContext::Regular), WHITESPACE);
    assert_eq!(lexer.next_token(RLexContext::Regular), R_DOUBLE_LITERAL);
    assert!(!lexer.has_preceding_line_break());

    lexer.rewind(checkpoint);
    assert!(lexer.has_preceding_line_break());
}

#[test]
fn rewind_restores_the_bom_length() {
    let mut lexer = RLexer::from_str("\u{FEFF}1");

    assert_eq!(lexer.next_token(RLexContext::Regular), UNICODE_BOM);
    assert_eq!(lexer.unicode_bom_length, 3);
    let checkpoint = lexer.checkpoint();

    assert_eq!(lexer.next_token(RLexContext::Regular), R_DOUBLE_LITERAL);

    lexer.rewind(checkpoint);
    assert_eq!(lexer.unicode_bom_length, 3);
}

/// Diagnostics raised after the checkpoint are dropped, so a rewind cannot leave
/// the parser reporting an error about text it decided not to lex that way
#[test]
fn rewind_truncates_diagnostics() {
    let mut lexer = RLexer::from_str("a %in");

    assert_eq!(lexer.next_token(RLexContext::Regular), IDENT);
    assert_eq!(lexer.next_token(RLexContext::Regular), WHITESPACE);
    let checkpoint = lexer.checkpoint();

    assert_eq!(lexer.next_token(RLexContext::Regular), SPECIAL);
    assert_eq!(lexer.diagnostics.len(), 1);

    lexer.rewind(checkpoint);
    assert_eq!(lexer.diagnostics.len(), 0);
}

/// The `None` that sends an `r` down the identifier arm instead
#[test]
fn a_name_is_not_a_raw_string_open() {
    assert_eq!(RStringDelimiter::try_from_raw_string_open(""), None);
    assert_eq!(RStringDelimiter::try_from_raw_string_open("rnorm"), None);
    assert_eq!(RStringDelimiter::try_from_raw_string_open("r\"-x("), None);
    assert_eq!(RStringDelimiter::try_from_raw_string_open("\""), None);
}
