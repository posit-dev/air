use super::*;

/// None of these is a prefix of a longer token, so they can be written adjacent
#[test]
fn single_byte_puncts() {
    assert_lex!(
        ";,{}()?~$@\\+/^",
        SEMICOLON: 1,
        COMMA: 1,
        L_CURLY: 1,
        R_CURLY: 1,
        L_PAREN: 1,
        R_PAREN: 1,
        WAT: 1,
        TILDE: 1,
        DOLLAR: 1,
        AT: 1,
        BACKSLASH: 1,
        PLUS: 1,
        DIVIDE: 1,
        EXPONENTIATE: 1,
    );
}

#[test]
fn less_than() {
    assert_lex!("<", LESS_THAN: 1);
    assert_lex!("<=", LESS_THAN_OR_EQUAL_TO: 2);
    assert_lex!("<-", ASSIGN: 2);
    assert_lex!("<<-", SUPER_ASSIGN: 3);
    assert_lex!("<<", LESS_THAN: 1, LESS_THAN: 1);
}

#[test]
fn greater_than() {
    assert_lex!(">", GREATER_THAN: 1);
    assert_lex!(">=", GREATER_THAN_OR_EQUAL_TO: 2);
    assert_lex!(">>", GREATER_THAN: 1, GREATER_THAN: 1);
}

#[test]
fn minus() {
    assert_lex!("-", MINUS: 1);
    assert_lex!("->", ASSIGN_RIGHT: 2);
    assert_lex!("->>", SUPER_ASSIGN_RIGHT: 3);
}

#[test]
fn colon() {
    assert_lex!(":", COLON: 1);
    assert_lex!("::", COLON2: 2);
    assert_lex!(":::", COLON3: 3);
    assert_lex!("::::", COLON3: 3, COLON: 1);
    assert_lex!(":=", WALRUS: 2);
}

#[test]
fn star() {
    assert_lex!("*", MULTIPLY: 1);
    assert_lex!("**", EXPONENTIATE2: 2);
    assert_lex!("***", EXPONENTIATE2: 2, MULTIPLY: 1);
}

#[test]
fn pipes() {
    assert_lex!("|", OR: 1);
    assert_lex!("||", OR2: 2);
    assert_lex!("|>", PIPE: 2);
}

#[test]
fn ampersands() {
    assert_lex!("&", AND: 1);
    assert_lex!("&&", AND2: 2);
}

#[test]
fn equals() {
    assert_lex!("=", EQUAL: 1);
    assert_lex!("==", EQUAL2: 2);
    assert_lex!("===", EQUAL2: 2, EQUAL: 1);
}

#[test]
fn bang() {
    assert_lex!("!", BANG: 1);
    assert_lex!("!=", NOT_EQUAL: 2);
}

#[test]
fn adjacency_changes_the_meaning() {
    assert_lex!("1<-2", R_DOUBLE_LITERAL: 1, ASSIGN: 2, R_DOUBLE_LITERAL: 1);
    assert_lex!(
        "1 < -2",
        R_DOUBLE_LITERAL: 1,
        WHITESPACE: 1,
        LESS_THAN: 1,
        WHITESPACE: 1,
        MINUS: 1,
        R_DOUBLE_LITERAL: 1,
    );
}

#[test]
fn a_byte_that_starts_no_token() {
    assert_lex_err!(
        "1\u{1}",
        R_DOUBLE_LITERAL: 1, ERROR_TOKEN: 1
        => "Unexpected character"
    );
}
