use crate::prelude::*;
use air_r_syntax::RSubset2;
use air_r_syntax::RSubset2Fields;
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRSubset2;
impl FormatNodeRule<RSubset2> for FormatRSubset2 {
    fn fmt_fields(&self, node: &RSubset2, f: &mut RFormatter) -> FormatResult<()> {
        let RSubset2Fields {
            function,
            arguments,
        } = node.as_fields();

        write!(f, [function.format(), arguments.format()])
    }
}
