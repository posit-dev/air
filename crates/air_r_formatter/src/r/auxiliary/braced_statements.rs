use crate::comments::RComments;
use crate::prelude::*;
use air_r_syntax::AnyRExpression;
use air_r_syntax::AnyRStatement;
use air_r_syntax::RArgument;
use air_r_syntax::RBracedStatements;
use air_r_syntax::RBracedStatementsFields;
use air_r_syntax::RExpressionStatement;
use air_r_syntax::RStatementList;
use air_r_syntax::RSyntaxToken;
use biome_formatter::format_args;
use biome_formatter::write;
use biome_rowan::SyntaxResult;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRBracedStatements;
impl FormatNodeRule<RBracedStatements> for FormatRBracedStatements {
    fn fmt_fields(&self, node: &RBracedStatements, f: &mut RFormatter) -> FormatResult<()> {
        let RBracedStatementsFields {
            l_curly_token,
            statements,
            r_curly_token,
        } = node.as_fields();

        match braced_statements_variant(node, f.comments()) {
            // Examples:
            //
            // ```r
            // {}
            // {;}
            // function() {}
            // while(waiting()) {}
            // for (x in xs) {}
            // switch(on, a = {}, b = 2)
            // ```
            BracedStatementsVariant::Empty => {
                write!(
                    f,
                    [
                        l_curly_token.format(),
                        format_empty_statements(&statements),
                        r_curly_token.format()
                    ]
                )
            }
            // Examples:
            //
            // ```r
            // {
            //   # comment
            // }
            //
            // {
            //   # comment
            //   ; # comment
            //   # comment
            // }
            // ```
            BracedStatementsVariant::EmptyWithDanglingComments => {
                write!(
                    f,
                    [
                        l_curly_token.format(),
                        format_empty_statements(&statements),
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
            BracedStatementsVariant::CurlyCurly(node) => fmt_curly_curly(&node, f),
            // Examples:
            //
            // ```r
            // {
            //   expr
            //   expr
            // }
            // ```
            BracedStatementsVariant::NotEmpty => {
                write!(
                    f,
                    [
                        l_curly_token.format(),
                        block_indent(&statements.format()),
                        r_curly_token.format()
                    ]
                )
            }
        }
    }

    fn fmt_dangling_comments(&self, _: &RBracedStatements, _: &mut RFormatter) -> FormatResult<()> {
        // Formatted inside of `fmt_fields`
        // Only possible with `BracedStatementsVariant::EmptyWithDanglingComments`
        Ok(())
    }
}

pub(crate) enum BracedStatementsVariant {
    Empty,
    EmptyWithDanglingComments,
    CurlyCurly(RCurlyCurly),
    NotEmpty,
}

/// Categorize [RBracedStatements] into its appropriate variant
///
/// Used when formatting a [RBracedStatements], and in `call_arguments.rs` when
/// determining if a [RBracedStatements] is groupable or not.
pub(crate) fn braced_statements_variant(
    node: &RBracedStatements,
    comments: &RComments,
) -> BracedStatementsVariant {
    // Note that `REmptyStatement` counts as empty. And if we only have
    // `REmptyStatement`s, then remember that the comment placer will have
    // preemptively moved any comments that might have been on the lone `;`s as
    // dangling comments on the `RBracedStatements` `node` instead, which
    // `BracedStatementsVariant::EmptyWithDanglingComments` will still cover.
    let is_empty = !node.statements().iter().any(|statement| match statement {
        AnyRStatement::REmptyStatement(_) => false,
        AnyRStatement::RExpressionStatement(_) => true,
    });

    if is_empty {
        if comments.has_dangling_comments(node.syntax()) {
            return BracedStatementsVariant::EmptyWithDanglingComments;
        } else {
            return BracedStatementsVariant::Empty;
        }
    }

    if let Some(node) = as_curly_curly(node) {
        return BracedStatementsVariant::CurlyCurly(node);
    }

    BracedStatementsVariant::NotEmpty
}

pub(crate) struct RCurlyCurly {
    outer_l_curly_token: SyntaxResult<RSyntaxToken>,
    inner_l_curly_token: SyntaxResult<RSyntaxToken>,

    /// Interior statement list of the curly-curly expression
    ///
    /// Note that this always contains a single statement, but to simplify comment
    /// formatting we return the entire `RStatementList` to call `.format()` on.
    statements: RStatementList,

    inner_r_curly_token: SyntaxResult<RSyntaxToken>,
    outer_r_curly_token: SyntaxResult<RSyntaxToken>,

    /// Statement wrapping the interior `{` node of the curly-curly,
    /// and the interior `{` node itself
    ///
    /// Needed for marking both of the nodes as suppression-checked and for manually
    /// formatting leading and trailing comments of `statement_inner`
    statement_inner: RExpressionStatement,
    braced_statements_inner: RBracedStatements,
}

/// Convert `RBracedStatements` into `RCurlyCurly`
///
/// Returns:
/// - `Some(RCurlyCurly)` if this braced expression looks like a curly-curly
/// - `None` otherwise
///
/// The spec for curly-curly that we are targeting is:
/// - An outer `{` expression with exactly 1 child, an inner `{`.
/// - An inner `{` expression with exactly 1 child, a symbol.
/// - An ancestor of the outer `{` must be an argument node.
///
/// We allow semicolons after `}` to still count as curly-curly for idempotence
/// reasons, i.e. `{{ x };}` counts, and the `;` is removed like all other
/// semicolon handling in Air.
fn as_curly_curly(node: &RBracedStatements) -> Option<RCurlyCurly> {
    let RBracedStatementsFields {
        l_curly_token: outer_l_curly_token,
        statements: outer_statements,
        r_curly_token: outer_r_curly_token,
    } = node.as_fields();

    if outer_statements.len() != 1 {
        return None;
    }

    // Unwrap: Length check ensures there is exactly 1 child
    let statement_inner = outer_statements.first().unwrap();

    let statement_inner = match statement_inner {
        AnyRStatement::REmptyStatement(_) => {
            // Lone `;` is obviously not curly-curly
            return None;
        }
        AnyRStatement::RExpressionStatement(statement_inner) => statement_inner,
    };

    // Check that the child of the outer `{` is another `{`
    let braced_statements_inner = match statement_inner.expression().ok()? {
        AnyRExpression::RBracedStatements(braced_statements_inner) => braced_statements_inner,
        _ => return None,
    };

    let RBracedStatementsFields {
        l_curly_token: inner_l_curly_token,
        statements,
        r_curly_token: inner_r_curly_token,
    } = braced_statements_inner.as_fields();

    // Check that inner child `{` itself only has 1 child, i.e. the `expr` in `{{ expr }}`
    if statements.len() != 1 {
        return None;
    }

    // Unwrap: Length check ensures there is exactly 1 child
    let statement = statements.first().unwrap();

    let statement = match statement {
        AnyRStatement::REmptyStatement(_) => {
            // Lone `;` is obviously not curly-curly
            return None;
        }
        AnyRStatement::RExpressionStatement(statement) => statement,
    };

    // Check that the actual expression inside the `{{` is a simple identifier
    if !matches!(statement.expression(), Ok(AnyRExpression::RIdentifier(_))) {
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
        statements,
        inner_r_curly_token,
        outer_r_curly_token,
        statement_inner,
        braced_statements_inner,
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
        statements,
        inner_r_curly_token,
        outer_r_curly_token,
        statement_inner,
        braced_statements_inner,
    } = node;

    let comments = f.comments();

    // For example, `{{ x }; }`. We remove this semicolon, as we do with all
    // semicolons in Air.
    let semicolon_token = statement_inner.semicolon_token();

    let statement_inner = statement_inner.syntax();
    let braced_statements_inner = braced_statements_inner.syntax();

    // It's only possible to suppress the formatting of the outer curlies of curly-curly,
    // so we don't need to branch based on `comments.has_skip_directive()`, but we do need
    // to mark the nodes as suppression checked
    comments.mark_suppression_checked(statement_inner);
    comments.mark_suppression_checked(braced_statements_inner);

    // It's impossible to have ANY comments on the inner `{` node.
    // - Leading and trailing comments attach to the outermost node,
    //   `statement_inner`.
    // - Dangling comments are only possible when `braced_statements_inner` is
    //   empty, and we would have bailed earlier because it doesn't "look" like
    //   curly-curly then.
    if comments.has_comments(braced_statements_inner) {
        panic!("Inner `{{` node of curly-curly can't have comments.");
    }

    // It's impossible to have dangling comments on the inner `RExpressionStatement`
    // node, since it always has an expression
    if comments.has_dangling_comments(statement_inner) {
        panic!("Inner statement of curly-curly can't have dangling comments.");
    }

    // We are sure we have curly-curly!
    //
    // We are going to format it all in one go, keeping in mind:
    // - We must `group()` it so the `soft_space_or_block_indent()` stays soft when possible
    // - Remember that comments can only be attached to nodes (not tokens), so we just
    //   have to worry about manually handling comments on `statement_inner`, since
    //   it is the only node with comments that we skip calling `format()` on.
    // - We push comments between the `{ {` as leading on the first `{`
    // - We push comments between the `} }` as trailing on the second `}`
    // - We remove the inner `;` of `}; }`, for idempotence and to match Air generally
    write!(
        f,
        [group(&format_args![
            format_leading_comments(statement_inner),
            outer_l_curly_token.format(),
            inner_l_curly_token.format(),
            soft_space_or_block_indent(&statements.format()),
            inner_r_curly_token.format(),
            semicolon_token.as_ref().map(format_removed),
            outer_r_curly_token.format(),
            format_trailing_comments(statement_inner),
        ])]
    )
}

/// Removes the `;` of each `REmptyStatement` in an otherwise empty `{}`
fn format_empty_statements(statements: &RStatementList) -> impl Format<RFormatContext> + '_ {
    format_with(move |f| {
        for statement in statements.iter() {
            match statement {
                AnyRStatement::REmptyStatement(statement) => {
                    // `FormatREmptyStatement` debug asserts that this won't
                    // have any comments on it (the comment placer has already
                    // moved them elsewhere) and then does `format_removed()` on
                    // the `semicolon_token`
                    write!(f, [statement.format()])?
                }
                AnyRStatement::RExpressionStatement(_) => {
                    unreachable!("An empty `{{}}` can't hold an `RExpressionStatement`.")
                }
            }
        }
        Ok(())
    })
}
