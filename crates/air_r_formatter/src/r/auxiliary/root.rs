use crate::prelude::*;
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

        write!(
            f,
            [
                bom_token.format(),
                expressions.format(),
                hard_line_break(),
                format_removed(&eof_token?),
            ]
        )
    }
}
