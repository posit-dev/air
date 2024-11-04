use crate::prelude::*;
use air_r_syntax::RNamedArgument;
use air_r_syntax::RNamedArgumentFields;
use biome_formatter::write;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRNamedArgument;
impl FormatNodeRule<RNamedArgument> for FormatRNamedArgument {
    fn fmt_fields(&self, node: &RNamedArgument, f: &mut RFormatter) -> FormatResult<()> {
        let RNamedArgumentFields {
            name,
            eq_token,
            value,
        } = node.as_fields();

        write!(
            f,
            [
                name.format(),
                space(),
                eq_token.format(),
                space(),
                value.format()
            ]
        )
    }
}
