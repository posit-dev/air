use super::*;

#[test]
fn double_quoted() {
    assert_lex!("\"a\"", R_DOUBLE_QUOTED_STRING_LITERAL: 3);
}

#[test]
fn single_quoted() {
    assert_lex!("'abc'", R_SINGLE_QUOTED_STRING_LITERAL: 5);
}

#[test]
fn an_empty_string_is_one_token() {
    assert_lex!("\"\"", R_DOUBLE_QUOTED_STRING_LITERAL: 2);
    assert_lex!("''", R_SINGLE_QUOTED_STRING_LITERAL: 2);
    assert_lex!("r\"()\"", R_RAW_STRING_LITERAL: 5);
}

#[test]
fn escapes_stay_inside_the_string() {
    assert_lex!("\"a\\\"b\"", R_DOUBLE_QUOTED_STRING_LITERAL: 6);
    assert_lex!("\"a\\\\\"", R_DOUBLE_QUOTED_STRING_LITERAL: 5);
    assert_lex!("'a\\'b'", R_SINGLE_QUOTED_STRING_LITERAL: 6);
}

#[test]
fn strings_may_span_newlines() {
    assert_lex!("\"foo\nbar\"", R_DOUBLE_QUOTED_STRING_LITERAL: 9);
}

#[test]
fn a_crlf_inside_a_string_is_content() {
    assert_lex!(
        "\"multiline\r\nstring\"\r\n",
        R_DOUBLE_QUOTED_STRING_LITERAL: 19,
        NEWLINE: 2
    );
}

#[test]
fn the_other_quote_is_inert() {
    assert_lex!("\"it's\"", R_DOUBLE_QUOTED_STRING_LITERAL: 6);
    assert_lex!("'say \"hi\"'", R_SINGLE_QUOTED_STRING_LITERAL: 10);
}

#[test]
fn comments_and_operators_are_inert() {
    assert_lex!("\"# %in% `a`\"", R_DOUBLE_QUOTED_STRING_LITERAL: 12);
}

/// There is no closing delimiter, so the token cannot claim to be a string. What
/// keeps the error useful is that it points at the opening delimiter rather than
/// highlighting the rest of the file.
#[test]
fn unterminated_runs_to_the_end_of_the_file() {
    assert_lex_err!("\"abc", ERROR_TOKEN: 4 => "Unterminated string");
    assert_lex_err!("\"", ERROR_TOKEN: 1 => "Unterminated string");
    assert_lex_err!("'a\nb", ERROR_TOKEN: 4 => "Unterminated string");
}

/// A backslash at the very end of the file must not read past it
#[test]
fn a_trailing_escape_does_not_run_past_the_end() {
    assert_lex_err!("\"a\\", ERROR_TOKEN: 3 => "Unterminated string");
}

#[test]
fn in_context() {
    assert_lex!(
        "x <- \"a\"",
        IDENT: 1,
        WHITESPACE: 1,
        ASSIGN: 2,
        WHITESPACE: 1,
        R_DOUBLE_QUOTED_STRING_LITERAL: 3,
    );
}
