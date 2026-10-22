use super::*;
use air_r_syntax::RSyntaxKind::R_BRACK;
use air_r_syntax::RSyntaxKind::R_BRACK2;
use air_r_syntax::RSyntaxKind::R_DOUBLE_LITERAL;
use biome_parser::prelude::NthToken;
use biome_rowan::TextSize;

enum Pos {
    Leading,
    Trailing,
}

/// Drive the token source to `EOF` and hand back the trivia it collected
fn trivia(source: &str) -> Vec<Trivia> {
    drain(RTokenSource::from_str(source))
}

fn drain(mut source: RTokenSource) -> Vec<Trivia> {
    while source.current() != EOF {
        source.bump();
    }

    source.finish().0
}

fn ws(start: u32, end: u32, position: Pos) -> Trivia {
    piece(TriviaPieceKind::Whitespace, start, end, position)
}

fn nl(start: u32, end: u32) -> Trivia {
    piece(TriviaPieceKind::Newline, start, end, Pos::Leading)
}

fn cmt(start: u32, end: u32, position: Pos) -> Trivia {
    piece(TriviaPieceKind::SingleLineComment, start, end, position)
}

fn skipped(start: u32, end: u32) -> Trivia {
    piece(TriviaPieceKind::Skipped, start, end, Pos::Leading)
}

fn piece(kind: TriviaPieceKind, start: u32, end: u32, position: Pos) -> Trivia {
    let range = TextRange::new(TextSize::from(start), TextSize::from(end));
    Trivia::new(kind, range, matches!(position, Pos::Trailing))
}

#[track_caller]
fn assert_eq_trivia(actual: Vec<Trivia>, expected: Vec<Trivia>) {
    // `Trivia` implements neither `PartialEq` nor a useful `Debug`
    let key = |trivia: &Trivia| (trivia.kind(), trivia.text_range(), trivia.trailing());
    let actual: Vec<_> = actual.iter().map(key).collect();
    let expected: Vec<_> = expected.iter().map(key).collect();

    assert_eq!(actual, expected);
}

#[test]
fn test_parse_trivia_smoke_test() {
    assert_eq_trivia(
        trivia("1 + 1"),
        vec![ws(1, 2, Pos::Trailing), ws(3, 4, Pos::Trailing)],
    );
}

#[test]
fn test_parse_trivia_tab_test() {
    assert_eq_trivia(
        trivia("1\t+\t\n\t1"),
        vec![
            ws(1, 2, Pos::Trailing),
            ws(3, 4, Pos::Trailing),
            nl(4, 5),
            ws(5, 6, Pos::Leading),
        ],
    );
}

#[test]
fn test_parse_trivia_trailing_test() {
    assert_eq_trivia(
        trivia("1 + \n1"),
        vec![ws(1, 2, Pos::Trailing), ws(3, 4, Pos::Trailing), nl(4, 5)],
    );

    assert_eq_trivia(
        trivia("1  \n "),
        vec![ws(1, 3, Pos::Trailing), nl(3, 4), ws(4, 5, Pos::Leading)],
    );

    assert_eq_trivia(
        trivia("1 + \r\n1"),
        vec![ws(1, 2, Pos::Trailing), ws(3, 4, Pos::Trailing), nl(4, 6)],
    );
}

#[test]
fn test_parse_trivia_before_first_token() {
    assert_eq_trivia(trivia("  \n1"), vec![ws(0, 2, Pos::Leading), nl(2, 3)]);
}

#[test]
fn test_parse_trivia_comment_test() {
    assert_eq_trivia(
        trivia("1 #"),
        vec![ws(1, 2, Pos::Trailing), cmt(2, 3, Pos::Trailing)],
    );
}

#[test]
fn test_parse_trivia_comment_crlf_test() {
    // Trailing comment: `1 # hi\r\n` (r-lib/tree-sitter-r#184)
    assert_eq_trivia(
        trivia("1 # hi\r\n"),
        vec![ws(1, 2, Pos::Trailing), cmt(2, 6, Pos::Trailing), nl(6, 8)],
    );

    // Own-line comment: `# hi\r\n1`
    assert_eq_trivia(trivia("# hi\r\n1"), vec![cmt(0, 4, Pos::Leading), nl(4, 6)]);
}

#[test]
fn test_parse_trivia_crlf_leading_and_trailing_comments() {
    assert_eq_trivia(
        trivia("# Leading\r\n1 + 1 # Trailing\r\n# Leading\r\n"),
        vec![
            cmt(0, 9, Pos::Leading),
            nl(9, 11),
            ws(12, 13, Pos::Trailing),
            ws(14, 15, Pos::Trailing),
            ws(16, 17, Pos::Trailing),
            cmt(17, 27, Pos::Trailing),
            nl(27, 29),
            cmt(29, 38, Pos::Leading),
            nl(38, 40),
        ],
    );
}

#[test]
fn test_parse_trivia_comment_nothing_else_test() {
    assert_eq_trivia(trivia("#"), vec![cmt(0, 1, Pos::Leading)]);
}

#[test]
fn test_parse_trivia_comment_end_of_document_test() {
    assert_eq_trivia(trivia("1\n#"), vec![nl(1, 2), cmt(2, 3, Pos::Leading)]);
}

#[test]
fn test_parse_trivia_whitespace_between_comments_test() {
    assert_eq_trivia(
        trivia("1 #\n#\n2"),
        vec![
            ws(1, 2, Pos::Trailing),
            cmt(2, 3, Pos::Trailing),
            nl(3, 4),
            cmt(4, 5, Pos::Leading),
            nl(5, 6),
        ],
    );
}

#[test]
fn test_parse_trivia_comment_beginning_of_document_test() {
    assert_eq_trivia(trivia("#\n1"), vec![cmt(0, 1, Pos::Leading), nl(1, 2)]);
}

/// A skipped token becomes a `Skipped` piece, and everything up to the next
/// token leads that token rather than trailing what was skipped
#[test]
fn test_skip_as_trivia() {
    let mut source = RTokenSource::from_str("1 2");
    source.skip_as_trivia();

    assert_eq!(source.current(), R_DOUBLE_LITERAL);
    assert_eq!(source.current_range(), TextRange::new(2.into(), 3.into()));

    assert_eq_trivia(drain(source), vec![skipped(0, 1), ws(1, 2, Pos::Leading)]);
}

/// Lookahead drives the inner lexer past the current token, so using
/// [`BufferedLexer`] is useful to allow us to still answer for the token the
/// parser is actually on.
#[test]
fn test_line_break_survives_lookahead() {
    let mut source = RTokenSource::from_str("1\n2");
    assert!(!source.has_preceding_line_break());

    assert_eq!(source.nth(1), R_DOUBLE_LITERAL);
    assert!(source.has_nth_preceding_line_break(1));

    assert_eq!(source.current_range(), TextRange::new(0.into(), 1.into()));
    assert!(!source.has_preceding_line_break());

    source.bump();
    assert!(source.has_preceding_line_break());
}

/// `parse_argument()`'s lookahead buffers the token after an argument name,
/// which inside `[[` is the `]` that the next bump asks to join
#[test]
fn test_re_lex_after_lookahead() {
    let mut source = RTokenSource::from_str("a]]");
    assert_eq!(source.nth(1), R_BRACK);

    source.bump();
    assert_eq!(source.current(), R_BRACK);

    assert_eq!(source.re_lex(RReLexContext::JoinBrackets), R_BRACK2);
    assert_eq!(source.current_range(), TextRange::new(1.into(), 3.into()));
}
