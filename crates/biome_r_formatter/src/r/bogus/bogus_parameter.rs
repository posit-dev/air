use crate::FormatBogusNodeRule;
use biome_r_syntax::RBogusParameter;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRBogusParameter;
impl FormatBogusNodeRule<RBogusParameter> for FormatRBogusParameter {}
