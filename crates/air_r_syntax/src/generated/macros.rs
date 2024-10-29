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
                $crate::RSyntaxKind::R_BINARY_EXPRESSION => {
                    let $pattern = unsafe { $crate::RBinaryExpression::new_unchecked(node) };
                    $body
                }
                $crate::RSyntaxKind::R_DEFAULT_PARAMETER => {
                    let $pattern = unsafe { $crate::RDefaultParameter::new_unchecked(node) };
                    $body
                }
                $crate::RSyntaxKind::R_DOTS_PARAMETER => {
                    let $pattern = unsafe { $crate::RDotsParameter::new_unchecked(node) };
                    $body
                }
                $crate::RSyntaxKind::R_DOUBLE_VALUE => {
                    let $pattern = unsafe { $crate::RDoubleValue::new_unchecked(node) };
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
                $crate::RSyntaxKind::R_IDENTIFIER_PARAMETER => {
                    let $pattern = unsafe { $crate::RIdentifierParameter::new_unchecked(node) };
                    $body
                }
                $crate::RSyntaxKind::R_INTEGER_VALUE => {
                    let $pattern = unsafe { $crate::RIntegerValue::new_unchecked(node) };
                    $body
                }
                $crate::RSyntaxKind::R_LOGICAL_VALUE => {
                    let $pattern = unsafe { $crate::RLogicalValue::new_unchecked(node) };
                    $body
                }
                $crate::RSyntaxKind::R_NULL_VALUE => {
                    let $pattern = unsafe { $crate::RNullValue::new_unchecked(node) };
                    $body
                }
                $crate::RSyntaxKind::R_PARAMETERS => {
                    let $pattern = unsafe { $crate::RParameters::new_unchecked(node) };
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
                $crate::RSyntaxKind::R_BOGUS => {
                    let $pattern = unsafe { $crate::RBogus::new_unchecked(node) };
                    $body
                }
                $crate::RSyntaxKind::R_BOGUS_EXPRESSION => {
                    let $pattern = unsafe { $crate::RBogusExpression::new_unchecked(node) };
                    $body
                }
                $crate::RSyntaxKind::R_BOGUS_PARAMETER => {
                    let $pattern = unsafe { $crate::RBogusParameter::new_unchecked(node) };
                    $body
                }
                $crate::RSyntaxKind::R_BOGUS_VALUE => {
                    let $pattern = unsafe { $crate::RBogusValue::new_unchecked(node) };
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
