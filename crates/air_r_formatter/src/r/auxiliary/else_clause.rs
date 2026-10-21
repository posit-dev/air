use crate::prelude::*;
use crate::r::auxiliary::if_statement::BracedStatements;
use crate::r::auxiliary::if_statement::FormatIfBody;
use air_r_syntax::RElseClause;
use air_r_syntax::RElseClauseFields;
use biome_formatter::FormatRuleWithOptions;
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRElseClause {
    pub(crate) braced_statements: BracedStatements,
}

#[derive(Debug)]
pub(crate) struct FormatRElseClauseOptions {
    pub(crate) braced_statements: BracedStatements,
}

impl FormatRuleWithOptions<RElseClause> for FormatRElseClause {
    type Options = FormatRElseClauseOptions;

    fn with_options(mut self, options: Self::Options) -> Self {
        self.braced_statements = options.braced_statements;
        self
    }
}

impl FormatNodeRule<RElseClause> for FormatRElseClause {
    fn fmt_fields(&self, node: &RElseClause, f: &mut RFormatter) -> FormatResult<()> {
        let RElseClauseFields {
            else_token,
            alternative,
        } = node.as_fields();

        let else_token = else_token?;
        let alternative = alternative?;

        write!(
            f,
            [
                else_token.format(),
                space(),
                &FormatIfBody::new_alternative(&alternative, self.braced_statements)
            ]
        )
    }
}
