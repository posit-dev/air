use crate::FormatBogusNodeRule;
use air_r_syntax::RBogus;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRBogus;
impl FormatBogusNodeRule<RBogus> for FormatRBogus {}
