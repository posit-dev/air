use crate::comments::RComments;
use crate::prelude::*;
use air_r_syntax::AnyRExpression;
use air_r_syntax::RArgument;
use air_r_syntax::RBracedExpressions;
use air_r_syntax::RBracedExpressionsFields;
use air_r_syntax::RExpressionList;
use air_r_syntax::RSyntaxToken;
use biome_formatter::format_args;
use biome_formatter::write;
use biome_rowan::SyntaxResult;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRBracedExpressions;
impl FormatNodeRule<RBracedExpressions> for FormatRBracedExpressions {
    fn fmt_fields(&self, node: &RBracedExpressions, f: &mut RFormatter) -> FormatResult<()> {
        let RBracedExpressionsFields {
            l_curly_token,
            expressions,
            r_curly_token,
        } = node.as_fields();

        match braced_expressions_variant(node, f.comments()) {
            // Examples:
            //
            // ```r
            // {}
            // function() {}
            // while(waiting()) {}
            // for (x in xs) {}
            // switch(on, a = {}, b = 2)
            // ```
            BracedExpressionsVariant::Empty => {
                write!(f, [l_curly_token.format(), r_curly_token.format()])
            }
            // Examples:
            //
            // ```r
            // {
            //   # comment
            // }
            // ```
            BracedExpressionsVariant::EmptyWithDanglingComments => {
                write!(
                    f,
                    [
                        l_curly_token.format(),
                        format_dangling_comments(node.syntax()).with_block_indent(),
                        r_curly_token.format()
                    ]
                )
            }
            // Examples:
            //
            // ```r
            // fn({{ expr }})
            // ```
            BracedExpressionsVariant::CurlyCurly(node) => fmt_curly_curly(&node, f),
            // Examples:
            //
            // ```r
            // {
            //   expr
            //   expr
            // }
            // ```
            BracedExpressionsVariant::NotEmpty => {
                write!(
                    f,
                    [
                        l_curly_token.format(),
                        block_indent(&expressions.format()),
                        r_curly_token.format()
                    ]
                )
            }
        }
    }

    fn fmt_dangling_comments(
        &self,
        _: &RBracedExpressions,
        _: &mut RFormatter,
    ) -> FormatResult<()> {
        // Formatted inside of `fmt_fields`
        // Only possible with `BracedExpressionsVariant::EmptyWithDanglingComments`
        Ok(())
    }
}

pub(crate) enum BracedExpressionsVariant {
    Empty,
    EmptyWithDanglingComments,
    CurlyCurly(RCurlyCurly),
    NotEmpty,
}

/// Categorize [RBracedExpressions] into its appropriate variant
///
/// Used when formatting a [RBracedExpressions], and in `call_arguments.rs` when
/// determining if a [RBracedExpressions] is groupable or not.
pub(crate) fn braced_expressions_variant(
    node: &RBracedExpressions,
    comments: &RComments,
) -> BracedExpressionsVariant {
    if node.expressions().is_empty() {
        if comments.has_dangling_comments(node.syntax()) {
            return BracedExpressionsVariant::EmptyWithDanglingComments;
        } else {
            return BracedExpressionsVariant::Empty;
        }
    }

    if let Some(node) = as_curly_curly(node) {
        return BracedExpressionsVariant::CurlyCurly(node);
    }

    BracedExpressionsVariant::NotEmpty
}

pub(crate) struct RCurlyCurly {
    outer_l_curly_token: SyntaxResult<RSyntaxToken>,
    inner_l_curly_token: SyntaxResult<RSyntaxToken>,

    /// Interior expression list of the curly-curly expression
    ///
    /// Note that this always contains a single expression, but to simplify comment
    /// formatting we return the entire `RExpressionList` to call `.format()` on.
    expression: RExpressionList,

    inner_r_curly_token: SyntaxResult<RSyntaxToken>,
    outer_r_curly_token: SyntaxResult<RSyntaxToken>,

    /// Interior `{` node of the curly-curly
    ///
    /// Needed for marking the node as suppression-checked and for manually
    /// formatting leading and trailing comments
    node_inner: RBracedExpressions,
}

/// Convert `RBracedExpressions` into `RCurlyCurly`
///
/// Returns:
/// - `Some(RCurlyCurly)` if this braced expression looks like a curly-curly
/// - `None` otherwise
///
/// The spec for curly-curly that we are targeting is:
/// - An outer `{` expression with exactly 1 child, an inner `{`.
/// - An inner `{` expression with exactly 1 child, a symbol.
/// - An ancestor of the outer `{` must be an argument node.
fn as_curly_curly(node: &RBracedExpressions) -> Option<RCurlyCurly> {
    let RBracedExpressionsFields {
        l_curly_token: outer_l_curly_token,
        expressions: outer_expressions,
        r_curly_token: outer_r_curly_token,
    } = node.as_fields();

    if outer_expressions.len() != 1 {
        return None;
    }

    // Unwrap: Length check ensures there is exactly 1 child
    let node_inner = outer_expressions.first().unwrap();

    // Check that the child of the outer `{` is another `{`
    let node_inner = match node_inner {
        AnyRExpression::RBracedExpressions(node_inner) => node_inner,
        _ => return None,
    };

    let RBracedExpressionsFields {
        l_curly_token: inner_l_curly_token,
        expressions: expression,
        r_curly_token: inner_r_curly_token,
    } = node_inner.as_fields();

    // Check that inner child `{` itself only has 1 child, i.e. the `expr` in `{{ expr }}`
    if expression.len() != 1 {
        return None;
    }

    // Unwrap: Length check ensures there is exactly 1 child
    let symbol = expression.first().unwrap();

    // Check that the actual expression inside the `{{` is a simple identifier
    if !matches!(symbol, AnyRExpression::RIdentifier(_)) {
        return None;
    }

    // Check that an ancestor of the outer `{` is an argument node
    //
    // Curly-curly is only valid when inlined as a function argument, but
    // it can still be inside of a more complex inlined expression. As long
    // as one parent is an argument, we consider it to be curly-curly.
    //
    // This is likely the most expensive check, so we do this last when we are
    // mostly sure we have a curly-curly node.
    //
    // ```r
    // my_plus <- function(data, var) {
    //   dplyr::mutate(data, plus = {{ var }} + 1)
    // }
    // my_plus(mtcars, mpg)
    // ```
    let has_argument_parent = node
        .syntax()
        .ancestors()
        .any(|syntax| RArgument::can_cast(syntax.kind()));

    if !has_argument_parent {
        return None;
    }

    Some(RCurlyCurly {
        outer_l_curly_token,
        inner_l_curly_token,
        expression,
        inner_r_curly_token,
        outer_r_curly_token,
        node_inner,
    })
}

/// Format curly-curly braced expressions
///
/// These are typically very simple, of the form:
///
/// ```r
/// fn({{ var }})
/// ```
///
/// In these cases:
/// - We want `{{` and `}}` to hug tightly, with no space of soft break between them
/// - We don't force a hard block indent. It almost always fits on one line.
fn fmt_curly_curly(node: &RCurlyCurly, f: &mut RFormatter) -> FormatResult<()> {
    let RCurlyCurly {
        outer_l_curly_token,
        inner_l_curly_token,
        expression,
        inner_r_curly_token,
        outer_r_curly_token,
        node_inner,
    } = node;

    let comments = f.comments();
    let node_inner = node_inner.syntax();

    // It's only possible to suppress the formatting of the outer curlies of curly-curly,
    // so we don't need to branch based on `comments.is_suppressed()`, but we do need to
    // mark the node as suppression checked
    comments.mark_suppression_checked(node_inner);

    // It's impossible to have dangling comments on `node_inner`. That's only possible
    // when `node_inner` is an empty braced expression, and we would have bailed earlier
    // if that was the case, because it doesn't "look" like curly-curly then.
    if comments.has_dangling_comments(node_inner) {
        panic!("Inner `{{` of curly-curly can't have dangling comments.");
    }

    // We are sure we have curly-curly!
    //
    // We are going to format it all in one go, keeping in mind:
    // - We must `group()` it so the `soft_space_or_block_indent()` stays soft when possible
    // - Remember that comments can only be attached to nodes (not tokens), so we just
    //   have to worry about manually handling comments on the `node_inner` `RBracedExpressions`,
    //   since that is the only node we skip calling `format()` on.
    // - We push comments between the `{ {` as leading on the first `{`
    // - We push comments between the `} }` as trailing on the second `}`
    write!(
        f,
        [group(&format_args![
            format_leading_comments(node_inner),
            outer_l_curly_token.format(),
            inner_l_curly_token.format(),
            soft_space_or_block_indent(&expression.format()),
            inner_r_curly_token.format(),
            outer_r_curly_token.format(),
            format_trailing_comments(node_inner),
        ])]
    )
}
