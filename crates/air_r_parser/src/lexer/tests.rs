//! Lexer test infrastructure

mod backticks;
mod bom;
mod brackets;
mod checkpoint;
mod comments;
mod coverage;
mod dots;
mod error_token;
mod identifiers;
mod keywords;
mod line_breaks;
mod numbers;
mod puncts;
mod raw_strings;
mod specials;
mod strings;
mod whitespace;

use super::*;
use biome_diagnostics::PrintDescription;

#[derive(Debug)]
pub(super) struct LexedToken {
    kind: RSyntaxKind,
    range: TextRange,
    preceding_line_break: bool,
}

/// Lex `source` to completion
pub(super) fn lex(source: &str) -> (Vec<LexedToken>, Vec<ParseDiagnostic>) {
    let mut lexer = RLexer::from_str(source);
    let mut tokens = Vec::new();

    loop {
        let kind = lexer.next_token(RLexContext::Regular);
        if kind == EOF {
            break;
        }

        tokens.push(LexedToken {
            kind,
            range: lexer.current_range(),
            preceding_line_break: lexer.has_preceding_line_break(),
        });
    }

    (tokens, lexer.finish())
}

/// Assert the exact token stream, and that the lexer emitted no diagnostics
macro_rules! assert_lex {
    ($source:expr, $($kind:ident : $length:expr),* $(,)?) => {
        $crate::lexer::tests::check_lex($source, &[$(($kind, $length)),*], &[])
    };
}
// `macro_rules!` scoping is textual, so the three macros reach the submodules
// declared above them by path rather than by falling into scope
pub(super) use assert_lex;

/// [`assert_lex!`] plus the diagnostics expected, in order
macro_rules! assert_lex_err {
    ($source:expr, $($kind:ident : $length:expr),* $(,)? => $($message:literal),+ $(,)?) => {
        $crate::lexer::tests::check_lex($source, &[$(($kind, $length)),*], &[$($message),+])
    };
}
pub(super) use assert_lex_err;

/// Assert `PRECEDING_LINE_BREAK` on each non-trivia token, in order
macro_rules! assert_line_breaks {
    ($source:expr, $($kind:ident : $preceded:literal),* $(,)?) => {
        $crate::lexer::tests::check_line_breaks($source, &[$(($kind, $preceded)),*])
    };
}
pub(super) use assert_line_breaks;

#[track_caller]
pub(super) fn check_lex(source: &str, expected: &[(RSyntaxKind, u32)], messages: &[&str]) {
    let (tokens, diagnostics) = lex(source);
    let render = render(source, &tokens);

    assert_eq!(
        tokens.len(),
        expected.len(),
        "expected {} tokens, lexed {}:\n{render}",
        expected.len(),
        tokens.len(),
    );

    // The ranges must cover the whole source
    let mut offset = TextSize::from(0);
    for (index, (token, (want_kind, want_length))) in tokens.iter().zip(expected).enumerate() {
        assert_eq!(token.kind, *want_kind, "token {index}:\n{render}");
        assert_eq!(
            token.range.len(),
            TextSize::from(*want_length),
            "token {index} has the wrong length:\n{render}",
        );
        assert_eq!(
            token.range.start(),
            offset,
            "token {index} leaves a gap:\n{render}",
        );
        offset = token.range.end();
    }
    assert_eq!(
        usize::from(offset),
        source.len(),
        "the tokens stop short of the end of the source:\n{render}",
    );

    let actual: Vec<String> = diagnostics
        .iter()
        .map(|diagnostic| PrintDescription(diagnostic).to_string())
        .collect();

    assert_eq!(
        actual.len(),
        messages.len(),
        "expected {} diagnostics, got:\n{actual:#?}",
        messages.len(),
    );
    for (actual, expected) in actual.iter().zip(messages) {
        assert!(
            actual.contains(expected),
            "expected {expected:?} in {actual:?}"
        );
    }
}

#[track_caller]
pub(super) fn check_line_breaks(source: &str, expected: &[(RSyntaxKind, bool)]) {
    let (tokens, _) = lex(source);
    let render = render(source, &tokens);

    let actual: Vec<_> = tokens
        .iter()
        .filter(|token| !token.kind.is_trivia())
        .collect();

    assert_eq!(
        actual.len(),
        expected.len(),
        "expected {} non-trivia tokens, lexed {}:\n{render}",
        expected.len(),
        actual.len(),
    );
    for (index, (token, (want_kind, want_break))) in actual.iter().zip(expected).enumerate() {
        assert_eq!(token.kind, *want_kind, "token {index}:\n{render}");
        assert_eq!(
            token.preceding_line_break, *want_break,
            "token {index} reports the wrong preceding line break:\n{render}",
        );
    }
}

/// The whole stream, for any failure. A message naming only the offending token
/// sends you back to re-run with `--no-capture` and print the rest yourself.
fn render(source: &str, tokens: &[LexedToken]) -> String {
    tokens
        .iter()
        .map(|token| {
            let break_marker = if token.preceding_line_break {
                " nl"
            } else {
                ""
            };
            format!(
                "  {:?}@{:?} {:?}{break_marker}",
                token.kind, token.range, &source[token.range]
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}
