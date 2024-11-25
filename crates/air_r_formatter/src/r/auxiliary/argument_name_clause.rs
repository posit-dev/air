use crate::prelude::*;
use air_r_syntax::RArgumentNameClause;
use air_r_syntax::RArgumentNameClauseFields;
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRArgumentNameClause;
impl FormatNodeRule<RArgumentNameClause> for FormatRArgumentNameClause {
    fn fmt_fields(&self, node: &RArgumentNameClause, f: &mut RFormatter) -> FormatResult<()> {
        let RArgumentNameClauseFields { name, eq_token } = node.as_fields();
        write!(f, [name.format(), space(), eq_token.format()])
    }
}
