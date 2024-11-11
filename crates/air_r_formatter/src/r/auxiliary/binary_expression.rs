use crate::prelude::*;
use air_r_syntax::AnyRExpression;
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

            // Chain
            kind if is_chainable_binary_operator_kind(kind) => {
                fmt_binary_chain(left, operator, right, f)
            }

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

struct TailPiece {
    operator: SyntaxToken<RLanguage>,
    right: AnyRExpression,
    enclosing: Option<RBinaryExpression>,
}

fn fmt_binary_chain(
    mut left: AnyRExpression,
    operator: SyntaxToken<RLanguage>,
    right: AnyRExpression,
    f: &mut Formatter<RFormatContext>,
) -> FormatResult<()> {
    // For the lead node in a binary chain, comments are handled in the standard way as
    // `FormatRBinaryExpression` is formatted, so no `encosing` node is tracked.
    let mut tail = vec![TailPiece {
        operator,
        right,
        enclosing: None,
    }];

    // As long as the LHS is another chainable binary expression, continue collecting
    // `operator` and `right` to make one big tail that gets formatted all at once
    // within a single `indent()`, respecting a singular group expansion request.
    while let Some(node) = as_chainable_binary_expression(&left) {
        // It's only possible to suppress the formatting of the whole binary expression formatting OR
        // the formatting of the right hand side value but not of a nested binary expression.
        f.context()
            .comments()
            .mark_suppression_checked(node.syntax());

        tail.push(TailPiece {
            operator: node.operator()?,
            right: node.right()?,
            enclosing: Some(node.clone()),
        });

        left = node.left()?;
    }

    let chain = format_with(|f| {
        // Reverse the `tail` pieces to generate the correct ordering
        let tail = tail.iter().rev();

        // Each `(operator, right)` pair is joined with a single space. Non-breaking!
        // The `operator` must be on the same line as the previous `right` for R to parse
        // it correctly.
        for TailPiece {
            operator,
            right,
            enclosing,
        } in tail
        {
            if let Some(enclosing) = enclosing {
                // Safety checks
                let comments = f.comments();
                let enclosing = enclosing.syntax();

                if comments.has_leading_comments(enclosing) {
                    unreachable!("Non-root nodes in a binary chain can't have leading comments.");
                }
                if comments.has_dangling_comments(enclosing) {
                    unreachable!("Non-root nodes in a binary chain can't have dangling comments.");
                }
            }

            write!(
                f,
                [
                    space(),
                    operator.format(),
                    soft_line_break_or_space(),
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

fn as_chainable_binary_expression(node: &AnyRExpression) -> Option<&RBinaryExpression> {
    let node = node.as_r_binary_expression()?;

    let Ok(operator) = node.operator() else {
        // Ignore errors at this point, someone else will propagate them
        return None;
    };

    if is_chainable_binary_operator_kind(operator.kind()) {
        Some(node)
    } else {
        None
    }
}

fn is_chainable_binary_operator_kind(kind: RSyntaxKind) -> bool {
    match kind {
        // Pipes
        RSyntaxKind::PIPE
        | RSyntaxKind::SPECIAL

        // Formulas
        | RSyntaxKind::TILDE

        // Logical operators
        | RSyntaxKind::OR
        | RSyntaxKind::OR2
        | RSyntaxKind::AND
        | RSyntaxKind::AND2

        // Comparison operators
        | RSyntaxKind::GREATER_THAN
        | RSyntaxKind::GREATER_THAN_OR_EQUAL_TO
        | RSyntaxKind::LESS_THAN
        | RSyntaxKind::LESS_THAN_OR_EQUAL_TO
        | RSyntaxKind::EQUAL2
        | RSyntaxKind::NOT_EQUAL

        // Arithmetic operators
        | RSyntaxKind::PLUS
        | RSyntaxKind::MINUS
        | RSyntaxKind::MULTIPLY
        | RSyntaxKind::DIVIDE => true,

        _ => false
    }
}

/// Check if the user has inserted a leading newline before any of the `rights`.
/// If so, we respect that and treat it as a request to break ALL of the binary operators
/// in the chain. Note this is a case of irreversible formatting!
///
/// ```r
/// # Fits on one line, but newline before `mutate()` forces ALL pipes to break
/// df %>%
///   mutate(x = 1) %>% filter(x == y)
/// ```
///
/// ```r
/// # Fits on one line, but newline before `filter()` forces ALL pipes to break
/// df %>% mutate(x = 1) %>%
///   filter(x == y)
/// ```
///
/// ```r
/// # Fits on one line, but newline before `%>%` forces ALL pipes to break.
/// # Note this is only valid inside `(`, `[`, or `[[`. At top level and inside
/// # `{` this is an R syntax error.
/// (df %>% mutate(x = 1)
///   %>% filter(x == y))
/// ```
fn needs_user_requested_expansion(tail: &[TailPiece]) -> bool {
    // TODO: This should be configurable by an option, since it is a case of
    // irreversible formatting
    tail.iter().any(|piece| {
        piece.operator.has_leading_newline() || piece.right.syntax().has_leading_newline()
    })
}
