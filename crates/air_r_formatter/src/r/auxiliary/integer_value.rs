use crate::prelude::*;
use air_r_syntax::RIntegerValue;
use air_r_syntax::RIntegerValueFields;
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRIntegerValue;
impl FormatNodeRule<RIntegerValue> for FormatRIntegerValue {
    fn fmt_fields(&self, node: &RIntegerValue, f: &mut RFormatter) -> FormatResult<()> {
        let RIntegerValueFields { value_token } = node.as_fields();
        write![f, [value_token.format()]]
    }
}
