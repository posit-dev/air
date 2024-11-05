use crate::prelude::*;
use crate::statement_body::FormatStatementBody;
use air_r_syntax::RWhileStatement;
use air_r_syntax::RWhileStatementFields;
use biome_formatter::format_args;
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRWhileStatement;
impl FormatNodeRule<RWhileStatement> for FormatRWhileStatement {
    fn fmt_fields(&self, node: &RWhileStatement, f: &mut RFormatter) -> FormatResult<()> {
        let RWhileStatementFields {
            while_token,
            l_paren_token,
            condition,
            r_paren_token,
            body,
        } = node.as_fields();

        write!(
            f,
            [group(&format_args!(
                while_token.format(),
                space(),
                l_paren_token.format(),
                group(&soft_block_indent(&condition.format())),
                r_paren_token.format(),
                FormatStatementBody::new(&body?)
            ))]
        )
    }
}
