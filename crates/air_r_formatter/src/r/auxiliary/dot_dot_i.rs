use crate::prelude::*;
use air_r_syntax::RDotDotI;
use air_r_syntax::RDotDotIFields;
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRDotDotI;
impl FormatNodeRule<RDotDotI> for FormatRDotDotI {
    fn fmt_fields(&self, node: &RDotDotI, f: &mut RFormatter) -> FormatResult<()> {
        let RDotDotIFields { value_token } = node.as_fields();
        write!(f, [value_token.format()])
    }
}
