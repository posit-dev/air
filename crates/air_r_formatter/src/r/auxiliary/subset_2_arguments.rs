use crate::prelude::*;
use crate::r::auxiliary::call_arguments::FormatRCallLikeArguments;
use air_r_syntax::RSubset2Arguments;
use air_r_syntax::RSubset2ArgumentsFields;
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRSubset2Arguments;
impl FormatNodeRule<RSubset2Arguments> for FormatRSubset2Arguments {
    fn fmt_fields(&self, node: &RSubset2Arguments, f: &mut RFormatter) -> FormatResult<()> {
        // TODO: Special handling for comments? See `handle_array_holes` for JS.

        let RSubset2ArgumentsFields {
            l_brack2_token,
            items,
            r_brack2_token,
        } = node.as_fields();

        // Special case where the dangling comment has no node to attach to:
        //
        // ```r
        // x[[
        //   # dangling comment
        // ]]
        // ```
        //
        // If we don't handle it specially, it can break idempotence
        if items.is_empty() {
            return write!(
                f,
                [
                    l_brack2_token.format(),
                    format_dangling_comments(node.syntax()).with_soft_block_indent(),
                    r_brack2_token.format()
                ]
            );
        }

        FormatRCallLikeArguments::new(l_brack2_token, items, r_brack2_token).fmt(f)
    }

    fn fmt_dangling_comments(&self, _: &RSubset2Arguments, _: &mut RFormatter) -> FormatResult<()> {
        // Formatted inside of `fmt_fields`
        // Only applicable for the empty arguments case
        Ok(())
    }
}
