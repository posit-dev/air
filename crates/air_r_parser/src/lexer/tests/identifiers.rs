use super::*;

#[test]
fn plain_names() {
    assert_lex!("foo", IDENT: 3);
    assert_lex!("a1", IDENT: 2);
    assert_lex!("foo.bar", IDENT: 7);
    assert_lex!("x_1.y", IDENT: 5);
}

/// `_` is not a valid way to start an R name, but tree-sitter-r is deliberately
/// lax: a lone `_` is the native pipe placeholder, and lexing `_foo` as one name
/// is what lets a downstream consumer report it as an invalid one. Splitting it
/// into `_` and `foo` would make that check impossible (r-lib/tree-sitter-r#71).
///
/// TODO: We may consider making this stricter now that we don't rely on tree sitter
#[test]
fn leading_underscore_is_one_name() {
    assert_lex!("_", IDENT: 1);
    assert_lex!("_foo", IDENT: 4);
}

#[test]
fn names_may_start_with_a_dot() {
    assert_lex!(".", IDENT: 1);
    assert_lex!("..", IDENT: 2);
    assert_lex!(".foo", IDENT: 4);
    assert_lex!("._foo", IDENT: 5);
}

#[test]
fn unicode_starts_and_continues() {
    assert_lex!("café", IDENT: 5);
    assert_lex!("μ", IDENT: 2);
    assert_lex!("日本語", IDENT: 9);
    assert_lex!("aμ1", IDENT: 4);
}

/// A character that is neither XID_Start nor punctuation starts no token at all
#[test]
fn a_character_that_is_not_an_identifier_start() {
    assert_lex_err!("€", ERROR_TOKEN: 3 => "Unexpected character");
}
