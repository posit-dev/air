use crate::prelude::*;
use crate::statement_body::FormatStatementBody;
use air_r_syntax::RElseClause;
use air_r_syntax::RElseClauseFields;
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRElseClause;
impl FormatNodeRule<RElseClause> for FormatRElseClause {
    fn fmt_fields(&self, node: &RElseClause, f: &mut RFormatter) -> FormatResult<()> {
        use air_r_syntax::AnyRExpression::*;

        let RElseClauseFields {
            else_token,
            alternative,
        } = node.as_fields();

        let alternative = alternative?;

        write!(
            f,
            [
                else_token.format(),
                group(
                    &FormatStatementBody::new(&alternative)
                        .with_forced_space(matches!(alternative, RIfStatement(_)))
                )
            ]
        )
    }
}
