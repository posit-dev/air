use crate::prelude::*;
use air_r_syntax::RReturnExpression;
use air_r_syntax::RReturnExpressionFields;
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRReturnExpression;
impl FormatNodeRule<RReturnExpression> for FormatRReturnExpression {
    fn fmt_fields(&self, node: &RReturnExpression, f: &mut RFormatter) -> FormatResult<()> {
        let RReturnExpressionFields { return_token } = node.as_fields();
        write!(f, [return_token.format()])
    }
}
