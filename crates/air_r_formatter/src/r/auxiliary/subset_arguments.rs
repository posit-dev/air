use crate::prelude::*;
use crate::r::auxiliary::call_arguments::FormatRCallLikeArguments;
use air_r_syntax::RSubsetArguments;
use air_r_syntax::RSubsetArgumentsFields;
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRSubsetArguments;
impl FormatNodeRule<RSubsetArguments> for FormatRSubsetArguments {
    fn fmt_fields(&self, node: &RSubsetArguments, f: &mut RFormatter) -> FormatResult<()> {
        // TODO: Special handling for comments? See `handle_array_holes` for JS.

        let RSubsetArgumentsFields {
            l_brack_token,
            items,
            r_brack_token,
        } = node.as_fields();

        // Special case where the dangling comment has no node to attach to:
        //
        // ```r
        // x[
        //   # dangling comment
        // ]
        // ```
        //
        // If we don't handle it specially, it can break idempotence
        if items.is_empty() {
            return write!(
                f,
                [
                    l_brack_token.format(),
                    format_dangling_comments(node.syntax()).with_soft_block_indent(),
                    r_brack_token.format()
                ]
            );
        }

        FormatRCallLikeArguments::new(l_brack_token, items, r_brack_token).fmt(f)
    }

    fn fmt_dangling_comments(&self, _: &RSubsetArguments, _: &mut RFormatter) -> FormatResult<()> {
        // Formatted inside of `fmt_fields`
        // Only applicable for the empty arguments case
        Ok(())
    }
}
