use crate::{formatter::FormatterExt, prelude::*};
use air_r_syntax::RExpressionList;
use biome_formatter::FormatRuleWithOptions;

#[derive(Default, Debug, Clone, Copy)]
pub(crate) enum ExpressionListKind {
    #[default]
    Braces,

    Program,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRExpressionList {
    pub(crate) kind: ExpressionListKind,
}

#[derive(Default, Debug, Clone)]
pub(crate) struct FormatRExpressionListOptions {
    pub(crate) kind: ExpressionListKind,
}

impl FormatRule<RExpressionList> for FormatRExpressionList {
    type Context = RFormatContext;
    fn fmt(&self, node: &RExpressionList, f: &mut RFormatter) -> FormatResult<()> {
        let stretchy = matches!(self.kind, ExpressionListKind::Program);
        let mut join = f.join_nodes_with_stretchy_hardline(stretchy);

        for rule in node {
            join.entry(rule.syntax(), &format_or_verbatim(rule.format()));
        }

        join.finish()
    }
}

impl FormatRuleWithOptions<RExpressionList> for FormatRExpressionList {
    type Options = FormatRExpressionListOptions;

    fn with_options(mut self, options: Self::Options) -> Self {
        self.kind = options.kind;
        self
    }
}
