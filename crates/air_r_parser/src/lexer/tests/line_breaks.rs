//! The sticky `PRECEDING_LINE_BREAK` flag.
//!
//! The lexer answers "was there a line break here", which is a lexical fact. The
//! parser decides whether that fact means anything, which in R is statement
//! termination and `else` attachment.

use super::*;

#[test]
fn the_first_token_has_no_preceding_break() {
    assert_line_breaks!("1", R_DOUBLE_LITERAL: false);
    assert_line_breaks!("1 + 2", R_DOUBLE_LITERAL: false, PLUS: false, R_DOUBLE_LITERAL: false);
}

#[test]
fn a_leading_newline_still_counts() {
    assert_line_breaks!("\n1", R_DOUBLE_LITERAL: true);
}

#[test]
fn set_by_a_newline_and_cleared_by_the_next_real_token() {
    assert_line_breaks!(
        "1\n2 3",
        R_DOUBLE_LITERAL: false,
        R_DOUBLE_LITERAL: true,
        R_DOUBLE_LITERAL: false,
    );
}

#[test]
fn survives_whitespace_and_comments() {
    // The break is reported on `+`, not on the whitespace that precedes it and
    // not on the `2` that follows it
    assert_line_breaks!(
        "1\n  # c\n  + 2",
        R_DOUBLE_LITERAL: false,
        PLUS: true,
        R_DOUBLE_LITERAL: false,
    );
}

#[test]
fn a_trailing_comment_does_not_set_it() {
    assert_line_breaks!(
        "1 # c\n2",
        R_DOUBLE_LITERAL: false,
        R_DOUBLE_LITERAL: true,
    );
}

#[test]
fn strings_report_the_break() {
    assert_line_breaks!(
        "1\n\"a\"",
        R_DOUBLE_LITERAL: false,
        R_DOUBLE_QUOTED_STRING_LITERAL: true,
    );
}
