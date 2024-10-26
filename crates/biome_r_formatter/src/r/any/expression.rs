//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_r_syntax::AnyRExpression;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyRExpression;
impl FormatRule<AnyRExpression> for FormatAnyRExpression {
    type Context = RFormatContext;
    fn fmt(&self, node: &AnyRExpression, f: &mut RFormatter) -> FormatResult<()> {
        match node {
            AnyRExpression::AnyRValue(node) => node.format().fmt(f),
            AnyRExpression::RBinaryExpression(node) => node.format().fmt(f),
            AnyRExpression::RBogusExpression(node) => node.format().fmt(f),
            AnyRExpression::RFunctionDefinition(node) => node.format().fmt(f),
            AnyRExpression::RIdentifier(node) => node.format().fmt(f),
        }
    }
}
