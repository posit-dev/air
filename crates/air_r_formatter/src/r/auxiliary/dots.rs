use crate::prelude::*;
use air_r_syntax::RDots;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRDots;
impl FormatNodeRule<RDots> for FormatRDots {
    fn fmt_fields(&self, node: &RDots, f: &mut RFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
