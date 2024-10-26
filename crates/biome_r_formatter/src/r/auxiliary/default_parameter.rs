use crate::prelude::*;
use biome_r_syntax::RDefaultParameter;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRDefaultParameter;
impl FormatNodeRule<RDefaultParameter> for FormatRDefaultParameter {
    fn fmt_fields(&self, node: &RDefaultParameter, f: &mut RFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
