use crate::prelude::*;
use air_r_syntax::RBreakExpression;
use air_r_syntax::RBreakExpressionFields;
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRBreakExpression;
impl FormatNodeRule<RBreakExpression> for FormatRBreakExpression {
    fn fmt_fields(&self, node: &RBreakExpression, f: &mut RFormatter) -> FormatResult<()> {
        let RBreakExpressionFields { break_token } = node.as_fields();
        write!(f, [break_token.format()])
    }
}
