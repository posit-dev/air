use crate::FormatBogusNodeRule;
use air_r_syntax::RBogusSelector;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRBogusSelector;
impl FormatBogusNodeRule<RBogusSelector> for FormatRBogusSelector {}
