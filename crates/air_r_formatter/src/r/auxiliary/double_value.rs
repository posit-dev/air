use crate::prelude::*;
use air_r_syntax::RDoubleValue;
use biome_formatter::token::number::format_number_token;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRDoubleValue;
impl FormatNodeRule<RDoubleValue> for FormatRDoubleValue {
    fn fmt_fields(&self, node: &RDoubleValue, f: &mut RFormatter) -> FormatResult<()> {
        format_number_token(&node.value_token()?).fmt(f)
    }
}
