//! Generated file, do not edit by hand, see `xtask/codegen`

#![allow(clippy::enum_variant_names)]
#![allow(clippy::match_like_matches_macro)]
use crate::{
    macros::map_syntax_node,
    RLanguage as Language, RSyntaxElement as SyntaxElement,
    RSyntaxElementChildren as SyntaxElementChildren,
    RSyntaxKind::{self as SyntaxKind, *},
    RSyntaxList as SyntaxList, RSyntaxNode as SyntaxNode, RSyntaxToken as SyntaxToken,
};
use biome_rowan::{support, AstNode, RawSyntaxKind, SyntaxKindSet, SyntaxResult};
#[allow(unused)]
use biome_rowan::{
    AstNodeList, AstNodeListIterator, AstNodeSlotMap, AstSeparatedList,
    AstSeparatedListNodesIterator,
};
use serde::ser::SerializeSeq;
use serde::{Serialize, Serializer};
use std::fmt::{Debug, Formatter};
#[doc = r" Sentinel value indicating a missing element in a dynamic node, where"]
#[doc = r" the slots are not statically known."]
#[allow(dead_code)]
pub(crate) const SLOT_MAP_EMPTY_VALUE: u8 = u8::MAX;
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct RBinaryExpression {
    pub(crate) syntax: SyntaxNode,
}
impl RBinaryExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> RBinaryExpressionFields {
        RBinaryExpressionFields {
            left: self.left(),
            operator_token: self.operator_token(),
            right: self.right(),
        }
    }
    pub fn left(&self) -> SyntaxResult<AnyRExpression> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn operator_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn right(&self) -> SyntaxResult<AnyRExpression> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl Serialize for RBinaryExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct RBinaryExpressionFields {
    pub left: SyntaxResult<AnyRExpression>,
    pub operator_token: SyntaxResult<SyntaxToken>,
    pub right: SyntaxResult<AnyRExpression>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct RDefaultParameter {
    pub(crate) syntax: SyntaxNode,
}
impl RDefaultParameter {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> RDefaultParameterFields {
        RDefaultParameterFields {
            name_token: self.name_token(),
            eq_token: self.eq_token(),
            default: self.default(),
        }
    }
    pub fn name_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn eq_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn default(&self) -> Option<AnyRExpression> {
        support::node(&self.syntax, 2usize)
    }
}
impl Serialize for RDefaultParameter {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct RDefaultParameterFields {
    pub name_token: SyntaxResult<SyntaxToken>,
    pub eq_token: SyntaxResult<SyntaxToken>,
    pub default: Option<AnyRExpression>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct RDotsParameter {
    pub(crate) syntax: SyntaxNode,
}
impl RDotsParameter {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> RDotsParameterFields {
        RDotsParameterFields {
            name_token: self.name_token(),
        }
    }
    pub fn name_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for RDotsParameter {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct RDotsParameterFields {
    pub name_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct RDoubleValue {
    pub(crate) syntax: SyntaxNode,
}
impl RDoubleValue {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> RDoubleValueFields {
        RDoubleValueFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for RDoubleValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct RDoubleValueFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct RFunctionDefinition {
    pub(crate) syntax: SyntaxNode,
}
impl RFunctionDefinition {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> RFunctionDefinitionFields {
        RFunctionDefinitionFields {
            function_token: self.function_token(),
            parameters: self.parameters(),
            body: self.body(),
        }
    }
    pub fn function_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn parameters(&self) -> SyntaxResult<RParameters> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn body(&self) -> SyntaxResult<AnyRExpression> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl Serialize for RFunctionDefinition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct RFunctionDefinitionFields {
    pub function_token: SyntaxResult<SyntaxToken>,
    pub parameters: SyntaxResult<RParameters>,
    pub body: SyntaxResult<AnyRExpression>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct RIdentifier {
    pub(crate) syntax: SyntaxNode,
}
impl RIdentifier {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> RIdentifierFields {
        RIdentifierFields {
            name_token: self.name_token(),
        }
    }
    pub fn name_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for RIdentifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct RIdentifierFields {
    pub name_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct RIdentifierParameter {
    pub(crate) syntax: SyntaxNode,
}
impl RIdentifierParameter {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> RIdentifierParameterFields {
        RIdentifierParameterFields {
            name_token: self.name_token(),
        }
    }
    pub fn name_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for RIdentifierParameter {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct RIdentifierParameterFields {
    pub name_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct RIntegerValue {
    pub(crate) syntax: SyntaxNode,
}
impl RIntegerValue {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> RIntegerValueFields {
        RIntegerValueFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for RIntegerValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct RIntegerValueFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct RLogicalValue {
    pub(crate) syntax: SyntaxNode,
}
impl RLogicalValue {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> RLogicalValueFields {
        RLogicalValueFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for RLogicalValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct RLogicalValueFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct RNullValue {
    pub(crate) syntax: SyntaxNode,
}
impl RNullValue {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> RNullValueFields {
        RNullValueFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for RNullValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct RNullValueFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct RParameters {
    pub(crate) syntax: SyntaxNode,
}
impl RParameters {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> RParametersFields {
        RParametersFields {
            l_paren_token: self.l_paren_token(),
            items: self.items(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn items(&self) -> RParameterList {
        support::list(&self.syntax, 1usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for RParameters {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct RParametersFields {
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub items: RParameterList,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct RRoot {
    pub(crate) syntax: SyntaxNode,
}
impl RRoot {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> RRootFields {
        RRootFields {
            bom_token: self.bom_token(),
            expressions: self.expressions(),
            eof_token: self.eof_token(),
        }
    }
    pub fn bom_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 0usize)
    }
    pub fn expressions(&self) -> RExpressionList {
        support::list(&self.syntax, 1usize)
    }
    pub fn eof_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for RRoot {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct RRootFields {
    pub bom_token: Option<SyntaxToken>,
    pub expressions: RExpressionList,
    pub eof_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct RStringValue {
    pub(crate) syntax: SyntaxNode,
}
impl RStringValue {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> RStringValueFields {
        RStringValueFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for RStringValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct RStringValueFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyRExpression {
    AnyRValue(AnyRValue),
    RBinaryExpression(RBinaryExpression),
    RBogusExpression(RBogusExpression),
    RFunctionDefinition(RFunctionDefinition),
    RIdentifier(RIdentifier),
}
impl AnyRExpression {
    pub fn as_any_r_value(&self) -> Option<&AnyRValue> {
        match &self {
            AnyRExpression::AnyRValue(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_r_binary_expression(&self) -> Option<&RBinaryExpression> {
        match &self {
            AnyRExpression::RBinaryExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_r_bogus_expression(&self) -> Option<&RBogusExpression> {
        match &self {
            AnyRExpression::RBogusExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_r_function_definition(&self) -> Option<&RFunctionDefinition> {
        match &self {
            AnyRExpression::RFunctionDefinition(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_r_identifier(&self) -> Option<&RIdentifier> {
        match &self {
            AnyRExpression::RIdentifier(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyRParameter {
    RBogusParameter(RBogusParameter),
    RDefaultParameter(RDefaultParameter),
    RDotsParameter(RDotsParameter),
    RIdentifierParameter(RIdentifierParameter),
}
impl AnyRParameter {
    pub fn as_r_bogus_parameter(&self) -> Option<&RBogusParameter> {
        match &self {
            AnyRParameter::RBogusParameter(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_r_default_parameter(&self) -> Option<&RDefaultParameter> {
        match &self {
            AnyRParameter::RDefaultParameter(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_r_dots_parameter(&self) -> Option<&RDotsParameter> {
        match &self {
            AnyRParameter::RDotsParameter(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_r_identifier_parameter(&self) -> Option<&RIdentifierParameter> {
        match &self {
            AnyRParameter::RIdentifierParameter(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyRValue {
    RBogusValue(RBogusValue),
    RDoubleValue(RDoubleValue),
    RIntegerValue(RIntegerValue),
    RLogicalValue(RLogicalValue),
    RNullValue(RNullValue),
    RStringValue(RStringValue),
}
impl AnyRValue {
    pub fn as_r_bogus_value(&self) -> Option<&RBogusValue> {
        match &self {
            AnyRValue::RBogusValue(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_r_double_value(&self) -> Option<&RDoubleValue> {
        match &self {
            AnyRValue::RDoubleValue(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_r_integer_value(&self) -> Option<&RIntegerValue> {
        match &self {
            AnyRValue::RIntegerValue(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_r_logical_value(&self) -> Option<&RLogicalValue> {
        match &self {
            AnyRValue::RLogicalValue(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_r_null_value(&self) -> Option<&RNullValue> {
        match &self {
            AnyRValue::RNullValue(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_r_string_value(&self) -> Option<&RStringValue> {
        match &self {
            AnyRValue::RStringValue(item) => Some(item),
            _ => None,
        }
    }
}
impl AstNode for RBinaryExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(R_BINARY_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == R_BINARY_EXPRESSION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for RBinaryExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RBinaryExpression")
            .field("left", &support::DebugSyntaxResult(self.left()))
            .field(
                "operator_token",
                &support::DebugSyntaxResult(self.operator_token()),
            )
            .field("right", &support::DebugSyntaxResult(self.right()))
            .finish()
    }
}
impl From<RBinaryExpression> for SyntaxNode {
    fn from(n: RBinaryExpression) -> SyntaxNode {
        n.syntax
    }
}
impl From<RBinaryExpression> for SyntaxElement {
    fn from(n: RBinaryExpression) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for RDefaultParameter {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(R_DEFAULT_PARAMETER as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == R_DEFAULT_PARAMETER
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for RDefaultParameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RDefaultParameter")
            .field("name_token", &support::DebugSyntaxResult(self.name_token()))
            .field("eq_token", &support::DebugSyntaxResult(self.eq_token()))
            .field("default", &support::DebugOptionalElement(self.default()))
            .finish()
    }
}
impl From<RDefaultParameter> for SyntaxNode {
    fn from(n: RDefaultParameter) -> SyntaxNode {
        n.syntax
    }
}
impl From<RDefaultParameter> for SyntaxElement {
    fn from(n: RDefaultParameter) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for RDotsParameter {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(R_DOTS_PARAMETER as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == R_DOTS_PARAMETER
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for RDotsParameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RDotsParameter")
            .field("name_token", &support::DebugSyntaxResult(self.name_token()))
            .finish()
    }
}
impl From<RDotsParameter> for SyntaxNode {
    fn from(n: RDotsParameter) -> SyntaxNode {
        n.syntax
    }
}
impl From<RDotsParameter> for SyntaxElement {
    fn from(n: RDotsParameter) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for RDoubleValue {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(R_DOUBLE_VALUE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == R_DOUBLE_VALUE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for RDoubleValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RDoubleValue")
            .field(
                "value_token",
                &support::DebugSyntaxResult(self.value_token()),
            )
            .finish()
    }
}
impl From<RDoubleValue> for SyntaxNode {
    fn from(n: RDoubleValue) -> SyntaxNode {
        n.syntax
    }
}
impl From<RDoubleValue> for SyntaxElement {
    fn from(n: RDoubleValue) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for RFunctionDefinition {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(R_FUNCTION_DEFINITION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == R_FUNCTION_DEFINITION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for RFunctionDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RFunctionDefinition")
            .field(
                "function_token",
                &support::DebugSyntaxResult(self.function_token()),
            )
            .field("parameters", &support::DebugSyntaxResult(self.parameters()))
            .field("body", &support::DebugSyntaxResult(self.body()))
            .finish()
    }
}
impl From<RFunctionDefinition> for SyntaxNode {
    fn from(n: RFunctionDefinition) -> SyntaxNode {
        n.syntax
    }
}
impl From<RFunctionDefinition> for SyntaxElement {
    fn from(n: RFunctionDefinition) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for RIdentifier {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(R_IDENTIFIER as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == R_IDENTIFIER
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for RIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RIdentifier")
            .field("name_token", &support::DebugSyntaxResult(self.name_token()))
            .finish()
    }
}
impl From<RIdentifier> for SyntaxNode {
    fn from(n: RIdentifier) -> SyntaxNode {
        n.syntax
    }
}
impl From<RIdentifier> for SyntaxElement {
    fn from(n: RIdentifier) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for RIdentifierParameter {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(R_IDENTIFIER_PARAMETER as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == R_IDENTIFIER_PARAMETER
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for RIdentifierParameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RIdentifierParameter")
            .field("name_token", &support::DebugSyntaxResult(self.name_token()))
            .finish()
    }
}
impl From<RIdentifierParameter> for SyntaxNode {
    fn from(n: RIdentifierParameter) -> SyntaxNode {
        n.syntax
    }
}
impl From<RIdentifierParameter> for SyntaxElement {
    fn from(n: RIdentifierParameter) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for RIntegerValue {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(R_INTEGER_VALUE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == R_INTEGER_VALUE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for RIntegerValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RIntegerValue")
            .field(
                "value_token",
                &support::DebugSyntaxResult(self.value_token()),
            )
            .finish()
    }
}
impl From<RIntegerValue> for SyntaxNode {
    fn from(n: RIntegerValue) -> SyntaxNode {
        n.syntax
    }
}
impl From<RIntegerValue> for SyntaxElement {
    fn from(n: RIntegerValue) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for RLogicalValue {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(R_LOGICAL_VALUE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == R_LOGICAL_VALUE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for RLogicalValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RLogicalValue")
            .field(
                "value_token",
                &support::DebugSyntaxResult(self.value_token()),
            )
            .finish()
    }
}
impl From<RLogicalValue> for SyntaxNode {
    fn from(n: RLogicalValue) -> SyntaxNode {
        n.syntax
    }
}
impl From<RLogicalValue> for SyntaxElement {
    fn from(n: RLogicalValue) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for RNullValue {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(R_NULL_VALUE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == R_NULL_VALUE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for RNullValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RNullValue")
            .field(
                "value_token",
                &support::DebugSyntaxResult(self.value_token()),
            )
            .finish()
    }
}
impl From<RNullValue> for SyntaxNode {
    fn from(n: RNullValue) -> SyntaxNode {
        n.syntax
    }
}
impl From<RNullValue> for SyntaxElement {
    fn from(n: RNullValue) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for RParameters {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(R_PARAMETERS as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == R_PARAMETERS
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for RParameters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RParameters")
            .field(
                "l_paren_token",
                &support::DebugSyntaxResult(self.l_paren_token()),
            )
            .field("items", &self.items())
            .field(
                "r_paren_token",
                &support::DebugSyntaxResult(self.r_paren_token()),
            )
            .finish()
    }
}
impl From<RParameters> for SyntaxNode {
    fn from(n: RParameters) -> SyntaxNode {
        n.syntax
    }
}
impl From<RParameters> for SyntaxElement {
    fn from(n: RParameters) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for RRoot {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = SyntaxKindSet::from_raw(RawSyntaxKind(R_ROOT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == R_ROOT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for RRoot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RRoot")
            .field(
                "bom_token",
                &support::DebugOptionalElement(self.bom_token()),
            )
            .field("expressions", &self.expressions())
            .field("eof_token", &support::DebugSyntaxResult(self.eof_token()))
            .finish()
    }
}
impl From<RRoot> for SyntaxNode {
    fn from(n: RRoot) -> SyntaxNode {
        n.syntax
    }
}
impl From<RRoot> for SyntaxElement {
    fn from(n: RRoot) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for RStringValue {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(R_STRING_VALUE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == R_STRING_VALUE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for RStringValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RStringValue")
            .field(
                "value_token",
                &support::DebugSyntaxResult(self.value_token()),
            )
            .finish()
    }
}
impl From<RStringValue> for SyntaxNode {
    fn from(n: RStringValue) -> SyntaxNode {
        n.syntax
    }
}
impl From<RStringValue> for SyntaxElement {
    fn from(n: RStringValue) -> SyntaxElement {
        n.syntax.into()
    }
}
impl From<RBinaryExpression> for AnyRExpression {
    fn from(node: RBinaryExpression) -> AnyRExpression {
        AnyRExpression::RBinaryExpression(node)
    }
}
impl From<RBogusExpression> for AnyRExpression {
    fn from(node: RBogusExpression) -> AnyRExpression {
        AnyRExpression::RBogusExpression(node)
    }
}
impl From<RFunctionDefinition> for AnyRExpression {
    fn from(node: RFunctionDefinition) -> AnyRExpression {
        AnyRExpression::RFunctionDefinition(node)
    }
}
impl From<RIdentifier> for AnyRExpression {
    fn from(node: RIdentifier) -> AnyRExpression {
        AnyRExpression::RIdentifier(node)
    }
}
impl AstNode for AnyRExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = AnyRValue::KIND_SET
        .union(RBinaryExpression::KIND_SET)
        .union(RBogusExpression::KIND_SET)
        .union(RFunctionDefinition::KIND_SET)
        .union(RIdentifier::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            R_BINARY_EXPRESSION | R_BOGUS_EXPRESSION | R_FUNCTION_DEFINITION | R_IDENTIFIER => true,
            k if AnyRValue::can_cast(k) => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            R_BINARY_EXPRESSION => AnyRExpression::RBinaryExpression(RBinaryExpression { syntax }),
            R_BOGUS_EXPRESSION => AnyRExpression::RBogusExpression(RBogusExpression { syntax }),
            R_FUNCTION_DEFINITION => {
                AnyRExpression::RFunctionDefinition(RFunctionDefinition { syntax })
            }
            R_IDENTIFIER => AnyRExpression::RIdentifier(RIdentifier { syntax }),
            _ => {
                if let Some(any_r_value) = AnyRValue::cast(syntax) {
                    return Some(AnyRExpression::AnyRValue(any_r_value));
                }
                return None;
            }
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyRExpression::RBinaryExpression(it) => &it.syntax,
            AnyRExpression::RBogusExpression(it) => &it.syntax,
            AnyRExpression::RFunctionDefinition(it) => &it.syntax,
            AnyRExpression::RIdentifier(it) => &it.syntax,
            AnyRExpression::AnyRValue(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyRExpression::RBinaryExpression(it) => it.syntax,
            AnyRExpression::RBogusExpression(it) => it.syntax,
            AnyRExpression::RFunctionDefinition(it) => it.syntax,
            AnyRExpression::RIdentifier(it) => it.syntax,
            AnyRExpression::AnyRValue(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyRExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyRExpression::AnyRValue(it) => std::fmt::Debug::fmt(it, f),
            AnyRExpression::RBinaryExpression(it) => std::fmt::Debug::fmt(it, f),
            AnyRExpression::RBogusExpression(it) => std::fmt::Debug::fmt(it, f),
            AnyRExpression::RFunctionDefinition(it) => std::fmt::Debug::fmt(it, f),
            AnyRExpression::RIdentifier(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyRExpression> for SyntaxNode {
    fn from(n: AnyRExpression) -> SyntaxNode {
        match n {
            AnyRExpression::AnyRValue(it) => it.into(),
            AnyRExpression::RBinaryExpression(it) => it.into(),
            AnyRExpression::RBogusExpression(it) => it.into(),
            AnyRExpression::RFunctionDefinition(it) => it.into(),
            AnyRExpression::RIdentifier(it) => it.into(),
        }
    }
}
impl From<AnyRExpression> for SyntaxElement {
    fn from(n: AnyRExpression) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<RBogusParameter> for AnyRParameter {
    fn from(node: RBogusParameter) -> AnyRParameter {
        AnyRParameter::RBogusParameter(node)
    }
}
impl From<RDefaultParameter> for AnyRParameter {
    fn from(node: RDefaultParameter) -> AnyRParameter {
        AnyRParameter::RDefaultParameter(node)
    }
}
impl From<RDotsParameter> for AnyRParameter {
    fn from(node: RDotsParameter) -> AnyRParameter {
        AnyRParameter::RDotsParameter(node)
    }
}
impl From<RIdentifierParameter> for AnyRParameter {
    fn from(node: RIdentifierParameter) -> AnyRParameter {
        AnyRParameter::RIdentifierParameter(node)
    }
}
impl AstNode for AnyRParameter {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = RBogusParameter::KIND_SET
        .union(RDefaultParameter::KIND_SET)
        .union(RDotsParameter::KIND_SET)
        .union(RIdentifierParameter::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            R_BOGUS_PARAMETER | R_DEFAULT_PARAMETER | R_DOTS_PARAMETER | R_IDENTIFIER_PARAMETER
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            R_BOGUS_PARAMETER => AnyRParameter::RBogusParameter(RBogusParameter { syntax }),
            R_DEFAULT_PARAMETER => AnyRParameter::RDefaultParameter(RDefaultParameter { syntax }),
            R_DOTS_PARAMETER => AnyRParameter::RDotsParameter(RDotsParameter { syntax }),
            R_IDENTIFIER_PARAMETER => {
                AnyRParameter::RIdentifierParameter(RIdentifierParameter { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyRParameter::RBogusParameter(it) => &it.syntax,
            AnyRParameter::RDefaultParameter(it) => &it.syntax,
            AnyRParameter::RDotsParameter(it) => &it.syntax,
            AnyRParameter::RIdentifierParameter(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyRParameter::RBogusParameter(it) => it.syntax,
            AnyRParameter::RDefaultParameter(it) => it.syntax,
            AnyRParameter::RDotsParameter(it) => it.syntax,
            AnyRParameter::RIdentifierParameter(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyRParameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyRParameter::RBogusParameter(it) => std::fmt::Debug::fmt(it, f),
            AnyRParameter::RDefaultParameter(it) => std::fmt::Debug::fmt(it, f),
            AnyRParameter::RDotsParameter(it) => std::fmt::Debug::fmt(it, f),
            AnyRParameter::RIdentifierParameter(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyRParameter> for SyntaxNode {
    fn from(n: AnyRParameter) -> SyntaxNode {
        match n {
            AnyRParameter::RBogusParameter(it) => it.into(),
            AnyRParameter::RDefaultParameter(it) => it.into(),
            AnyRParameter::RDotsParameter(it) => it.into(),
            AnyRParameter::RIdentifierParameter(it) => it.into(),
        }
    }
}
impl From<AnyRParameter> for SyntaxElement {
    fn from(n: AnyRParameter) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<RBogusValue> for AnyRValue {
    fn from(node: RBogusValue) -> AnyRValue {
        AnyRValue::RBogusValue(node)
    }
}
impl From<RDoubleValue> for AnyRValue {
    fn from(node: RDoubleValue) -> AnyRValue {
        AnyRValue::RDoubleValue(node)
    }
}
impl From<RIntegerValue> for AnyRValue {
    fn from(node: RIntegerValue) -> AnyRValue {
        AnyRValue::RIntegerValue(node)
    }
}
impl From<RLogicalValue> for AnyRValue {
    fn from(node: RLogicalValue) -> AnyRValue {
        AnyRValue::RLogicalValue(node)
    }
}
impl From<RNullValue> for AnyRValue {
    fn from(node: RNullValue) -> AnyRValue {
        AnyRValue::RNullValue(node)
    }
}
impl From<RStringValue> for AnyRValue {
    fn from(node: RStringValue) -> AnyRValue {
        AnyRValue::RStringValue(node)
    }
}
impl AstNode for AnyRValue {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = RBogusValue::KIND_SET
        .union(RDoubleValue::KIND_SET)
        .union(RIntegerValue::KIND_SET)
        .union(RLogicalValue::KIND_SET)
        .union(RNullValue::KIND_SET)
        .union(RStringValue::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            R_BOGUS_VALUE
                | R_DOUBLE_VALUE
                | R_INTEGER_VALUE
                | R_LOGICAL_VALUE
                | R_NULL_VALUE
                | R_STRING_VALUE
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            R_BOGUS_VALUE => AnyRValue::RBogusValue(RBogusValue { syntax }),
            R_DOUBLE_VALUE => AnyRValue::RDoubleValue(RDoubleValue { syntax }),
            R_INTEGER_VALUE => AnyRValue::RIntegerValue(RIntegerValue { syntax }),
            R_LOGICAL_VALUE => AnyRValue::RLogicalValue(RLogicalValue { syntax }),
            R_NULL_VALUE => AnyRValue::RNullValue(RNullValue { syntax }),
            R_STRING_VALUE => AnyRValue::RStringValue(RStringValue { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyRValue::RBogusValue(it) => &it.syntax,
            AnyRValue::RDoubleValue(it) => &it.syntax,
            AnyRValue::RIntegerValue(it) => &it.syntax,
            AnyRValue::RLogicalValue(it) => &it.syntax,
            AnyRValue::RNullValue(it) => &it.syntax,
            AnyRValue::RStringValue(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyRValue::RBogusValue(it) => it.syntax,
            AnyRValue::RDoubleValue(it) => it.syntax,
            AnyRValue::RIntegerValue(it) => it.syntax,
            AnyRValue::RLogicalValue(it) => it.syntax,
            AnyRValue::RNullValue(it) => it.syntax,
            AnyRValue::RStringValue(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyRValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyRValue::RBogusValue(it) => std::fmt::Debug::fmt(it, f),
            AnyRValue::RDoubleValue(it) => std::fmt::Debug::fmt(it, f),
            AnyRValue::RIntegerValue(it) => std::fmt::Debug::fmt(it, f),
            AnyRValue::RLogicalValue(it) => std::fmt::Debug::fmt(it, f),
            AnyRValue::RNullValue(it) => std::fmt::Debug::fmt(it, f),
            AnyRValue::RStringValue(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyRValue> for SyntaxNode {
    fn from(n: AnyRValue) -> SyntaxNode {
        match n {
            AnyRValue::RBogusValue(it) => it.into(),
            AnyRValue::RDoubleValue(it) => it.into(),
            AnyRValue::RIntegerValue(it) => it.into(),
            AnyRValue::RLogicalValue(it) => it.into(),
            AnyRValue::RNullValue(it) => it.into(),
            AnyRValue::RStringValue(it) => it.into(),
        }
    }
}
impl From<AnyRValue> for SyntaxElement {
    fn from(n: AnyRValue) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl std::fmt::Display for AnyRExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyRParameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyRValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for RBinaryExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for RDefaultParameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for RDotsParameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for RDoubleValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for RFunctionDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for RIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for RIdentifierParameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for RIntegerValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for RLogicalValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for RNullValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for RParameters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for RRoot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for RStringValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub struct RBogus {
    syntax: SyntaxNode,
}
impl RBogus {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn items(&self) -> SyntaxElementChildren {
        support::elements(&self.syntax)
    }
}
impl AstNode for RBogus {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(R_BOGUS as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == R_BOGUS
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for RBogus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RBogus")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<RBogus> for SyntaxNode {
    fn from(n: RBogus) -> SyntaxNode {
        n.syntax
    }
}
impl From<RBogus> for SyntaxElement {
    fn from(n: RBogus) -> SyntaxElement {
        n.syntax.into()
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub struct RBogusExpression {
    syntax: SyntaxNode,
}
impl RBogusExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn items(&self) -> SyntaxElementChildren {
        support::elements(&self.syntax)
    }
}
impl AstNode for RBogusExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(R_BOGUS_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == R_BOGUS_EXPRESSION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for RBogusExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RBogusExpression")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<RBogusExpression> for SyntaxNode {
    fn from(n: RBogusExpression) -> SyntaxNode {
        n.syntax
    }
}
impl From<RBogusExpression> for SyntaxElement {
    fn from(n: RBogusExpression) -> SyntaxElement {
        n.syntax.into()
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub struct RBogusParameter {
    syntax: SyntaxNode,
}
impl RBogusParameter {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn items(&self) -> SyntaxElementChildren {
        support::elements(&self.syntax)
    }
}
impl AstNode for RBogusParameter {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(R_BOGUS_PARAMETER as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == R_BOGUS_PARAMETER
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for RBogusParameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RBogusParameter")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<RBogusParameter> for SyntaxNode {
    fn from(n: RBogusParameter) -> SyntaxNode {
        n.syntax
    }
}
impl From<RBogusParameter> for SyntaxElement {
    fn from(n: RBogusParameter) -> SyntaxElement {
        n.syntax.into()
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub struct RBogusValue {
    syntax: SyntaxNode,
}
impl RBogusValue {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn items(&self) -> SyntaxElementChildren {
        support::elements(&self.syntax)
    }
}
impl AstNode for RBogusValue {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(R_BOGUS_VALUE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == R_BOGUS_VALUE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for RBogusValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RBogusValue")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<RBogusValue> for SyntaxNode {
    fn from(n: RBogusValue) -> SyntaxNode {
        n.syntax
    }
}
impl From<RBogusValue> for SyntaxElement {
    fn from(n: RBogusValue) -> SyntaxElement {
        n.syntax.into()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct RExpressionList {
    syntax_list: SyntaxList,
}
impl RExpressionList {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self {
            syntax_list: syntax.into_list(),
        }
    }
}
impl AstNode for RExpressionList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(R_EXPRESSION_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == R_EXPRESSION_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<RExpressionList> {
        if Self::can_cast(syntax.kind()) {
            Some(RExpressionList {
                syntax_list: syntax.into_list(),
            })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        self.syntax_list.node()
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax_list.into_node()
    }
}
impl Serialize for RExpressionList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for e in self.iter() {
            seq.serialize_element(&e)?;
        }
        seq.end()
    }
}
impl AstNodeList for RExpressionList {
    type Language = Language;
    type Node = AnyRExpression;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for RExpressionList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("RExpressionList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &RExpressionList {
    type Item = AnyRExpression;
    type IntoIter = AstNodeListIterator<Language, AnyRExpression>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for RExpressionList {
    type Item = AnyRExpression;
    type IntoIter = AstNodeListIterator<Language, AnyRExpression>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct RParameterList {
    syntax_list: SyntaxList,
}
impl RParameterList {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self {
            syntax_list: syntax.into_list(),
        }
    }
}
impl AstNode for RParameterList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(R_PARAMETER_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == R_PARAMETER_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<RParameterList> {
        if Self::can_cast(syntax.kind()) {
            Some(RParameterList {
                syntax_list: syntax.into_list(),
            })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        self.syntax_list.node()
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax_list.into_node()
    }
}
impl Serialize for RParameterList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for e in self.iter() {
            seq.serialize_element(&e)?;
        }
        seq.end()
    }
}
impl AstSeparatedList for RParameterList {
    type Language = Language;
    type Node = AnyRParameter;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for RParameterList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("RParameterList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for RParameterList {
    type Item = SyntaxResult<AnyRParameter>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyRParameter>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for &RParameterList {
    type Item = SyntaxResult<AnyRParameter>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyRParameter>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone)]
pub struct DebugSyntaxElementChildren(pub SyntaxElementChildren);
impl Debug for DebugSyntaxElementChildren {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_list()
            .entries(self.clone().0.map(DebugSyntaxElement))
            .finish()
    }
}
struct DebugSyntaxElement(SyntaxElement);
impl Debug for DebugSyntaxElement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            SyntaxElement::Node(node) => {
                map_syntax_node ! (node . clone () , node => std :: fmt :: Debug :: fmt (& node , f))
            }
            SyntaxElement::Token(token) => Debug::fmt(token, f),
        }
    }
}
