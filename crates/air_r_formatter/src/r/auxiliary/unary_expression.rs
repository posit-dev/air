use crate::prelude::*;
use air_r_syntax::AnyRExpression;
use air_r_syntax::RSyntaxKind;
use air_r_syntax::RSyntaxToken;
use air_r_syntax::{RUnaryExpression, RUnaryExpressionFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRUnaryExpression;
impl FormatNodeRule<RUnaryExpression> for FormatRUnaryExpression {
    fn fmt_fields(&self, node: &RUnaryExpression, f: &mut RFormatter) -> FormatResult<()> {
        let RUnaryExpressionFields { operator, argument } = node.as_fields();

        let operator = operator?;
        let argument = argument?;

        let needs_space = is_complex_unary_formula(&operator, &argument);

        let argument = format_with(|f| {
            if f.comments().has_comments(argument.syntax())
                && !f.comments().is_suppressed(argument.syntax())
            {
                // If an existing comment separates the operator and its argument, we are
                // forced to line break
                write!(f, [soft_block_indent(&argument.format())])
            } else {
                // Otherwise we never line break
                write!(f, [argument.format()])
            }
        });

        write!(f, [operator.format(), maybe_space(needs_space), argument])
    }
}

/// Is this a complex unary formula, like `~ .x + 1`?
///
/// Simple unary formulas like `~foo` and `~1` don't have a space between the `~` and the
/// argument (often used in `tribble()` calls, ggvis, the survey package, etc), but
/// "complex" unary formulas do.
///
/// We say a unary formula is "complex" if the `argument` is non-terminal.
fn is_complex_unary_formula(operator: &RSyntaxToken, argument: &AnyRExpression) -> bool {
    matches!(operator.kind(), RSyntaxKind::TILDE) && !is_terminal(argument)
}

/// Is this [AnyRExpression] considered terminal (or leaf-like)?
///
/// This is a bit hand wavy, but we consider a node to be terminal if we format it
/// verbatim. The goal is for anything that looks something like a call to get a space,
/// i.e. `~ x + 1`, but for very simple tokens to not get a space, i.e. `~foo` or `~1` or
/// `~NULL`.
fn is_terminal(node: &AnyRExpression) -> bool {
    match node {
        // Identifiers
        AnyRExpression::RIdentifier(_) => true,

        // Integer, double, complex, and string literals
        AnyRExpression::AnyRValue(_) => true,

        // `TRUE` and `FALSE`
        AnyRExpression::RTrueExpression(_) | AnyRExpression::RFalseExpression(_) => true,

        // `NA` variants, `NaN`, and `NULL`
        AnyRExpression::RNaExpression(_)
        | AnyRExpression::RNanExpression(_)
        | AnyRExpression::RNullExpression(_) => true,

        // `Inf`
        AnyRExpression::RInfExpression(_) => true,

        // `...` and `..i`
        AnyRExpression::RDots(_) | AnyRExpression::RDotDotI(_) => true,

        // `next` and `break`
        AnyRExpression::RNextExpression(_) | AnyRExpression::RBreakExpression(_) => true,

        // Anything else is non-terminal
        _ => false,
    }
}
