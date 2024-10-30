use crate::prelude::*;
use air_r_syntax::RStringValue;
use air_r_syntax::RStringValueFields;
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRStringValue;
impl FormatNodeRule<RStringValue> for FormatRStringValue {
    fn fmt_fields(&self, node: &RStringValue, f: &mut RFormatter) -> FormatResult<()> {
        let RStringValueFields { value_token } = node.as_fields();
        write![f, [value_token.format()]]
    }
}
