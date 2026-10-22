use super::*;

#[test]
fn dispatch() {
    assert_lex!(
        "... ..1 .5 .foo . ..",
        DOTS: 3,
        WHITESPACE: 1,
        DOTDOTI: 3,
        WHITESPACE: 1,
        R_DOUBLE_LITERAL: 2,
        WHITESPACE: 1,
        IDENT: 4,
        WHITESPACE: 1,
        IDENT: 1,
        WHITESPACE: 1,
        IDENT: 2,
    );
}

#[test]
fn dot_dot_i_takes_every_digit() {
    assert_lex!("..1", DOTDOTI: 3);
    assert_lex!("..12", DOTDOTI: 4);
}

/// Anything the name scanner picks up beyond the `...` or the digits leaves an
/// ordinary identifier, however dot-like the start of it looks
#[test]
fn a_name_that_continues_is_an_identifier() {
    assert_lex!("....", IDENT: 4);
    assert_lex!("...a", IDENT: 4);
    assert_lex!("...1", IDENT: 4);
    assert_lex!("..1a", IDENT: 4);
    assert_lex!("..1.", IDENT: 4);
    assert_lex!("..1e5", IDENT: 5);
}

/// A digit after the `.` always makes a number, never a name, in R and in
/// tree-sitter-r alike: otherwise `.1L` and `.1i` lex as identifiers
/// (r-lib/tree-sitter-r#190)
#[test]
fn a_digit_after_the_dot_makes_a_number() {
    assert_lex!(".5", R_DOUBLE_LITERAL: 2);
    assert_lex!(".1L", R_INTEGER_LITERAL: 3);
    assert_lex!(".1i", R_COMPLEX_LITERAL: 3);
    // The number stops at the first character it cannot use, and a name starts
    // there
    assert_lex!(".5abc", R_DOUBLE_LITERAL: 2, IDENT: 3);
}

#[test]
fn dots_in_context() {
    assert_lex!(
        "f(..., ..1)",
        IDENT: 1,
        L_PAREN: 1,
        DOTS: 3,
        COMMA: 1,
        WHITESPACE: 1,
        DOTDOTI: 3,
        R_PAREN: 1,
    );
}
