use crate::parser::RParser;
use crate::syntax::binding_power::BindingPower;
use crate::syntax::expr::parse_expression_with_binding_power;
use biome_parser::parsed_syntax::ParsedSyntax;

/// Parse a function, loop, or if body
///
/// Generally treated like a normal expression, but R's precedence table puts
/// `?` at a lower binding power than a function, loop, or if body, so `function() a ? b`
/// needs to parse as `(function () a) ? b`.
pub(crate) fn parse_body_expression(p: &mut RParser) -> ParsedSyntax {
    // Treated as left associative
    let binding_power = BindingPower::FUNCTION_OR_LOOP_OR_IF.increment();
    parse_expression_with_binding_power(p, binding_power)
}
