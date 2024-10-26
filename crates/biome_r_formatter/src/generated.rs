//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::{
    AsFormat, FormatBogusNodeRule, FormatNodeRule, IntoFormat, RFormatContext, RFormatter,
};
use biome_formatter::{FormatOwnedWithRule, FormatRefWithRule, FormatResult, FormatRule};
impl FormatRule<biome_r_syntax::RBinaryExpression>
    for crate::r::auxiliary::binary_expression::FormatRBinaryExpression
{
    type Context = RFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_r_syntax::RBinaryExpression,
        f: &mut RFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_r_syntax::RBinaryExpression>::fmt(self, node, f)
    }
}
impl AsFormat<RFormatContext> for biome_r_syntax::RBinaryExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_r_syntax::RBinaryExpression,
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
impl IntoFormat<RFormatContext> for biome_r_syntax::RBinaryExpression {
    type Format = FormatOwnedWithRule<
        biome_r_syntax::RBinaryExpression,
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
impl FormatRule<biome_r_syntax::RDefaultParameter>
    for crate::r::auxiliary::default_parameter::FormatRDefaultParameter
{
    type Context = RFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_r_syntax::RDefaultParameter,
        f: &mut RFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_r_syntax::RDefaultParameter>::fmt(self, node, f)
    }
}
impl AsFormat<RFormatContext> for biome_r_syntax::RDefaultParameter {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_r_syntax::RDefaultParameter,
        crate::r::auxiliary::default_parameter::FormatRDefaultParameter,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::r::auxiliary::default_parameter::FormatRDefaultParameter::default(),
        )
    }
}
impl IntoFormat<RFormatContext> for biome_r_syntax::RDefaultParameter {
    type Format = FormatOwnedWithRule<
        biome_r_syntax::RDefaultParameter,
        crate::r::auxiliary::default_parameter::FormatRDefaultParameter,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::r::auxiliary::default_parameter::FormatRDefaultParameter::default(),
        )
    }
}
impl FormatRule<biome_r_syntax::RDotsParameter>
    for crate::r::auxiliary::dots_parameter::FormatRDotsParameter
{
    type Context = RFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_r_syntax::RDotsParameter, f: &mut RFormatter) -> FormatResult<()> {
        FormatNodeRule::<biome_r_syntax::RDotsParameter>::fmt(self, node, f)
    }
}
impl AsFormat<RFormatContext> for biome_r_syntax::RDotsParameter {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_r_syntax::RDotsParameter,
        crate::r::auxiliary::dots_parameter::FormatRDotsParameter,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::r::auxiliary::dots_parameter::FormatRDotsParameter::default(),
        )
    }
}
impl IntoFormat<RFormatContext> for biome_r_syntax::RDotsParameter {
    type Format = FormatOwnedWithRule<
        biome_r_syntax::RDotsParameter,
        crate::r::auxiliary::dots_parameter::FormatRDotsParameter,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::r::auxiliary::dots_parameter::FormatRDotsParameter::default(),
        )
    }
}
impl FormatRule<biome_r_syntax::RDoubleValue>
    for crate::r::auxiliary::double_value::FormatRDoubleValue
{
    type Context = RFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_r_syntax::RDoubleValue, f: &mut RFormatter) -> FormatResult<()> {
        FormatNodeRule::<biome_r_syntax::RDoubleValue>::fmt(self, node, f)
    }
}
impl AsFormat<RFormatContext> for biome_r_syntax::RDoubleValue {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_r_syntax::RDoubleValue,
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
impl IntoFormat<RFormatContext> for biome_r_syntax::RDoubleValue {
    type Format = FormatOwnedWithRule<
        biome_r_syntax::RDoubleValue,
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
impl FormatRule<biome_r_syntax::RFunctionDefinition>
    for crate::r::auxiliary::function_definition::FormatRFunctionDefinition
{
    type Context = RFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_r_syntax::RFunctionDefinition,
        f: &mut RFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_r_syntax::RFunctionDefinition>::fmt(self, node, f)
    }
}
impl AsFormat<RFormatContext> for biome_r_syntax::RFunctionDefinition {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_r_syntax::RFunctionDefinition,
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
impl IntoFormat<RFormatContext> for biome_r_syntax::RFunctionDefinition {
    type Format = FormatOwnedWithRule<
        biome_r_syntax::RFunctionDefinition,
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
impl FormatRule<biome_r_syntax::RIdentifier>
    for crate::r::auxiliary::identifier::FormatRIdentifier
{
    type Context = RFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_r_syntax::RIdentifier, f: &mut RFormatter) -> FormatResult<()> {
        FormatNodeRule::<biome_r_syntax::RIdentifier>::fmt(self, node, f)
    }
}
impl AsFormat<RFormatContext> for biome_r_syntax::RIdentifier {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_r_syntax::RIdentifier,
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
impl IntoFormat<RFormatContext> for biome_r_syntax::RIdentifier {
    type Format = FormatOwnedWithRule<
        biome_r_syntax::RIdentifier,
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
impl FormatRule<biome_r_syntax::RIdentifierParameter>
    for crate::r::auxiliary::identifier_parameter::FormatRIdentifierParameter
{
    type Context = RFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_r_syntax::RIdentifierParameter,
        f: &mut RFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_r_syntax::RIdentifierParameter>::fmt(self, node, f)
    }
}
impl AsFormat<RFormatContext> for biome_r_syntax::RIdentifierParameter {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_r_syntax::RIdentifierParameter,
        crate::r::auxiliary::identifier_parameter::FormatRIdentifierParameter,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::r::auxiliary::identifier_parameter::FormatRIdentifierParameter::default(),
        )
    }
}
impl IntoFormat<RFormatContext> for biome_r_syntax::RIdentifierParameter {
    type Format = FormatOwnedWithRule<
        biome_r_syntax::RIdentifierParameter,
        crate::r::auxiliary::identifier_parameter::FormatRIdentifierParameter,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::r::auxiliary::identifier_parameter::FormatRIdentifierParameter::default(),
        )
    }
}
impl FormatRule<biome_r_syntax::RIntegerValue>
    for crate::r::auxiliary::integer_value::FormatRIntegerValue
{
    type Context = RFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_r_syntax::RIntegerValue, f: &mut RFormatter) -> FormatResult<()> {
        FormatNodeRule::<biome_r_syntax::RIntegerValue>::fmt(self, node, f)
    }
}
impl AsFormat<RFormatContext> for biome_r_syntax::RIntegerValue {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_r_syntax::RIntegerValue,
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
impl IntoFormat<RFormatContext> for biome_r_syntax::RIntegerValue {
    type Format = FormatOwnedWithRule<
        biome_r_syntax::RIntegerValue,
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
impl FormatRule<biome_r_syntax::RLogicalValue>
    for crate::r::auxiliary::logical_value::FormatRLogicalValue
{
    type Context = RFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_r_syntax::RLogicalValue, f: &mut RFormatter) -> FormatResult<()> {
        FormatNodeRule::<biome_r_syntax::RLogicalValue>::fmt(self, node, f)
    }
}
impl AsFormat<RFormatContext> for biome_r_syntax::RLogicalValue {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_r_syntax::RLogicalValue,
        crate::r::auxiliary::logical_value::FormatRLogicalValue,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::r::auxiliary::logical_value::FormatRLogicalValue::default(),
        )
    }
}
impl IntoFormat<RFormatContext> for biome_r_syntax::RLogicalValue {
    type Format = FormatOwnedWithRule<
        biome_r_syntax::RLogicalValue,
        crate::r::auxiliary::logical_value::FormatRLogicalValue,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::r::auxiliary::logical_value::FormatRLogicalValue::default(),
        )
    }
}
impl FormatRule<biome_r_syntax::RNullValue> for crate::r::auxiliary::null_value::FormatRNullValue {
    type Context = RFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_r_syntax::RNullValue, f: &mut RFormatter) -> FormatResult<()> {
        FormatNodeRule::<biome_r_syntax::RNullValue>::fmt(self, node, f)
    }
}
impl AsFormat<RFormatContext> for biome_r_syntax::RNullValue {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_r_syntax::RNullValue,
        crate::r::auxiliary::null_value::FormatRNullValue,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::r::auxiliary::null_value::FormatRNullValue::default(),
        )
    }
}
impl IntoFormat<RFormatContext> for biome_r_syntax::RNullValue {
    type Format = FormatOwnedWithRule<
        biome_r_syntax::RNullValue,
        crate::r::auxiliary::null_value::FormatRNullValue,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::r::auxiliary::null_value::FormatRNullValue::default(),
        )
    }
}
impl FormatRule<biome_r_syntax::RParameters>
    for crate::r::auxiliary::parameters::FormatRParameters
{
    type Context = RFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_r_syntax::RParameters, f: &mut RFormatter) -> FormatResult<()> {
        FormatNodeRule::<biome_r_syntax::RParameters>::fmt(self, node, f)
    }
}
impl AsFormat<RFormatContext> for biome_r_syntax::RParameters {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_r_syntax::RParameters,
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
impl IntoFormat<RFormatContext> for biome_r_syntax::RParameters {
    type Format = FormatOwnedWithRule<
        biome_r_syntax::RParameters,
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
impl FormatRule<biome_r_syntax::RRoot> for crate::r::auxiliary::root::FormatRRoot {
    type Context = RFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_r_syntax::RRoot, f: &mut RFormatter) -> FormatResult<()> {
        FormatNodeRule::<biome_r_syntax::RRoot>::fmt(self, node, f)
    }
}
impl AsFormat<RFormatContext> for biome_r_syntax::RRoot {
    type Format<'a> =
        FormatRefWithRule<'a, biome_r_syntax::RRoot, crate::r::auxiliary::root::FormatRRoot>;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(self, crate::r::auxiliary::root::FormatRRoot::default())
    }
}
impl IntoFormat<RFormatContext> for biome_r_syntax::RRoot {
    type Format =
        FormatOwnedWithRule<biome_r_syntax::RRoot, crate::r::auxiliary::root::FormatRRoot>;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(self, crate::r::auxiliary::root::FormatRRoot::default())
    }
}
impl FormatRule<biome_r_syntax::RStringValue>
    for crate::r::auxiliary::string_value::FormatRStringValue
{
    type Context = RFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_r_syntax::RStringValue, f: &mut RFormatter) -> FormatResult<()> {
        FormatNodeRule::<biome_r_syntax::RStringValue>::fmt(self, node, f)
    }
}
impl AsFormat<RFormatContext> for biome_r_syntax::RStringValue {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_r_syntax::RStringValue,
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
impl IntoFormat<RFormatContext> for biome_r_syntax::RStringValue {
    type Format = FormatOwnedWithRule<
        biome_r_syntax::RStringValue,
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
impl AsFormat<RFormatContext> for biome_r_syntax::RExpressionList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_r_syntax::RExpressionList,
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
impl IntoFormat<RFormatContext> for biome_r_syntax::RExpressionList {
    type Format = FormatOwnedWithRule<
        biome_r_syntax::RExpressionList,
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
impl AsFormat<RFormatContext> for biome_r_syntax::RParameterList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_r_syntax::RParameterList,
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
impl IntoFormat<RFormatContext> for biome_r_syntax::RParameterList {
    type Format = FormatOwnedWithRule<
        biome_r_syntax::RParameterList,
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
impl FormatRule<biome_r_syntax::RBogus> for crate::r::bogus::bogus::FormatRBogus {
    type Context = RFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_r_syntax::RBogus, f: &mut RFormatter) -> FormatResult<()> {
        FormatBogusNodeRule::<biome_r_syntax::RBogus>::fmt(self, node, f)
    }
}
impl AsFormat<RFormatContext> for biome_r_syntax::RBogus {
    type Format<'a> =
        FormatRefWithRule<'a, biome_r_syntax::RBogus, crate::r::bogus::bogus::FormatRBogus>;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(self, crate::r::bogus::bogus::FormatRBogus::default())
    }
}
impl IntoFormat<RFormatContext> for biome_r_syntax::RBogus {
    type Format = FormatOwnedWithRule<biome_r_syntax::RBogus, crate::r::bogus::bogus::FormatRBogus>;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(self, crate::r::bogus::bogus::FormatRBogus::default())
    }
}
impl FormatRule<biome_r_syntax::RBogusExpression>
    for crate::r::bogus::bogus_expression::FormatRBogusExpression
{
    type Context = RFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_r_syntax::RBogusExpression, f: &mut RFormatter) -> FormatResult<()> {
        FormatBogusNodeRule::<biome_r_syntax::RBogusExpression>::fmt(self, node, f)
    }
}
impl AsFormat<RFormatContext> for biome_r_syntax::RBogusExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_r_syntax::RBogusExpression,
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
impl IntoFormat<RFormatContext> for biome_r_syntax::RBogusExpression {
    type Format = FormatOwnedWithRule<
        biome_r_syntax::RBogusExpression,
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
impl FormatRule<biome_r_syntax::RBogusParameter>
    for crate::r::bogus::bogus_parameter::FormatRBogusParameter
{
    type Context = RFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_r_syntax::RBogusParameter, f: &mut RFormatter) -> FormatResult<()> {
        FormatBogusNodeRule::<biome_r_syntax::RBogusParameter>::fmt(self, node, f)
    }
}
impl AsFormat<RFormatContext> for biome_r_syntax::RBogusParameter {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_r_syntax::RBogusParameter,
        crate::r::bogus::bogus_parameter::FormatRBogusParameter,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::r::bogus::bogus_parameter::FormatRBogusParameter::default(),
        )
    }
}
impl IntoFormat<RFormatContext> for biome_r_syntax::RBogusParameter {
    type Format = FormatOwnedWithRule<
        biome_r_syntax::RBogusParameter,
        crate::r::bogus::bogus_parameter::FormatRBogusParameter,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::r::bogus::bogus_parameter::FormatRBogusParameter::default(),
        )
    }
}
impl FormatRule<biome_r_syntax::RBogusValue> for crate::r::bogus::bogus_value::FormatRBogusValue {
    type Context = RFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_r_syntax::RBogusValue, f: &mut RFormatter) -> FormatResult<()> {
        FormatBogusNodeRule::<biome_r_syntax::RBogusValue>::fmt(self, node, f)
    }
}
impl AsFormat<RFormatContext> for biome_r_syntax::RBogusValue {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_r_syntax::RBogusValue,
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
impl IntoFormat<RFormatContext> for biome_r_syntax::RBogusValue {
    type Format = FormatOwnedWithRule<
        biome_r_syntax::RBogusValue,
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
impl AsFormat<RFormatContext> for biome_r_syntax::AnyRExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_r_syntax::AnyRExpression,
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
impl IntoFormat<RFormatContext> for biome_r_syntax::AnyRExpression {
    type Format = FormatOwnedWithRule<
        biome_r_syntax::AnyRExpression,
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
impl AsFormat<RFormatContext> for biome_r_syntax::AnyRParameter {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_r_syntax::AnyRParameter,
        crate::r::any::parameter::FormatAnyRParameter,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::r::any::parameter::FormatAnyRParameter::default(),
        )
    }
}
impl IntoFormat<RFormatContext> for biome_r_syntax::AnyRParameter {
    type Format = FormatOwnedWithRule<
        biome_r_syntax::AnyRParameter,
        crate::r::any::parameter::FormatAnyRParameter,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::r::any::parameter::FormatAnyRParameter::default(),
        )
    }
}
impl AsFormat<RFormatContext> for biome_r_syntax::AnyRValue {
    type Format<'a> =
        FormatRefWithRule<'a, biome_r_syntax::AnyRValue, crate::r::any::value::FormatAnyRValue>;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(self, crate::r::any::value::FormatAnyRValue::default())
    }
}
impl IntoFormat<RFormatContext> for biome_r_syntax::AnyRValue {
    type Format =
        FormatOwnedWithRule<biome_r_syntax::AnyRValue, crate::r::any::value::FormatAnyRValue>;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(self, crate::r::any::value::FormatAnyRValue::default())
    }
}
