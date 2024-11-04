use crate::prelude::*;
use air_r_syntax::RDotsArgument;
use air_r_syntax::RDotsArgumentFields;
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRDotsArgument;
impl FormatNodeRule<RDotsArgument> for FormatRDotsArgument {
    fn fmt_fields(&self, node: &RDotsArgument, f: &mut RFormatter) -> FormatResult<()> {
        let RDotsArgumentFields { value_token } = node.as_fields();
        write!(f, [value_token.format()])
    }
}
