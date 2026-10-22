//! Coverage test to ensure every token has lexer coverage

use super::*;
use biome_rowan::RawSyntaxKind;
use std::collections::HashSet;

/// Well-formed sources, which between them must produce every lexable kind but
/// `ERROR_TOKEN` and `R_BRACK2`. The test names the gaps for you.
const SOURCES: &[&str] = &[
    "\u{FEFF}",
    ";,{}()?~$@\\+/^",
    "[ ] [[ ]]",
    "<- <<- := -> ->> - = == != < <= > >=",
    "| || & && |> * ** : :: ::: !",
    "function for in while repeat if else next break",
    "TRUE FALSE NULL Inf NaN NA NA_integer_ NA_real_ NA_complex_ NA_character_",
    "foo ... ..1 1 1L 1i %in%\n# comment\n",
    "\"a\" 'b' r\"(c)\"",
];

/// Sources for the kinds no well-formed source can reach. `ERROR_TOKEN` is the
/// only one, and it comes with a diagnostic by construction, so it needs a list
/// of its own rather than a hole in [`SOURCES`].
const ERROR_SOURCES: &[&str] = &["\u{1}"];

/// The kinds only a re-lex can produce.
///
/// `R_BRACK2` is the only one
fn re_lexed_kinds() -> HashSet<RSyntaxKind> {
    let mut lexer = RLexer::from_str("]]");
    lexer.next_token(RLexContext::Regular);
    HashSet::from([lexer.re_lex(RReLexContext::JoinBrackets)])
}

fn is_lexable(kind: RSyntaxKind) -> bool {
    !matches!(kind, TOMBSTONE | EOF | __LAST) && (kind as u16) < (R_ROOT as u16)
}

#[test]
fn every_lexable_kind_is_produced() {
    let mut produced = HashSet::new();
    for source in SOURCES {
        let (tokens, diagnostics) = lex(source);
        assert!(
            diagnostics.is_empty(),
            "coverage sources are well-formed R; {source:?} lexed with diagnostics",
        );
        produced.extend(tokens.into_iter().map(|token| token.kind));
    }
    for source in ERROR_SOURCES {
        let (tokens, diagnostics) = lex(source);
        assert!(
            !diagnostics.is_empty(),
            "{source:?} is in ERROR_SOURCES but lexed cleanly",
        );
        produced.extend(tokens.into_iter().map(|token| token.kind));
    }
    produced.extend(re_lexed_kinds());

    let missing: Vec<_> = (0..RSyntaxKind::__LAST as u16)
        .map(|raw| RSyntaxKind::from_raw(RawSyntaxKind(raw)))
        .filter(|kind| is_lexable(*kind) && !produced.contains(kind))
        .collect();

    assert!(missing.is_empty(), "no test lexes: {missing:#?}");
}
