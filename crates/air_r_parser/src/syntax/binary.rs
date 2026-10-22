use crate::diagnostics;
use crate::parser::RParser;
use crate::syntax::associativity::Associativity;
use crate::syntax::binding_power::BindingPower;
use crate::syntax::expr::parse_expression_with_binding_power;
use air_r_syntax::RSyntaxKind;
use air_r_syntax::RSyntaxKind::R_BINARY_EXPRESSION;
use biome_parser::CompletedMarker;
use biome_parser::Parser;

pub(crate) fn parse_binary_expression(
    p: &mut RParser,
    lhs: CompletedMarker,
    kind: RSyntaxKind,
    binding_power: BindingPower,
    associativity: Associativity,
) -> CompletedMarker {
    let binding_power = match associativity {
        Associativity::Left => binding_power.increment(),
        Associativity::Right => binding_power,
    };
    let m = lhs.precede(p);
    p.bump(kind);
    parse_expression_with_binding_power(p, binding_power)
        .or_add_diagnostic(p, diagnostics::expected_expression);
    m.complete(p, R_BINARY_EXPRESSION)
}
