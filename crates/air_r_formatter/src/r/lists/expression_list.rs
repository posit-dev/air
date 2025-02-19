use crate::{formatter::FormatterExt, joiner::Joiner, prelude::*};
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

impl FormatRExpressionList {
    fn fmt_with_joiner<'fmt, 'buf, J, Separator>(
        &self,
        mut join: J,
        node: &RExpressionList,
    ) -> FormatResult<()>
    where
        Separator: Format<RFormatContext>,
        J: Joiner<'fmt, 'buf, Separator, RFormatContext>,
    {
        for rule in node {
            join.entry(rule.syntax(), &format_or_verbatim(rule.format()));
        }

        join.finish()
    }
}

impl FormatRule<RExpressionList> for FormatRExpressionList {
    type Context = RFormatContext;
    fn fmt(&self, node: &RExpressionList, f: &mut RFormatter) -> FormatResult<()> {
        match self.kind {
            ExpressionListKind::Braces => self.fmt_with_joiner(f.join_nodes_with_hardline(), node),
            ExpressionListKind::Program => {
                self.fmt_with_joiner(f.join_nodes_with_stretchy_hardline(), node)
            }
        }
    }
}

impl FormatRuleWithOptions<RExpressionList> for FormatRExpressionList {
    type Options = FormatRExpressionListOptions;

    fn with_options(mut self, options: Self::Options) -> Self {
        self.kind = options.kind;
        self
    }
}
