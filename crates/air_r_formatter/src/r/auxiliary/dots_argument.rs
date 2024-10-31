use crate::prelude::*;
use air_r_syntax::RDotsArgument;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRDotsArgument;
impl FormatNodeRule<RDotsArgument> for FormatRDotsArgument {
    fn fmt_fields(&self, node: &RDotsArgument, f: &mut RFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
