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
            operator: self.operator(),
            right: self.right(),
        }
    }
    pub fn left(&self) -> SyntaxResult<AnyRExpression> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn operator(&self) -> SyntaxResult<SyntaxToken> {
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
    pub operator: SyntaxResult<SyntaxToken>,
    pub right: SyntaxResult<AnyRExpression>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct RBracedExpressions {
    pub(crate) syntax: SyntaxNode,
}
impl RBracedExpressions {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> RBracedExpressionsFields {
        RBracedExpressionsFields {
            l_curly_token: self.l_curly_token(),
            expressions: self.expressions(),
            r_curly_token: self.r_curly_token(),
        }
    }
    pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn expressions(&self) -> RExpressionList {
        support::list(&self.syntax, 1usize)
    }
    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for RBracedExpressions {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct RBracedExpressionsFields {
    pub l_curly_token: SyntaxResult<SyntaxToken>,
    pub expressions: RExpressionList,
    pub r_curly_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct RBreakExpression {
    pub(crate) syntax: SyntaxNode,
}
impl RBreakExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> RBreakExpressionFields {
        RBreakExpressionFields {
            break_token: self.break_token(),
        }
    }
    pub fn break_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for RBreakExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct RBreakExpressionFields {
    pub break_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct RCall {
    pub(crate) syntax: SyntaxNode,
}
impl RCall {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> RCallFields {
        RCallFields {
            function: self.function(),
            arguments: self.arguments(),
        }
    }
    pub fn function(&self) -> SyntaxResult<AnyRExpression> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn arguments(&self) -> SyntaxResult<RCallArguments> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for RCall {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct RCallFields {
    pub function: SyntaxResult<AnyRExpression>,
    pub arguments: SyntaxResult<RCallArguments>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct RCallArguments {
    pub(crate) syntax: SyntaxNode,
}
impl RCallArguments {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> RCallArgumentsFields {
        RCallArgumentsFields {
            l_paren_token: self.l_paren_token(),
            items: self.items(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn items(&self) -> RArgumentList {
        support::list(&self.syntax, 1usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for RCallArguments {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct RCallArgumentsFields {
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub items: RArgumentList,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct RComplexValue {
    pub(crate) syntax: SyntaxNode,
}
impl RComplexValue {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> RComplexValueFields {
        RComplexValueFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for RComplexValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct RComplexValueFields {
    pub value_token: SyntaxResult<SyntaxToken>,
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
pub struct RDotDotI {
    pub(crate) syntax: SyntaxNode,
}
impl RDotDotI {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> RDotDotIFields {
        RDotDotIFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for RDotDotI {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct RDotDotIFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct RDots {
    pub(crate) syntax: SyntaxNode,
}
impl RDots {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> RDotsFields {
        RDotsFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for RDots {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct RDotsFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct RDotsArgument {
    pub(crate) syntax: SyntaxNode,
}
impl RDotsArgument {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> RDotsArgumentFields {
        RDotsArgumentFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for RDotsArgument {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct RDotsArgumentFields {
    pub value_token: SyntaxResult<SyntaxToken>,
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
pub struct RElseClause {
    pub(crate) syntax: SyntaxNode,
}
impl RElseClause {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> RElseClauseFields {
        RElseClauseFields {
            else_token: self.else_token(),
            alternative: self.alternative(),
        }
    }
    pub fn else_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn alternative(&self) -> SyntaxResult<AnyRExpression> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for RElseClause {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct RElseClauseFields {
    pub else_token: SyntaxResult<SyntaxToken>,
    pub alternative: SyntaxResult<AnyRExpression>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct RFalseExpression {
    pub(crate) syntax: SyntaxNode,
}
impl RFalseExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> RFalseExpressionFields {
        RFalseExpressionFields {
            false_token: self.false_token(),
        }
    }
    pub fn false_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for RFalseExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct RFalseExpressionFields {
    pub false_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct RForStatement {
    pub(crate) syntax: SyntaxNode,
}
impl RForStatement {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> RForStatementFields {
        RForStatementFields {
            for_token: self.for_token(),
            l_paren_token: self.l_paren_token(),
            variable: self.variable(),
            in_token: self.in_token(),
            sequence: self.sequence(),
            r_paren_token: self.r_paren_token(),
            body: self.body(),
        }
    }
    pub fn for_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn variable(&self) -> SyntaxResult<RIdentifier> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn in_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
    pub fn sequence(&self) -> SyntaxResult<AnyRExpression> {
        support::required_node(&self.syntax, 4usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 5usize)
    }
    pub fn body(&self) -> SyntaxResult<AnyRExpression> {
        support::required_node(&self.syntax, 6usize)
    }
}
impl Serialize for RForStatement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct RForStatementFields {
    pub for_token: SyntaxResult<SyntaxToken>,
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub variable: SyntaxResult<RIdentifier>,
    pub in_token: SyntaxResult<SyntaxToken>,
    pub sequence: SyntaxResult<AnyRExpression>,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
    pub body: SyntaxResult<AnyRExpression>,
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
            name: self.name(),
            parameters: self.parameters(),
            body: self.body(),
        }
    }
    pub fn name(&self) -> SyntaxResult<SyntaxToken> {
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
    pub name: SyntaxResult<SyntaxToken>,
    pub parameters: SyntaxResult<RParameters>,
    pub body: SyntaxResult<AnyRExpression>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct RHoleArgument {
    pub(crate) syntax: SyntaxNode,
}
impl RHoleArgument {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> RHoleArgumentFields {
        RHoleArgumentFields {}
    }
}
impl Serialize for RHoleArgument {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct RHoleArgumentFields {}
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
pub struct RIfStatement {
    pub(crate) syntax: SyntaxNode,
}
impl RIfStatement {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> RIfStatementFields {
        RIfStatementFields {
            if_token: self.if_token(),
            l_paren_token: self.l_paren_token(),
            condition: self.condition(),
            r_paren_token: self.r_paren_token(),
            consequence: self.consequence(),
            else_clause: self.else_clause(),
        }
    }
    pub fn if_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn condition(&self) -> SyntaxResult<AnyRExpression> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
    pub fn consequence(&self) -> SyntaxResult<AnyRExpression> {
        support::required_node(&self.syntax, 4usize)
    }
    pub fn else_clause(&self) -> Option<RElseClause> {
        support::node(&self.syntax, 5usize)
    }
}
impl Serialize for RIfStatement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct RIfStatementFields {
    pub if_token: SyntaxResult<SyntaxToken>,
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub condition: SyntaxResult<AnyRExpression>,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
    pub consequence: SyntaxResult<AnyRExpression>,
    pub else_clause: Option<RElseClause>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct RInfExpression {
    pub(crate) syntax: SyntaxNode,
}
impl RInfExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> RInfExpressionFields {
        RInfExpressionFields {
            inf_token: self.inf_token(),
        }
    }
    pub fn inf_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for RInfExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct RInfExpressionFields {
    pub inf_token: SyntaxResult<SyntaxToken>,
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
pub struct RNaExpression {
    pub(crate) syntax: SyntaxNode,
}
impl RNaExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> RNaExpressionFields {
        RNaExpressionFields {
            value: self.value(),
        }
    }
    pub fn value(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for RNaExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct RNaExpressionFields {
    pub value: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct RNamedArgument {
    pub(crate) syntax: SyntaxNode,
}
impl RNamedArgument {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> RNamedArgumentFields {
        RNamedArgumentFields {
            name: self.name(),
            eq_token: self.eq_token(),
            value: self.value(),
        }
    }
    pub fn name(&self) -> SyntaxResult<AnyRArgumentName> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn eq_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn value(&self) -> Option<AnyRExpression> {
        support::node(&self.syntax, 2usize)
    }
}
impl Serialize for RNamedArgument {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct RNamedArgumentFields {
    pub name: SyntaxResult<AnyRArgumentName>,
    pub eq_token: SyntaxResult<SyntaxToken>,
    pub value: Option<AnyRExpression>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct RNanExpression {
    pub(crate) syntax: SyntaxNode,
}
impl RNanExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> RNanExpressionFields {
        RNanExpressionFields {
            nan_token: self.nan_token(),
        }
    }
    pub fn nan_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for RNanExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct RNanExpressionFields {
    pub nan_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct RNextExpression {
    pub(crate) syntax: SyntaxNode,
}
impl RNextExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> RNextExpressionFields {
        RNextExpressionFields {
            next_token: self.next_token(),
        }
    }
    pub fn next_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for RNextExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct RNextExpressionFields {
    pub next_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct RNullExpression {
    pub(crate) syntax: SyntaxNode,
}
impl RNullExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> RNullExpressionFields {
        RNullExpressionFields {
            null_token: self.null_token(),
        }
    }
    pub fn null_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for RNullExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct RNullExpressionFields {
    pub null_token: SyntaxResult<SyntaxToken>,
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
pub struct RParenthesizedExpression {
    pub(crate) syntax: SyntaxNode,
}
impl RParenthesizedExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> RParenthesizedExpressionFields {
        RParenthesizedExpressionFields {
            l_paren_token: self.l_paren_token(),
            body: self.body(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn body(&self) -> SyntaxResult<AnyRExpression> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for RParenthesizedExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct RParenthesizedExpressionFields {
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub body: SyntaxResult<AnyRExpression>,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct RRepeatStatement {
    pub(crate) syntax: SyntaxNode,
}
impl RRepeatStatement {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> RRepeatStatementFields {
        RRepeatStatementFields {
            repeat_token: self.repeat_token(),
            body: self.body(),
        }
    }
    pub fn repeat_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn body(&self) -> SyntaxResult<AnyRExpression> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for RRepeatStatement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct RRepeatStatementFields {
    pub repeat_token: SyntaxResult<SyntaxToken>,
    pub body: SyntaxResult<AnyRExpression>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct RReturnExpression {
    pub(crate) syntax: SyntaxNode,
}
impl RReturnExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> RReturnExpressionFields {
        RReturnExpressionFields {
            return_token: self.return_token(),
        }
    }
    pub fn return_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for RReturnExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct RReturnExpressionFields {
    pub return_token: SyntaxResult<SyntaxToken>,
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
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct RTrueExpression {
    pub(crate) syntax: SyntaxNode,
}
impl RTrueExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> RTrueExpressionFields {
        RTrueExpressionFields {
            true_token: self.true_token(),
        }
    }
    pub fn true_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for RTrueExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct RTrueExpressionFields {
    pub true_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct RUnaryExpression {
    pub(crate) syntax: SyntaxNode,
}
impl RUnaryExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> RUnaryExpressionFields {
        RUnaryExpressionFields {
            operator: self.operator(),
            argument: self.argument(),
        }
    }
    pub fn operator(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn argument(&self) -> SyntaxResult<AnyRExpression> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for RUnaryExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct RUnaryExpressionFields {
    pub operator: SyntaxResult<SyntaxToken>,
    pub argument: SyntaxResult<AnyRExpression>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct RUnnamedArgument {
    pub(crate) syntax: SyntaxNode,
}
impl RUnnamedArgument {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> RUnnamedArgumentFields {
        RUnnamedArgumentFields {
            value: self.value(),
        }
    }
    pub fn value(&self) -> SyntaxResult<AnyRExpression> {
        support::required_node(&self.syntax, 0usize)
    }
}
impl Serialize for RUnnamedArgument {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct RUnnamedArgumentFields {
    pub value: SyntaxResult<AnyRExpression>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct RWhileStatement {
    pub(crate) syntax: SyntaxNode,
}
impl RWhileStatement {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> RWhileStatementFields {
        RWhileStatementFields {
            while_token: self.while_token(),
            l_paren_token: self.l_paren_token(),
            condition: self.condition(),
            r_paren_token: self.r_paren_token(),
            body: self.body(),
        }
    }
    pub fn while_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn condition(&self) -> SyntaxResult<AnyRExpression> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
    pub fn body(&self) -> SyntaxResult<AnyRExpression> {
        support::required_node(&self.syntax, 4usize)
    }
}
impl Serialize for RWhileStatement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct RWhileStatementFields {
    pub while_token: SyntaxResult<SyntaxToken>,
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub condition: SyntaxResult<AnyRExpression>,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
    pub body: SyntaxResult<AnyRExpression>,
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyRArgument {
    RBogusArgument(RBogusArgument),
    RDotsArgument(RDotsArgument),
    RHoleArgument(RHoleArgument),
    RNamedArgument(RNamedArgument),
    RUnnamedArgument(RUnnamedArgument),
}
impl AnyRArgument {
    pub fn as_r_bogus_argument(&self) -> Option<&RBogusArgument> {
        match &self {
            AnyRArgument::RBogusArgument(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_r_dots_argument(&self) -> Option<&RDotsArgument> {
        match &self {
            AnyRArgument::RDotsArgument(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_r_hole_argument(&self) -> Option<&RHoleArgument> {
        match &self {
            AnyRArgument::RHoleArgument(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_r_named_argument(&self) -> Option<&RNamedArgument> {
        match &self {
            AnyRArgument::RNamedArgument(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_r_unnamed_argument(&self) -> Option<&RUnnamedArgument> {
        match &self {
            AnyRArgument::RUnnamedArgument(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyRArgumentName {
    RDots(RDots),
    RIdentifier(RIdentifier),
    RStringValue(RStringValue),
}
impl AnyRArgumentName {
    pub fn as_r_dots(&self) -> Option<&RDots> {
        match &self {
            AnyRArgumentName::RDots(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_r_identifier(&self) -> Option<&RIdentifier> {
        match &self {
            AnyRArgumentName::RIdentifier(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_r_string_value(&self) -> Option<&RStringValue> {
        match &self {
            AnyRArgumentName::RStringValue(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyRExpression {
    AnyRValue(AnyRValue),
    RBinaryExpression(RBinaryExpression),
    RBogusExpression(RBogusExpression),
    RBracedExpressions(RBracedExpressions),
    RBreakExpression(RBreakExpression),
    RCall(RCall),
    RDotDotI(RDotDotI),
    RFalseExpression(RFalseExpression),
    RForStatement(RForStatement),
    RFunctionDefinition(RFunctionDefinition),
    RIdentifier(RIdentifier),
    RIfStatement(RIfStatement),
    RInfExpression(RInfExpression),
    RNaExpression(RNaExpression),
    RNanExpression(RNanExpression),
    RNextExpression(RNextExpression),
    RNullExpression(RNullExpression),
    RParenthesizedExpression(RParenthesizedExpression),
    RRepeatStatement(RRepeatStatement),
    RReturnExpression(RReturnExpression),
    RTrueExpression(RTrueExpression),
    RUnaryExpression(RUnaryExpression),
    RWhileStatement(RWhileStatement),
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
    pub fn as_r_braced_expressions(&self) -> Option<&RBracedExpressions> {
        match &self {
            AnyRExpression::RBracedExpressions(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_r_break_expression(&self) -> Option<&RBreakExpression> {
        match &self {
            AnyRExpression::RBreakExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_r_call(&self) -> Option<&RCall> {
        match &self {
            AnyRExpression::RCall(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_r_dot_dot_i(&self) -> Option<&RDotDotI> {
        match &self {
            AnyRExpression::RDotDotI(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_r_false_expression(&self) -> Option<&RFalseExpression> {
        match &self {
            AnyRExpression::RFalseExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_r_for_statement(&self) -> Option<&RForStatement> {
        match &self {
            AnyRExpression::RForStatement(item) => Some(item),
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
    pub fn as_r_if_statement(&self) -> Option<&RIfStatement> {
        match &self {
            AnyRExpression::RIfStatement(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_r_inf_expression(&self) -> Option<&RInfExpression> {
        match &self {
            AnyRExpression::RInfExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_r_na_expression(&self) -> Option<&RNaExpression> {
        match &self {
            AnyRExpression::RNaExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_r_nan_expression(&self) -> Option<&RNanExpression> {
        match &self {
            AnyRExpression::RNanExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_r_next_expression(&self) -> Option<&RNextExpression> {
        match &self {
            AnyRExpression::RNextExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_r_null_expression(&self) -> Option<&RNullExpression> {
        match &self {
            AnyRExpression::RNullExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_r_parenthesized_expression(&self) -> Option<&RParenthesizedExpression> {
        match &self {
            AnyRExpression::RParenthesizedExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_r_repeat_statement(&self) -> Option<&RRepeatStatement> {
        match &self {
            AnyRExpression::RRepeatStatement(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_r_return_expression(&self) -> Option<&RReturnExpression> {
        match &self {
            AnyRExpression::RReturnExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_r_true_expression(&self) -> Option<&RTrueExpression> {
        match &self {
            AnyRExpression::RTrueExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_r_unary_expression(&self) -> Option<&RUnaryExpression> {
        match &self {
            AnyRExpression::RUnaryExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_r_while_statement(&self) -> Option<&RWhileStatement> {
        match &self {
            AnyRExpression::RWhileStatement(item) => Some(item),
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
    RComplexValue(RComplexValue),
    RDoubleValue(RDoubleValue),
    RIntegerValue(RIntegerValue),
    RStringValue(RStringValue),
}
impl AnyRValue {
    pub fn as_r_bogus_value(&self) -> Option<&RBogusValue> {
        match &self {
            AnyRValue::RBogusValue(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_r_complex_value(&self) -> Option<&RComplexValue> {
        match &self {
            AnyRValue::RComplexValue(item) => Some(item),
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
            .field("operator", &support::DebugSyntaxResult(self.operator()))
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
impl AstNode for RBracedExpressions {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(R_BRACED_EXPRESSIONS as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == R_BRACED_EXPRESSIONS
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
impl std::fmt::Debug for RBracedExpressions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RBracedExpressions")
            .field(
                "l_curly_token",
                &support::DebugSyntaxResult(self.l_curly_token()),
            )
            .field("expressions", &self.expressions())
            .field(
                "r_curly_token",
                &support::DebugSyntaxResult(self.r_curly_token()),
            )
            .finish()
    }
}
impl From<RBracedExpressions> for SyntaxNode {
    fn from(n: RBracedExpressions) -> SyntaxNode {
        n.syntax
    }
}
impl From<RBracedExpressions> for SyntaxElement {
    fn from(n: RBracedExpressions) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for RBreakExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(R_BREAK_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == R_BREAK_EXPRESSION
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
impl std::fmt::Debug for RBreakExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RBreakExpression")
            .field(
                "break_token",
                &support::DebugSyntaxResult(self.break_token()),
            )
            .finish()
    }
}
impl From<RBreakExpression> for SyntaxNode {
    fn from(n: RBreakExpression) -> SyntaxNode {
        n.syntax
    }
}
impl From<RBreakExpression> for SyntaxElement {
    fn from(n: RBreakExpression) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for RCall {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = SyntaxKindSet::from_raw(RawSyntaxKind(R_CALL as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == R_CALL
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
impl std::fmt::Debug for RCall {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RCall")
            .field("function", &support::DebugSyntaxResult(self.function()))
            .field("arguments", &support::DebugSyntaxResult(self.arguments()))
            .finish()
    }
}
impl From<RCall> for SyntaxNode {
    fn from(n: RCall) -> SyntaxNode {
        n.syntax
    }
}
impl From<RCall> for SyntaxElement {
    fn from(n: RCall) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for RCallArguments {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(R_CALL_ARGUMENTS as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == R_CALL_ARGUMENTS
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
impl std::fmt::Debug for RCallArguments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RCallArguments")
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
impl From<RCallArguments> for SyntaxNode {
    fn from(n: RCallArguments) -> SyntaxNode {
        n.syntax
    }
}
impl From<RCallArguments> for SyntaxElement {
    fn from(n: RCallArguments) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for RComplexValue {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(R_COMPLEX_VALUE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == R_COMPLEX_VALUE
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
impl std::fmt::Debug for RComplexValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RComplexValue")
            .field(
                "value_token",
                &support::DebugSyntaxResult(self.value_token()),
            )
            .finish()
    }
}
impl From<RComplexValue> for SyntaxNode {
    fn from(n: RComplexValue) -> SyntaxNode {
        n.syntax
    }
}
impl From<RComplexValue> for SyntaxElement {
    fn from(n: RComplexValue) -> SyntaxElement {
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
impl AstNode for RDotDotI {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(R_DOT_DOT_I as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == R_DOT_DOT_I
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
impl std::fmt::Debug for RDotDotI {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RDotDotI")
            .field(
                "value_token",
                &support::DebugSyntaxResult(self.value_token()),
            )
            .finish()
    }
}
impl From<RDotDotI> for SyntaxNode {
    fn from(n: RDotDotI) -> SyntaxNode {
        n.syntax
    }
}
impl From<RDotDotI> for SyntaxElement {
    fn from(n: RDotDotI) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for RDots {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = SyntaxKindSet::from_raw(RawSyntaxKind(R_DOTS as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == R_DOTS
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
impl std::fmt::Debug for RDots {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RDots")
            .field(
                "value_token",
                &support::DebugSyntaxResult(self.value_token()),
            )
            .finish()
    }
}
impl From<RDots> for SyntaxNode {
    fn from(n: RDots) -> SyntaxNode {
        n.syntax
    }
}
impl From<RDots> for SyntaxElement {
    fn from(n: RDots) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for RDotsArgument {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(R_DOTS_ARGUMENT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == R_DOTS_ARGUMENT
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
impl std::fmt::Debug for RDotsArgument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RDotsArgument")
            .field(
                "value_token",
                &support::DebugSyntaxResult(self.value_token()),
            )
            .finish()
    }
}
impl From<RDotsArgument> for SyntaxNode {
    fn from(n: RDotsArgument) -> SyntaxNode {
        n.syntax
    }
}
impl From<RDotsArgument> for SyntaxElement {
    fn from(n: RDotsArgument) -> SyntaxElement {
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
impl AstNode for RElseClause {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(R_ELSE_CLAUSE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == R_ELSE_CLAUSE
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
impl std::fmt::Debug for RElseClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RElseClause")
            .field("else_token", &support::DebugSyntaxResult(self.else_token()))
            .field(
                "alternative",
                &support::DebugSyntaxResult(self.alternative()),
            )
            .finish()
    }
}
impl From<RElseClause> for SyntaxNode {
    fn from(n: RElseClause) -> SyntaxNode {
        n.syntax
    }
}
impl From<RElseClause> for SyntaxElement {
    fn from(n: RElseClause) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for RFalseExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(R_FALSE_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == R_FALSE_EXPRESSION
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
impl std::fmt::Debug for RFalseExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RFalseExpression")
            .field(
                "false_token",
                &support::DebugSyntaxResult(self.false_token()),
            )
            .finish()
    }
}
impl From<RFalseExpression> for SyntaxNode {
    fn from(n: RFalseExpression) -> SyntaxNode {
        n.syntax
    }
}
impl From<RFalseExpression> for SyntaxElement {
    fn from(n: RFalseExpression) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for RForStatement {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(R_FOR_STATEMENT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == R_FOR_STATEMENT
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
impl std::fmt::Debug for RForStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RForStatement")
            .field("for_token", &support::DebugSyntaxResult(self.for_token()))
            .field(
                "l_paren_token",
                &support::DebugSyntaxResult(self.l_paren_token()),
            )
            .field("variable", &support::DebugSyntaxResult(self.variable()))
            .field("in_token", &support::DebugSyntaxResult(self.in_token()))
            .field("sequence", &support::DebugSyntaxResult(self.sequence()))
            .field(
                "r_paren_token",
                &support::DebugSyntaxResult(self.r_paren_token()),
            )
            .field("body", &support::DebugSyntaxResult(self.body()))
            .finish()
    }
}
impl From<RForStatement> for SyntaxNode {
    fn from(n: RForStatement) -> SyntaxNode {
        n.syntax
    }
}
impl From<RForStatement> for SyntaxElement {
    fn from(n: RForStatement) -> SyntaxElement {
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
            .field("name", &support::DebugSyntaxResult(self.name()))
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
impl AstNode for RHoleArgument {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(R_HOLE_ARGUMENT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == R_HOLE_ARGUMENT
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
impl std::fmt::Debug for RHoleArgument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RHoleArgument").finish()
    }
}
impl From<RHoleArgument> for SyntaxNode {
    fn from(n: RHoleArgument) -> SyntaxNode {
        n.syntax
    }
}
impl From<RHoleArgument> for SyntaxElement {
    fn from(n: RHoleArgument) -> SyntaxElement {
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
impl AstNode for RIfStatement {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(R_IF_STATEMENT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == R_IF_STATEMENT
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
impl std::fmt::Debug for RIfStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RIfStatement")
            .field("if_token", &support::DebugSyntaxResult(self.if_token()))
            .field(
                "l_paren_token",
                &support::DebugSyntaxResult(self.l_paren_token()),
            )
            .field("condition", &support::DebugSyntaxResult(self.condition()))
            .field(
                "r_paren_token",
                &support::DebugSyntaxResult(self.r_paren_token()),
            )
            .field(
                "consequence",
                &support::DebugSyntaxResult(self.consequence()),
            )
            .field(
                "else_clause",
                &support::DebugOptionalElement(self.else_clause()),
            )
            .finish()
    }
}
impl From<RIfStatement> for SyntaxNode {
    fn from(n: RIfStatement) -> SyntaxNode {
        n.syntax
    }
}
impl From<RIfStatement> for SyntaxElement {
    fn from(n: RIfStatement) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for RInfExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(R_INF_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == R_INF_EXPRESSION
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
impl std::fmt::Debug for RInfExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RInfExpression")
            .field("inf_token", &support::DebugSyntaxResult(self.inf_token()))
            .finish()
    }
}
impl From<RInfExpression> for SyntaxNode {
    fn from(n: RInfExpression) -> SyntaxNode {
        n.syntax
    }
}
impl From<RInfExpression> for SyntaxElement {
    fn from(n: RInfExpression) -> SyntaxElement {
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
impl AstNode for RNaExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(R_NA_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == R_NA_EXPRESSION
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
impl std::fmt::Debug for RNaExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RNaExpression")
            .field("value", &support::DebugSyntaxResult(self.value()))
            .finish()
    }
}
impl From<RNaExpression> for SyntaxNode {
    fn from(n: RNaExpression) -> SyntaxNode {
        n.syntax
    }
}
impl From<RNaExpression> for SyntaxElement {
    fn from(n: RNaExpression) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for RNamedArgument {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(R_NAMED_ARGUMENT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == R_NAMED_ARGUMENT
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
impl std::fmt::Debug for RNamedArgument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RNamedArgument")
            .field("name", &support::DebugSyntaxResult(self.name()))
            .field("eq_token", &support::DebugSyntaxResult(self.eq_token()))
            .field("value", &support::DebugOptionalElement(self.value()))
            .finish()
    }
}
impl From<RNamedArgument> for SyntaxNode {
    fn from(n: RNamedArgument) -> SyntaxNode {
        n.syntax
    }
}
impl From<RNamedArgument> for SyntaxElement {
    fn from(n: RNamedArgument) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for RNanExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(R_NAN_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == R_NAN_EXPRESSION
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
impl std::fmt::Debug for RNanExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RNanExpression")
            .field("nan_token", &support::DebugSyntaxResult(self.nan_token()))
            .finish()
    }
}
impl From<RNanExpression> for SyntaxNode {
    fn from(n: RNanExpression) -> SyntaxNode {
        n.syntax
    }
}
impl From<RNanExpression> for SyntaxElement {
    fn from(n: RNanExpression) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for RNextExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(R_NEXT_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == R_NEXT_EXPRESSION
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
impl std::fmt::Debug for RNextExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RNextExpression")
            .field("next_token", &support::DebugSyntaxResult(self.next_token()))
            .finish()
    }
}
impl From<RNextExpression> for SyntaxNode {
    fn from(n: RNextExpression) -> SyntaxNode {
        n.syntax
    }
}
impl From<RNextExpression> for SyntaxElement {
    fn from(n: RNextExpression) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for RNullExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(R_NULL_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == R_NULL_EXPRESSION
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
impl std::fmt::Debug for RNullExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RNullExpression")
            .field("null_token", &support::DebugSyntaxResult(self.null_token()))
            .finish()
    }
}
impl From<RNullExpression> for SyntaxNode {
    fn from(n: RNullExpression) -> SyntaxNode {
        n.syntax
    }
}
impl From<RNullExpression> for SyntaxElement {
    fn from(n: RNullExpression) -> SyntaxElement {
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
impl AstNode for RParenthesizedExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(R_PARENTHESIZED_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == R_PARENTHESIZED_EXPRESSION
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
impl std::fmt::Debug for RParenthesizedExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RParenthesizedExpression")
            .field(
                "l_paren_token",
                &support::DebugSyntaxResult(self.l_paren_token()),
            )
            .field("body", &support::DebugSyntaxResult(self.body()))
            .field(
                "r_paren_token",
                &support::DebugSyntaxResult(self.r_paren_token()),
            )
            .finish()
    }
}
impl From<RParenthesizedExpression> for SyntaxNode {
    fn from(n: RParenthesizedExpression) -> SyntaxNode {
        n.syntax
    }
}
impl From<RParenthesizedExpression> for SyntaxElement {
    fn from(n: RParenthesizedExpression) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for RRepeatStatement {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(R_REPEAT_STATEMENT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == R_REPEAT_STATEMENT
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
impl std::fmt::Debug for RRepeatStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RRepeatStatement")
            .field(
                "repeat_token",
                &support::DebugSyntaxResult(self.repeat_token()),
            )
            .field("body", &support::DebugSyntaxResult(self.body()))
            .finish()
    }
}
impl From<RRepeatStatement> for SyntaxNode {
    fn from(n: RRepeatStatement) -> SyntaxNode {
        n.syntax
    }
}
impl From<RRepeatStatement> for SyntaxElement {
    fn from(n: RRepeatStatement) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for RReturnExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(R_RETURN_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == R_RETURN_EXPRESSION
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
impl std::fmt::Debug for RReturnExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RReturnExpression")
            .field(
                "return_token",
                &support::DebugSyntaxResult(self.return_token()),
            )
            .finish()
    }
}
impl From<RReturnExpression> for SyntaxNode {
    fn from(n: RReturnExpression) -> SyntaxNode {
        n.syntax
    }
}
impl From<RReturnExpression> for SyntaxElement {
    fn from(n: RReturnExpression) -> SyntaxElement {
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
impl AstNode for RTrueExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(R_TRUE_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == R_TRUE_EXPRESSION
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
impl std::fmt::Debug for RTrueExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RTrueExpression")
            .field("true_token", &support::DebugSyntaxResult(self.true_token()))
            .finish()
    }
}
impl From<RTrueExpression> for SyntaxNode {
    fn from(n: RTrueExpression) -> SyntaxNode {
        n.syntax
    }
}
impl From<RTrueExpression> for SyntaxElement {
    fn from(n: RTrueExpression) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for RUnaryExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(R_UNARY_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == R_UNARY_EXPRESSION
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
impl std::fmt::Debug for RUnaryExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RUnaryExpression")
            .field("operator", &support::DebugSyntaxResult(self.operator()))
            .field("argument", &support::DebugSyntaxResult(self.argument()))
            .finish()
    }
}
impl From<RUnaryExpression> for SyntaxNode {
    fn from(n: RUnaryExpression) -> SyntaxNode {
        n.syntax
    }
}
impl From<RUnaryExpression> for SyntaxElement {
    fn from(n: RUnaryExpression) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for RUnnamedArgument {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(R_UNNAMED_ARGUMENT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == R_UNNAMED_ARGUMENT
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
impl std::fmt::Debug for RUnnamedArgument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RUnnamedArgument")
            .field("value", &support::DebugSyntaxResult(self.value()))
            .finish()
    }
}
impl From<RUnnamedArgument> for SyntaxNode {
    fn from(n: RUnnamedArgument) -> SyntaxNode {
        n.syntax
    }
}
impl From<RUnnamedArgument> for SyntaxElement {
    fn from(n: RUnnamedArgument) -> SyntaxElement {
        n.syntax.into()
    }
}
impl AstNode for RWhileStatement {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(R_WHILE_STATEMENT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == R_WHILE_STATEMENT
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
impl std::fmt::Debug for RWhileStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RWhileStatement")
            .field(
                "while_token",
                &support::DebugSyntaxResult(self.while_token()),
            )
            .field(
                "l_paren_token",
                &support::DebugSyntaxResult(self.l_paren_token()),
            )
            .field("condition", &support::DebugSyntaxResult(self.condition()))
            .field(
                "r_paren_token",
                &support::DebugSyntaxResult(self.r_paren_token()),
            )
            .field("body", &support::DebugSyntaxResult(self.body()))
            .finish()
    }
}
impl From<RWhileStatement> for SyntaxNode {
    fn from(n: RWhileStatement) -> SyntaxNode {
        n.syntax
    }
}
impl From<RWhileStatement> for SyntaxElement {
    fn from(n: RWhileStatement) -> SyntaxElement {
        n.syntax.into()
    }
}
impl From<RBogusArgument> for AnyRArgument {
    fn from(node: RBogusArgument) -> AnyRArgument {
        AnyRArgument::RBogusArgument(node)
    }
}
impl From<RDotsArgument> for AnyRArgument {
    fn from(node: RDotsArgument) -> AnyRArgument {
        AnyRArgument::RDotsArgument(node)
    }
}
impl From<RHoleArgument> for AnyRArgument {
    fn from(node: RHoleArgument) -> AnyRArgument {
        AnyRArgument::RHoleArgument(node)
    }
}
impl From<RNamedArgument> for AnyRArgument {
    fn from(node: RNamedArgument) -> AnyRArgument {
        AnyRArgument::RNamedArgument(node)
    }
}
impl From<RUnnamedArgument> for AnyRArgument {
    fn from(node: RUnnamedArgument) -> AnyRArgument {
        AnyRArgument::RUnnamedArgument(node)
    }
}
impl AstNode for AnyRArgument {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = RBogusArgument::KIND_SET
        .union(RDotsArgument::KIND_SET)
        .union(RHoleArgument::KIND_SET)
        .union(RNamedArgument::KIND_SET)
        .union(RUnnamedArgument::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            R_BOGUS_ARGUMENT
                | R_DOTS_ARGUMENT
                | R_HOLE_ARGUMENT
                | R_NAMED_ARGUMENT
                | R_UNNAMED_ARGUMENT
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            R_BOGUS_ARGUMENT => AnyRArgument::RBogusArgument(RBogusArgument { syntax }),
            R_DOTS_ARGUMENT => AnyRArgument::RDotsArgument(RDotsArgument { syntax }),
            R_HOLE_ARGUMENT => AnyRArgument::RHoleArgument(RHoleArgument { syntax }),
            R_NAMED_ARGUMENT => AnyRArgument::RNamedArgument(RNamedArgument { syntax }),
            R_UNNAMED_ARGUMENT => AnyRArgument::RUnnamedArgument(RUnnamedArgument { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyRArgument::RBogusArgument(it) => &it.syntax,
            AnyRArgument::RDotsArgument(it) => &it.syntax,
            AnyRArgument::RHoleArgument(it) => &it.syntax,
            AnyRArgument::RNamedArgument(it) => &it.syntax,
            AnyRArgument::RUnnamedArgument(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyRArgument::RBogusArgument(it) => it.syntax,
            AnyRArgument::RDotsArgument(it) => it.syntax,
            AnyRArgument::RHoleArgument(it) => it.syntax,
            AnyRArgument::RNamedArgument(it) => it.syntax,
            AnyRArgument::RUnnamedArgument(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyRArgument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyRArgument::RBogusArgument(it) => std::fmt::Debug::fmt(it, f),
            AnyRArgument::RDotsArgument(it) => std::fmt::Debug::fmt(it, f),
            AnyRArgument::RHoleArgument(it) => std::fmt::Debug::fmt(it, f),
            AnyRArgument::RNamedArgument(it) => std::fmt::Debug::fmt(it, f),
            AnyRArgument::RUnnamedArgument(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyRArgument> for SyntaxNode {
    fn from(n: AnyRArgument) -> SyntaxNode {
        match n {
            AnyRArgument::RBogusArgument(it) => it.into(),
            AnyRArgument::RDotsArgument(it) => it.into(),
            AnyRArgument::RHoleArgument(it) => it.into(),
            AnyRArgument::RNamedArgument(it) => it.into(),
            AnyRArgument::RUnnamedArgument(it) => it.into(),
        }
    }
}
impl From<AnyRArgument> for SyntaxElement {
    fn from(n: AnyRArgument) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<RDots> for AnyRArgumentName {
    fn from(node: RDots) -> AnyRArgumentName {
        AnyRArgumentName::RDots(node)
    }
}
impl From<RIdentifier> for AnyRArgumentName {
    fn from(node: RIdentifier) -> AnyRArgumentName {
        AnyRArgumentName::RIdentifier(node)
    }
}
impl From<RStringValue> for AnyRArgumentName {
    fn from(node: RStringValue) -> AnyRArgumentName {
        AnyRArgumentName::RStringValue(node)
    }
}
impl AstNode for AnyRArgumentName {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = RDots::KIND_SET
        .union(RIdentifier::KIND_SET)
        .union(RStringValue::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, R_DOTS | R_IDENTIFIER | R_STRING_VALUE)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            R_DOTS => AnyRArgumentName::RDots(RDots { syntax }),
            R_IDENTIFIER => AnyRArgumentName::RIdentifier(RIdentifier { syntax }),
            R_STRING_VALUE => AnyRArgumentName::RStringValue(RStringValue { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyRArgumentName::RDots(it) => &it.syntax,
            AnyRArgumentName::RIdentifier(it) => &it.syntax,
            AnyRArgumentName::RStringValue(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyRArgumentName::RDots(it) => it.syntax,
            AnyRArgumentName::RIdentifier(it) => it.syntax,
            AnyRArgumentName::RStringValue(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyRArgumentName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyRArgumentName::RDots(it) => std::fmt::Debug::fmt(it, f),
            AnyRArgumentName::RIdentifier(it) => std::fmt::Debug::fmt(it, f),
            AnyRArgumentName::RStringValue(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyRArgumentName> for SyntaxNode {
    fn from(n: AnyRArgumentName) -> SyntaxNode {
        match n {
            AnyRArgumentName::RDots(it) => it.into(),
            AnyRArgumentName::RIdentifier(it) => it.into(),
            AnyRArgumentName::RStringValue(it) => it.into(),
        }
    }
}
impl From<AnyRArgumentName> for SyntaxElement {
    fn from(n: AnyRArgumentName) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
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
impl From<RBracedExpressions> for AnyRExpression {
    fn from(node: RBracedExpressions) -> AnyRExpression {
        AnyRExpression::RBracedExpressions(node)
    }
}
impl From<RBreakExpression> for AnyRExpression {
    fn from(node: RBreakExpression) -> AnyRExpression {
        AnyRExpression::RBreakExpression(node)
    }
}
impl From<RCall> for AnyRExpression {
    fn from(node: RCall) -> AnyRExpression {
        AnyRExpression::RCall(node)
    }
}
impl From<RDotDotI> for AnyRExpression {
    fn from(node: RDotDotI) -> AnyRExpression {
        AnyRExpression::RDotDotI(node)
    }
}
impl From<RFalseExpression> for AnyRExpression {
    fn from(node: RFalseExpression) -> AnyRExpression {
        AnyRExpression::RFalseExpression(node)
    }
}
impl From<RForStatement> for AnyRExpression {
    fn from(node: RForStatement) -> AnyRExpression {
        AnyRExpression::RForStatement(node)
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
impl From<RIfStatement> for AnyRExpression {
    fn from(node: RIfStatement) -> AnyRExpression {
        AnyRExpression::RIfStatement(node)
    }
}
impl From<RInfExpression> for AnyRExpression {
    fn from(node: RInfExpression) -> AnyRExpression {
        AnyRExpression::RInfExpression(node)
    }
}
impl From<RNaExpression> for AnyRExpression {
    fn from(node: RNaExpression) -> AnyRExpression {
        AnyRExpression::RNaExpression(node)
    }
}
impl From<RNanExpression> for AnyRExpression {
    fn from(node: RNanExpression) -> AnyRExpression {
        AnyRExpression::RNanExpression(node)
    }
}
impl From<RNextExpression> for AnyRExpression {
    fn from(node: RNextExpression) -> AnyRExpression {
        AnyRExpression::RNextExpression(node)
    }
}
impl From<RNullExpression> for AnyRExpression {
    fn from(node: RNullExpression) -> AnyRExpression {
        AnyRExpression::RNullExpression(node)
    }
}
impl From<RParenthesizedExpression> for AnyRExpression {
    fn from(node: RParenthesizedExpression) -> AnyRExpression {
        AnyRExpression::RParenthesizedExpression(node)
    }
}
impl From<RRepeatStatement> for AnyRExpression {
    fn from(node: RRepeatStatement) -> AnyRExpression {
        AnyRExpression::RRepeatStatement(node)
    }
}
impl From<RReturnExpression> for AnyRExpression {
    fn from(node: RReturnExpression) -> AnyRExpression {
        AnyRExpression::RReturnExpression(node)
    }
}
impl From<RTrueExpression> for AnyRExpression {
    fn from(node: RTrueExpression) -> AnyRExpression {
        AnyRExpression::RTrueExpression(node)
    }
}
impl From<RUnaryExpression> for AnyRExpression {
    fn from(node: RUnaryExpression) -> AnyRExpression {
        AnyRExpression::RUnaryExpression(node)
    }
}
impl From<RWhileStatement> for AnyRExpression {
    fn from(node: RWhileStatement) -> AnyRExpression {
        AnyRExpression::RWhileStatement(node)
    }
}
impl AstNode for AnyRExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = AnyRValue::KIND_SET
        .union(RBinaryExpression::KIND_SET)
        .union(RBogusExpression::KIND_SET)
        .union(RBracedExpressions::KIND_SET)
        .union(RBreakExpression::KIND_SET)
        .union(RCall::KIND_SET)
        .union(RDotDotI::KIND_SET)
        .union(RFalseExpression::KIND_SET)
        .union(RForStatement::KIND_SET)
        .union(RFunctionDefinition::KIND_SET)
        .union(RIdentifier::KIND_SET)
        .union(RIfStatement::KIND_SET)
        .union(RInfExpression::KIND_SET)
        .union(RNaExpression::KIND_SET)
        .union(RNanExpression::KIND_SET)
        .union(RNextExpression::KIND_SET)
        .union(RNullExpression::KIND_SET)
        .union(RParenthesizedExpression::KIND_SET)
        .union(RRepeatStatement::KIND_SET)
        .union(RReturnExpression::KIND_SET)
        .union(RTrueExpression::KIND_SET)
        .union(RUnaryExpression::KIND_SET)
        .union(RWhileStatement::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            R_BINARY_EXPRESSION
            | R_BOGUS_EXPRESSION
            | R_BRACED_EXPRESSIONS
            | R_BREAK_EXPRESSION
            | R_CALL
            | R_DOT_DOT_I
            | R_FALSE_EXPRESSION
            | R_FOR_STATEMENT
            | R_FUNCTION_DEFINITION
            | R_IDENTIFIER
            | R_IF_STATEMENT
            | R_INF_EXPRESSION
            | R_NA_EXPRESSION
            | R_NAN_EXPRESSION
            | R_NEXT_EXPRESSION
            | R_NULL_EXPRESSION
            | R_PARENTHESIZED_EXPRESSION
            | R_REPEAT_STATEMENT
            | R_RETURN_EXPRESSION
            | R_TRUE_EXPRESSION
            | R_UNARY_EXPRESSION
            | R_WHILE_STATEMENT => true,
            k if AnyRValue::can_cast(k) => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            R_BINARY_EXPRESSION => AnyRExpression::RBinaryExpression(RBinaryExpression { syntax }),
            R_BOGUS_EXPRESSION => AnyRExpression::RBogusExpression(RBogusExpression { syntax }),
            R_BRACED_EXPRESSIONS => {
                AnyRExpression::RBracedExpressions(RBracedExpressions { syntax })
            }
            R_BREAK_EXPRESSION => AnyRExpression::RBreakExpression(RBreakExpression { syntax }),
            R_CALL => AnyRExpression::RCall(RCall { syntax }),
            R_DOT_DOT_I => AnyRExpression::RDotDotI(RDotDotI { syntax }),
            R_FALSE_EXPRESSION => AnyRExpression::RFalseExpression(RFalseExpression { syntax }),
            R_FOR_STATEMENT => AnyRExpression::RForStatement(RForStatement { syntax }),
            R_FUNCTION_DEFINITION => {
                AnyRExpression::RFunctionDefinition(RFunctionDefinition { syntax })
            }
            R_IDENTIFIER => AnyRExpression::RIdentifier(RIdentifier { syntax }),
            R_IF_STATEMENT => AnyRExpression::RIfStatement(RIfStatement { syntax }),
            R_INF_EXPRESSION => AnyRExpression::RInfExpression(RInfExpression { syntax }),
            R_NA_EXPRESSION => AnyRExpression::RNaExpression(RNaExpression { syntax }),
            R_NAN_EXPRESSION => AnyRExpression::RNanExpression(RNanExpression { syntax }),
            R_NEXT_EXPRESSION => AnyRExpression::RNextExpression(RNextExpression { syntax }),
            R_NULL_EXPRESSION => AnyRExpression::RNullExpression(RNullExpression { syntax }),
            R_PARENTHESIZED_EXPRESSION => {
                AnyRExpression::RParenthesizedExpression(RParenthesizedExpression { syntax })
            }
            R_REPEAT_STATEMENT => AnyRExpression::RRepeatStatement(RRepeatStatement { syntax }),
            R_RETURN_EXPRESSION => AnyRExpression::RReturnExpression(RReturnExpression { syntax }),
            R_TRUE_EXPRESSION => AnyRExpression::RTrueExpression(RTrueExpression { syntax }),
            R_UNARY_EXPRESSION => AnyRExpression::RUnaryExpression(RUnaryExpression { syntax }),
            R_WHILE_STATEMENT => AnyRExpression::RWhileStatement(RWhileStatement { syntax }),
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
            AnyRExpression::RBracedExpressions(it) => &it.syntax,
            AnyRExpression::RBreakExpression(it) => &it.syntax,
            AnyRExpression::RCall(it) => &it.syntax,
            AnyRExpression::RDotDotI(it) => &it.syntax,
            AnyRExpression::RFalseExpression(it) => &it.syntax,
            AnyRExpression::RForStatement(it) => &it.syntax,
            AnyRExpression::RFunctionDefinition(it) => &it.syntax,
            AnyRExpression::RIdentifier(it) => &it.syntax,
            AnyRExpression::RIfStatement(it) => &it.syntax,
            AnyRExpression::RInfExpression(it) => &it.syntax,
            AnyRExpression::RNaExpression(it) => &it.syntax,
            AnyRExpression::RNanExpression(it) => &it.syntax,
            AnyRExpression::RNextExpression(it) => &it.syntax,
            AnyRExpression::RNullExpression(it) => &it.syntax,
            AnyRExpression::RParenthesizedExpression(it) => &it.syntax,
            AnyRExpression::RRepeatStatement(it) => &it.syntax,
            AnyRExpression::RReturnExpression(it) => &it.syntax,
            AnyRExpression::RTrueExpression(it) => &it.syntax,
            AnyRExpression::RUnaryExpression(it) => &it.syntax,
            AnyRExpression::RWhileStatement(it) => &it.syntax,
            AnyRExpression::AnyRValue(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyRExpression::RBinaryExpression(it) => it.syntax,
            AnyRExpression::RBogusExpression(it) => it.syntax,
            AnyRExpression::RBracedExpressions(it) => it.syntax,
            AnyRExpression::RBreakExpression(it) => it.syntax,
            AnyRExpression::RCall(it) => it.syntax,
            AnyRExpression::RDotDotI(it) => it.syntax,
            AnyRExpression::RFalseExpression(it) => it.syntax,
            AnyRExpression::RForStatement(it) => it.syntax,
            AnyRExpression::RFunctionDefinition(it) => it.syntax,
            AnyRExpression::RIdentifier(it) => it.syntax,
            AnyRExpression::RIfStatement(it) => it.syntax,
            AnyRExpression::RInfExpression(it) => it.syntax,
            AnyRExpression::RNaExpression(it) => it.syntax,
            AnyRExpression::RNanExpression(it) => it.syntax,
            AnyRExpression::RNextExpression(it) => it.syntax,
            AnyRExpression::RNullExpression(it) => it.syntax,
            AnyRExpression::RParenthesizedExpression(it) => it.syntax,
            AnyRExpression::RRepeatStatement(it) => it.syntax,
            AnyRExpression::RReturnExpression(it) => it.syntax,
            AnyRExpression::RTrueExpression(it) => it.syntax,
            AnyRExpression::RUnaryExpression(it) => it.syntax,
            AnyRExpression::RWhileStatement(it) => it.syntax,
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
            AnyRExpression::RBracedExpressions(it) => std::fmt::Debug::fmt(it, f),
            AnyRExpression::RBreakExpression(it) => std::fmt::Debug::fmt(it, f),
            AnyRExpression::RCall(it) => std::fmt::Debug::fmt(it, f),
            AnyRExpression::RDotDotI(it) => std::fmt::Debug::fmt(it, f),
            AnyRExpression::RFalseExpression(it) => std::fmt::Debug::fmt(it, f),
            AnyRExpression::RForStatement(it) => std::fmt::Debug::fmt(it, f),
            AnyRExpression::RFunctionDefinition(it) => std::fmt::Debug::fmt(it, f),
            AnyRExpression::RIdentifier(it) => std::fmt::Debug::fmt(it, f),
            AnyRExpression::RIfStatement(it) => std::fmt::Debug::fmt(it, f),
            AnyRExpression::RInfExpression(it) => std::fmt::Debug::fmt(it, f),
            AnyRExpression::RNaExpression(it) => std::fmt::Debug::fmt(it, f),
            AnyRExpression::RNanExpression(it) => std::fmt::Debug::fmt(it, f),
            AnyRExpression::RNextExpression(it) => std::fmt::Debug::fmt(it, f),
            AnyRExpression::RNullExpression(it) => std::fmt::Debug::fmt(it, f),
            AnyRExpression::RParenthesizedExpression(it) => std::fmt::Debug::fmt(it, f),
            AnyRExpression::RRepeatStatement(it) => std::fmt::Debug::fmt(it, f),
            AnyRExpression::RReturnExpression(it) => std::fmt::Debug::fmt(it, f),
            AnyRExpression::RTrueExpression(it) => std::fmt::Debug::fmt(it, f),
            AnyRExpression::RUnaryExpression(it) => std::fmt::Debug::fmt(it, f),
            AnyRExpression::RWhileStatement(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyRExpression> for SyntaxNode {
    fn from(n: AnyRExpression) -> SyntaxNode {
        match n {
            AnyRExpression::AnyRValue(it) => it.into(),
            AnyRExpression::RBinaryExpression(it) => it.into(),
            AnyRExpression::RBogusExpression(it) => it.into(),
            AnyRExpression::RBracedExpressions(it) => it.into(),
            AnyRExpression::RBreakExpression(it) => it.into(),
            AnyRExpression::RCall(it) => it.into(),
            AnyRExpression::RDotDotI(it) => it.into(),
            AnyRExpression::RFalseExpression(it) => it.into(),
            AnyRExpression::RForStatement(it) => it.into(),
            AnyRExpression::RFunctionDefinition(it) => it.into(),
            AnyRExpression::RIdentifier(it) => it.into(),
            AnyRExpression::RIfStatement(it) => it.into(),
            AnyRExpression::RInfExpression(it) => it.into(),
            AnyRExpression::RNaExpression(it) => it.into(),
            AnyRExpression::RNanExpression(it) => it.into(),
            AnyRExpression::RNextExpression(it) => it.into(),
            AnyRExpression::RNullExpression(it) => it.into(),
            AnyRExpression::RParenthesizedExpression(it) => it.into(),
            AnyRExpression::RRepeatStatement(it) => it.into(),
            AnyRExpression::RReturnExpression(it) => it.into(),
            AnyRExpression::RTrueExpression(it) => it.into(),
            AnyRExpression::RUnaryExpression(it) => it.into(),
            AnyRExpression::RWhileStatement(it) => it.into(),
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
impl From<RComplexValue> for AnyRValue {
    fn from(node: RComplexValue) -> AnyRValue {
        AnyRValue::RComplexValue(node)
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
impl From<RStringValue> for AnyRValue {
    fn from(node: RStringValue) -> AnyRValue {
        AnyRValue::RStringValue(node)
    }
}
impl AstNode for AnyRValue {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = RBogusValue::KIND_SET
        .union(RComplexValue::KIND_SET)
        .union(RDoubleValue::KIND_SET)
        .union(RIntegerValue::KIND_SET)
        .union(RStringValue::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            R_BOGUS_VALUE | R_COMPLEX_VALUE | R_DOUBLE_VALUE | R_INTEGER_VALUE | R_STRING_VALUE
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            R_BOGUS_VALUE => AnyRValue::RBogusValue(RBogusValue { syntax }),
            R_COMPLEX_VALUE => AnyRValue::RComplexValue(RComplexValue { syntax }),
            R_DOUBLE_VALUE => AnyRValue::RDoubleValue(RDoubleValue { syntax }),
            R_INTEGER_VALUE => AnyRValue::RIntegerValue(RIntegerValue { syntax }),
            R_STRING_VALUE => AnyRValue::RStringValue(RStringValue { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyRValue::RBogusValue(it) => &it.syntax,
            AnyRValue::RComplexValue(it) => &it.syntax,
            AnyRValue::RDoubleValue(it) => &it.syntax,
            AnyRValue::RIntegerValue(it) => &it.syntax,
            AnyRValue::RStringValue(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyRValue::RBogusValue(it) => it.syntax,
            AnyRValue::RComplexValue(it) => it.syntax,
            AnyRValue::RDoubleValue(it) => it.syntax,
            AnyRValue::RIntegerValue(it) => it.syntax,
            AnyRValue::RStringValue(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyRValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyRValue::RBogusValue(it) => std::fmt::Debug::fmt(it, f),
            AnyRValue::RComplexValue(it) => std::fmt::Debug::fmt(it, f),
            AnyRValue::RDoubleValue(it) => std::fmt::Debug::fmt(it, f),
            AnyRValue::RIntegerValue(it) => std::fmt::Debug::fmt(it, f),
            AnyRValue::RStringValue(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyRValue> for SyntaxNode {
    fn from(n: AnyRValue) -> SyntaxNode {
        match n {
            AnyRValue::RBogusValue(it) => it.into(),
            AnyRValue::RComplexValue(it) => it.into(),
            AnyRValue::RDoubleValue(it) => it.into(),
            AnyRValue::RIntegerValue(it) => it.into(),
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
impl std::fmt::Display for AnyRArgument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyRArgumentName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
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
impl std::fmt::Display for RBracedExpressions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for RBreakExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for RCall {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for RCallArguments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for RComplexValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for RDefaultParameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for RDotDotI {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for RDots {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for RDotsArgument {
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
impl std::fmt::Display for RElseClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for RFalseExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for RForStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for RFunctionDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for RHoleArgument {
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
impl std::fmt::Display for RIfStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for RInfExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for RIntegerValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for RNaExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for RNamedArgument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for RNanExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for RNextExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for RNullExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for RParameters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for RParenthesizedExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for RRepeatStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for RReturnExpression {
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
impl std::fmt::Display for RTrueExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for RUnaryExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for RUnnamedArgument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for RWhileStatement {
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
pub struct RBogusArgument {
    syntax: SyntaxNode,
}
impl RBogusArgument {
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
impl AstNode for RBogusArgument {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(R_BOGUS_ARGUMENT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == R_BOGUS_ARGUMENT
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
impl std::fmt::Debug for RBogusArgument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RBogusArgument")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<RBogusArgument> for SyntaxNode {
    fn from(n: RBogusArgument) -> SyntaxNode {
        n.syntax
    }
}
impl From<RBogusArgument> for SyntaxElement {
    fn from(n: RBogusArgument) -> SyntaxElement {
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
pub struct RArgumentList {
    syntax_list: SyntaxList,
}
impl RArgumentList {
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
impl AstNode for RArgumentList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(R_ARGUMENT_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == R_ARGUMENT_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<RArgumentList> {
        if Self::can_cast(syntax.kind()) {
            Some(RArgumentList {
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
impl Serialize for RArgumentList {
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
impl AstSeparatedList for RArgumentList {
    type Language = Language;
    type Node = AnyRArgument;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for RArgumentList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("RArgumentList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for RArgumentList {
    type Item = SyntaxResult<AnyRArgument>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyRArgument>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for &RArgumentList {
    type Item = SyntaxResult<AnyRArgument>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyRArgument>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
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
