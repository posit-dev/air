use crate::prelude::*;
use air_r_syntax::RSubsetArguments;
use air_r_syntax::RSubsetArgumentsFields;
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRSubsetArguments;
impl FormatNodeRule<RSubsetArguments> for FormatRSubsetArguments {
    fn fmt_fields(&self, node: &RSubsetArguments, f: &mut RFormatter) -> FormatResult<()> {
        let RSubsetArgumentsFields {
            l_brack_token,
            items,
            r_brack_token,
        } = node.as_fields();

        // TODO: Rebase on top of `call_arguments.rs` reworking, probably remove
        // the `group(soft_block_indent())` in favor of it happening internally?
        // Or possibly we will need a special `FormatRCallArgumentList` formatter
        // to differentiate call `items` from subset or subset2 `items`.
        write!(
            f,
            [
                l_brack_token.format(),
                group(&soft_block_indent(&items.format())),
                r_brack_token.format()
            ]
        )
    }
}
