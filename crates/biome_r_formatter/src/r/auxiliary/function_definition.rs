use std::fmt::Debug;

use crate::prelude::*;
use biome_formatter::write;
use biome_r_syntax::RFunctionDefinition;
use biome_r_syntax::RFunctionDefinitionFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRFunctionDefinition;
impl FormatNodeRule<RFunctionDefinition> for FormatRFunctionDefinition {
    fn fmt_fields(&self, node: &RFunctionDefinition, f: &mut RFormatter) -> FormatResult<()> {
        let RFunctionDefinitionFields {
            function_token,
            parameters,
            body,
        } = node.as_fields();

        write!(
            f,
            [
                function_token.format(),
                group(&parameters.format()),
                space(),
                body.format()
            ]
        )
    }
}
