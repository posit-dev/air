use crate::prelude::*;
use air_r_syntax::RBinaryExpression;
use air_r_syntax::RBinaryExpressionFields;
use air_r_syntax::RSyntaxKind;
use biome_formatter::format_args;
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRBinaryExpression;
impl FormatNodeRule<RBinaryExpression> for FormatRBinaryExpression {
    fn fmt_fields(&self, node: &RBinaryExpression, f: &mut RFormatter) -> FormatResult<()> {
        let RBinaryExpressionFields {
            left,
            operator,
            right,
        } = node.as_fields();

        let left = left?;
        let operator = operator?;
        let right = right?;

        match operator.kind() {
            // Sticky operators whose LHS and RHS stick to (no spaces or line breaks)
            RSyntaxKind::WAT
            | RSyntaxKind::EXPONENTIATE
            | RSyntaxKind::EXPONENTIATE2
            | RSyntaxKind::COLON => {
                write!(
                    f,
                    [group(&format_args![
                        left.format(),
                        operator.format(),
                        right.format()
                    ])]
                )
            }

            // For assignment, keep LHS and RHS on the same line, separated by
            // a single space
            RSyntaxKind::EQUAL
            | RSyntaxKind::ASSIGN
            | RSyntaxKind::ASSIGN_RIGHT
            | RSyntaxKind::SUPER_ASSIGN
            | RSyntaxKind::SUPER_ASSIGN_RIGHT => {
                write!(
                    f,
                    [group(&format_args![
                        left.format(),
                        space(),
                        operator.format(),
                        space(),
                        right.format()
                    ])]
                )
            }

            // Other operators have spaces around them and allow the RHS to break
            _ => {
                write!(
                    f,
                    [group(&format_args![
                        left.format(),
                        indent(&format_once(|f| {
                            write!(
                                f,
                                [
                                    space(),
                                    operator.format(),
                                    soft_line_break_or_space(),
                                    right.format()
                                ]
                            )
                        }))
                    ])]
                )
            }
        }
    }
}
