use crate::loop_body::FormatLoopBody;
use crate::prelude::*;
use air_r_syntax::RRepeatStatement;
use air_r_syntax::RRepeatStatementFields;
use biome_formatter::format_args;
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRRepeatStatement;
impl FormatNodeRule<RRepeatStatement> for FormatRRepeatStatement {
    fn fmt_fields(&self, node: &RRepeatStatement, f: &mut RFormatter) -> FormatResult<()> {
        let RRepeatStatementFields { repeat_token, body } = node.as_fields();

        write!(
            f,
            [group(&format_args!(
                repeat_token.format(),
                space(),
                FormatLoopBody::new(&body?)
            ))]
        )
    }
}
