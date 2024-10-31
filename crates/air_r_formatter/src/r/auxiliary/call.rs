use crate::prelude::*;
use air_r_syntax::RCall;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRCall;
impl FormatNodeRule<RCall> for FormatRCall {
    fn fmt_fields(&self, node: &RCall, f: &mut RFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
