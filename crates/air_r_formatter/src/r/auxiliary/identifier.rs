use crate::prelude::*;
use air_r_syntax::RIdentifier;
use air_r_syntax::RIdentifierFields;
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRIdentifier;
impl FormatNodeRule<RIdentifier> for FormatRIdentifier {
    fn fmt_fields(&self, node: &RIdentifier, f: &mut RFormatter) -> FormatResult<()> {
        let RIdentifierFields { name_token } = node.as_fields();
        write![f, [group(&name_token.format())]]
    }
}
