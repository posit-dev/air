use std::fmt::Debug;

use crate::prelude::*;
use crate::statement_body::FormatStatementBody;
use air_r_syntax::RFunctionDefinition;
use air_r_syntax::RFunctionDefinitionFields;
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRFunctionDefinition;
impl FormatNodeRule<RFunctionDefinition> for FormatRFunctionDefinition {
    fn fmt_fields(&self, node: &RFunctionDefinition, f: &mut RFormatter) -> FormatResult<()> {
        let RFunctionDefinitionFields {
            name,
            parameters,
            body,
        } = node.as_fields();

        write!(
            f,
            [
                name.format(),
                group(&parameters.format()),
                group(&FormatStatementBody::new(&body?)),
            ]
        )
    }
}
