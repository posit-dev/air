use crate::prelude::*;
use air_r_syntax::RUnnamedArgument;
use air_r_syntax::RUnnamedArgumentFields;
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRUnnamedArgument;
impl FormatNodeRule<RUnnamedArgument> for FormatRUnnamedArgument {
    fn fmt_fields(&self, node: &RUnnamedArgument, f: &mut RFormatter) -> FormatResult<()> {
        let RUnnamedArgumentFields { value } = node.as_fields();
        write!(f, [value.format()])
    }
}
