use crate::prelude::*;
use air_r_syntax::{RNamespaceExpression, RNamespaceExpressionFields};
use biome_formatter::format_args;
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRNamespaceExpression;
impl FormatNodeRule<RNamespaceExpression> for FormatRNamespaceExpression {
    fn fmt_fields(&self, node: &RNamespaceExpression, f: &mut RFormatter) -> FormatResult<()> {
        let RNamespaceExpressionFields {
            left,
            operator,
            right,
        } = node.as_fields();

        write!(
            f,
            [group(&format_args![
                left.format(),
                operator.format(),
                right.format()
            ])]
        )
    }
}
