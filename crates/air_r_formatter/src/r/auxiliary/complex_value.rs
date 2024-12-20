use crate::prelude::*;
use air_r_syntax::RComplexValue;
use air_r_syntax::RComplexValueFields;
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRComplexValue;
impl FormatNodeRule<RComplexValue> for FormatRComplexValue {
    fn fmt_fields(&self, node: &RComplexValue, f: &mut RFormatter) -> FormatResult<()> {
        let RComplexValueFields { value_token } = node.as_fields();
        write![f, [value_token.format()]]
    }
}
