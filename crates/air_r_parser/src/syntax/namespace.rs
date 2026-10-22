use crate::diagnostics;
use crate::parser::RParser;
use crate::syntax::selector::parse_selector;
use air_r_syntax::AnyRSelector;
use air_r_syntax::RSyntaxKind;
use air_r_syntax::RSyntaxKind::R_BOGUS_SELECTOR;
use air_r_syntax::RSyntaxKind::R_NAMESPACE_EXPRESSION;
use biome_parser::CompletedMarker;
use biome_parser::Parser;
use biome_rowan::AstNode;

pub(crate) fn parse_namespace_expression(
    p: &mut RParser,
    mut lhs: CompletedMarker,
    kind: RSyntaxKind,
) -> CompletedMarker {
    let kind_range = p.cur_range();

    // `::` and `:::` restrict their left operand as well, and it is already parsed
    if !AnyRSelector::can_cast(lhs.kind(p)) {
        p.error(diagnostics::expected_lhs_selector(
            kind,
            kind_range,
            lhs.range(p),
        ));
        lhs = lhs.precede(p).complete(p, R_BOGUS_SELECTOR);
    }

    let m = lhs.precede(p);
    p.bump(kind);

    if p.scope().newlines_are_significant() && p.has_preceding_line_break() {
        p.error(diagnostics::namespace_rhs_selector_after_line_break(
            kind, kind_range,
        ));
    } else {
        parse_selector(p).or_add_diagnostic(p, |_, range| {
            diagnostics::expected_rhs_selector(kind, kind_range, range)
        });
    }

    m.complete(p, R_NAMESPACE_EXPRESSION)
}
