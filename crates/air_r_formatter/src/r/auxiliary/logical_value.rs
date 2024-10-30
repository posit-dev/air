use crate::prelude::*;
use air_r_syntax::RLogicalValue;
use air_r_syntax::RLogicalValueFields;
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRLogicalValue;
impl FormatNodeRule<RLogicalValue> for FormatRLogicalValue {
    fn fmt_fields(&self, node: &RLogicalValue, f: &mut RFormatter) -> FormatResult<()> {
        let RLogicalValueFields { value_token } = node.as_fields();
        write![f, [value_token.format()]]
    }
}
