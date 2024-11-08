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
            AnyRExpression::RBreakExpression(node) => node.format().fmt(f),
            AnyRExpression::RCall(node) => node.format().fmt(f),
            AnyRExpression::RDotDotI(node) => node.format().fmt(f),
            AnyRExpression::RExtractExpression(node) => node.format().fmt(f),
            AnyRExpression::RFalseExpression(node) => node.format().fmt(f),
            AnyRExpression::RForStatement(node) => node.format().fmt(f),
            AnyRExpression::RFunctionDefinition(node) => node.format().fmt(f),
            AnyRExpression::RIdentifier(node) => node.format().fmt(f),
            AnyRExpression::RIfStatement(node) => node.format().fmt(f),
            AnyRExpression::RInfExpression(node) => node.format().fmt(f),
            AnyRExpression::RNaExpression(node) => node.format().fmt(f),
            AnyRExpression::RNamespaceExpression(node) => node.format().fmt(f),
            AnyRExpression::RNanExpression(node) => node.format().fmt(f),
            AnyRExpression::RNextExpression(node) => node.format().fmt(f),
            AnyRExpression::RNullExpression(node) => node.format().fmt(f),
            AnyRExpression::RParenthesizedExpression(node) => node.format().fmt(f),
            AnyRExpression::RRepeatStatement(node) => node.format().fmt(f),
            AnyRExpression::RReturnExpression(node) => node.format().fmt(f),
            AnyRExpression::RTrueExpression(node) => node.format().fmt(f),
            AnyRExpression::RUnaryExpression(node) => node.format().fmt(f),
            AnyRExpression::RWhileStatement(node) => node.format().fmt(f),
        }
    }
}
