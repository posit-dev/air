use crate::prelude::*;
use air_r_syntax::RExpressionStatement;
use air_r_syntax::RExpressionStatementFields;
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRExpressionStatement;
impl FormatNodeRule<RExpressionStatement> for FormatRExpressionStatement {
    fn fmt_fields(&self, node: &RExpressionStatement, f: &mut RFormatter) -> FormatResult<()> {
        let RExpressionStatementFields {
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
