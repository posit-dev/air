use crate::prelude::*;
use crate::separated::FormatAstSeparatedListExtension;
use air_r_syntax::RCall;
use air_r_syntax::RCallArguments;
use air_r_syntax::RCallArgumentsFields;
use biome_formatter::separated::TrailingSeparator;
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

        // Special case where we have a test call where we never want to break
        // even if we exceed the line length
        let is_test_call = node
            .parent::<RCall>()
            .as_ref()
            .map_or(Ok(false), |call| call.is_test_call())?;

        if is_test_call {
            let items = format_with(|f| {
                f.join_with(space())
                    .entries(
                        items
                            .format_separated(",")
                            .with_trailing_separator(TrailingSeparator::Disallowed),
                    )
                    .finish()
            });

            return write!(f, [l_paren_token.format(), &items, r_paren_token.format()]);
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
