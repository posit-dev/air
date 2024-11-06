use crate::prelude::*;
use air_r_syntax::RTrueExpression;
use air_r_syntax::RTrueExpressionFields;
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRTrueExpression;
impl FormatNodeRule<RTrueExpression> for FormatRTrueExpression {
    fn fmt_fields(&self, node: &RTrueExpression, f: &mut RFormatter) -> FormatResult<()> {
        let RTrueExpressionFields { true_token } = node.as_fields();
        write!(f, [true_token.format()])
    }
}
