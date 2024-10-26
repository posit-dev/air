use crate::prelude::*;
use biome_formatter::token::number::format_number_token;
use biome_r_syntax::RDoubleValue;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRDoubleValue;
impl FormatNodeRule<RDoubleValue> for FormatRDoubleValue {
    fn fmt_fields(&self, node: &RDoubleValue, f: &mut RFormatter) -> FormatResult<()> {
        format_number_token(&node.value_token()?).fmt(f)
    }
}
