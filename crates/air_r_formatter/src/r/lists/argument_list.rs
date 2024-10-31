use crate::prelude::*;
use crate::FormattedIterExt;
use air_r_syntax::RArgumentList;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRArgumentList;
impl FormatRule<RArgumentList> for FormatRArgumentList {
    type Context = RFormatContext;
    fn fmt(&self, node: &RArgumentList, f: &mut RFormatter) -> FormatResult<()> {
        f.join().entries(node.iter().formatted()).finish()
    }
}
