use crate::parser::RParser;
use air_r_syntax::RSyntaxKind;
use biome_parser::CompletedMarker;
use biome_parser::Parser;

/// Bumps a single token and wraps it in `kind_complete`
pub(crate) fn parse_literal_expression(
    p: &mut RParser,
    kind_bump: RSyntaxKind,
    kind_complete: RSyntaxKind,
) -> CompletedMarker {
    let m = p.start();
    p.bump(kind_bump);
    m.complete(p, kind_complete)
}
