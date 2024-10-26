use crate::prelude::*;
use biome_r_syntax::RStringValue;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRStringValue;
impl FormatNodeRule<RStringValue> for FormatRStringValue {
    fn fmt_fields(&self, node: &RStringValue, f: &mut RFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
