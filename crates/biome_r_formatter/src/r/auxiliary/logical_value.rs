use crate::prelude::*;
use biome_r_syntax::RLogicalValue;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRLogicalValue;
impl FormatNodeRule<RLogicalValue> for FormatRLogicalValue {
    fn fmt_fields(&self, node: &RLogicalValue, f: &mut RFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
