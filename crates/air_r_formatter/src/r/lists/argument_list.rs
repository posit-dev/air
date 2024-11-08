use crate::prelude::*;
use air_r_syntax::RArgumentList;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRArgumentList;
impl FormatRule<RArgumentList> for FormatRArgumentList {
    type Context = RFormatContext;
    fn fmt(&self, _node: &RArgumentList, _f: &mut RFormatter) -> FormatResult<()> {
        unreachable!("Should have been handled by `RCallArguments`.");
    }
}
