use super::*;

/// The token spans the whole character, not its first byte. An off-by-one here
/// would split a multi-byte character and break round-tripping.
#[test]
fn spans_the_whole_character() {
    assert_lex_err!("\u{1}", ERROR_TOKEN: 1 => "Unexpected character");
    assert_lex_err!("£", ERROR_TOKEN: 2 => "Unexpected character");
    assert_lex_err!("€", ERROR_TOKEN: 3 => "Unexpected character");
    assert_lex_err!("🎉", ERROR_TOKEN: 4 => "Unexpected character");
}

/// One token and one diagnostic per character, each naming its own character.
/// Merging a run into a single token would be cheaper to report and worse to
/// read, and it would hide the second character from the parser entirely.
#[test]
fn each_character_is_its_own_token() {
    assert_lex_err!(
        "€£",
        ERROR_TOKEN: 3, ERROR_TOKEN: 2
        => "Unexpected character `€`", "Unexpected character `£`"
    );
}

/// The lexer does not stop, and the tokens after the error are the ones a clean
/// source would have produced
#[test]
fn lexing_continues_afterwards() {
    assert_lex_err!(
        "1 + \u{1} + 2",
        R_DOUBLE_LITERAL: 1, WHITESPACE: 1, PLUS: 1, WHITESPACE: 1,
        ERROR_TOKEN: 1,
        WHITESPACE: 1, PLUS: 1, WHITESPACE: 1, R_DOUBLE_LITERAL: 1
        => "Unexpected character"
    );
}

#[test]
fn breaks_an_identifier_in_two() {
    assert_lex_err!(
        "a€b",
        IDENT: 1, ERROR_TOKEN: 3, IDENT: 1
        => "Unexpected character"
    );
}

/// An error token is not trivia, so it takes the sticky break flag and clears it
/// rather than passing it along to the next real token
#[test]
fn carries_a_preceding_line_break() {
    assert_line_breaks!(
        "\n\u{1}1",
        ERROR_TOKEN: true,
        R_DOUBLE_LITERAL: false,
    );
}
