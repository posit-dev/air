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

        let l_paren_token = l_paren_token?;
        let body = body?;
        let r_paren_token = r_paren_token?;

        let body = format_with(|f| {
            if body.syntax().has_comments_direct() {
                // Fully expand the parentheses to allow placement of comments
                write!(f, [block_indent(&body.format())])
            } else {
                write!(f, [body.format()])
            }
        });

        write!(f, [l_paren_token.format(), body, r_paren_token.format()])
    }
}
