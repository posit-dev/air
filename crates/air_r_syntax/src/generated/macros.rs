//! Generated file, do not edit by hand, see `xtask/codegen`

#[doc = r" Reconstruct an AstNode from a SyntaxNode"]
#[doc = r""]
#[doc = r" This macros performs a match over the [kind](biome_rowan::SyntaxNode::kind)"]
#[doc = r" of the provided [biome_rowan::SyntaxNode] and constructs the appropriate"]
#[doc = r" AstNode type for it, then execute the provided expression over it."]
#[doc = r""]
#[doc = r" # Examples"]
#[doc = r""]
#[doc = r" ```ignore"]
#[doc = r" map_syntax_node!(syntax_node, node => node.format())"]
#[doc = r" ```"]
#[macro_export]
macro_rules! map_syntax_node {
    ($ node : expr , $ pattern : pat => $ body : expr) => {
        match $node {
            node => match $crate::RSyntaxNode::kind(&node) {
                $crate::RSyntaxKind::R_ARGUMENT => {
                    let $pattern = unsafe { $crate::RArgument::new_unchecked(node) };
                    $body
                }
                $crate::RSyntaxKind::R_ARGUMENT_NAME_CLAUSE => {
                    let $pattern = unsafe { $crate::RArgumentNameClause::new_unchecked(node) };
                    $body
                }
                $crate::RSyntaxKind::R_BINARY_EXPRESSION => {
                    let $pattern = unsafe { $crate::RBinaryExpression::new_unchecked(node) };
                    $body
                }
                $crate::RSyntaxKind::R_BRACED_EXPRESSIONS => {
                    let $pattern = unsafe { $crate::RBracedExpressions::new_unchecked(node) };
                    $body
                }
                $crate::RSyntaxKind::R_BREAK_EXPRESSION => {
                    let $pattern = unsafe { $crate::RBreakExpression::new_unchecked(node) };
                    $body
                }
                $crate::RSyntaxKind::R_CALL => {
                    let $pattern = unsafe { $crate::RCall::new_unchecked(node) };
                    $body
                }
                $crate::RSyntaxKind::R_CALL_ARGUMENTS => {
                    let $pattern = unsafe { $crate::RCallArguments::new_unchecked(node) };
                    $body
                }
                $crate::RSyntaxKind::R_COMPLEX_VALUE => {
                    let $pattern = unsafe { $crate::RComplexValue::new_unchecked(node) };
                    $body
                }
                $crate::RSyntaxKind::R_DOT_DOT_I => {
                    let $pattern = unsafe { $crate::RDotDotI::new_unchecked(node) };
                    $body
                }
                $crate::RSyntaxKind::R_DOTS => {
                    let $pattern = unsafe { $crate::RDots::new_unchecked(node) };
                    $body
                }
                $crate::RSyntaxKind::R_DOUBLE_VALUE => {
                    let $pattern = unsafe { $crate::RDoubleValue::new_unchecked(node) };
                    $body
                }
                $crate::RSyntaxKind::R_ELSE_CLAUSE => {
                    let $pattern = unsafe { $crate::RElseClause::new_unchecked(node) };
                    $body
                }
                $crate::RSyntaxKind::R_EXTRACT_EXPRESSION => {
                    let $pattern = unsafe { $crate::RExtractExpression::new_unchecked(node) };
                    $body
                }
                $crate::RSyntaxKind::R_FALSE_EXPRESSION => {
                    let $pattern = unsafe { $crate::RFalseExpression::new_unchecked(node) };
                    $body
                }
                $crate::RSyntaxKind::R_FOR_STATEMENT => {
                    let $pattern = unsafe { $crate::RForStatement::new_unchecked(node) };
                    $body
                }
                $crate::RSyntaxKind::R_FUNCTION_DEFINITION => {
                    let $pattern = unsafe { $crate::RFunctionDefinition::new_unchecked(node) };
                    $body
                }
                $crate::RSyntaxKind::R_IDENTIFIER => {
                    let $pattern = unsafe { $crate::RIdentifier::new_unchecked(node) };
                    $body
                }
                $crate::RSyntaxKind::R_IF_STATEMENT => {
                    let $pattern = unsafe { $crate::RIfStatement::new_unchecked(node) };
                    $body
                }
                $crate::RSyntaxKind::R_INF_EXPRESSION => {
                    let $pattern = unsafe { $crate::RInfExpression::new_unchecked(node) };
                    $body
                }
                $crate::RSyntaxKind::R_INTEGER_VALUE => {
                    let $pattern = unsafe { $crate::RIntegerValue::new_unchecked(node) };
                    $body
                }
                $crate::RSyntaxKind::R_NA_EXPRESSION => {
                    let $pattern = unsafe { $crate::RNaExpression::new_unchecked(node) };
                    $body
                }
                $crate::RSyntaxKind::R_NAMESPACE_EXPRESSION => {
                    let $pattern = unsafe { $crate::RNamespaceExpression::new_unchecked(node) };
                    $body
                }
                $crate::RSyntaxKind::R_NAN_EXPRESSION => {
                    let $pattern = unsafe { $crate::RNanExpression::new_unchecked(node) };
                    $body
                }
                $crate::RSyntaxKind::R_NEXT_EXPRESSION => {
                    let $pattern = unsafe { $crate::RNextExpression::new_unchecked(node) };
                    $body
                }
                $crate::RSyntaxKind::R_NULL_EXPRESSION => {
                    let $pattern = unsafe { $crate::RNullExpression::new_unchecked(node) };
                    $body
                }
                $crate::RSyntaxKind::R_PARAMETER => {
                    let $pattern = unsafe { $crate::RParameter::new_unchecked(node) };
                    $body
                }
                $crate::RSyntaxKind::R_PARAMETER_DEFAULT => {
                    let $pattern = unsafe { $crate::RParameterDefault::new_unchecked(node) };
                    $body
                }
                $crate::RSyntaxKind::R_PARAMETERS => {
                    let $pattern = unsafe { $crate::RParameters::new_unchecked(node) };
                    $body
                }
                $crate::RSyntaxKind::R_PARENTHESIZED_EXPRESSION => {
                    let $pattern = unsafe { $crate::RParenthesizedExpression::new_unchecked(node) };
                    $body
                }
                $crate::RSyntaxKind::R_REPEAT_STATEMENT => {
                    let $pattern = unsafe { $crate::RRepeatStatement::new_unchecked(node) };
                    $body
                }
                $crate::RSyntaxKind::R_RETURN_EXPRESSION => {
                    let $pattern = unsafe { $crate::RReturnExpression::new_unchecked(node) };
                    $body
                }
                $crate::RSyntaxKind::R_ROOT => {
                    let $pattern = unsafe { $crate::RRoot::new_unchecked(node) };
                    $body
                }
                $crate::RSyntaxKind::R_STRING_VALUE => {
                    let $pattern = unsafe { $crate::RStringValue::new_unchecked(node) };
                    $body
                }
                $crate::RSyntaxKind::R_SUBSET => {
                    let $pattern = unsafe { $crate::RSubset::new_unchecked(node) };
                    $body
                }
                $crate::RSyntaxKind::R_SUBSET2 => {
                    let $pattern = unsafe { $crate::RSubset2::new_unchecked(node) };
                    $body
                }
                $crate::RSyntaxKind::R_SUBSET2_ARGUMENTS => {
                    let $pattern = unsafe { $crate::RSubset2Arguments::new_unchecked(node) };
                    $body
                }
                $crate::RSyntaxKind::R_SUBSET_ARGUMENTS => {
                    let $pattern = unsafe { $crate::RSubsetArguments::new_unchecked(node) };
                    $body
                }
                $crate::RSyntaxKind::R_TRUE_EXPRESSION => {
                    let $pattern = unsafe { $crate::RTrueExpression::new_unchecked(node) };
                    $body
                }
                $crate::RSyntaxKind::R_UNARY_EXPRESSION => {
                    let $pattern = unsafe { $crate::RUnaryExpression::new_unchecked(node) };
                    $body
                }
                $crate::RSyntaxKind::R_WHILE_STATEMENT => {
                    let $pattern = unsafe { $crate::RWhileStatement::new_unchecked(node) };
                    $body
                }
                $crate::RSyntaxKind::R_BOGUS => {
                    let $pattern = unsafe { $crate::RBogus::new_unchecked(node) };
                    $body
                }
                $crate::RSyntaxKind::R_BOGUS_EXPRESSION => {
                    let $pattern = unsafe { $crate::RBogusExpression::new_unchecked(node) };
                    $body
                }
                $crate::RSyntaxKind::R_BOGUS_VALUE => {
                    let $pattern = unsafe { $crate::RBogusValue::new_unchecked(node) };
                    $body
                }
                $crate::RSyntaxKind::R_ARGUMENT_LIST => {
                    let $pattern = unsafe { $crate::RArgumentList::new_unchecked(node) };
                    $body
                }
                $crate::RSyntaxKind::R_EXPRESSION_LIST => {
                    let $pattern = unsafe { $crate::RExpressionList::new_unchecked(node) };
                    $body
                }
                $crate::RSyntaxKind::R_PARAMETER_LIST => {
                    let $pattern = unsafe { $crate::RParameterList::new_unchecked(node) };
                    $body
                }
                _ => unreachable!(),
            },
        }
    };
}
pub(crate) use map_syntax_node;
