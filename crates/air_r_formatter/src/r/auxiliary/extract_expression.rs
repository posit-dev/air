use crate::prelude::*;
use air_r_syntax::RExtractExpression;
use air_r_syntax::RExtractExpressionFields;
use biome_formatter::format_args;
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRExtractExpression;
impl FormatNodeRule<RExtractExpression> for FormatRExtractExpression {
    fn fmt_fields(&self, node: &RExtractExpression, f: &mut RFormatter) -> FormatResult<()> {
        let RExtractExpressionFields {
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
