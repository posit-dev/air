use crate::FormatBogusNodeRule;
use air_r_syntax::RBogusArgument;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRBogusArgument;
impl FormatBogusNodeRule<RBogusArgument> for FormatRBogusArgument {}
