//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::{
    AsFormat, FormatBogusNodeRule, FormatNodeRule, IntoFormat, RFormatContext, RFormatter,
};
use biome_formatter::{FormatOwnedWithRule, FormatRefWithRule, FormatResult, FormatRule};
impl FormatRule<air_r_syntax::RBinaryExpression>
    for crate::r::auxiliary::binary_expression::FormatRBinaryExpression
{
    type Context = RFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &air_r_syntax::RBinaryExpression, f: &mut RFormatter) -> FormatResult<()> {
        FormatNodeRule::<air_r_syntax::RBinaryExpression>::fmt(self, node, f)
    }
}
impl AsFormat<RFormatContext> for air_r_syntax::RBinaryExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        air_r_syntax::RBinaryExpression,
        crate::r::auxiliary::binary_expression::FormatRBinaryExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::r::auxiliary::binary_expression::FormatRBinaryExpression::default(),
        )
    }
}
impl IntoFormat<RFormatContext> for air_r_syntax::RBinaryExpression {
    type Format = FormatOwnedWithRule<
        air_r_syntax::RBinaryExpression,
        crate::r::auxiliary::binary_expression::FormatRBinaryExpression,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::r::auxiliary::binary_expression::FormatRBinaryExpression::default(),
        )
    }
}
impl FormatRule<air_r_syntax::RBracedExpressions>
    for crate::r::auxiliary::braced_expressions::FormatRBracedExpressions
{
    type Context = RFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &air_r_syntax::RBracedExpressions, f: &mut RFormatter) -> FormatResult<()> {
        FormatNodeRule::<air_r_syntax::RBracedExpressions>::fmt(self, node, f)
    }
}
impl AsFormat<RFormatContext> for air_r_syntax::RBracedExpressions {
    type Format<'a> = FormatRefWithRule<
        'a,
        air_r_syntax::RBracedExpressions,
        crate::r::auxiliary::braced_expressions::FormatRBracedExpressions,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::r::auxiliary::braced_expressions::FormatRBracedExpressions::default(),
        )
    }
}
impl IntoFormat<RFormatContext> for air_r_syntax::RBracedExpressions {
    type Format = FormatOwnedWithRule<
        air_r_syntax::RBracedExpressions,
        crate::r::auxiliary::braced_expressions::FormatRBracedExpressions,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::r::auxiliary::braced_expressions::FormatRBracedExpressions::default(),
        )
    }
}
impl FormatRule<air_r_syntax::RBreakExpression>
    for crate::r::auxiliary::break_expression::FormatRBreakExpression
{
    type Context = RFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &air_r_syntax::RBreakExpression, f: &mut RFormatter) -> FormatResult<()> {
        FormatNodeRule::<air_r_syntax::RBreakExpression>::fmt(self, node, f)
    }
}
impl AsFormat<RFormatContext> for air_r_syntax::RBreakExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        air_r_syntax::RBreakExpression,
        crate::r::auxiliary::break_expression::FormatRBreakExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::r::auxiliary::break_expression::FormatRBreakExpression::default(),
        )
    }
}
impl IntoFormat<RFormatContext> for air_r_syntax::RBreakExpression {
    type Format = FormatOwnedWithRule<
        air_r_syntax::RBreakExpression,
        crate::r::auxiliary::break_expression::FormatRBreakExpression,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::r::auxiliary::break_expression::FormatRBreakExpression::default(),
        )
    }
}
impl FormatRule<air_r_syntax::RCall> for crate::r::auxiliary::call::FormatRCall {
    type Context = RFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &air_r_syntax::RCall, f: &mut RFormatter) -> FormatResult<()> {
        FormatNodeRule::<air_r_syntax::RCall>::fmt(self, node, f)
    }
}
impl AsFormat<RFormatContext> for air_r_syntax::RCall {
    type Format<'a> =
        FormatRefWithRule<'a, air_r_syntax::RCall, crate::r::auxiliary::call::FormatRCall>;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(self, crate::r::auxiliary::call::FormatRCall::default())
    }
}
impl IntoFormat<RFormatContext> for air_r_syntax::RCall {
    type Format = FormatOwnedWithRule<air_r_syntax::RCall, crate::r::auxiliary::call::FormatRCall>;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(self, crate::r::auxiliary::call::FormatRCall::default())
    }
}
impl FormatRule<air_r_syntax::RCallArguments>
    for crate::r::auxiliary::call_arguments::FormatRCallArguments
{
    type Context = RFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &air_r_syntax::RCallArguments, f: &mut RFormatter) -> FormatResult<()> {
        FormatNodeRule::<air_r_syntax::RCallArguments>::fmt(self, node, f)
    }
}
impl AsFormat<RFormatContext> for air_r_syntax::RCallArguments {
    type Format<'a> = FormatRefWithRule<
        'a,
        air_r_syntax::RCallArguments,
        crate::r::auxiliary::call_arguments::FormatRCallArguments,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::r::auxiliary::call_arguments::FormatRCallArguments::default(),
        )
    }
}
impl IntoFormat<RFormatContext> for air_r_syntax::RCallArguments {
    type Format = FormatOwnedWithRule<
        air_r_syntax::RCallArguments,
        crate::r::auxiliary::call_arguments::FormatRCallArguments,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::r::auxiliary::call_arguments::FormatRCallArguments::default(),
        )
    }
}
impl FormatRule<air_r_syntax::RComplexValue>
    for crate::r::auxiliary::complex_value::FormatRComplexValue
{
    type Context = RFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &air_r_syntax::RComplexValue, f: &mut RFormatter) -> FormatResult<()> {
        FormatNodeRule::<air_r_syntax::RComplexValue>::fmt(self, node, f)
    }
}
impl AsFormat<RFormatContext> for air_r_syntax::RComplexValue {
    type Format<'a> = FormatRefWithRule<
        'a,
        air_r_syntax::RComplexValue,
        crate::r::auxiliary::complex_value::FormatRComplexValue,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::r::auxiliary::complex_value::FormatRComplexValue::default(),
        )
    }
}
impl IntoFormat<RFormatContext> for air_r_syntax::RComplexValue {
    type Format = FormatOwnedWithRule<
        air_r_syntax::RComplexValue,
        crate::r::auxiliary::complex_value::FormatRComplexValue,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::r::auxiliary::complex_value::FormatRComplexValue::default(),
        )
    }
}
impl FormatRule<air_r_syntax::RDotDotI> for crate::r::auxiliary::dot_dot_i::FormatRDotDotI {
    type Context = RFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &air_r_syntax::RDotDotI, f: &mut RFormatter) -> FormatResult<()> {
        FormatNodeRule::<air_r_syntax::RDotDotI>::fmt(self, node, f)
    }
}
impl AsFormat<RFormatContext> for air_r_syntax::RDotDotI {
    type Format<'a> = FormatRefWithRule<
        'a,
        air_r_syntax::RDotDotI,
        crate::r::auxiliary::dot_dot_i::FormatRDotDotI,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::r::auxiliary::dot_dot_i::FormatRDotDotI::default(),
        )
    }
}
impl IntoFormat<RFormatContext> for air_r_syntax::RDotDotI {
    type Format =
        FormatOwnedWithRule<air_r_syntax::RDotDotI, crate::r::auxiliary::dot_dot_i::FormatRDotDotI>;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::r::auxiliary::dot_dot_i::FormatRDotDotI::default(),
        )
    }
}
impl FormatRule<air_r_syntax::RDots> for crate::r::auxiliary::dots::FormatRDots {
    type Context = RFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &air_r_syntax::RDots, f: &mut RFormatter) -> FormatResult<()> {
        FormatNodeRule::<air_r_syntax::RDots>::fmt(self, node, f)
    }
}
impl AsFormat<RFormatContext> for air_r_syntax::RDots {
    type Format<'a> =
        FormatRefWithRule<'a, air_r_syntax::RDots, crate::r::auxiliary::dots::FormatRDots>;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(self, crate::r::auxiliary::dots::FormatRDots::default())
    }
}
impl IntoFormat<RFormatContext> for air_r_syntax::RDots {
    type Format = FormatOwnedWithRule<air_r_syntax::RDots, crate::r::auxiliary::dots::FormatRDots>;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(self, crate::r::auxiliary::dots::FormatRDots::default())
    }
}
impl FormatRule<air_r_syntax::RDoubleValue>
    for crate::r::auxiliary::double_value::FormatRDoubleValue
{
    type Context = RFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &air_r_syntax::RDoubleValue, f: &mut RFormatter) -> FormatResult<()> {
        FormatNodeRule::<air_r_syntax::RDoubleValue>::fmt(self, node, f)
    }
}
impl AsFormat<RFormatContext> for air_r_syntax::RDoubleValue {
    type Format<'a> = FormatRefWithRule<
        'a,
        air_r_syntax::RDoubleValue,
        crate::r::auxiliary::double_value::FormatRDoubleValue,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::r::auxiliary::double_value::FormatRDoubleValue::default(),
        )
    }
}
impl IntoFormat<RFormatContext> for air_r_syntax::RDoubleValue {
    type Format = FormatOwnedWithRule<
        air_r_syntax::RDoubleValue,
        crate::r::auxiliary::double_value::FormatRDoubleValue,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::r::auxiliary::double_value::FormatRDoubleValue::default(),
        )
    }
}
impl FormatRule<air_r_syntax::RElseClause> for crate::r::auxiliary::else_clause::FormatRElseClause {
    type Context = RFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &air_r_syntax::RElseClause, f: &mut RFormatter) -> FormatResult<()> {
        FormatNodeRule::<air_r_syntax::RElseClause>::fmt(self, node, f)
    }
}
impl AsFormat<RFormatContext> for air_r_syntax::RElseClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        air_r_syntax::RElseClause,
        crate::r::auxiliary::else_clause::FormatRElseClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::r::auxiliary::else_clause::FormatRElseClause::default(),
        )
    }
}
impl IntoFormat<RFormatContext> for air_r_syntax::RElseClause {
    type Format = FormatOwnedWithRule<
        air_r_syntax::RElseClause,
        crate::r::auxiliary::else_clause::FormatRElseClause,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::r::auxiliary::else_clause::FormatRElseClause::default(),
        )
    }
}
impl FormatRule<air_r_syntax::RExtractExpression>
    for crate::r::auxiliary::extract_expression::FormatRExtractExpression
{
    type Context = RFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &air_r_syntax::RExtractExpression, f: &mut RFormatter) -> FormatResult<()> {
        FormatNodeRule::<air_r_syntax::RExtractExpression>::fmt(self, node, f)
    }
}
impl AsFormat<RFormatContext> for air_r_syntax::RExtractExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        air_r_syntax::RExtractExpression,
        crate::r::auxiliary::extract_expression::FormatRExtractExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::r::auxiliary::extract_expression::FormatRExtractExpression::default(),
        )
    }
}
impl IntoFormat<RFormatContext> for air_r_syntax::RExtractExpression {
    type Format = FormatOwnedWithRule<
        air_r_syntax::RExtractExpression,
        crate::r::auxiliary::extract_expression::FormatRExtractExpression,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::r::auxiliary::extract_expression::FormatRExtractExpression::default(),
        )
    }
}
impl FormatRule<air_r_syntax::RFalseExpression>
    for crate::r::auxiliary::false_expression::FormatRFalseExpression
{
    type Context = RFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &air_r_syntax::RFalseExpression, f: &mut RFormatter) -> FormatResult<()> {
        FormatNodeRule::<air_r_syntax::RFalseExpression>::fmt(self, node, f)
    }
}
impl AsFormat<RFormatContext> for air_r_syntax::RFalseExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        air_r_syntax::RFalseExpression,
        crate::r::auxiliary::false_expression::FormatRFalseExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::r::auxiliary::false_expression::FormatRFalseExpression::default(),
        )
    }
}
impl IntoFormat<RFormatContext> for air_r_syntax::RFalseExpression {
    type Format = FormatOwnedWithRule<
        air_r_syntax::RFalseExpression,
        crate::r::auxiliary::false_expression::FormatRFalseExpression,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::r::auxiliary::false_expression::FormatRFalseExpression::default(),
        )
    }
}
impl FormatRule<air_r_syntax::RForStatement>
    for crate::r::auxiliary::for_statement::FormatRForStatement
{
    type Context = RFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &air_r_syntax::RForStatement, f: &mut RFormatter) -> FormatResult<()> {
        FormatNodeRule::<air_r_syntax::RForStatement>::fmt(self, node, f)
    }
}
impl AsFormat<RFormatContext> for air_r_syntax::RForStatement {
    type Format<'a> = FormatRefWithRule<
        'a,
        air_r_syntax::RForStatement,
        crate::r::auxiliary::for_statement::FormatRForStatement,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::r::auxiliary::for_statement::FormatRForStatement::default(),
        )
    }
}
impl IntoFormat<RFormatContext> for air_r_syntax::RForStatement {
    type Format = FormatOwnedWithRule<
        air_r_syntax::RForStatement,
        crate::r::auxiliary::for_statement::FormatRForStatement,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::r::auxiliary::for_statement::FormatRForStatement::default(),
        )
    }
}
impl FormatRule<air_r_syntax::RFunctionDefinition>
    for crate::r::auxiliary::function_definition::FormatRFunctionDefinition
{
    type Context = RFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &air_r_syntax::RFunctionDefinition,
        f: &mut RFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<air_r_syntax::RFunctionDefinition>::fmt(self, node, f)
    }
}
impl AsFormat<RFormatContext> for air_r_syntax::RFunctionDefinition {
    type Format<'a> = FormatRefWithRule<
        'a,
        air_r_syntax::RFunctionDefinition,
        crate::r::auxiliary::function_definition::FormatRFunctionDefinition,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::r::auxiliary::function_definition::FormatRFunctionDefinition::default(),
        )
    }
}
impl IntoFormat<RFormatContext> for air_r_syntax::RFunctionDefinition {
    type Format = FormatOwnedWithRule<
        air_r_syntax::RFunctionDefinition,
        crate::r::auxiliary::function_definition::FormatRFunctionDefinition,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::r::auxiliary::function_definition::FormatRFunctionDefinition::default(),
        )
    }
}
impl FormatRule<air_r_syntax::RHoleArgument>
    for crate::r::auxiliary::hole_argument::FormatRHoleArgument
{
    type Context = RFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &air_r_syntax::RHoleArgument, f: &mut RFormatter) -> FormatResult<()> {
        FormatNodeRule::<air_r_syntax::RHoleArgument>::fmt(self, node, f)
    }
}
impl AsFormat<RFormatContext> for air_r_syntax::RHoleArgument {
    type Format<'a> = FormatRefWithRule<
        'a,
        air_r_syntax::RHoleArgument,
        crate::r::auxiliary::hole_argument::FormatRHoleArgument,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::r::auxiliary::hole_argument::FormatRHoleArgument::default(),
        )
    }
}
impl IntoFormat<RFormatContext> for air_r_syntax::RHoleArgument {
    type Format = FormatOwnedWithRule<
        air_r_syntax::RHoleArgument,
        crate::r::auxiliary::hole_argument::FormatRHoleArgument,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::r::auxiliary::hole_argument::FormatRHoleArgument::default(),
        )
    }
}
impl FormatRule<air_r_syntax::RIdentifier> for crate::r::auxiliary::identifier::FormatRIdentifier {
    type Context = RFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &air_r_syntax::RIdentifier, f: &mut RFormatter) -> FormatResult<()> {
        FormatNodeRule::<air_r_syntax::RIdentifier>::fmt(self, node, f)
    }
}
impl AsFormat<RFormatContext> for air_r_syntax::RIdentifier {
    type Format<'a> = FormatRefWithRule<
        'a,
        air_r_syntax::RIdentifier,
        crate::r::auxiliary::identifier::FormatRIdentifier,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::r::auxiliary::identifier::FormatRIdentifier::default(),
        )
    }
}
impl IntoFormat<RFormatContext> for air_r_syntax::RIdentifier {
    type Format = FormatOwnedWithRule<
        air_r_syntax::RIdentifier,
        crate::r::auxiliary::identifier::FormatRIdentifier,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::r::auxiliary::identifier::FormatRIdentifier::default(),
        )
    }
}
impl FormatRule<air_r_syntax::RIfStatement>
    for crate::r::auxiliary::if_statement::FormatRIfStatement
{
    type Context = RFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &air_r_syntax::RIfStatement, f: &mut RFormatter) -> FormatResult<()> {
        FormatNodeRule::<air_r_syntax::RIfStatement>::fmt(self, node, f)
    }
}
impl AsFormat<RFormatContext> for air_r_syntax::RIfStatement {
    type Format<'a> = FormatRefWithRule<
        'a,
        air_r_syntax::RIfStatement,
        crate::r::auxiliary::if_statement::FormatRIfStatement,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::r::auxiliary::if_statement::FormatRIfStatement::default(),
        )
    }
}
impl IntoFormat<RFormatContext> for air_r_syntax::RIfStatement {
    type Format = FormatOwnedWithRule<
        air_r_syntax::RIfStatement,
        crate::r::auxiliary::if_statement::FormatRIfStatement,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::r::auxiliary::if_statement::FormatRIfStatement::default(),
        )
    }
}
impl FormatRule<air_r_syntax::RInfExpression>
    for crate::r::auxiliary::inf_expression::FormatRInfExpression
{
    type Context = RFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &air_r_syntax::RInfExpression, f: &mut RFormatter) -> FormatResult<()> {
        FormatNodeRule::<air_r_syntax::RInfExpression>::fmt(self, node, f)
    }
}
impl AsFormat<RFormatContext> for air_r_syntax::RInfExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        air_r_syntax::RInfExpression,
        crate::r::auxiliary::inf_expression::FormatRInfExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::r::auxiliary::inf_expression::FormatRInfExpression::default(),
        )
    }
}
impl IntoFormat<RFormatContext> for air_r_syntax::RInfExpression {
    type Format = FormatOwnedWithRule<
        air_r_syntax::RInfExpression,
        crate::r::auxiliary::inf_expression::FormatRInfExpression,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::r::auxiliary::inf_expression::FormatRInfExpression::default(),
        )
    }
}
impl FormatRule<air_r_syntax::RIntegerValue>
    for crate::r::auxiliary::integer_value::FormatRIntegerValue
{
    type Context = RFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &air_r_syntax::RIntegerValue, f: &mut RFormatter) -> FormatResult<()> {
        FormatNodeRule::<air_r_syntax::RIntegerValue>::fmt(self, node, f)
    }
}
impl AsFormat<RFormatContext> for air_r_syntax::RIntegerValue {
    type Format<'a> = FormatRefWithRule<
        'a,
        air_r_syntax::RIntegerValue,
        crate::r::auxiliary::integer_value::FormatRIntegerValue,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::r::auxiliary::integer_value::FormatRIntegerValue::default(),
        )
    }
}
impl IntoFormat<RFormatContext> for air_r_syntax::RIntegerValue {
    type Format = FormatOwnedWithRule<
        air_r_syntax::RIntegerValue,
        crate::r::auxiliary::integer_value::FormatRIntegerValue,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::r::auxiliary::integer_value::FormatRIntegerValue::default(),
        )
    }
}
impl FormatRule<air_r_syntax::RNaExpression>
    for crate::r::auxiliary::na_expression::FormatRNaExpression
{
    type Context = RFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &air_r_syntax::RNaExpression, f: &mut RFormatter) -> FormatResult<()> {
        FormatNodeRule::<air_r_syntax::RNaExpression>::fmt(self, node, f)
    }
}
impl AsFormat<RFormatContext> for air_r_syntax::RNaExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        air_r_syntax::RNaExpression,
        crate::r::auxiliary::na_expression::FormatRNaExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::r::auxiliary::na_expression::FormatRNaExpression::default(),
        )
    }
}
impl IntoFormat<RFormatContext> for air_r_syntax::RNaExpression {
    type Format = FormatOwnedWithRule<
        air_r_syntax::RNaExpression,
        crate::r::auxiliary::na_expression::FormatRNaExpression,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::r::auxiliary::na_expression::FormatRNaExpression::default(),
        )
    }
}
impl FormatRule<air_r_syntax::RNamedArgument>
    for crate::r::auxiliary::named_argument::FormatRNamedArgument
{
    type Context = RFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &air_r_syntax::RNamedArgument, f: &mut RFormatter) -> FormatResult<()> {
        FormatNodeRule::<air_r_syntax::RNamedArgument>::fmt(self, node, f)
    }
}
impl AsFormat<RFormatContext> for air_r_syntax::RNamedArgument {
    type Format<'a> = FormatRefWithRule<
        'a,
        air_r_syntax::RNamedArgument,
        crate::r::auxiliary::named_argument::FormatRNamedArgument,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::r::auxiliary::named_argument::FormatRNamedArgument::default(),
        )
    }
}
impl IntoFormat<RFormatContext> for air_r_syntax::RNamedArgument {
    type Format = FormatOwnedWithRule<
        air_r_syntax::RNamedArgument,
        crate::r::auxiliary::named_argument::FormatRNamedArgument,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::r::auxiliary::named_argument::FormatRNamedArgument::default(),
        )
    }
}
impl FormatRule<air_r_syntax::RNamespaceExpression>
    for crate::r::auxiliary::namespace_expression::FormatRNamespaceExpression
{
    type Context = RFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &air_r_syntax::RNamespaceExpression,
        f: &mut RFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<air_r_syntax::RNamespaceExpression>::fmt(self, node, f)
    }
}
impl AsFormat<RFormatContext> for air_r_syntax::RNamespaceExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        air_r_syntax::RNamespaceExpression,
        crate::r::auxiliary::namespace_expression::FormatRNamespaceExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::r::auxiliary::namespace_expression::FormatRNamespaceExpression::default(),
        )
    }
}
impl IntoFormat<RFormatContext> for air_r_syntax::RNamespaceExpression {
    type Format = FormatOwnedWithRule<
        air_r_syntax::RNamespaceExpression,
        crate::r::auxiliary::namespace_expression::FormatRNamespaceExpression,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::r::auxiliary::namespace_expression::FormatRNamespaceExpression::default(),
        )
    }
}
impl FormatRule<air_r_syntax::RNanExpression>
    for crate::r::auxiliary::nan_expression::FormatRNanExpression
{
    type Context = RFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &air_r_syntax::RNanExpression, f: &mut RFormatter) -> FormatResult<()> {
        FormatNodeRule::<air_r_syntax::RNanExpression>::fmt(self, node, f)
    }
}
impl AsFormat<RFormatContext> for air_r_syntax::RNanExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        air_r_syntax::RNanExpression,
        crate::r::auxiliary::nan_expression::FormatRNanExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::r::auxiliary::nan_expression::FormatRNanExpression::default(),
        )
    }
}
impl IntoFormat<RFormatContext> for air_r_syntax::RNanExpression {
    type Format = FormatOwnedWithRule<
        air_r_syntax::RNanExpression,
        crate::r::auxiliary::nan_expression::FormatRNanExpression,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::r::auxiliary::nan_expression::FormatRNanExpression::default(),
        )
    }
}
impl FormatRule<air_r_syntax::RNextExpression>
    for crate::r::auxiliary::next_expression::FormatRNextExpression
{
    type Context = RFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &air_r_syntax::RNextExpression, f: &mut RFormatter) -> FormatResult<()> {
        FormatNodeRule::<air_r_syntax::RNextExpression>::fmt(self, node, f)
    }
}
impl AsFormat<RFormatContext> for air_r_syntax::RNextExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        air_r_syntax::RNextExpression,
        crate::r::auxiliary::next_expression::FormatRNextExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::r::auxiliary::next_expression::FormatRNextExpression::default(),
        )
    }
}
impl IntoFormat<RFormatContext> for air_r_syntax::RNextExpression {
    type Format = FormatOwnedWithRule<
        air_r_syntax::RNextExpression,
        crate::r::auxiliary::next_expression::FormatRNextExpression,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::r::auxiliary::next_expression::FormatRNextExpression::default(),
        )
    }
}
impl FormatRule<air_r_syntax::RNullExpression>
    for crate::r::auxiliary::null_expression::FormatRNullExpression
{
    type Context = RFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &air_r_syntax::RNullExpression, f: &mut RFormatter) -> FormatResult<()> {
        FormatNodeRule::<air_r_syntax::RNullExpression>::fmt(self, node, f)
    }
}
impl AsFormat<RFormatContext> for air_r_syntax::RNullExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        air_r_syntax::RNullExpression,
        crate::r::auxiliary::null_expression::FormatRNullExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::r::auxiliary::null_expression::FormatRNullExpression::default(),
        )
    }
}
impl IntoFormat<RFormatContext> for air_r_syntax::RNullExpression {
    type Format = FormatOwnedWithRule<
        air_r_syntax::RNullExpression,
        crate::r::auxiliary::null_expression::FormatRNullExpression,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::r::auxiliary::null_expression::FormatRNullExpression::default(),
        )
    }
}
impl FormatRule<air_r_syntax::RParameter> for crate::r::auxiliary::parameter::FormatRParameter {
    type Context = RFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &air_r_syntax::RParameter, f: &mut RFormatter) -> FormatResult<()> {
        FormatNodeRule::<air_r_syntax::RParameter>::fmt(self, node, f)
    }
}
impl AsFormat<RFormatContext> for air_r_syntax::RParameter {
    type Format<'a> = FormatRefWithRule<
        'a,
        air_r_syntax::RParameter,
        crate::r::auxiliary::parameter::FormatRParameter,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::r::auxiliary::parameter::FormatRParameter::default(),
        )
    }
}
impl IntoFormat<RFormatContext> for air_r_syntax::RParameter {
    type Format = FormatOwnedWithRule<
        air_r_syntax::RParameter,
        crate::r::auxiliary::parameter::FormatRParameter,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::r::auxiliary::parameter::FormatRParameter::default(),
        )
    }
}
impl FormatRule<air_r_syntax::RParameterDefault>
    for crate::r::auxiliary::parameter_default::FormatRParameterDefault
{
    type Context = RFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &air_r_syntax::RParameterDefault, f: &mut RFormatter) -> FormatResult<()> {
        FormatNodeRule::<air_r_syntax::RParameterDefault>::fmt(self, node, f)
    }
}
impl AsFormat<RFormatContext> for air_r_syntax::RParameterDefault {
    type Format<'a> = FormatRefWithRule<
        'a,
        air_r_syntax::RParameterDefault,
        crate::r::auxiliary::parameter_default::FormatRParameterDefault,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::r::auxiliary::parameter_default::FormatRParameterDefault::default(),
        )
    }
}
impl IntoFormat<RFormatContext> for air_r_syntax::RParameterDefault {
    type Format = FormatOwnedWithRule<
        air_r_syntax::RParameterDefault,
        crate::r::auxiliary::parameter_default::FormatRParameterDefault,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::r::auxiliary::parameter_default::FormatRParameterDefault::default(),
        )
    }
}
impl FormatRule<air_r_syntax::RParameters> for crate::r::auxiliary::parameters::FormatRParameters {
    type Context = RFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &air_r_syntax::RParameters, f: &mut RFormatter) -> FormatResult<()> {
        FormatNodeRule::<air_r_syntax::RParameters>::fmt(self, node, f)
    }
}
impl AsFormat<RFormatContext> for air_r_syntax::RParameters {
    type Format<'a> = FormatRefWithRule<
        'a,
        air_r_syntax::RParameters,
        crate::r::auxiliary::parameters::FormatRParameters,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::r::auxiliary::parameters::FormatRParameters::default(),
        )
    }
}
impl IntoFormat<RFormatContext> for air_r_syntax::RParameters {
    type Format = FormatOwnedWithRule<
        air_r_syntax::RParameters,
        crate::r::auxiliary::parameters::FormatRParameters,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::r::auxiliary::parameters::FormatRParameters::default(),
        )
    }
}
impl FormatRule<air_r_syntax::RParenthesizedExpression>
    for crate::r::auxiliary::parenthesized_expression::FormatRParenthesizedExpression
{
    type Context = RFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &air_r_syntax::RParenthesizedExpression,
        f: &mut RFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<air_r_syntax::RParenthesizedExpression>::fmt(self, node, f)
    }
}
impl AsFormat<RFormatContext> for air_r_syntax::RParenthesizedExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        air_r_syntax::RParenthesizedExpression,
        crate::r::auxiliary::parenthesized_expression::FormatRParenthesizedExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::r::auxiliary::parenthesized_expression::FormatRParenthesizedExpression::default(
            ),
        )
    }
}
impl IntoFormat<RFormatContext> for air_r_syntax::RParenthesizedExpression {
    type Format = FormatOwnedWithRule<
        air_r_syntax::RParenthesizedExpression,
        crate::r::auxiliary::parenthesized_expression::FormatRParenthesizedExpression,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::r::auxiliary::parenthesized_expression::FormatRParenthesizedExpression::default(
            ),
        )
    }
}
impl FormatRule<air_r_syntax::RRepeatStatement>
    for crate::r::auxiliary::repeat_statement::FormatRRepeatStatement
{
    type Context = RFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &air_r_syntax::RRepeatStatement, f: &mut RFormatter) -> FormatResult<()> {
        FormatNodeRule::<air_r_syntax::RRepeatStatement>::fmt(self, node, f)
    }
}
impl AsFormat<RFormatContext> for air_r_syntax::RRepeatStatement {
    type Format<'a> = FormatRefWithRule<
        'a,
        air_r_syntax::RRepeatStatement,
        crate::r::auxiliary::repeat_statement::FormatRRepeatStatement,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::r::auxiliary::repeat_statement::FormatRRepeatStatement::default(),
        )
    }
}
impl IntoFormat<RFormatContext> for air_r_syntax::RRepeatStatement {
    type Format = FormatOwnedWithRule<
        air_r_syntax::RRepeatStatement,
        crate::r::auxiliary::repeat_statement::FormatRRepeatStatement,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::r::auxiliary::repeat_statement::FormatRRepeatStatement::default(),
        )
    }
}
impl FormatRule<air_r_syntax::RReturnExpression>
    for crate::r::auxiliary::return_expression::FormatRReturnExpression
{
    type Context = RFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &air_r_syntax::RReturnExpression, f: &mut RFormatter) -> FormatResult<()> {
        FormatNodeRule::<air_r_syntax::RReturnExpression>::fmt(self, node, f)
    }
}
impl AsFormat<RFormatContext> for air_r_syntax::RReturnExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        air_r_syntax::RReturnExpression,
        crate::r::auxiliary::return_expression::FormatRReturnExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::r::auxiliary::return_expression::FormatRReturnExpression::default(),
        )
    }
}
impl IntoFormat<RFormatContext> for air_r_syntax::RReturnExpression {
    type Format = FormatOwnedWithRule<
        air_r_syntax::RReturnExpression,
        crate::r::auxiliary::return_expression::FormatRReturnExpression,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::r::auxiliary::return_expression::FormatRReturnExpression::default(),
        )
    }
}
impl FormatRule<air_r_syntax::RRoot> for crate::r::auxiliary::root::FormatRRoot {
    type Context = RFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &air_r_syntax::RRoot, f: &mut RFormatter) -> FormatResult<()> {
        FormatNodeRule::<air_r_syntax::RRoot>::fmt(self, node, f)
    }
}
impl AsFormat<RFormatContext> for air_r_syntax::RRoot {
    type Format<'a> =
        FormatRefWithRule<'a, air_r_syntax::RRoot, crate::r::auxiliary::root::FormatRRoot>;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(self, crate::r::auxiliary::root::FormatRRoot::default())
    }
}
impl IntoFormat<RFormatContext> for air_r_syntax::RRoot {
    type Format = FormatOwnedWithRule<air_r_syntax::RRoot, crate::r::auxiliary::root::FormatRRoot>;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(self, crate::r::auxiliary::root::FormatRRoot::default())
    }
}
impl FormatRule<air_r_syntax::RStringValue>
    for crate::r::auxiliary::string_value::FormatRStringValue
{
    type Context = RFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &air_r_syntax::RStringValue, f: &mut RFormatter) -> FormatResult<()> {
        FormatNodeRule::<air_r_syntax::RStringValue>::fmt(self, node, f)
    }
}
impl AsFormat<RFormatContext> for air_r_syntax::RStringValue {
    type Format<'a> = FormatRefWithRule<
        'a,
        air_r_syntax::RStringValue,
        crate::r::auxiliary::string_value::FormatRStringValue,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::r::auxiliary::string_value::FormatRStringValue::default(),
        )
    }
}
impl IntoFormat<RFormatContext> for air_r_syntax::RStringValue {
    type Format = FormatOwnedWithRule<
        air_r_syntax::RStringValue,
        crate::r::auxiliary::string_value::FormatRStringValue,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::r::auxiliary::string_value::FormatRStringValue::default(),
        )
    }
}
impl FormatRule<air_r_syntax::RSubset> for crate::r::auxiliary::subset::FormatRSubset {
    type Context = RFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &air_r_syntax::RSubset, f: &mut RFormatter) -> FormatResult<()> {
        FormatNodeRule::<air_r_syntax::RSubset>::fmt(self, node, f)
    }
}
impl AsFormat<RFormatContext> for air_r_syntax::RSubset {
    type Format<'a> =
        FormatRefWithRule<'a, air_r_syntax::RSubset, crate::r::auxiliary::subset::FormatRSubset>;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(self, crate::r::auxiliary::subset::FormatRSubset::default())
    }
}
impl IntoFormat<RFormatContext> for air_r_syntax::RSubset {
    type Format =
        FormatOwnedWithRule<air_r_syntax::RSubset, crate::r::auxiliary::subset::FormatRSubset>;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(self, crate::r::auxiliary::subset::FormatRSubset::default())
    }
}
impl FormatRule<air_r_syntax::RSubset2> for crate::r::auxiliary::subset_2::FormatRSubset2 {
    type Context = RFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &air_r_syntax::RSubset2, f: &mut RFormatter) -> FormatResult<()> {
        FormatNodeRule::<air_r_syntax::RSubset2>::fmt(self, node, f)
    }
}
impl AsFormat<RFormatContext> for air_r_syntax::RSubset2 {
    type Format<'a> = FormatRefWithRule<
        'a,
        air_r_syntax::RSubset2,
        crate::r::auxiliary::subset_2::FormatRSubset2,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::r::auxiliary::subset_2::FormatRSubset2::default(),
        )
    }
}
impl IntoFormat<RFormatContext> for air_r_syntax::RSubset2 {
    type Format =
        FormatOwnedWithRule<air_r_syntax::RSubset2, crate::r::auxiliary::subset_2::FormatRSubset2>;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::r::auxiliary::subset_2::FormatRSubset2::default(),
        )
    }
}
impl FormatRule<air_r_syntax::RSubset2Arguments>
    for crate::r::auxiliary::subset_2_arguments::FormatRSubset2Arguments
{
    type Context = RFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &air_r_syntax::RSubset2Arguments, f: &mut RFormatter) -> FormatResult<()> {
        FormatNodeRule::<air_r_syntax::RSubset2Arguments>::fmt(self, node, f)
    }
}
impl AsFormat<RFormatContext> for air_r_syntax::RSubset2Arguments {
    type Format<'a> = FormatRefWithRule<
        'a,
        air_r_syntax::RSubset2Arguments,
        crate::r::auxiliary::subset_2_arguments::FormatRSubset2Arguments,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::r::auxiliary::subset_2_arguments::FormatRSubset2Arguments::default(),
        )
    }
}
impl IntoFormat<RFormatContext> for air_r_syntax::RSubset2Arguments {
    type Format = FormatOwnedWithRule<
        air_r_syntax::RSubset2Arguments,
        crate::r::auxiliary::subset_2_arguments::FormatRSubset2Arguments,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::r::auxiliary::subset_2_arguments::FormatRSubset2Arguments::default(),
        )
    }
}
impl FormatRule<air_r_syntax::RSubsetArguments>
    for crate::r::auxiliary::subset_arguments::FormatRSubsetArguments
{
    type Context = RFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &air_r_syntax::RSubsetArguments, f: &mut RFormatter) -> FormatResult<()> {
        FormatNodeRule::<air_r_syntax::RSubsetArguments>::fmt(self, node, f)
    }
}
impl AsFormat<RFormatContext> for air_r_syntax::RSubsetArguments {
    type Format<'a> = FormatRefWithRule<
        'a,
        air_r_syntax::RSubsetArguments,
        crate::r::auxiliary::subset_arguments::FormatRSubsetArguments,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::r::auxiliary::subset_arguments::FormatRSubsetArguments::default(),
        )
    }
}
impl IntoFormat<RFormatContext> for air_r_syntax::RSubsetArguments {
    type Format = FormatOwnedWithRule<
        air_r_syntax::RSubsetArguments,
        crate::r::auxiliary::subset_arguments::FormatRSubsetArguments,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::r::auxiliary::subset_arguments::FormatRSubsetArguments::default(),
        )
    }
}
impl FormatRule<air_r_syntax::RTrueExpression>
    for crate::r::auxiliary::true_expression::FormatRTrueExpression
{
    type Context = RFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &air_r_syntax::RTrueExpression, f: &mut RFormatter) -> FormatResult<()> {
        FormatNodeRule::<air_r_syntax::RTrueExpression>::fmt(self, node, f)
    }
}
impl AsFormat<RFormatContext> for air_r_syntax::RTrueExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        air_r_syntax::RTrueExpression,
        crate::r::auxiliary::true_expression::FormatRTrueExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::r::auxiliary::true_expression::FormatRTrueExpression::default(),
        )
    }
}
impl IntoFormat<RFormatContext> for air_r_syntax::RTrueExpression {
    type Format = FormatOwnedWithRule<
        air_r_syntax::RTrueExpression,
        crate::r::auxiliary::true_expression::FormatRTrueExpression,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::r::auxiliary::true_expression::FormatRTrueExpression::default(),
        )
    }
}
impl FormatRule<air_r_syntax::RUnaryExpression>
    for crate::r::auxiliary::unary_expression::FormatRUnaryExpression
{
    type Context = RFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &air_r_syntax::RUnaryExpression, f: &mut RFormatter) -> FormatResult<()> {
        FormatNodeRule::<air_r_syntax::RUnaryExpression>::fmt(self, node, f)
    }
}
impl AsFormat<RFormatContext> for air_r_syntax::RUnaryExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        air_r_syntax::RUnaryExpression,
        crate::r::auxiliary::unary_expression::FormatRUnaryExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::r::auxiliary::unary_expression::FormatRUnaryExpression::default(),
        )
    }
}
impl IntoFormat<RFormatContext> for air_r_syntax::RUnaryExpression {
    type Format = FormatOwnedWithRule<
        air_r_syntax::RUnaryExpression,
        crate::r::auxiliary::unary_expression::FormatRUnaryExpression,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::r::auxiliary::unary_expression::FormatRUnaryExpression::default(),
        )
    }
}
impl FormatRule<air_r_syntax::RUnnamedArgument>
    for crate::r::auxiliary::unnamed_argument::FormatRUnnamedArgument
{
    type Context = RFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &air_r_syntax::RUnnamedArgument, f: &mut RFormatter) -> FormatResult<()> {
        FormatNodeRule::<air_r_syntax::RUnnamedArgument>::fmt(self, node, f)
    }
}
impl AsFormat<RFormatContext> for air_r_syntax::RUnnamedArgument {
    type Format<'a> = FormatRefWithRule<
        'a,
        air_r_syntax::RUnnamedArgument,
        crate::r::auxiliary::unnamed_argument::FormatRUnnamedArgument,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::r::auxiliary::unnamed_argument::FormatRUnnamedArgument::default(),
        )
    }
}
impl IntoFormat<RFormatContext> for air_r_syntax::RUnnamedArgument {
    type Format = FormatOwnedWithRule<
        air_r_syntax::RUnnamedArgument,
        crate::r::auxiliary::unnamed_argument::FormatRUnnamedArgument,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::r::auxiliary::unnamed_argument::FormatRUnnamedArgument::default(),
        )
    }
}
impl FormatRule<air_r_syntax::RWhileStatement>
    for crate::r::auxiliary::while_statement::FormatRWhileStatement
{
    type Context = RFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &air_r_syntax::RWhileStatement, f: &mut RFormatter) -> FormatResult<()> {
        FormatNodeRule::<air_r_syntax::RWhileStatement>::fmt(self, node, f)
    }
}
impl AsFormat<RFormatContext> for air_r_syntax::RWhileStatement {
    type Format<'a> = FormatRefWithRule<
        'a,
        air_r_syntax::RWhileStatement,
        crate::r::auxiliary::while_statement::FormatRWhileStatement,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::r::auxiliary::while_statement::FormatRWhileStatement::default(),
        )
    }
}
impl IntoFormat<RFormatContext> for air_r_syntax::RWhileStatement {
    type Format = FormatOwnedWithRule<
        air_r_syntax::RWhileStatement,
        crate::r::auxiliary::while_statement::FormatRWhileStatement,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::r::auxiliary::while_statement::FormatRWhileStatement::default(),
        )
    }
}
impl AsFormat<RFormatContext> for air_r_syntax::RArgumentList {
    type Format<'a> = FormatRefWithRule<
        'a,
        air_r_syntax::RArgumentList,
        crate::r::lists::argument_list::FormatRArgumentList,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::r::lists::argument_list::FormatRArgumentList::default(),
        )
    }
}
impl IntoFormat<RFormatContext> for air_r_syntax::RArgumentList {
    type Format = FormatOwnedWithRule<
        air_r_syntax::RArgumentList,
        crate::r::lists::argument_list::FormatRArgumentList,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::r::lists::argument_list::FormatRArgumentList::default(),
        )
    }
}
impl AsFormat<RFormatContext> for air_r_syntax::RExpressionList {
    type Format<'a> = FormatRefWithRule<
        'a,
        air_r_syntax::RExpressionList,
        crate::r::lists::expression_list::FormatRExpressionList,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::r::lists::expression_list::FormatRExpressionList::default(),
        )
    }
}
impl IntoFormat<RFormatContext> for air_r_syntax::RExpressionList {
    type Format = FormatOwnedWithRule<
        air_r_syntax::RExpressionList,
        crate::r::lists::expression_list::FormatRExpressionList,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::r::lists::expression_list::FormatRExpressionList::default(),
        )
    }
}
impl AsFormat<RFormatContext> for air_r_syntax::RParameterList {
    type Format<'a> = FormatRefWithRule<
        'a,
        air_r_syntax::RParameterList,
        crate::r::lists::parameter_list::FormatRParameterList,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::r::lists::parameter_list::FormatRParameterList::default(),
        )
    }
}
impl IntoFormat<RFormatContext> for air_r_syntax::RParameterList {
    type Format = FormatOwnedWithRule<
        air_r_syntax::RParameterList,
        crate::r::lists::parameter_list::FormatRParameterList,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::r::lists::parameter_list::FormatRParameterList::default(),
        )
    }
}
impl FormatRule<air_r_syntax::RBogus> for crate::r::bogus::bogus::FormatRBogus {
    type Context = RFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &air_r_syntax::RBogus, f: &mut RFormatter) -> FormatResult<()> {
        FormatBogusNodeRule::<air_r_syntax::RBogus>::fmt(self, node, f)
    }
}
impl AsFormat<RFormatContext> for air_r_syntax::RBogus {
    type Format<'a> =
        FormatRefWithRule<'a, air_r_syntax::RBogus, crate::r::bogus::bogus::FormatRBogus>;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(self, crate::r::bogus::bogus::FormatRBogus::default())
    }
}
impl IntoFormat<RFormatContext> for air_r_syntax::RBogus {
    type Format = FormatOwnedWithRule<air_r_syntax::RBogus, crate::r::bogus::bogus::FormatRBogus>;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(self, crate::r::bogus::bogus::FormatRBogus::default())
    }
}
impl FormatRule<air_r_syntax::RBogusArgument>
    for crate::r::bogus::bogus_argument::FormatRBogusArgument
{
    type Context = RFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &air_r_syntax::RBogusArgument, f: &mut RFormatter) -> FormatResult<()> {
        FormatBogusNodeRule::<air_r_syntax::RBogusArgument>::fmt(self, node, f)
    }
}
impl AsFormat<RFormatContext> for air_r_syntax::RBogusArgument {
    type Format<'a> = FormatRefWithRule<
        'a,
        air_r_syntax::RBogusArgument,
        crate::r::bogus::bogus_argument::FormatRBogusArgument,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::r::bogus::bogus_argument::FormatRBogusArgument::default(),
        )
    }
}
impl IntoFormat<RFormatContext> for air_r_syntax::RBogusArgument {
    type Format = FormatOwnedWithRule<
        air_r_syntax::RBogusArgument,
        crate::r::bogus::bogus_argument::FormatRBogusArgument,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::r::bogus::bogus_argument::FormatRBogusArgument::default(),
        )
    }
}
impl FormatRule<air_r_syntax::RBogusExpression>
    for crate::r::bogus::bogus_expression::FormatRBogusExpression
{
    type Context = RFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &air_r_syntax::RBogusExpression, f: &mut RFormatter) -> FormatResult<()> {
        FormatBogusNodeRule::<air_r_syntax::RBogusExpression>::fmt(self, node, f)
    }
}
impl AsFormat<RFormatContext> for air_r_syntax::RBogusExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        air_r_syntax::RBogusExpression,
        crate::r::bogus::bogus_expression::FormatRBogusExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::r::bogus::bogus_expression::FormatRBogusExpression::default(),
        )
    }
}
impl IntoFormat<RFormatContext> for air_r_syntax::RBogusExpression {
    type Format = FormatOwnedWithRule<
        air_r_syntax::RBogusExpression,
        crate::r::bogus::bogus_expression::FormatRBogusExpression,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::r::bogus::bogus_expression::FormatRBogusExpression::default(),
        )
    }
}
impl FormatRule<air_r_syntax::RBogusValue> for crate::r::bogus::bogus_value::FormatRBogusValue {
    type Context = RFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &air_r_syntax::RBogusValue, f: &mut RFormatter) -> FormatResult<()> {
        FormatBogusNodeRule::<air_r_syntax::RBogusValue>::fmt(self, node, f)
    }
}
impl AsFormat<RFormatContext> for air_r_syntax::RBogusValue {
    type Format<'a> = FormatRefWithRule<
        'a,
        air_r_syntax::RBogusValue,
        crate::r::bogus::bogus_value::FormatRBogusValue,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::r::bogus::bogus_value::FormatRBogusValue::default(),
        )
    }
}
impl IntoFormat<RFormatContext> for air_r_syntax::RBogusValue {
    type Format = FormatOwnedWithRule<
        air_r_syntax::RBogusValue,
        crate::r::bogus::bogus_value::FormatRBogusValue,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::r::bogus::bogus_value::FormatRBogusValue::default(),
        )
    }
}
impl AsFormat<RFormatContext> for air_r_syntax::AnyRArgument {
    type Format<'a> = FormatRefWithRule<
        'a,
        air_r_syntax::AnyRArgument,
        crate::r::any::argument::FormatAnyRArgument,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(self, crate::r::any::argument::FormatAnyRArgument::default())
    }
}
impl IntoFormat<RFormatContext> for air_r_syntax::AnyRArgument {
    type Format = FormatOwnedWithRule<
        air_r_syntax::AnyRArgument,
        crate::r::any::argument::FormatAnyRArgument,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(self, crate::r::any::argument::FormatAnyRArgument::default())
    }
}
impl AsFormat<RFormatContext> for air_r_syntax::AnyRArgumentName {
    type Format<'a> = FormatRefWithRule<
        'a,
        air_r_syntax::AnyRArgumentName,
        crate::r::any::argument_name::FormatAnyRArgumentName,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::r::any::argument_name::FormatAnyRArgumentName::default(),
        )
    }
}
impl IntoFormat<RFormatContext> for air_r_syntax::AnyRArgumentName {
    type Format = FormatOwnedWithRule<
        air_r_syntax::AnyRArgumentName,
        crate::r::any::argument_name::FormatAnyRArgumentName,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::r::any::argument_name::FormatAnyRArgumentName::default(),
        )
    }
}
impl AsFormat<RFormatContext> for air_r_syntax::AnyRExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        air_r_syntax::AnyRExpression,
        crate::r::any::expression::FormatAnyRExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::r::any::expression::FormatAnyRExpression::default(),
        )
    }
}
impl IntoFormat<RFormatContext> for air_r_syntax::AnyRExpression {
    type Format = FormatOwnedWithRule<
        air_r_syntax::AnyRExpression,
        crate::r::any::expression::FormatAnyRExpression,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::r::any::expression::FormatAnyRExpression::default(),
        )
    }
}
impl AsFormat<RFormatContext> for air_r_syntax::AnyRParameterName {
    type Format<'a> = FormatRefWithRule<
        'a,
        air_r_syntax::AnyRParameterName,
        crate::r::any::parameter_name::FormatAnyRParameterName,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::r::any::parameter_name::FormatAnyRParameterName::default(),
        )
    }
}
impl IntoFormat<RFormatContext> for air_r_syntax::AnyRParameterName {
    type Format = FormatOwnedWithRule<
        air_r_syntax::AnyRParameterName,
        crate::r::any::parameter_name::FormatAnyRParameterName,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::r::any::parameter_name::FormatAnyRParameterName::default(),
        )
    }
}
impl AsFormat<RFormatContext> for air_r_syntax::AnyRSelector {
    type Format<'a> = FormatRefWithRule<
        'a,
        air_r_syntax::AnyRSelector,
        crate::r::any::selector::FormatAnyRSelector,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(self, crate::r::any::selector::FormatAnyRSelector::default())
    }
}
impl IntoFormat<RFormatContext> for air_r_syntax::AnyRSelector {
    type Format = FormatOwnedWithRule<
        air_r_syntax::AnyRSelector,
        crate::r::any::selector::FormatAnyRSelector,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(self, crate::r::any::selector::FormatAnyRSelector::default())
    }
}
impl AsFormat<RFormatContext> for air_r_syntax::AnyRValue {
    type Format<'a> =
        FormatRefWithRule<'a, air_r_syntax::AnyRValue, crate::r::any::value::FormatAnyRValue>;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(self, crate::r::any::value::FormatAnyRValue::default())
    }
}
impl IntoFormat<RFormatContext> for air_r_syntax::AnyRValue {
    type Format =
        FormatOwnedWithRule<air_r_syntax::AnyRValue, crate::r::any::value::FormatAnyRValue>;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(self, crate::r::any::value::FormatAnyRValue::default())
    }
}
