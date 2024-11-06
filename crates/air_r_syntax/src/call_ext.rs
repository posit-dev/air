use biome_rowan::AstNode;
use biome_rowan::AstSeparatedList;
use biome_rowan::SyntaxResult;

use crate::AnyRArgument;
use crate::AnyRExpression;
use crate::AnyRValue;
use crate::RCall;
use crate::RCallArguments;

impl RCall {
    pub fn is_test_call(&self) -> SyntaxResult<bool> {
        let callee = self.function()?;
        let arguments = self.arguments()?;
        Ok(Self::is_test_that_call(&callee, &arguments))
    }

    /// Tests for:
    ///
    /// ```r
    /// test_that("description", {
    ///   1 + 1
    /// })
    /// ```
    fn is_test_that_call(callee: &AnyRExpression, arguments: &RCallArguments) -> bool {
        let mut arguments = arguments.items().iter();

        // Unwraps `AnyRArgument` that are internally named or unnamed arguments
        // into their `AnyRExpression`
        let argument_expression = |arg| match arg {
            AnyRArgument::RNamedArgument(arg) => arg.value(),
            AnyRArgument::RUnnamedArgument(arg) => arg.value().ok(),
            AnyRArgument::RBogusArgument(_)
            | AnyRArgument::RDotsArgument(_)
            | AnyRArgument::RHoleArgument(_) => None,
        };

        // Must have exactly 2 arguments
        let Some(Ok(first)) = arguments.next() else {
            return false;
        };
        let Some(Ok(second)) = arguments.next() else {
            return false;
        };
        let None = arguments.next() else { return false };

        // Both args must be named or unnamed arguments
        let Some(first) = argument_expression(first) else {
            return false;
        };
        let Some(second) = argument_expression(second) else {
            return false;
        };

        // First must be a string
        if !matches!(first, AnyRExpression::AnyRValue(AnyRValue::RStringValue(_))) {
            return false;
        }

        // Second must be braces
        if !matches!(second, AnyRExpression::RBracedExpressions(_)) {
            return false;
        }

        // Callee must be `"test_that"`
        // (reserving string comparison for the end, may be most expensive part)
        callee.text().eq("test_that")
    }
}
