use crate::diagnostics;
use crate::parser::RParser;
use crate::syntax::binding_power::BindingPower;
use crate::syntax::expr::parse_expression_with_binding_power;
use air_r_syntax::RSyntaxKind;
use air_r_syntax::RSyntaxKind::R_UNARY_EXPRESSION;
use biome_parser::CompletedMarker;
use biome_parser::Parser;

pub(crate) fn parse_unary_expression(
    p: &mut RParser,
    kind: RSyntaxKind,
    binding_power: BindingPower,
) -> CompletedMarker {
    let m = p.start();
    p.bump(kind);
    parse_expression_with_binding_power(p, binding_power)
        .or_add_diagnostic(p, diagnostics::expected_expression);
    m.complete(p, R_UNARY_EXPRESSION)
}
