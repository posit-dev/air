use crate::prelude::*;
use air_r_syntax::RStatement;
use air_r_syntax::RStatementFields;
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRStatement;
impl FormatNodeRule<RStatement> for FormatRStatement {
    fn fmt_fields(&self, node: &RStatement, f: &mut RFormatter) -> FormatResult<()> {
        let RStatementFields {
            expression,
            semicolon_token,
        } = node.as_fields();

        write!(f, [expression.format()])?;

        // We never write the `;`! We always remove these, and expressions are
        // separated by a newline instead.
        if let Some(semicolon_token) = semicolon_token {
            write!(f, [format_removed(&semicolon_token)])?;
        }

        Ok(())
    }
}
