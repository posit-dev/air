use crate::prelude::*;
use air_r_syntax::AnyRExpression;
use air_r_syntax::RBracedExpressions;
use air_r_syntax::RBracedExpressionsFields;
use air_r_syntax::RExpressionList;
use air_r_syntax::RSyntaxToken;
use biome_formatter::format_args;
use biome_formatter::write;
use biome_formatter::CstFormatContext;
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

        // Check if we are formatting empty braces, like `{ }`
        if expressions.is_empty() {
            return fmt_empty(&l_curly_token, &r_curly_token, node, f);
        }

        // Check if we are formatting curly-curly, like `{{ expr }}`
        if let Some(node) = as_curly_curly(node) {
            return fmt_curly_curly(&node, f);
        }

        write!(
            f,
            [
                l_curly_token.format(),
                block_indent(&expressions.format()),
                r_curly_token.format()
            ]
        )
    }

    fn fmt_dangling_comments(
        &self,
        _: &RBracedExpressions,
        _: &mut RFormatter,
    ) -> FormatResult<()> {
        // Formatted inside of `fmt_fields`
        // Only possible with empty `{}`
        Ok(())
    }
}

/// Format an empty braced expression node
///
/// i.e. `{ }` or `{\n # hi\n}`
///
/// We unconditionally hard line break between the `{` and `}`. We could consider not
/// doing this for certain syntax, like if statements, but that gets a little hand wavy.
fn fmt_empty(
    l_curly_token: &SyntaxResult<RSyntaxToken>,
    r_curly_token: &SyntaxResult<RSyntaxToken>,
    node: &RBracedExpressions,
    f: &mut RFormatter,
) -> FormatResult<()> {
    let comments = f.context().comments();
    let has_dangling_comments = comments.has_dangling_comments(node.syntax());

    if has_dangling_comments {
        write!(
            f,
            [
                l_curly_token.format(),
                format_dangling_comments(node.syntax()).with_block_indent(),
                r_curly_token.format()
            ]
        )
    } else {
        write!(
            f,
            [
                l_curly_token.format(),
                hard_line_break(),
                r_curly_token.format()
            ]
        )
    }
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
///
/// > A `{` expression with exactly 1 child, where that 1 child is also a `{` expression
/// > with exactly 1 child.
pub(crate) fn as_curly_curly(node: &RBracedExpressions) -> Option<RCurlyCurly> {
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
/// {{ var }}
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
