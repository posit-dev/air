use crate::prelude::*;
use air_r_syntax::RCallArguments;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRCallArguments;
impl FormatNodeRule<RCallArguments> for FormatRCallArguments {
    fn fmt_fields(&self, node: &RCallArguments, f: &mut RFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
