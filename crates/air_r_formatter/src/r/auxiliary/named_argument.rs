use crate::prelude::*;
use air_r_syntax::RNamedArgument;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRNamedArgument;
impl FormatNodeRule<RNamedArgument> for FormatRNamedArgument {
    fn fmt_fields(&self, node: &RNamedArgument, f: &mut RFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
