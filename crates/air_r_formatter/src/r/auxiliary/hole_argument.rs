use crate::prelude::*;
use air_r_syntax::RHoleArgument;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRHoleArgument;
impl FormatNodeRule<RHoleArgument> for FormatRHoleArgument {
    fn fmt_fields(&self, node: &RHoleArgument, f: &mut RFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
