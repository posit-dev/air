use crate::prelude::*;
use biome_r_syntax::RIntegerValue;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRIntegerValue;
impl FormatNodeRule<RIntegerValue> for FormatRIntegerValue {
    fn fmt_fields(&self, node: &RIntegerValue, f: &mut RFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
