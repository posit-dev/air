use biome_rowan::AstSeparatedList;
use biome_rowan::SyntaxResult;

use crate::AnyRArgument;
use crate::AnyRExpression;
use crate::AnyRValue;
use crate::RCall;
use crate::RCallArguments;

impl RCall {
    pub fn is_test_call(&self) -> SyntaxResult<bool> {
        let arguments = self.arguments()?;
        Ok(Self::is_titled_block_call(&arguments))
    }

    /// Tests for titled blocks like `test_that()`:
    ///
    /// ```r
    /// test_that("description", {
    ///   1 + 1
    /// })
    ///
    /// # We don't test explicitly for `test_that`, so this works too
    /// furrr_test_that("description", {
    ///
    /// })
    ///
    /// # But unfortunately not this, but we are ok with that
    /// test_that_cli(config = c("a", "b"), "description", {
    ///
    /// })
    /// ```
    fn is_titled_block_call(arguments: &RCallArguments) -> bool {
        let mut arguments = arguments.items().iter();

        // Unwraps `AnyRArgument` that are internally named or unnamed arguments
        // into their `AnyRExpression`
        let argument_expression = |arg| match arg {
            AnyRArgument::RNamedArgument(arg) => arg.value(),
            AnyRArgument::RUnnamedArgument(arg) => arg.value().ok(),
            AnyRArgument::RBogusArgument(_) | AnyRArgument::RHoleArgument(_) => None,
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

        true
    }
}
