use crate::prelude::*;
use air_r_syntax::RNullValue;
use air_r_syntax::RNullValueFields;
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRNullValue;
impl FormatNodeRule<RNullValue> for FormatRNullValue {
    fn fmt_fields(&self, node: &RNullValue, f: &mut RFormatter) -> FormatResult<()> {
        let RNullValueFields { value_token } = node.as_fields();
        write![f, [value_token.format()]]
    }
}
