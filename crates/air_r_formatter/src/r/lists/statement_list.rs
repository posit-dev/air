use crate::{formatter_ext::FormatterExt, joiner_ext::EmptyLines, prelude::*};
use air_r_syntax::RStatementList;
use biome_formatter::FormatRuleWithOptions;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRStatementList {
    pub(crate) empty_lines: EmptyLines,
}

#[derive(Default, Debug, Clone)]
pub(crate) struct FormatRStatementListOptions {
    pub(crate) empty_lines: EmptyLines,
}

impl FormatRule<RStatementList> for FormatRStatementList {
    type Context = RFormatContext;
    fn fmt(&self, node: &RStatementList, f: &mut RFormatter) -> FormatResult<()> {
        let mut join = f.join_nodes_with_hardline_ext(self.empty_lines);

        for rule in node {
            join.entry(rule.syntax(), &format_or_verbatim(rule.format()));
        }

        join.finish()
    }
}

impl FormatRuleWithOptions<RStatementList> for FormatRStatementList {
    type Options = FormatRStatementListOptions;

    fn with_options(mut self, options: Self::Options) -> Self {
        self.empty_lines = options.empty_lines;
        self
    }
}
