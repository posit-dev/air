use crate::diagnostics;
use crate::parser::RParser;
use crate::syntax::selector::parse_selector;
use air_r_syntax::RSyntaxKind;
use air_r_syntax::RSyntaxKind::R_EXTRACT_EXPRESSION;
use biome_parser::CompletedMarker;
use biome_parser::Parser;

pub(crate) fn parse_extract_expression(
    p: &mut RParser,
    lhs: CompletedMarker,
    kind: RSyntaxKind,
) -> CompletedMarker {
    let kind_range = p.cur_range();
    let m = lhs.precede(p);
    p.bump(kind);
    parse_selector(p).or_add_diagnostic(p, |_, range| {
        diagnostics::expected_rhs_selector(kind, kind_range, range)
    });
    m.complete(p, R_EXTRACT_EXPRESSION)
}
