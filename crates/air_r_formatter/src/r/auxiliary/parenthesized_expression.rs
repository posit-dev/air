use crate::prelude::*;
use air_r_syntax::RParenthesizedExpression;
use air_r_syntax::RParenthesizedExpressionFields;
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRParenthesizedExpression;
impl FormatNodeRule<RParenthesizedExpression> for FormatRParenthesizedExpression {
    fn fmt_fields(&self, node: &RParenthesizedExpression, f: &mut RFormatter) -> FormatResult<()> {
        let RParenthesizedExpressionFields {
            l_paren_token,
            body,
            r_paren_token,
        } = node.as_fields();

        write!(
            f,
            [
                l_paren_token.format(),
                group(&soft_block_indent(&body.format())),
                r_paren_token.format()
            ]
        )
    }
}
