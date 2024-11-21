use crate::prelude::*;
use air_r_syntax::AnyRExpression;
use air_r_syntax::OperatorPrecedence;
use air_r_syntax::RBinaryExpression;
use air_r_syntax::RBinaryExpressionFields;
use air_r_syntax::RLanguage;
use air_r_syntax::RSyntaxKind;
use biome_formatter::format_args;
use biome_formatter::write;
use biome_formatter::CstFormatContext;
use biome_rowan::AstNode;
use biome_rowan::SyntaxToken;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRBinaryExpression;
impl FormatNodeRule<RBinaryExpression> for FormatRBinaryExpression {
    fn fmt_fields(&self, node: &RBinaryExpression, f: &mut RFormatter) -> FormatResult<()> {
        let RBinaryExpressionFields {
            left,
            operator,
            right,
        } = node.as_fields();

        let left = left?;
        let operator = operator?;
        let right = right?;

        match operator.kind() {
            // Sticky
            RSyntaxKind::WAT
            | RSyntaxKind::EXPONENTIATE
            | RSyntaxKind::EXPONENTIATE2
            | RSyntaxKind::COLON => fmt_binary_sticky(left, operator, right, f),

            // Assignment
            RSyntaxKind::EQUAL
            | RSyntaxKind::WALRUS
            | RSyntaxKind::ASSIGN
            | RSyntaxKind::ASSIGN_RIGHT
            | RSyntaxKind::SUPER_ASSIGN
            | RSyntaxKind::SUPER_ASSIGN_RIGHT => fmt_binary_assignment(left, operator, right, f),

            // Chainable (pipes, logical, arithmetic)
            kind if is_chainable_binary_operator(kind)  => fmt_binary_chain(left, operator, right, f),

            // Not chainable
            // Formulas (debatable)
            | RSyntaxKind::TILDE
            // Comparison operators
            | RSyntaxKind::GREATER_THAN
            | RSyntaxKind::GREATER_THAN_OR_EQUAL_TO
            | RSyntaxKind::LESS_THAN
            | RSyntaxKind::LESS_THAN_OR_EQUAL_TO
            | RSyntaxKind::EQUAL2
            | RSyntaxKind::NOT_EQUAL=> fmt_binary(left, operator, right, f),

            kind => unreachable!("Unexpected binary operator kind {kind:?}"),
        }
    }
}

/// Sticky expressions whose LHS and RHS stick to the operator (no spaces or line breaks)
fn fmt_binary_sticky(
    left: AnyRExpression,
    operator: SyntaxToken<RLanguage>,
    right: AnyRExpression,
    f: &mut Formatter<RFormatContext>,
) -> FormatResult<()> {
    write!(
        f,
        [group(&format_args![
            left.format(),
            operator.format(),
            right.format()
        ])]
    )
}

/// Assignment expressions keep LHS and RHS on the same line, separated by a single space
fn fmt_binary_assignment(
    left: AnyRExpression,
    operator: SyntaxToken<RLanguage>,
    right: AnyRExpression,
    f: &mut Formatter<RFormatContext>,
) -> FormatResult<()> {
    write!(
        f,
        [group(&format_args![
            left.format(),
            space(),
            operator.format(),
            space(),
            right.format()
        ])]
    )
}

/// Format a binary expression
///
/// These expressions are not chainable, they use a simple
/// `soft_line_break_or_space()` between the `operator` and
/// `right`, and each expression forms its own group.
fn fmt_binary(
    left: AnyRExpression,
    operator: SyntaxToken<RLanguage>,
    right: AnyRExpression,
    f: &mut Formatter<RFormatContext>,
) -> FormatResult<()> {
    write!(
        f,
        [group(&format_args![
            left.format(),
            space(),
            operator.format(),
            indent(&format_args![soft_line_break_or_space(), right.format()])
        ])]
    )
}

struct TailPiece {
    operator: SyntaxToken<RLanguage>,
    right: AnyRExpression,
    enclosing: Option<RBinaryExpression>,
}

/// Format a binary expression chain
///
/// Binary expression chains (like pipe chains or ggplot2 `+` chains) work by turning:
///
/// ```r
/// df |>
///   foo() %>%
///   bar()
/// ```
///
/// Which generates a tree like:
///
/// ```text
///        %>%
///       /  \
///      /    \
///    |>      bar()
///    /\
///   /  \
/// df    foo()
/// ```
///
/// Into a flat sequence of:
///
/// ```text
/// df
/// (|>, foo())
/// (%>%, bar())
///
/// # Or, put differently:
/// left
/// (operator, right) # Tail piece 1
/// (operator, right) # Tail piece 2
/// ```
///
/// which you can then iterate through and print in order. This allows you to `group()`
/// and `indent()` all of the `operator` and `right` nodes into a single block,
/// so if any pipes break, then they all break.
///
/// It accomplishes this by looking down the LHS of the tree, accumulating
/// `operator` and `right` as it goes, stopping at the first non-chainable
/// element (here, the `df`), which becomes the overarching `left`.
fn fmt_binary_chain(
    mut left: AnyRExpression,
    operator: SyntaxToken<RLanguage>,
    right: AnyRExpression,
    f: &mut Formatter<RFormatContext>,
) -> FormatResult<()> {
    let mut parent_operator = operator.kind();

    // For the lead node in a binary chain, comments are handled by the standard
    // formatting of `FormatRBinaryExpression`, so no `encosing` node is tracked.
    let mut tail = vec![TailPiece {
        operator,
        right,
        enclosing: None,
    }];

    // As long as the LHS is another chainable binary expression, continue collecting
    // `operator` and `right` to make one big tail that gets formatted all at once
    // within a single `indent()`, respecting a singular group expansion request.
    while let Some(node) = as_chainable_binary_expression(&left, parent_operator) {
        // It's only possible to suppress the formatting of the whole binary expression formatting OR
        // the formatting of the right hand side value but not of a nested binary expression.
        if f.context().comments().is_suppressed(node.syntax()) {
            tracing::warn!("Can't use a suppression comment partway through a binary chain.");
        }

        let operator = node.operator()?;
        let right = node.right()?;

        parent_operator = operator.kind();

        tail.push(TailPiece {
            operator,
            right,
            enclosing: Some(node.clone()),
        });

        left = node.left()?;
    }

    // Reverse the collected `tail` pieces to generate the correct ordering
    tail.reverse();

    let chain = format_with(|f| {
        // Each `(operator, right)` pair is joined with a single space. Non-breaking!
        // The `operator` must be on the same line as the previous `right` for R to parse
        // it correctly.
        for TailPiece {
            operator,
            right,
            enclosing,
        } in tail.iter()
        {
            if let Some(enclosing) = enclosing {
                // Safety: Non-root nodes in a binary chain can only have trailing comments
                let comments = f.comments();
                let enclosing = enclosing.syntax();

                if comments.has_leading_comments(enclosing) {
                    unreachable!("Non-root nodes in a binary chain can't have leading comments.");
                }
                if comments.has_dangling_comments(enclosing) {
                    unreachable!("Non-root nodes in a binary chain can't have dangling comments.");
                }
            }

            // Respect when the user requests empty lines between the `operator` and
            // `right`. This is common in pipe chains and is usually accompanied by a
            // comment providing details about the upcoming call.
            //
            // ```r
            // df |>
            //
            //   # Some important notes about this call
            //   foo() |>
            //
            //   # Some more important notes
            //   bar()
            // ```
            let user_requested_empty_line = get_lines_before(right.syntax()) > 1;

            write!(
                f,
                [
                    space(),
                    operator.format(),
                    if user_requested_empty_line {
                        empty_line()
                    } else {
                        soft_line_break_or_space()
                    },
                    right.format()
                ]
            )?;

            // Because we take over formatting of nested binary expressions, we also must
            // take over formatting of comments that are directly assigned to those binary
            // expression nodes. Practically the only possible comments are trailing ones
            // like below, and they are inserted after the `right` expression is written.
            // Technically, we write `foo()[comment][space]|>` but because we only allow
            // a space between `foo()` and `|>` with no soft line break, the comment is
            // nicely bumped outside the `|>` as well.
            //
            // ```r
            // df |>
            //   foo() |> # Trailing on the `df |> foo()` binary expression
            //   bar()
            // ```
            if let Some(enclosing) = enclosing {
                write!(f, [format_trailing_comments(enclosing.syntax())])?;
            }
        }

        Ok(())
    });

    write!(
        f,
        [group(&format_args![left.format(), indent(&chain)])
            .should_expand(needs_user_requested_expansion(&tail))]
    )
}

fn as_chainable_binary_expression(
    node: &AnyRExpression,
    parent_operator: RSyntaxKind,
) -> Option<&RBinaryExpression> {
    let node = node.as_r_binary_expression()?;

    let Ok(operator) = node.operator() else {
        // Ignore errors at this point, someone else will propagate them
        return None;
    };

    // Check if the new `operator` is chainable with its `parent_operator`
    if !can_chain(operator.kind(), parent_operator) {
        return None;
    }

    Some(node)
}

fn is_chainable_binary_operator(kind: RSyntaxKind) -> bool {
    // Note that these are all left-associative
    match kind {
        // Pipes
        RSyntaxKind::PIPE
        | RSyntaxKind::SPECIAL

        // Logical operators
        | RSyntaxKind::OR
        | RSyntaxKind::OR2
        | RSyntaxKind::AND
        | RSyntaxKind::AND2

        // Arithmetic operators
        | RSyntaxKind::PLUS
        | RSyntaxKind::MINUS
        | RSyntaxKind::MULTIPLY
        | RSyntaxKind::DIVIDE => true,

        _ => false
    }
}

/// Check if two binary operators can be chained together
///
/// We use the guiding principle that we want the reading order of
/// a chain to match the execution order.
///
/// We can chain binary operators if:
/// - They are a `kind` represented by `is_chainable_binary_operator()`
/// - The child operator has a precedence equal to or greater than the parent
///   operator.
///
/// # Piping into ggplot2
///
/// Consider this very common ggplot2 call:
///
/// ```r
/// df |>
///   ggplot() +
///   geom_bar()
/// ```
///
/// The precedences of `|>` and `+` don't match, but note that the `+` is the
/// parent operator and the `|>` is the child operator. `|>` has a higher
/// precedence than `+` does, which means that we can continue chaining (i.e.
/// reading order will match execution order if we keep chaining because
/// `df |> ggplot()` runs before the `+ geom_bar()`).
///
/// If you try and pipe OUT of a ggplot2 chain, then you get an indent at the
/// end of the chain. The final `|>` ends up on the RHS of `+` in the parse tree,
/// so it isn't considered chainable. The result follows our guiding principle
/// that reading order should follow execution order. The
/// `geom_bar() |> identity()` is executed first, and extra indent helps you see that.
///
/// ```r
/// df |>
///   ggplot() +
///   geom_bar() |>
///     identity()
/// ```
///
/// # On `>=` vs `=` for precedence comparison
///
/// Using `>=` rather than `=` is important. With user requested line breaks
/// like the example below, we do NOT want to respect this line break, as it
/// is not after the first operator of the chain.
///
/// ```r
/// # Input
/// df |> ggplot() +
///   geom_bar() + geom_line()
///
/// # Expected output (ignore line break, flatten)
/// df |> ggplot() + geom_bar() + geom_line()
///
/// # Not
/// df |> ggplot() +
///   geom_bar() +
///   geom_line()
/// ```
///
/// If we had used `=`, that would create two groups (based on where precedence
/// switches) like:
///
/// ```text
/// left: df
/// right: |> ggplot()
/// ```
///
/// ```text
/// left: df |> ggplot()
/// right: + geom_bar() + geom_line()
/// ```
///
/// This means there are 2 spots where the user can request a line break
/// and have it respected, which is confusing and unexpected.
///
/// Similarly, with long chains automatic line breaks can generate confusing
/// breaks when `=` is used due to having multiple groups generated:
///
/// ```r
/// # Input
/// df |> ggplot() + geom_bar() + geom_line() + geom_foo() + geom_bar() + geom_baz()
///
/// # Expected output (every operator breaks)
/// df |>
///   ggplot() +
///   geom_bar() +
///   geom_line() +
///   geom_foo() +
///   geom_bar() +
///   geom_baz()
///
/// # Not
/// df |> ggplot() +
///   geom_bar() +
///   geom_line() +
///   geom_foo() +
///   geom_bar() +
///   geom_baz()
/// ```
fn can_chain(operator: RSyntaxKind, parent_operator: RSyntaxKind) -> bool {
    // We know `parent_operator` is chainable, but is the new one?
    //
    // Note that this ensures we don't have to deal with right-associative
    // operators here because the chainable operators are left-associative.
    if !is_chainable_binary_operator(operator) {
        return false;
    }

    // Safety: `is_chainable_binary_operator()` ensures that `operator` is a
    // binary operator. The algorithm ensures that `parent_operator` is a binary operator.
    let operator_precedence = OperatorPrecedence::try_from_binary_operator(operator).unwrap();
    let parent_operator_precedence =
        OperatorPrecedence::try_from_binary_operator(parent_operator).unwrap();

    operator_precedence >= parent_operator_precedence
}

/// Check if the user has inserted a leading newline before the very first `right`.
/// If so, we respect that and treat it as a request to break ALL of the binary operators
/// in the chain. Note this is a case of irreversible formatting!
///
/// ```r
/// # Fits on one line, but newline before `mutate()` forces ALL pipes to break
///
/// # Input
/// df %>%
///   mutate(x = 1) %>% filter(x == y)
///
/// # Output
/// df %>%
///   mutate(x = 1) %>%
///   filter(x == y)
/// ```
///
/// Note that removing this line break is a request to flatten if possible. By only having
/// this special behavior on the very first pipe, we make it easy to request flattening.
///
/// ```r
/// # Say we start here and want to flatten
/// df %>%
///   mutate(x = 1) %>%
///   filter(x == y)
///
/// # Remove the first line break and run air
/// df %>% mutate(x = 1) %>%
///   filter(x == y)
///
/// # Output
/// df %>% mutate(x = 1) %>% filter(x == y)
/// ```
///
/// ```r
/// # Fits on one line, newline before `%>%` does NOT force all pipes to break
/// # because we are very strict about it coming between the first `%>%` and the
/// # first `right`.
/// #
/// # Note this syntax is only valid inside `(`, `[`, or `[[`. At top level and inside
/// # `{` this is an R syntax error.
///
/// # Input
/// (df
///   %>% mutate(x = 1) %>% filter(x == y))
///
/// # Output
/// (df %>% mutate(x = 1) %>% filter(x == y))
/// ```
fn needs_user_requested_expansion(tail: &[TailPiece]) -> bool {
    // TODO: This should be configurable by an option, since it is a case of
    // irreversible formatting
    tail.first()
        .map_or(false, |piece| piece.right.syntax().has_leading_newline())
}
