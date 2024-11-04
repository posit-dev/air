use crate::prelude::*;
use air_r_syntax::RHoleArgument;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRHoleArgument;
impl FormatNodeRule<RHoleArgument> for FormatRHoleArgument {
    fn fmt_fields(&self, _node: &RHoleArgument, _f: &mut RFormatter) -> FormatResult<()> {
        Ok(())
    }
}
