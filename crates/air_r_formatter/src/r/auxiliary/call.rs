use crate::prelude::*;
use air_r_syntax::RCall;
use air_r_syntax::RCallFields;
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRCall;
impl FormatNodeRule<RCall> for FormatRCall {
    fn fmt_fields(&self, node: &RCall, f: &mut RFormatter) -> FormatResult<()> {
        let RCallFields {
            function,
            arguments,
        } = node.as_fields();

        write!(f, [function.format(), arguments.format()])
    }
}
