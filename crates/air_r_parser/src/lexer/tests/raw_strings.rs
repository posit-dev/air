use super::*;

#[test]
fn every_bracket_pair() {
    assert_lex!("r\"(abc)\"", R_RAW_STRING_LITERAL: 8);
    assert_lex!("r\"[abc]\"", R_RAW_STRING_LITERAL: 8);
    assert_lex!("r\"{abc}\"", R_RAW_STRING_LITERAL: 8);
    assert_lex!("r\"|abc|\"", R_RAW_STRING_LITERAL: 8);
}

#[test]
fn hyphen_counts() {
    assert_lex!("r\"(a)\"", R_RAW_STRING_LITERAL: 6);
    assert_lex!("r\"-(a)-\"", R_RAW_STRING_LITERAL: 8);
    assert_lex!("r\"---(a)---\"", R_RAW_STRING_LITERAL: 12);
    assert_lex!("r\"--|a|--\"", R_RAW_STRING_LITERAL: 10);
}

#[test]
fn uppercase_prefix_and_single_quotes() {
    assert_lex!("R'---[x]---'", R_RAW_STRING_LITERAL: 12);
    assert_lex!("R\"(x)\"", R_RAW_STRING_LITERAL: 6);
    assert_lex!("R'|x|'", R_RAW_STRING_LITERAL: 6);
}

/// A closing bracket that turns out not to start the closing sequence is
/// content, and the scan resumes from where the attempt stopped rather than from
/// the byte after the bracket. Getting that wrong makes both of these
/// unterminated (r-lib/tree-sitter-r#162).
#[test]
fn a_bracket_that_does_not_close() {
    assert_lex!("r\"(())\"", R_RAW_STRING_LITERAL: 7);
    assert_lex!("r\"-())-\"", R_RAW_STRING_LITERAL: 8);
    assert_lex!("r\"|||\"", R_RAW_STRING_LITERAL: 6);
    assert_lex!("r\"|a|b|\"", R_RAW_STRING_LITERAL: 8);
    assert_lex!("r\"-|a|-|-\"", R_RAW_STRING_LITERAL: 10);
}

/// The bracket matches but the hyphen count does not, so the string never
/// closes and the content runs to the end of the file
#[test]
fn the_hyphen_count_has_to_match() {
    assert_lex_err!("r\"-(a)\"", ERROR_TOKEN: 7 => "Unterminated string");
}

#[test]
fn unterminated_runs_to_the_end_of_the_file() {
    assert_lex_err!("r\"(abc", ERROR_TOKEN: 6 => "Unterminated string");
    assert_lex_err!("r\"|abc", ERROR_TOKEN: 6 => "Unterminated string");
}

/// The prefix has to be complete, so anything short of it falls through to the
/// identifier and punctuation arms
#[test]
fn an_incomplete_prefix_is_just_a_name() {
    assert_lex!("rnorm", IDENT: 5);
    assert_lex!("r + 1", IDENT: 1, WHITESPACE: 1, PLUS: 1, WHITESPACE: 1, R_DOUBLE_LITERAL: 1);
    // A quote with no bracket after it: an ordinary name and an ordinary string
    assert_lex!("r\"abc\"", IDENT: 1, R_DOUBLE_QUOTED_STRING_LITERAL: 5);
    assert_lex!("r\"-abc\"", IDENT: 1, R_DOUBLE_QUOTED_STRING_LITERAL: 6);
}

/// Escapes are not special inside a raw string
#[test]
fn backslashes_are_literal() {
    assert_lex!("r\"(a\\)\"", R_RAW_STRING_LITERAL: 7);
}

#[test]
fn raw_strings_may_span_newlines() {
    assert_lex!("r\"(a\nb)\"", R_RAW_STRING_LITERAL: 8);
}
