use crate::FormatBogusNodeRule;
use air_r_syntax::RBogusValue;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRBogusValue;
impl FormatBogusNodeRule<RBogusValue> for FormatRBogusValue {}
