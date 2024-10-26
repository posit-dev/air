use crate::prelude::*;
use biome_r_syntax::RDotsParameter;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRDotsParameter;
impl FormatNodeRule<RDotsParameter> for FormatRDotsParameter {
    fn fmt_fields(&self, node: &RDotsParameter, f: &mut RFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
