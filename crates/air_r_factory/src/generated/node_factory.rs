//! Generated file, do not edit by hand, see `xtask/codegen`

#![allow(clippy::redundant_closure)]
#![allow(clippy::too_many_arguments)]
use air_r_syntax::{
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
pub fn r_braced_expressions(
    l_curly_token: SyntaxToken,
    expressions: RExpressionList,
    r_curly_token: SyntaxToken,
) -> RBracedExpressions {
    RBracedExpressions::unwrap_cast(SyntaxNode::new_detached(
        RSyntaxKind::R_BRACED_EXPRESSIONS,
        [
            Some(SyntaxElement::Token(l_curly_token)),
            Some(SyntaxElement::Node(expressions.into_syntax())),
            Some(SyntaxElement::Token(r_curly_token)),
        ],
    ))
}
pub fn r_break_expression(break_token: SyntaxToken) -> RBreakExpression {
    RBreakExpression::unwrap_cast(SyntaxNode::new_detached(
        RSyntaxKind::R_BREAK_EXPRESSION,
        [Some(SyntaxElement::Token(break_token))],
    ))
}
pub fn r_call(function: AnyRExpression, arguments: RCallArguments) -> RCall {
    RCall::unwrap_cast(SyntaxNode::new_detached(
        RSyntaxKind::R_CALL,
        [
            Some(SyntaxElement::Node(function.into_syntax())),
            Some(SyntaxElement::Node(arguments.into_syntax())),
        ],
    ))
}
pub fn r_call_arguments(
    l_paren_token: SyntaxToken,
    items: RArgumentList,
    r_paren_token: SyntaxToken,
) -> RCallArguments {
    RCallArguments::unwrap_cast(SyntaxNode::new_detached(
        RSyntaxKind::R_CALL_ARGUMENTS,
        [
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(items.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn r_complex_value(value_token: SyntaxToken) -> RComplexValue {
    RComplexValue::unwrap_cast(SyntaxNode::new_detached(
        RSyntaxKind::R_COMPLEX_VALUE,
        [Some(SyntaxElement::Token(value_token))],
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
pub fn r_dot_dot_i(value_token: SyntaxToken) -> RDotDotI {
    RDotDotI::unwrap_cast(SyntaxNode::new_detached(
        RSyntaxKind::R_DOT_DOT_I,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn r_dots(value_token: SyntaxToken) -> RDots {
    RDots::unwrap_cast(SyntaxNode::new_detached(
        RSyntaxKind::R_DOTS,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn r_dots_argument(value_token: SyntaxToken) -> RDotsArgument {
    RDotsArgument::unwrap_cast(SyntaxNode::new_detached(
        RSyntaxKind::R_DOTS_ARGUMENT,
        [Some(SyntaxElement::Token(value_token))],
    ))
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
pub fn r_else_clause(else_token: SyntaxToken, alternative: AnyRExpression) -> RElseClause {
    RElseClause::unwrap_cast(SyntaxNode::new_detached(
        RSyntaxKind::R_ELSE_CLAUSE,
        [
            Some(SyntaxElement::Token(else_token)),
            Some(SyntaxElement::Node(alternative.into_syntax())),
        ],
    ))
}
pub fn r_extract_expression(
    left: AnyRExpression,
    operator_token: SyntaxToken,
    right: RSymbolOrString,
) -> RExtractExpression {
    RExtractExpression::unwrap_cast(SyntaxNode::new_detached(
        RSyntaxKind::R_EXTRACT_EXPRESSION,
        [
            Some(SyntaxElement::Node(left.into_syntax())),
            Some(SyntaxElement::Token(operator_token)),
            Some(SyntaxElement::Node(right.into_syntax())),
        ],
    ))
}
pub fn r_false_expression(false_token: SyntaxToken) -> RFalseExpression {
    RFalseExpression::unwrap_cast(SyntaxNode::new_detached(
        RSyntaxKind::R_FALSE_EXPRESSION,
        [Some(SyntaxElement::Token(false_token))],
    ))
}
pub fn r_for_statement(
    for_token: SyntaxToken,
    l_paren_token: SyntaxToken,
    variable: RIdentifier,
    in_token: SyntaxToken,
    sequence: AnyRExpression,
    r_paren_token: SyntaxToken,
    body: AnyRExpression,
) -> RForStatement {
    RForStatement::unwrap_cast(SyntaxNode::new_detached(
        RSyntaxKind::R_FOR_STATEMENT,
        [
            Some(SyntaxElement::Token(for_token)),
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(variable.into_syntax())),
            Some(SyntaxElement::Token(in_token)),
            Some(SyntaxElement::Node(sequence.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
            Some(SyntaxElement::Node(body.into_syntax())),
        ],
    ))
}
pub fn r_function_definition(
    name_token: SyntaxToken,
    parameters: RParameters,
    body: AnyRExpression,
) -> RFunctionDefinition {
    RFunctionDefinition::unwrap_cast(SyntaxNode::new_detached(
        RSyntaxKind::R_FUNCTION_DEFINITION,
        [
            Some(SyntaxElement::Token(name_token)),
            Some(SyntaxElement::Node(parameters.into_syntax())),
            Some(SyntaxElement::Node(body.into_syntax())),
        ],
    ))
}
pub fn r_hole_argument() -> RHoleArgument {
    RHoleArgument::unwrap_cast(SyntaxNode::new_detached(RSyntaxKind::R_HOLE_ARGUMENT, []))
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
pub fn r_if_statement(
    if_token: SyntaxToken,
    l_paren_token: SyntaxToken,
    condition: AnyRExpression,
    r_paren_token: SyntaxToken,
    consequence: AnyRExpression,
) -> RIfStatementBuilder {
    RIfStatementBuilder {
        if_token,
        l_paren_token,
        condition,
        r_paren_token,
        consequence,
        else_clause: None,
    }
}
pub struct RIfStatementBuilder {
    if_token: SyntaxToken,
    l_paren_token: SyntaxToken,
    condition: AnyRExpression,
    r_paren_token: SyntaxToken,
    consequence: AnyRExpression,
    else_clause: Option<RElseClause>,
}
impl RIfStatementBuilder {
    pub fn with_else_clause(mut self, else_clause: RElseClause) -> Self {
        self.else_clause = Some(else_clause);
        self
    }
    pub fn build(self) -> RIfStatement {
        RIfStatement::unwrap_cast(SyntaxNode::new_detached(
            RSyntaxKind::R_IF_STATEMENT,
            [
                Some(SyntaxElement::Token(self.if_token)),
                Some(SyntaxElement::Token(self.l_paren_token)),
                Some(SyntaxElement::Node(self.condition.into_syntax())),
                Some(SyntaxElement::Token(self.r_paren_token)),
                Some(SyntaxElement::Node(self.consequence.into_syntax())),
                self.else_clause
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn r_inf_expression(inf_token: SyntaxToken) -> RInfExpression {
    RInfExpression::unwrap_cast(SyntaxNode::new_detached(
        RSyntaxKind::R_INF_EXPRESSION,
        [Some(SyntaxElement::Token(inf_token))],
    ))
}
pub fn r_integer_value(value_token: SyntaxToken) -> RIntegerValue {
    RIntegerValue::unwrap_cast(SyntaxNode::new_detached(
        RSyntaxKind::R_INTEGER_VALUE,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn r_na_expression(value_token: SyntaxToken) -> RNaExpression {
    RNaExpression::unwrap_cast(SyntaxNode::new_detached(
        RSyntaxKind::R_NA_EXPRESSION,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn r_named_argument(name: AnyRArgumentName, eq_token: SyntaxToken) -> RNamedArgumentBuilder {
    RNamedArgumentBuilder {
        name,
        eq_token,
        value: None,
    }
}
pub struct RNamedArgumentBuilder {
    name: AnyRArgumentName,
    eq_token: SyntaxToken,
    value: Option<AnyRExpression>,
}
impl RNamedArgumentBuilder {
    pub fn with_value(mut self, value: AnyRExpression) -> Self {
        self.value = Some(value);
        self
    }
    pub fn build(self) -> RNamedArgument {
        RNamedArgument::unwrap_cast(SyntaxNode::new_detached(
            RSyntaxKind::R_NAMED_ARGUMENT,
            [
                Some(SyntaxElement::Node(self.name.into_syntax())),
                Some(SyntaxElement::Token(self.eq_token)),
                self.value
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn r_namespace_expression(
    left: RSymbolOrString,
    operator_token: SyntaxToken,
    right: RSymbolOrString,
) -> RNamespaceExpression {
    RNamespaceExpression::unwrap_cast(SyntaxNode::new_detached(
        RSyntaxKind::R_NAMESPACE_EXPRESSION,
        [
            Some(SyntaxElement::Node(left.into_syntax())),
            Some(SyntaxElement::Token(operator_token)),
            Some(SyntaxElement::Node(right.into_syntax())),
        ],
    ))
}
pub fn r_nan_expression(nan_token: SyntaxToken) -> RNanExpression {
    RNanExpression::unwrap_cast(SyntaxNode::new_detached(
        RSyntaxKind::R_NAN_EXPRESSION,
        [Some(SyntaxElement::Token(nan_token))],
    ))
}
pub fn r_next_expression(next_token: SyntaxToken) -> RNextExpression {
    RNextExpression::unwrap_cast(SyntaxNode::new_detached(
        RSyntaxKind::R_NEXT_EXPRESSION,
        [Some(SyntaxElement::Token(next_token))],
    ))
}
pub fn r_null_expression(null_token: SyntaxToken) -> RNullExpression {
    RNullExpression::unwrap_cast(SyntaxNode::new_detached(
        RSyntaxKind::R_NULL_EXPRESSION,
        [Some(SyntaxElement::Token(null_token))],
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
pub fn r_parenthesized_expression(
    l_paren_token: SyntaxToken,
    body: AnyRExpression,
    r_paren_token: SyntaxToken,
) -> RParenthesizedExpression {
    RParenthesizedExpression::unwrap_cast(SyntaxNode::new_detached(
        RSyntaxKind::R_PARENTHESIZED_EXPRESSION,
        [
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(body.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn r_repeat_statement(repeat_token: SyntaxToken, body: AnyRExpression) -> RRepeatStatement {
    RRepeatStatement::unwrap_cast(SyntaxNode::new_detached(
        RSyntaxKind::R_REPEAT_STATEMENT,
        [
            Some(SyntaxElement::Token(repeat_token)),
            Some(SyntaxElement::Node(body.into_syntax())),
        ],
    ))
}
pub fn r_return_expression(return_token: SyntaxToken) -> RReturnExpression {
    RReturnExpression::unwrap_cast(SyntaxNode::new_detached(
        RSyntaxKind::R_RETURN_EXPRESSION,
        [Some(SyntaxElement::Token(return_token))],
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
pub fn r_true_expression(true_token: SyntaxToken) -> RTrueExpression {
    RTrueExpression::unwrap_cast(SyntaxNode::new_detached(
        RSyntaxKind::R_TRUE_EXPRESSION,
        [Some(SyntaxElement::Token(true_token))],
    ))
}
pub fn r_unary_expression(
    operator_token: SyntaxToken,
    argument: AnyRExpression,
) -> RUnaryExpression {
    RUnaryExpression::unwrap_cast(SyntaxNode::new_detached(
        RSyntaxKind::R_UNARY_EXPRESSION,
        [
            Some(SyntaxElement::Token(operator_token)),
            Some(SyntaxElement::Node(argument.into_syntax())),
        ],
    ))
}
pub fn r_unnamed_argument(value: AnyRExpression) -> RUnnamedArgument {
    RUnnamedArgument::unwrap_cast(SyntaxNode::new_detached(
        RSyntaxKind::R_UNNAMED_ARGUMENT,
        [Some(SyntaxElement::Node(value.into_syntax()))],
    ))
}
pub fn r_while_statement(
    while_token: SyntaxToken,
    l_paren_token: SyntaxToken,
    condition: AnyRExpression,
    r_paren_token: SyntaxToken,
    body: AnyRExpression,
) -> RWhileStatement {
    RWhileStatement::unwrap_cast(SyntaxNode::new_detached(
        RSyntaxKind::R_WHILE_STATEMENT,
        [
            Some(SyntaxElement::Token(while_token)),
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(condition.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
            Some(SyntaxElement::Node(body.into_syntax())),
        ],
    ))
}
pub fn r_argument_list<I, S>(items: I, separators: S) -> RArgumentList
where
    I: IntoIterator<Item = AnyRArgument>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = RSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    RArgumentList::unwrap_cast(SyntaxNode::new_detached(
        RSyntaxKind::R_ARGUMENT_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
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
pub fn r_bogus_argument<I>(slots: I) -> RBogusArgument
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    RBogusArgument::unwrap_cast(SyntaxNode::new_detached(
        RSyntaxKind::R_BOGUS_ARGUMENT,
        slots,
    ))
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
