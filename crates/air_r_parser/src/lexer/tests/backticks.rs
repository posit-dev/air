use super::*;

#[test]
fn one_token_spanning_the_delimiters() {
    assert_lex!("`a b`", IDENT: 5);
    assert_lex!("`a b` <- 1", IDENT: 5, WHITESPACE: 1, ASSIGN: 2, WHITESPACE: 1, R_DOUBLE_LITERAL: 1);
}

#[test]
fn empty_name() {
    assert_lex!("``", IDENT: 2);
}

/// An escaped backtick does not close the name
#[test]
fn escapes_apply() {
    assert_lex!("`a\\`b`", IDENT: 6);
    assert_lex!("`a\\\\`", IDENT: 5);
}

#[test]
fn names_may_span_newlines() {
    assert_lex!(
        "`my\nvar` <- 1",
        IDENT: 8,
        WHITESPACE: 1,
        ASSIGN: 2,
        WHITESPACE: 1,
        R_DOUBLE_LITERAL: 1,
    );
}

/// There is no line-based early stop available, so an unterminated name really
/// does run to the end of the file, exactly as in R and in tree-sitter-r. What
/// keeps the error useful is that it points at the opening delimiter rather than
/// highlighting the rest of the file.
#[test]
fn unterminated_runs_to_the_end_of_the_file() {
    assert_lex_err!(
        "`a\nb <- 1",
        ERROR_TOKEN: 9
        => "Unterminated backtick-quoted name"
    );
}

#[test]
fn a_trailing_escape_does_not_run_past_the_end() {
    assert_lex_err!("`a\\", ERROR_TOKEN: 3 => "Unterminated backtick-quoted name");
}
