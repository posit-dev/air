use crate::prelude::*;
use air_r_syntax::RForStatement;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRForStatement;
impl FormatNodeRule<RForStatement> for FormatRForStatement {
    fn fmt_fields(&self, node: &RForStatement, f: &mut RFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
