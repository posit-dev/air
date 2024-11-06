use crate::prelude::*;
use air_r_syntax::RInfExpression;
use air_r_syntax::RInfExpressionFields;
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRInfExpression;
impl FormatNodeRule<RInfExpression> for FormatRInfExpression {
    fn fmt_fields(&self, node: &RInfExpression, f: &mut RFormatter) -> FormatResult<()> {
        let RInfExpressionFields { inf_token } = node.as_fields();
        write!(f, [inf_token.format()])
    }
}
