use crate::prelude::*;
use air_r_syntax::RBracedExpressions;
use air_r_syntax::RBracedExpressionsFields;
use biome_formatter::write;
use biome_formatter::CstFormatContext;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRBracedExpressions;
impl FormatNodeRule<RBracedExpressions> for FormatRBracedExpressions {
    fn fmt_fields(&self, node: &RBracedExpressions, f: &mut RFormatter) -> FormatResult<()> {
        let RBracedExpressionsFields {
            l_curly_token,
            expressions,
            r_curly_token,
        } = node.as_fields();

        write!(f, [l_curly_token.format()])?;

        let comments = f.context().comments();

        if expressions.is_empty() {
            // For empty `{}`, we unconditionally hard line break between the
            // tokens. We could consider not doing this for certain syntax,
            // like if statements, but that gets a little hand wavy.
            let has_dangling_comments = comments.has_dangling_comments(node.syntax());

            if has_dangling_comments {
                write!(
                    f,
                    [format_dangling_comments(node.syntax()).with_block_indent()]
                )?;
            } else {
                write!(f, [hard_line_break()])?;
            }
        } else {
            write!(f, [block_indent(&expressions.format())])?;
        }

        write!(f, [r_curly_token.format()])
    }

    fn fmt_dangling_comments(
        &self,
        _: &RBracedExpressions,
        _: &mut RFormatter,
    ) -> FormatResult<()> {
        // Formatted inside of `fmt_fields`
        Ok(())
    }
}
