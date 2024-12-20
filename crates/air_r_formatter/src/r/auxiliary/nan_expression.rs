use crate::prelude::*;
use air_r_syntax::RNanExpression;
use air_r_syntax::RNanExpressionFields;
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRNanExpression;
impl FormatNodeRule<RNanExpression> for FormatRNanExpression {
    fn fmt_fields(&self, node: &RNanExpression, f: &mut RFormatter) -> FormatResult<()> {
        let RNanExpressionFields { nan_token } = node.as_fields();
        write!(f, [nan_token.format()])
    }
}
