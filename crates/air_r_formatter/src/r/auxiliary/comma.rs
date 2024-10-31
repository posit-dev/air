use crate::prelude::*;
use air_r_syntax::RComma;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRComma;
impl FormatNodeRule<RComma> for FormatRComma {
    fn fmt_fields(&self, node: &RComma, f: &mut RFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
