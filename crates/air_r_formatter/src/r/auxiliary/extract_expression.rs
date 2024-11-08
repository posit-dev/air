use crate::prelude::*;
use air_r_syntax::AnyRExpression;
use air_r_syntax::RExtractExpression;
use air_r_syntax::RExtractExpressionFields;
use air_r_syntax::RSymbolOrString;
use air_r_syntax::RSyntaxToken;
use biome_formatter::format_args;
use biome_formatter::write;
use biome_rowan::SyntaxResult;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRExtractExpression;
impl FormatNodeRule<RExtractExpression> for FormatRExtractExpression {
    fn fmt_fields(&self, node: &RExtractExpression, f: &mut RFormatter) -> FormatResult<()> {
        let RExtractExpressionFields {
            left,
            operator,
            right,
        } = node.as_fields();

        write_sticky_binary_expression(left, operator, right, f)
    }
}

pub(crate) fn write_sticky_binary_expression(
    left: SyntaxResult<AnyRExpression>,
    operator: SyntaxResult<RSyntaxToken>,
    right: SyntaxResult<RSymbolOrString>,
    f: &mut RFormatter,
) -> FormatResult<()> {
    write!(
        f,
        [group(&format_args![
            left.format(),
            indent(&format_once(|f| {
                write!(f, [operator.format(), right.format()])
            }))
        ])]
    )
}
