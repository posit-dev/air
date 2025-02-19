use crate::prelude::*;
use crate::r::lists::expression_list::ExpressionListKind;
use crate::r::lists::expression_list::FormatRExpressionListOptions;
use air_r_syntax::RRoot;
use air_r_syntax::RRootFields;
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRRoot;
impl FormatNodeRule<RRoot> for FormatRRoot {
    fn fmt_fields(&self, node: &RRoot, f: &mut RFormatter) -> FormatResult<()> {
        let RRootFields {
            bom_token,
            expressions,
            eof_token,
        } = node.as_fields();

        let options = FormatRExpressionListOptions {
            kind: ExpressionListKind::Program,
        };

        write!(
            f,
            [
                bom_token.format(),
                expressions.format().with_options(options),
                hard_line_break(),
                format_removed(&eof_token?),
            ]
        )
    }
}
