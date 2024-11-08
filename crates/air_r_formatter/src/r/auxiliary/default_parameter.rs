use crate::prelude::*;
use air_r_syntax::RDefaultParameter;
use air_r_syntax::RDefaultParameterFields;
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRDefaultParameter;
impl FormatNodeRule<RDefaultParameter> for FormatRDefaultParameter {
    fn fmt_fields(&self, node: &RDefaultParameter, f: &mut RFormatter) -> FormatResult<()> {
        let RDefaultParameterFields {
            name_token,
            eq_token,
            default,
        } = node.as_fields();

        write![
            f,
            [
                name_token.format(),
                space(),
                eq_token.format(),
                space(),
                default.format()
            ]
        ]
    }
}
