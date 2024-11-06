use crate::prelude::*;
use air_r_syntax::RNullExpression;
use air_r_syntax::RNullExpressionFields;
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRNullExpression;
impl FormatNodeRule<RNullExpression> for FormatRNullExpression {
    fn fmt_fields(&self, node: &RNullExpression, f: &mut RFormatter) -> FormatResult<()> {
        let RNullExpressionFields { null_token } = node.as_fields();
        write!(f, [null_token.format()])
    }
}
