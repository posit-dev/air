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

        let needs_space = is_complex_anonymous_function(&operator, &argument);

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

/// Is this an anonymous function like `~ .x + 1`?
///
/// Simple unary formulas like `~foo` don't have a space between the `~` and the
/// argument (often used in tribble calls), but "complex" unary formulas do.
///
/// We define "complex" as anything except a simple identifier, so even `~ "foo"` results
/// in a space.
fn is_complex_anonymous_function(operator: &RSyntaxToken, argument: &AnyRExpression) -> bool {
    // Must be a `~`
    if !matches!(operator.kind(), RSyntaxKind::TILDE) {
        return false;
    }

    // "Complex" is anything except identifiers
    !matches!(argument, AnyRExpression::RIdentifier(_))
}
