use crate::prelude::*;
use air_r_syntax::RDots;
use air_r_syntax::RDotsFields;
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRDots;
impl FormatNodeRule<RDots> for FormatRDots {
    fn fmt_fields(&self, node: &RDots, f: &mut RFormatter) -> FormatResult<()> {
        let RDotsFields { value_token } = node.as_fields();
        write!(f, [value_token.format()])
    }
}
