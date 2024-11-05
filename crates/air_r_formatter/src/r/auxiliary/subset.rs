use crate::prelude::*;
use air_r_syntax::RSubset;
use air_r_syntax::RSubsetFields;
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRSubset;
impl FormatNodeRule<RSubset> for FormatRSubset {
    fn fmt_fields(&self, node: &RSubset, f: &mut RFormatter) -> FormatResult<()> {
        let RSubsetFields {
            function,
            arguments,
        } = node.as_fields();

        write!(f, [function.format(), arguments.format()])
    }
}
