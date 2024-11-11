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
            | RSyntaxKind::COLON => {
                FormatRBinaryStickyExpression::new(left, operator, right).fmt(f)
            }

            // Assignment
            RSyntaxKind::EQUAL
            | RSyntaxKind::WALRUS
            | RSyntaxKind::ASSIGN
            | RSyntaxKind::ASSIGN_RIGHT
            | RSyntaxKind::SUPER_ASSIGN
            | RSyntaxKind::SUPER_ASSIGN_RIGHT => {
                FormatRBinaryAssignmentExpression::new(left, operator, right).fmt(f)
            }

            // Chain
            kind if is_chainable_binary_operator_kind(kind) => {
                FormatRBinaryChainExpression::new(left, operator, right).fmt(f)
            }

            kind => unreachable!("Unexpected binary operator kind {kind:?}"),
        }
    }
}

/// Sticky expressions whose LHS and RHS stick to the operator (no spaces or line breaks)
struct FormatRBinaryStickyExpression {
    left: AnyRExpression,
    operator: SyntaxToken<RLanguage>,
    right: AnyRExpression,
}

impl FormatRBinaryStickyExpression {
    pub(crate) fn new(
        left: AnyRExpression,
        operator: SyntaxToken<RLanguage>,
        right: AnyRExpression,
    ) -> Self {
        Self {
            left,
            operator,
            right,
        }
    }
}

impl Format<RFormatContext> for FormatRBinaryStickyExpression {
    fn fmt(&self, f: &mut Formatter<RFormatContext>) -> FormatResult<()> {
        write!(
            f,
            [group(&format_args![
                self.left.format(),
                self.operator.format(),
                self.right.format()
            ])]
        )
    }
}

/// Assignment expressions keep LHS and RHS on the same line, separated by a single space
struct FormatRBinaryAssignmentExpression {
    left: AnyRExpression,
    operator: SyntaxToken<RLanguage>,
    right: AnyRExpression,
}

impl FormatRBinaryAssignmentExpression {
    pub(crate) fn new(
        left: AnyRExpression,
        operator: SyntaxToken<RLanguage>,
        right: AnyRExpression,
    ) -> Self {
        Self {
            left,
            operator,
            right,
        }
    }
}

impl Format<RFormatContext> for FormatRBinaryAssignmentExpression {
    fn fmt(&self, f: &mut Formatter<RFormatContext>) -> FormatResult<()> {
        write!(
            f,
            [group(&format_args![
                self.left.format(),
                space(),
                self.operator.format(),
                space(),
                self.right.format()
            ])]
        )
    }
}

struct FormatRBinaryChainExpression {
    left: AnyRExpression,
    operator: SyntaxToken<RLanguage>,
    right: AnyRExpression,
}

impl FormatRBinaryChainExpression {
    pub(crate) fn new(
        left: AnyRExpression,
        operator: SyntaxToken<RLanguage>,
        right: AnyRExpression,
    ) -> Self {
        Self {
            left,
            operator,
            right,
        }
    }
}

impl Format<RFormatContext> for FormatRBinaryChainExpression {
    fn fmt(&self, f: &mut Formatter<RFormatContext>) -> FormatResult<()> {
        let mut operators = Vec::new();
        let mut rights = Vec::new();

        let mut left = self.left.clone();
        let mut operator = self.operator.clone();
        let mut right = self.right.clone();

        loop {
            rights.push(right);
            operators.push(operator);

            if let Some(node) = as_chainable_binary_expression(&left) {
                // It's only possible to suppress the formatting of the whole binary expression formatting OR
                // the formatting of the right hand side value but not of a nested binary expression.
                f.context()
                    .comments()
                    .mark_suppression_checked(node.syntax());

                operator = node.operator()?;
                right = node.right()?;
                left = node.left()?;
            } else {
                break;
            }
        }

        let user_requested_expansion = rights
            .iter()
            .any(|node| node.syntax().has_leading_newline());

        let chain = format_with(|f| {
            let mut joiner = f.join_with(space());
            let iter = operators.iter().rev().zip(rights.iter().rev());

            for (operator, right) in iter {
                joiner.entry(&format_with(|f| {
                    write!(
                        f,
                        [
                            &operator.format(),
                            soft_line_break_or_space(),
                            &right.format(),
                        ]
                    )
                }));
            }

            joiner.finish()
        });

        write!(
            f,
            [group(&format_args![left.format(), space(), indent(&chain)])
                .should_expand(user_requested_expansion)]
        )
    }
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
