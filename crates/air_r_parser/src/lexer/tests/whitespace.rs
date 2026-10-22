use super::*;

#[test]
fn runs_coalesce() {
    assert_lex!("   ", WHITESPACE: 3);
    assert_lex!(" \t\u{0B}\u{0C} ", WHITESPACE: 5);
}

#[test]
fn whitespace_stops_at_a_line_break() {
    assert_lex!("  \n  ", WHITESPACE: 2, NEWLINE: 1, WHITESPACE: 2);
}

#[test]
fn line_feed_and_lone_carriage_return() {
    assert_lex!("\n", NEWLINE: 1);
    assert_lex!("\r", NEWLINE: 1);
}

#[test]
fn crlf_is_one_newline() {
    assert_lex!("\r\n", NEWLINE: 2);
}

#[test]
fn each_line_break_is_its_own_token() {
    assert_lex!("\n\n", NEWLINE: 1, NEWLINE: 1);
    // The first `\r` stands alone; only the second pairs with the `\n`
    assert_lex!("\r\r\n", NEWLINE: 1, NEWLINE: 2);
}
