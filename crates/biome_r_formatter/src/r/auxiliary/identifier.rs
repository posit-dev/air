use crate::prelude::*;
use biome_r_syntax::RIdentifier;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRIdentifier;
impl FormatNodeRule<RIdentifier> for FormatRIdentifier {
    fn fmt_fields(&self, node: &RIdentifier, f: &mut RFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
