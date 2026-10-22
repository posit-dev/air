use crate::prelude::*;
use air_r_syntax::REmptyStatement;
use air_r_syntax::REmptyStatementFields;
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatREmptyStatement;
impl FormatNodeRule<REmptyStatement> for FormatREmptyStatement {
    fn fmt_fields(&self, node: &REmptyStatement, f: &mut RFormatter) -> FormatResult<()> {
        let REmptyStatementFields { semicolon_token } = node.as_fields();

        // Comment placement preemptively moves comments off of empty statements
        debug_assert!(
            !f.comments().has_comments(node.syntax()),
            "`REmptyStatement` can't have comments."
        );

        // We never write semicolons
        write!(f, [format_removed(&semicolon_token?)])
    }
}
