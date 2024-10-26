use crate::FormatBogusNodeRule;
use biome_r_syntax::RBogusExpression;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRBogusExpression;
impl FormatBogusNodeRule<RBogusExpression> for FormatRBogusExpression {}
