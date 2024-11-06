use crate::prelude::*;
use air_r_syntax::{RUnaryExpression, RUnaryExpressionFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRUnaryExpression;
impl FormatNodeRule<RUnaryExpression> for FormatRUnaryExpression {
    fn fmt_fields(&self, node: &RUnaryExpression, f: &mut RFormatter) -> FormatResult<()> {
        let RUnaryExpressionFields { operator, argument } = node.as_fields();

        let operator = operator?;
        let argument = argument?;

        write!(f, [operator.format()])?;

        if f.comments().has_comments(argument.syntax())
            && !f.comments().is_suppressed(argument.syntax())
        {
            // We never break between the operator and its argument except if a
            // newline separates them
            write!(f, [soft_block_indent(&argument.format())])
        } else {
            // A unary operator always sticks to its argument
            write!(f, [argument.format()])
        }
    }
}
