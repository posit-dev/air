use crate::prelude::*;
use air_r_syntax::RDotsParameter;
use air_r_syntax::RDotsParameterFields;
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRDotsParameter;
impl FormatNodeRule<RDotsParameter> for FormatRDotsParameter {
    fn fmt_fields(&self, node: &RDotsParameter, f: &mut RFormatter) -> FormatResult<()> {
        let RDotsParameterFields { name_token } = node.as_fields();
        write!(f, [name_token.format()])
    }
}
