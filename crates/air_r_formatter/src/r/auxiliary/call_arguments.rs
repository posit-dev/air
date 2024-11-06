use crate::prelude::*;
use air_r_syntax::RCallArguments;
use air_r_syntax::RCallArgumentsFields;
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRCallArguments;
impl FormatNodeRule<RCallArguments> for FormatRCallArguments {
    fn fmt_fields(&self, node: &RCallArguments, f: &mut RFormatter) -> FormatResult<()> {
        let RCallArgumentsFields {
            l_paren_token,
            items,
            r_paren_token,
        } = node.as_fields();

        // Special case where the dangling comment has no node to attach to:
        //
        // ```r
        // fn(
        //   # dangling comment
        // )
        // ```
        //
        // If we don't handle it specially, it can break idempotence
        if items.is_empty() {
            return write!(
                f,
                [
                    l_paren_token.format(),
                    format_dangling_comments(node.syntax()).with_soft_block_indent(),
                    r_paren_token.format()
                ]
            );
        }

        write!(
            f,
            [
                l_paren_token.format(),
                items.format(),
                r_paren_token.format()
            ]
        )
    }

    fn fmt_dangling_comments(&self, _: &RCallArguments, _: &mut RFormatter) -> FormatResult<()> {
        // Formatted inside of `fmt_fields`
        Ok(())
    }
}
