use crate::prelude::*;
use air_r_syntax::RExpressionList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRExpressionList;
impl FormatRule<RExpressionList> for FormatRExpressionList {
    type Context = RFormatContext;
    fn fmt(&self, node: &RExpressionList, f: &mut RFormatter) -> FormatResult<()> {
        // This is one of the few cases where we _do_ want to respect empty
        // lines from the input, so we can use `join_nodes_with_hardline`.
        let mut join = f.join_nodes_with_hardline();

        for rule in node {
            join.entry(rule.syntax(), &format_or_verbatim(rule.format()));
        }

        join.finish()
    }
}
