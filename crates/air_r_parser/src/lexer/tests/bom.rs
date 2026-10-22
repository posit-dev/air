use super::*;

#[test]
fn bare_bom() {
    assert_lex!("\u{FEFF}", UNICODE_BOM: 3);
}

#[test]
fn bom_before_a_token() {
    assert_lex!("\u{FEFF}1", UNICODE_BOM: 3, R_DOUBLE_LITERAL: 1);
}

#[test]
fn a_bom_is_only_a_bom_at_offset_zero() {
    // U+FEFF is category Cf, so it is neither XID_Start nor XID_Continue and
    // cannot join the identifier either
    assert_lex_err!(
        "a\u{FEFF}",
        IDENT: 1, ERROR_TOKEN: 3
        => "Unexpected character"
    );
}

// `consume_potential_bom` matches on `source().get(0..3)`, so a source shorter
// than three bytes has to fall through rather than panic or mis-slice
#[test]
fn source_shorter_than_a_bom() {
    assert_lex!("a", IDENT: 1);
    assert_lex!("é", IDENT: 2);
}
