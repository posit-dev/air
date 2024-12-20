use crate::prelude::*;
use air_r_syntax::RNextExpression;
use air_r_syntax::RNextExpressionFields;
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRNextExpression;
impl FormatNodeRule<RNextExpression> for FormatRNextExpression {
    fn fmt_fields(&self, node: &RNextExpression, f: &mut RFormatter) -> FormatResult<()> {
        let RNextExpressionFields { next_token } = node.as_fields();
        write!(f, [next_token.format()])
    }
}
