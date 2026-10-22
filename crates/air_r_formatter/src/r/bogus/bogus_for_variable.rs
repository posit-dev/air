use crate::FormatBogusNodeRule;
use air_r_syntax::RBogusForVariable;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRBogusForVariable;
impl FormatBogusNodeRule<RBogusForVariable> for FormatRBogusForVariable {}
