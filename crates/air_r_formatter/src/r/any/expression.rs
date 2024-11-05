//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use air_r_syntax::AnyRExpression;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyRExpression;
impl FormatRule<AnyRExpression> for FormatAnyRExpression {
    type Context = RFormatContext;
    fn fmt(&self, node: &AnyRExpression, f: &mut RFormatter) -> FormatResult<()> {
        match node {
            AnyRExpression::AnyRValue(node) => node.format().fmt(f),
            AnyRExpression::RBinaryExpression(node) => node.format().fmt(f),
            AnyRExpression::RBogusExpression(node) => node.format().fmt(f),
            AnyRExpression::RBracedExpressions(node) => node.format().fmt(f),
            AnyRExpression::RCall(node) => node.format().fmt(f),
            AnyRExpression::RForStatement(node) => node.format().fmt(f),
            AnyRExpression::RFunctionDefinition(node) => node.format().fmt(f),
            AnyRExpression::RIdentifier(node) => node.format().fmt(f),
            AnyRExpression::RIfStatement(node) => node.format().fmt(f),
            AnyRExpression::RParenthesizedExpression(node) => node.format().fmt(f),
            AnyRExpression::RRepeatStatement(node) => node.format().fmt(f),
            AnyRExpression::RWhileStatement(node) => node.format().fmt(f),
        }
    }
}
