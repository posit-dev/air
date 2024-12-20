use crate::prelude::*;
use air_r_syntax::RDoubleValue;
use air_r_syntax::RDoubleValueFields;
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRDoubleValue;
impl FormatNodeRule<RDoubleValue> for FormatRDoubleValue {
    fn fmt_fields(&self, node: &RDoubleValue, f: &mut RFormatter) -> FormatResult<()> {
        let RDoubleValueFields { value_token } = node.as_fields();
        write![f, [value_token.format()]]
    }
}
