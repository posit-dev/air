use crate::prelude::*;
use crate::statement_body::FormatStatementBody;
use air_r_syntax::RIfStatement;
use air_r_syntax::RIfStatementFields;
use biome_formatter::format_args;
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRIfStatement;
impl FormatNodeRule<RIfStatement> for FormatRIfStatement {
    fn fmt_fields(&self, node: &RIfStatement, f: &mut RFormatter) -> FormatResult<()> {
        let RIfStatementFields {
            if_token,
            l_paren_token,
            condition,
            r_paren_token,
            consequence,
            else_clause,
        } = node.as_fields();

        write!(
            f,
            [group(&format_args![
                if_token.format(),
                space(),
                l_paren_token.format(),
                group(&soft_block_indent(&condition.format())),
                r_paren_token.format(),
                FormatStatementBody::new(&consequence?),
            ])]
        )?;

        // TODO: See more complex else handling (especially with comments)
        // in `if_statement.rs` for JS
        // Be careful about top level if statements. `else` has to be on the
        // same line as the end of `consequence` to parse correctly!
        if let Some(else_clause) = else_clause {
            let else_on_same_line = true;
            // let else_on_same_line = matches!(consequent, RBlockStatement(_));

            if else_on_same_line {
                write!(f, [space()])?;
            } else {
                write!(f, [hard_line_break()])?;
            }

            write!(f, [else_clause.format()])?;
        }

        Ok(())
    }
}
