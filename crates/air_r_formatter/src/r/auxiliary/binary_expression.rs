use crate::prelude::*;
use biome_formatter::format_args;
use biome_formatter::write;
use air_r_syntax::RBinaryExpression;
use air_r_syntax::RBinaryExpressionFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRBinaryExpression;
impl FormatNodeRule<RBinaryExpression> for FormatRBinaryExpression {
    fn fmt_fields(&self, node: &RBinaryExpression, f: &mut RFormatter) -> FormatResult<()> {
        let RBinaryExpressionFields {
            left,
            operator_token,
            right,
        } = node.as_fields();

        write!(
            f,
            [group(&format_args![
                left.format(),
                indent(&format_once(|f| {
                    write!(
                        f,
                        [
                            space(),
                            operator_token.format(),
                            soft_line_break_or_space(),
                            right.format()
                        ]
                    )
                }))
            ])]
        )
    }
}
