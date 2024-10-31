use crate::prelude::*;
use air_r_syntax::RUnnamedArgument;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRUnnamedArgument;
impl FormatNodeRule<RUnnamedArgument> for FormatRUnnamedArgument {
    fn fmt_fields(&self, node: &RUnnamedArgument, f: &mut RFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
