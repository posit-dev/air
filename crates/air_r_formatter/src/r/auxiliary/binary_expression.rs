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
}

fn fmt_binary_chain(
    mut left: AnyRExpression,
    operator: SyntaxToken<RLanguage>,
    right: AnyRExpression,
    f: &mut Formatter<RFormatContext>,
) -> FormatResult<()> {
    let mut tail = vec![TailPiece { operator, right }];

    // As long as the LHS is another chainable binary expression, continue collecting
    // `operators` and `rights` to make one big tail that gets formatted all at once
    // within a single `indent()` and respecting the same group expansion request.
    while let Some(node) = as_chainable_binary_expression(&left) {
        // It's only possible to suppress the formatting of the whole binary expression formatting OR
        // the formatting of the right hand side value but not of a nested binary expression.
        f.context()
            .comments()
            .mark_suppression_checked(node.syntax());

        tail.push(TailPiece {
            operator: node.operator()?,
            right: node.right()?,
        });

        left = node.left()?;
    }

    let chain = format_with(|f| {
        // Reverse the `tail` pieces to generate the correct ordering
        let tail = tail.iter().rev();

        // Each `(operator, right)` pair is joined with a single space. Non-breaking!
        // The `operator` must be on the same line as the previous `right` for R to parse
        // it correctly.
        let mut joiner = f.join_with(space());

        for TailPiece { operator, right } in tail {
            joiner.entry(&format_args![
                operator.format(),
                soft_line_break_or_space(),
                right.format()
            ]);
        }

        joiner.finish()
    });

    write!(
        f,
        [group(&format_args![left.format(), space(), indent(&chain)])
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
/// Note that we don't need to check for leading newlines before the `operator`, because
/// it isn't valid R syntax to do that.
fn needs_user_requested_expansion(tail: &[TailPiece]) -> bool {
    // TODO: This should be configurable by an option, since it is a case of
    // irreversible formatting
    tail.iter()
        .any(|piece| piece.right.syntax().has_leading_newline())
}
