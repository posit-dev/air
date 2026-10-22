use super::*;

#[test]
fn control_flow_keywords() {
    assert_lex!("function", FUNCTION_KW: 8);
    assert_lex!("for", FOR_KW: 3);
    assert_lex!("in", IN_KW: 2);
    assert_lex!("while", WHILE_KW: 5);
    assert_lex!("repeat", REPEAT_KW: 6);
    assert_lex!("if", IF_KW: 2);
    assert_lex!("else", ELSE_KW: 4);
    assert_lex!("next", NEXT_KW: 4);
    assert_lex!("break", BREAK_KW: 5);
}

#[test]
fn constants() {
    assert_lex!("TRUE", TRUE_KW: 4);
    assert_lex!("FALSE", FALSE_KW: 5);
    assert_lex!("NULL", NULL_KW: 4);
    assert_lex!("Inf", INF_KW: 3);
    assert_lex!("NaN", NAN_KW: 3);
}

#[test]
fn missing_values() {
    assert_lex!("NA", NA_LOGICAL_KW: 2);
    assert_lex!("NA_integer_", NA_INTEGER_KW: 11);
    assert_lex!("NA_real_", NA_DOUBLE_KW: 8);
    assert_lex!("NA_complex_", NA_COMPLEX_KW: 11);
    assert_lex!("NA_character_", NA_CHARACTER_KW: 13);
}

#[test]
fn keywords_are_case_sensitive() {
    assert_lex!("If", IDENT: 2);
    assert_lex!("Function", IDENT: 8);
    assert_lex!("true", IDENT: 4);
    assert_lex!("false", IDENT: 5);
    assert_lex!("null", IDENT: 4);
    assert_lex!("na", IDENT: 2);
    assert_lex!("INF", IDENT: 3);
}

/// Lexing the whole name before consulting the keyword table is what gets these
/// right for free. tree-sitter-r had to fix `elseμ` specifically (r-lib/tree-sitter-r#200).
#[test]
fn a_keyword_prefix_is_not_a_keyword() {
    assert_lex!("elseif", IDENT: 6);
    assert_lex!("else_idx", IDENT: 8);
    assert_lex!("elseμ", IDENT: 6);
    assert_lex!("NA_real", IDENT: 7);
    assert_lex!("NA_real__", IDENT: 9);
}

#[test]
fn backticked_keywords_stay_identifiers() {
    assert_lex!("`if`", IDENT: 4);
    assert_lex!("`TRUE`", IDENT: 6);
}
