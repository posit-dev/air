use crate::diagnostics;
use crate::parser::RParser;
use crate::syntax::binding_power::BindingPower;
use crate::syntax::expr::parse_expression;
use crate::syntax::restrictions::BinaryEqualsAssign;
use air_r_syntax::RSyntaxKind;
use air_r_syntax::RSyntaxKind::R_UNARY_EXPRESSION;
use biome_parser::CompletedMarker;
use biome_parser::Parser;

pub(crate) fn parse_unary_expression(
    p: &mut RParser,
    kind: RSyntaxKind,
    binding_power: BindingPower,
    binary_equals_assign: BinaryEqualsAssign,
) -> CompletedMarker {
    let m = p.start();
    p.bump(kind);
    parse_expression(p, binding_power, binary_equals_assign)
        .or_add_diagnostic(p, diagnostics::expected_expression);
    m.complete(p, R_UNARY_EXPRESSION)
}
