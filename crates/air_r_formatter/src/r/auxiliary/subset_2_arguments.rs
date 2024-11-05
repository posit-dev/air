use crate::prelude::*;
use air_r_syntax::RSubset2Arguments;
use air_r_syntax::RSubset2ArgumentsFields;
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRSubset2Arguments;
impl FormatNodeRule<RSubset2Arguments> for FormatRSubset2Arguments {
    fn fmt_fields(&self, node: &RSubset2Arguments, f: &mut RFormatter) -> FormatResult<()> {
        let RSubset2ArgumentsFields {
            l_brack2_token,
            items,
            r_brack2_token,
        } = node.as_fields();

        // TODO: Rebase on top of `call_arguments.rs` reworking, probably remove
        // the `group(soft_block_indent())` in favor of it happening internally?
        // Or possibly we will need a special `FormatRCallArgumentList` formatter
        // to differentiate call `items` from subset or subset2 `items`.
        write!(
            f,
            [
                l_brack2_token.format(),
                group(&soft_block_indent(&items.format())),
                r_brack2_token.format()
            ]
        )
    }
}
