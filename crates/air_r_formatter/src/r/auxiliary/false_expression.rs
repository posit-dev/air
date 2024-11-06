use crate::prelude::*;
use air_r_syntax::RFalseExpression;
use air_r_syntax::RFalseExpressionFields;
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRFalseExpression;
impl FormatNodeRule<RFalseExpression> for FormatRFalseExpression {
    fn fmt_fields(&self, node: &RFalseExpression, f: &mut RFormatter) -> FormatResult<()> {
        let RFalseExpressionFields { false_token } = node.as_fields();
        write!(f, [false_token.format()])
    }
}
