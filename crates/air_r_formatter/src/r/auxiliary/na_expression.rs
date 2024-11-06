use crate::prelude::*;
use air_r_syntax::RNaExpression;
use air_r_syntax::RNaExpressionFields;
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRNaExpression;
impl FormatNodeRule<RNaExpression> for FormatRNaExpression {
    fn fmt_fields(&self, node: &RNaExpression, f: &mut RFormatter) -> FormatResult<()> {
        let RNaExpressionFields { value } = node.as_fields();
        write!(f, [value.format()])
    }
}
