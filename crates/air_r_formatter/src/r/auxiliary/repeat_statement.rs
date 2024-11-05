use crate::prelude::*;
use air_r_syntax::RRepeatStatement;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRRepeatStatement;
impl FormatNodeRule<RRepeatStatement> for FormatRRepeatStatement {
    fn fmt_fields(&self, node: &RRepeatStatement, f: &mut RFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
