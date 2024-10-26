//! Generated file, do not edit by hand, see `xtask/codegen`

#![allow(clippy::redundant_closure)]
#![allow(clippy::too_many_arguments)]
use biome_r_syntax::{
    RSyntaxElement as SyntaxElement, RSyntaxNode as SyntaxNode, RSyntaxToken as SyntaxToken, *,
};
use biome_rowan::AstNode;
pub fn r_binary_expression(
    left: AnyRExpression,
    operator_token: SyntaxToken,
    right: AnyRExpression,
) -> RBinaryExpression {
    RBinaryExpression::unwrap_cast(SyntaxNode::new_detached(
        RSyntaxKind::R_BINARY_EXPRESSION,
        [
            Some(SyntaxElement::Node(left.into_syntax())),
            Some(SyntaxElement::Token(operator_token)),
            Some(SyntaxElement::Node(right.into_syntax())),
        ],
    ))
}
pub fn r_default_parameter(
    name_token: SyntaxToken,
    eq_token: SyntaxToken,
) -> RDefaultParameterBuilder {
    RDefaultParameterBuilder {
        name_token,
        eq_token,
        default: None,
    }
}
pub struct RDefaultParameterBuilder {
    name_token: SyntaxToken,
    eq_token: SyntaxToken,
    default: Option<AnyRExpression>,
}
impl RDefaultParameterBuilder {
    pub fn with_default(mut self, default: AnyRExpression) -> Self {
        self.default = Some(default);
        self
    }
    pub fn build(self) -> RDefaultParameter {
        RDefaultParameter::unwrap_cast(SyntaxNode::new_detached(
            RSyntaxKind::R_DEFAULT_PARAMETER,
            [
                Some(SyntaxElement::Token(self.name_token)),
                Some(SyntaxElement::Token(self.eq_token)),
                self.default
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn r_dots_parameter(name_token: SyntaxToken) -> RDotsParameter {
    RDotsParameter::unwrap_cast(SyntaxNode::new_detached(
        RSyntaxKind::R_DOTS_PARAMETER,
        [Some(SyntaxElement::Token(name_token))],
    ))
}
pub fn r_double_value(value_token: SyntaxToken) -> RDoubleValue {
    RDoubleValue::unwrap_cast(SyntaxNode::new_detached(
        RSyntaxKind::R_DOUBLE_VALUE,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn r_function_definition(
    function_token: SyntaxToken,
    parameters: RParameters,
    body: AnyRExpression,
) -> RFunctionDefinition {
    RFunctionDefinition::unwrap_cast(SyntaxNode::new_detached(
        RSyntaxKind::R_FUNCTION_DEFINITION,
        [
            Some(SyntaxElement::Token(function_token)),
            Some(SyntaxElement::Node(parameters.into_syntax())),
            Some(SyntaxElement::Node(body.into_syntax())),
        ],
    ))
}
pub fn r_identifier(name_token: SyntaxToken) -> RIdentifier {
    RIdentifier::unwrap_cast(SyntaxNode::new_detached(
        RSyntaxKind::R_IDENTIFIER,
        [Some(SyntaxElement::Token(name_token))],
    ))
}
pub fn r_identifier_parameter(name_token: SyntaxToken) -> RIdentifierParameter {
    RIdentifierParameter::unwrap_cast(SyntaxNode::new_detached(
        RSyntaxKind::R_IDENTIFIER_PARAMETER,
        [Some(SyntaxElement::Token(name_token))],
    ))
}
pub fn r_integer_value(value_token: SyntaxToken) -> RIntegerValue {
    RIntegerValue::unwrap_cast(SyntaxNode::new_detached(
        RSyntaxKind::R_INTEGER_VALUE,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn r_logical_value(value_token: SyntaxToken) -> RLogicalValue {
    RLogicalValue::unwrap_cast(SyntaxNode::new_detached(
        RSyntaxKind::R_LOGICAL_VALUE,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn r_null_value(value_token: SyntaxToken) -> RNullValue {
    RNullValue::unwrap_cast(SyntaxNode::new_detached(
        RSyntaxKind::R_NULL_VALUE,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn r_parameters(
    l_paren_token: SyntaxToken,
    items: RParameterList,
    r_paren_token: SyntaxToken,
) -> RParameters {
    RParameters::unwrap_cast(SyntaxNode::new_detached(
        RSyntaxKind::R_PARAMETERS,
        [
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(items.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn r_root(expressions: RExpressionList, eof_token: SyntaxToken) -> RRootBuilder {
    RRootBuilder {
        expressions,
        eof_token,
        bom_token: None,
    }
}
pub struct RRootBuilder {
    expressions: RExpressionList,
    eof_token: SyntaxToken,
    bom_token: Option<SyntaxToken>,
}
impl RRootBuilder {
    pub fn with_bom_token(mut self, bom_token: SyntaxToken) -> Self {
        self.bom_token = Some(bom_token);
        self
    }
    pub fn build(self) -> RRoot {
        RRoot::unwrap_cast(SyntaxNode::new_detached(
            RSyntaxKind::R_ROOT,
            [
                self.bom_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Node(self.expressions.into_syntax())),
                Some(SyntaxElement::Token(self.eof_token)),
            ],
        ))
    }
}
pub fn r_string_value(value_token: SyntaxToken) -> RStringValue {
    RStringValue::unwrap_cast(SyntaxNode::new_detached(
        RSyntaxKind::R_STRING_VALUE,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn r_expression_list<I>(items: I) -> RExpressionList
where
    I: IntoIterator<Item = AnyRExpression>,
    I::IntoIter: ExactSizeIterator,
{
    RExpressionList::unwrap_cast(SyntaxNode::new_detached(
        RSyntaxKind::R_EXPRESSION_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn r_parameter_list<I, S>(items: I, separators: S) -> RParameterList
where
    I: IntoIterator<Item = AnyRParameter>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = RSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    RParameterList::unwrap_cast(SyntaxNode::new_detached(
        RSyntaxKind::R_PARAMETER_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn r_bogus<I>(slots: I) -> RBogus
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    RBogus::unwrap_cast(SyntaxNode::new_detached(RSyntaxKind::R_BOGUS, slots))
}
pub fn r_bogus_expression<I>(slots: I) -> RBogusExpression
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    RBogusExpression::unwrap_cast(SyntaxNode::new_detached(
        RSyntaxKind::R_BOGUS_EXPRESSION,
        slots,
    ))
}
pub fn r_bogus_parameter<I>(slots: I) -> RBogusParameter
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    RBogusParameter::unwrap_cast(SyntaxNode::new_detached(
        RSyntaxKind::R_BOGUS_PARAMETER,
        slots,
    ))
}
pub fn r_bogus_value<I>(slots: I) -> RBogusValue
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    RBogusValue::unwrap_cast(SyntaxNode::new_detached(RSyntaxKind::R_BOGUS_VALUE, slots))
}
