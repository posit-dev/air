use super::*;

#[test]
fn decimals() {
    assert_lex!("1", R_DOUBLE_LITERAL: 1);
    assert_lex!("123", R_DOUBLE_LITERAL: 3);
    assert_lex!("1.", R_DOUBLE_LITERAL: 2);
    assert_lex!("1.5", R_DOUBLE_LITERAL: 3);
    assert_lex!(".5", R_DOUBLE_LITERAL: 2);
}

#[test]
fn suffixes() {
    assert_lex!("1L", R_INTEGER_LITERAL: 2);
    assert_lex!("1i", R_COMPLEX_LITERAL: 2);
    assert_lex!("1.5L", R_INTEGER_LITERAL: 4);
    assert_lex!("1e5L", R_INTEGER_LITERAL: 4);
    assert_lex!("1L(", R_INTEGER_LITERAL: 2, L_PAREN: 1);
}

/// Suffix is taken greedily
#[test]
fn a_suffix_is_always_taken() {
    assert_lex!("1Li", R_INTEGER_LITERAL: 2, IDENT: 1);
    assert_lex!("1LL", R_INTEGER_LITERAL: 2, IDENT: 1);
    assert_lex!("1iL", R_COMPLEX_LITERAL: 2, IDENT: 1);
    assert_lex!("1L_", R_INTEGER_LITERAL: 2, IDENT: 1);
    assert_lex!("1LForever", R_INTEGER_LITERAL: 2, IDENT: 7);
    assert_lex!("1e5iL", R_COMPLEX_LITERAL: 4, IDENT: 1);
    // `.` starts a name of its own here, rather than continuing the literal
    assert_lex!("1L.", R_INTEGER_LITERAL: 2, IDENT: 1);
    assert_lex!(".1L.", R_INTEGER_LITERAL: 3, IDENT: 1);
}

#[test]
fn exponents() {
    assert_lex!("1e5", R_DOUBLE_LITERAL: 3);
    assert_lex!("1e+3", R_DOUBLE_LITERAL: 4);
    assert_lex!("1e-3", R_DOUBLE_LITERAL: 4);
    assert_lex!("1.e5", R_DOUBLE_LITERAL: 4);
}

/// An `e` commits to an exponent, so its digits are required
#[test]
fn a_decimal_exponent_with_no_digits() {
    assert_lex_err!("1e", R_DOUBLE_LITERAL: 2 => "Unterminated exponent");
    assert_lex_err!("1e+", R_DOUBLE_LITERAL: 3 => "Unterminated exponent");
    assert_lex_err!("1e-", R_DOUBLE_LITERAL: 3 => "Unterminated exponent");
    // The literal still stops where it stopped, so the name after it is intact
    assert_lex_err!("1east", R_DOUBLE_LITERAL: 2, IDENT: 3 => "Unterminated exponent");
    assert_lex_err!("1e-a", R_DOUBLE_LITERAL: 3, IDENT: 1 => "Unterminated exponent");
    // A space does not rescue it either, in R or here
    assert_lex_err!(
        "1e + 2",
        R_DOUBLE_LITERAL: 2, WHITESPACE: 1, PLUS: 1, WHITESPACE: 1, R_DOUBLE_LITERAL: 1
        => "Unterminated exponent"
    );
}

#[test]
fn hexadecimals() {
    assert_lex!("0xFF", R_DOUBLE_LITERAL: 4);
    assert_lex!("0XFF", R_DOUBLE_LITERAL: 4);
    assert_lex!("0xFFL", R_INTEGER_LITERAL: 5);
    assert_lex!("0x1.8", R_DOUBLE_LITERAL: 5);
    // R accepts a hexadecimal with nothing after the dot, surprisingly
    assert_lex!("0x.", R_DOUBLE_LITERAL: 3);
    assert_lex!("0x.8", R_DOUBLE_LITERAL: 4);
    // `e` is a hexadecimal digit, not the start of a decimal exponent
    assert_lex!("0x1e", R_DOUBLE_LITERAL: 4);
}

#[test]
fn binary_exponents() {
    assert_lex!("0x1p3", R_DOUBLE_LITERAL: 5);
    assert_lex!("0x.8p1", R_DOUBLE_LITERAL: 6);
    assert_lex!("0x1p+3", R_DOUBLE_LITERAL: 6);
    assert_lex!("0x1p-3", R_DOUBLE_LITERAL: 6);
    assert_lex!("0x.p3", R_DOUBLE_LITERAL: 5);
    assert_lex!("0x1.8p3", R_DOUBLE_LITERAL: 7);
}

/// A `p` commits to a binary exponent
#[test]
fn a_binary_exponent_with_no_digits() {
    assert_lex_err!("0x1p", R_DOUBLE_LITERAL: 4 => "Unterminated binary exponent");
    assert_lex_err!("0x1p+", R_DOUBLE_LITERAL: 5 => "Unterminated binary exponent");
    assert_lex_err!("0x1p-", R_DOUBLE_LITERAL: 5 => "Unterminated binary exponent");
    assert_lex_err!("0x.p", R_DOUBLE_LITERAL: 4 => "Unterminated binary exponent");
    assert_lex_err!("0x.8p", R_DOUBLE_LITERAL: 5 => "Unterminated binary exponent");
}

/// `0x` commits to a hexadecimal
#[test]
fn a_hexadecimal_with_no_body() {
    assert_lex_err!("0x", R_DOUBLE_LITERAL: 2 => "Unterminated hexadecimal");
    assert_lex_err!("0xg", R_DOUBLE_LITERAL: 2, IDENT: 1 => "Unterminated hexadecimal");
    // The body may not start with the binary exponent either
    assert_lex_err!("0xp3", R_DOUBLE_LITERAL: 2, IDENT: 2 => "Unterminated hexadecimal");
}

#[test]
fn a_second_dot_starts_a_second_literal() {
    assert_lex!("1.5.3", R_DOUBLE_LITERAL: 3, R_DOUBLE_LITERAL: 2);
}
