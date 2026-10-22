use super::*;

#[test]
fn to_end_of_line() {
    assert_lex!("# comment\n1", COMMENT: 9, NEWLINE: 1, R_DOUBLE_LITERAL: 1);
}

#[test]
fn ending_the_file_without_a_newline() {
    assert_lex!("1 # comment", R_DOUBLE_LITERAL: 1, WHITESPACE: 1, COMMENT: 9);
}

#[test]
fn empty_comment() {
    assert_lex!("#", COMMENT: 1);
}

/// The `\r` belongs to the `NEWLINE`, not to the comment
#[test]
fn crlf_stays_out_of_the_comment() {
    assert_lex!("# x\r\n", COMMENT: 3, NEWLINE: 2);
}

#[test]
fn non_ascii_inside_a_comment() {
    assert_lex!("# héllo", COMMENT: 8);
}

/// Nothing inside a comment is lexed, however token-shaped it looks
#[test]
fn contents_are_inert() {
    assert_lex!("# \"a\" `b` %in% 1L", COMMENT: 17);
}
