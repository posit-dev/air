use crate::diagnostics;
use crate::parser::RParser;
use crate::syntax::associativity::Associativity;
use crate::syntax::binding_power::BindingPower;
use crate::syntax::expr::parse_expression;
use crate::syntax::restrictions::BinaryEqualsAssign;
use air_r_syntax::RSyntaxKind;
use air_r_syntax::RSyntaxKind::R_BINARY_EXPRESSION;
use air_r_syntax::RSyntaxKind::R_BOGUS_EXPRESSION;
use biome_parser::CompletedMarker;
use biome_parser::Parser;

pub(crate) fn parse_binary_expression(
    p: &mut RParser,
    lhs: CompletedMarker,
    kind: RSyntaxKind,
    binding_power: BindingPower,
    associativity: Associativity,
    binary_equals_assign: BinaryEqualsAssign,
) -> CompletedMarker {
    let binding_power = match associativity {
        Associativity::Left => binding_power.increment(),
        Associativity::Right => binding_power,
    };
    let m = lhs.precede(p);
    p.bump(kind);
    parse_expression(p, binding_power, binary_equals_assign)
        .or_add_diagnostic(p, diagnostics::expected_expression);
    m.complete(p, R_BINARY_EXPRESSION)
}

/// Parse through an unexpected `=` assignment
///
/// Goes with [crate::syntax::restrictions::BinaryEqualsAssign::Deny]
///
/// When we encounter an invalid `=` assignment, we wrap the entire
/// binary expression in an [R_BOGUS_EXPRESSION] and emit a diagnostic
/// on the range of the `=` sign:
///
/// ```text
/// fn(1 = x)
///      ^ diagnostic
///   |-----| bogus
/// ```
///
/// Same function body as [parse_binary_expression()], except for the
/// `p.error()` and the [R_BOGUS_EXPRESSION]. Keep them in sync!
pub(crate) fn parse_binary_expression_unexpected_binary_equals_assign(
    p: &mut RParser,
    lhs: CompletedMarker,
    kind: RSyntaxKind,
    binding_power: BindingPower,
    associativity: Associativity,
    binary_equals_assign: BinaryEqualsAssign,
) -> CompletedMarker {
    let binding_power = match associativity {
        Associativity::Left => binding_power.increment(),
        Associativity::Right => binding_power,
    };
    let m = lhs.precede(p);
    p.error(diagnostics::unexpected_binary_equals_assign(p.cur_range()));
    p.bump(kind);
    parse_expression(p, binding_power, binary_equals_assign)
        .or_add_diagnostic(p, diagnostics::expected_expression);
    m.complete(p, R_BOGUS_EXPRESSION)
}
