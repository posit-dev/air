use crate::prelude::*;
use air_r_syntax::RParameterDefault;
use air_r_syntax::RParameterDefaultFields;
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRParameterDefault;
impl FormatNodeRule<RParameterDefault> for FormatRParameterDefault {
    fn fmt_fields(&self, node: &RParameterDefault, f: &mut RFormatter) -> FormatResult<()> {
        let RParameterDefaultFields { eq_token, value } = node.as_fields();
        write!(f, [eq_token.format(), space(), value.format()])
    }
}
