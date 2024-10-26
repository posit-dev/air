use crate::prelude::*;
use air_r_syntax::RIdentifierParameter;
use air_r_syntax::RIdentifierParameterFields;
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRIdentifierParameter;
impl FormatNodeRule<RIdentifierParameter> for FormatRIdentifierParameter {
    fn fmt_fields(&self, node: &RIdentifierParameter, f: &mut RFormatter) -> FormatResult<()> {
        let RIdentifierParameterFields { name_token } = node.as_fields();
        write![f, [group(&name_token.format())]]
    }
}
