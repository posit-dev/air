use crate::FormatBogusNodeRule;
use air_r_syntax::RBogusParameterName;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRBogusParameterName;
impl FormatBogusNodeRule<RBogusParameterName> for FormatRBogusParameterName {}
