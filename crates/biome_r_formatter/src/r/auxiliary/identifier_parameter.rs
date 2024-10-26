use crate::prelude::*;
use biome_formatter::write;
use biome_r_syntax::RIdentifierParameter;
use biome_r_syntax::RIdentifierParameterFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRIdentifierParameter;
impl FormatNodeRule<RIdentifierParameter> for FormatRIdentifierParameter {
    fn fmt_fields(&self, node: &RIdentifierParameter, f: &mut RFormatter) -> FormatResult<()> {
        let RIdentifierParameterFields { name_token } = node.as_fields();
        write![f, [group(&name_token.format())]]
    }
}
