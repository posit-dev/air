use crate::prelude::*;
use biome_r_syntax::RNullValue;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRNullValue;
impl FormatNodeRule<RNullValue> for FormatRNullValue {
    fn fmt_fields(&self, node: &RNullValue, f: &mut RFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
