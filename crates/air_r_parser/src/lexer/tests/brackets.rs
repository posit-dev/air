//! `[[` is lexed greedily, `]]` is not

use super::*;

#[test]
fn adjacent_open_brackets_are_one_token() {
    assert_lex!("[[", L_BRACK2: 2);
}

#[test]
fn adjacent_close_brackets_are_not() {
    assert_lex!("]]", R_BRACK: 1, R_BRACK: 1);
}

#[test]
fn separated_open_brackets_are_not() {
    assert_lex!(
        "x[ [1] ]",
        IDENT: 1,
        L_BRACK: 1,
        WHITESPACE: 1,
        L_BRACK: 1,
        R_DOUBLE_LITERAL: 1,
        R_BRACK: 1,
        WHITESPACE: 1,
        R_BRACK: 1,
    );
}

#[test]
fn a_third_open_bracket_stands_alone() {
    assert_lex!("[[[", L_BRACK2: 2, L_BRACK: 1);
}

#[test]
fn join_takes_both_brackets() {
    let mut lexer = RLexer::from_str("]]a");

    assert_eq!(lexer.next_token(RLexContext::Regular), R_BRACK);
    assert_eq!(lexer.re_lex(RReLexContext::JoinBrackets), R_BRACK2);
    assert_eq!(lexer.current_range(), TextRange::new(0.into(), 2.into()));

    assert_eq!(lexer.next_token(RLexContext::Regular), IDENT);
    assert_eq!(lexer.current_range(), TextRange::new(2.into(), 3.into()));
}

/// `x[[a[[b]]]]` — the outer subset2 joins the pair the inner one left behind
#[test]
fn a_joined_pair_leaves_the_rest_of_the_run() {
    let mut lexer = RLexer::from_str("]]]]");

    assert_eq!(lexer.next_token(RLexContext::Regular), R_BRACK);
    assert_eq!(lexer.re_lex(RReLexContext::JoinBrackets), R_BRACK2);

    assert_eq!(lexer.next_token(RLexContext::Regular), R_BRACK);
    assert_eq!(lexer.re_lex(RReLexContext::JoinBrackets), R_BRACK2);
    assert_eq!(lexer.current_range(), TextRange::new(2.into(), 4.into()));
}

#[test]
fn join_is_idempotent() {
    let mut lexer = RLexer::from_str("]]");

    assert_eq!(lexer.next_token(RLexContext::Regular), R_BRACK);
    assert_eq!(lexer.re_lex(RReLexContext::JoinBrackets), R_BRACK2);
    assert_eq!(lexer.re_lex(RReLexContext::JoinBrackets), R_BRACK2);
    assert_eq!(lexer.current_range(), TextRange::new(0.into(), 2.into()));
}

/// `x[[1] ]` and issue #177's `x[[1]\n]` both land here: the brackets must be
/// adjacent, and a no-op join must leave the trivia between them intact
#[test]
fn join_needs_the_brackets_adjacent() {
    for source in ["] ]", "]\n]"] {
        let mut lexer = RLexer::from_str(source);

        assert_eq!(lexer.next_token(RLexContext::Regular), R_BRACK);
        assert_eq!(lexer.re_lex(RReLexContext::JoinBrackets), R_BRACK);
        assert_eq!(lexer.current_range(), TextRange::new(0.into(), 1.into()));

        assert!(matches!(
            lexer.next_token(RLexContext::Regular),
            WHITESPACE | NEWLINE
        ));
        assert_eq!(lexer.next_token(RLexContext::Regular), R_BRACK);
        assert_eq!(lexer.next_token(RLexContext::Regular), EOF);
    }
}

#[test]
fn join_is_a_no_op_at_the_end_of_the_source() {
    let mut lexer = RLexer::from_str("]");

    assert_eq!(lexer.next_token(RLexContext::Regular), R_BRACK);
    assert_eq!(lexer.re_lex(RReLexContext::JoinBrackets), R_BRACK);
    assert_eq!(lexer.current_range(), TextRange::new(0.into(), 1.into()));
}

/// A join rewinds to the start of the current token, so it must not lose the
/// line break flag
#[test]
fn join_preserves_the_preceding_line_break() {
    let mut lexer = RLexer::from_str("1\n]]");

    assert_eq!(lexer.next_token(RLexContext::Regular), R_DOUBLE_LITERAL);
    assert_eq!(lexer.next_token(RLexContext::Regular), NEWLINE);
    assert_eq!(lexer.next_token(RLexContext::Regular), R_BRACK);
    assert!(lexer.has_preceding_line_break());

    assert_eq!(lexer.re_lex(RReLexContext::JoinBrackets), R_BRACK2);
    assert!(lexer.has_preceding_line_break());
}
