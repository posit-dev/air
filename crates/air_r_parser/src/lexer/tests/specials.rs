use super::*;

#[test]
fn special_operators() {
    assert_lex!("%%", SPECIAL: 2);
    assert_lex!("%/%", SPECIAL: 3);
    assert_lex!("%in%", SPECIAL: 4);
    assert_lex!("%o%", SPECIAL: 3);
    assert_lex!("%>%", SPECIAL: 3);
}

#[test]
fn in_context() {
    assert_lex!(
        "a %in% b",
        IDENT: 1,
        WHITESPACE: 1,
        SPECIAL: 4,
        WHITESPACE: 1,
        IDENT: 1,
    );
}

/// `%foo` is still a `SPECIAL` carrying a diagnostic, not an `ERROR_TOKEN`: the
/// text is a well-formed operator that was cut short, so the parser can still
/// use it. `ERROR_TOKEN` is for characters that start nothing at all.
#[test]
fn stops_at_a_newline() {
    assert_lex_err!(
        "%foo\n1",
        SPECIAL: 4, NEWLINE: 1, R_DOUBLE_LITERAL: 1
        => "Unterminated"
    );
}

#[test]
fn unterminated_at_end_of_file() {
    assert_lex_err!("x %in", IDENT: 1, WHITESPACE: 1, SPECIAL: 3 => "Unterminated");
}

/// R's `SpecialValue` does not exclude backslash, so `%a\b%` is a valid operator
/// in R
#[test]
fn a_backslash_is_ordinary_text() {
    assert_lex!(
        "a %a\\b% b",
        IDENT: 1,
        WHITESPACE: 1,
        SPECIAL: 5,
        WHITESPACE: 1,
        IDENT: 1,
    );
}
