use crate::prelude::*;
use air_r_syntax::RWhileStatement;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRWhileStatement;
impl FormatNodeRule<RWhileStatement> for FormatRWhileStatement {
    fn fmt_fields(&self, node: &RWhileStatement, f: &mut RFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
