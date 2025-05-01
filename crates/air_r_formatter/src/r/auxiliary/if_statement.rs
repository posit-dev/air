use crate::prelude::*;
use crate::r::auxiliary::else_clause::FormatRElseClauseOptions;
use air_r_syntax::AnyRExpression;
use air_r_syntax::RIfStatement;
use air_r_syntax::RIfStatementFields;
use biome_formatter::format_args;
use biome_formatter::write;
use biome_formatter::FormatRuleWithOptions;
use biome_rowan::SyntaxResult;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRIfStatement {
    pub(crate) braced_expressions: Option<BracedExpressions>,
}

#[derive(Debug)]
pub(crate) struct FormatRIfStatementOptions {
    pub(crate) braced_expressions: Option<BracedExpressions>,
}

impl FormatRuleWithOptions<RIfStatement> for FormatRIfStatement {
    type Options = FormatRIfStatementOptions;

    fn with_options(mut self, options: Self::Options) -> Self {
        self.braced_expressions = options.braced_expressions;
        self
    }
}

impl FormatNodeRule<RIfStatement> for FormatRIfStatement {
    fn fmt_fields(&self, node: &RIfStatement, f: &mut RFormatter) -> FormatResult<()> {
        let RIfStatementFields {
            if_token,
            l_paren_token,
            condition,
            r_paren_token,
            consequence,
            else_clause,
        } = node.as_fields();

        // Compute `braced_expressions` if we are on a top level if statement,
        // otherwise use the passed through `braced_expressions`, like if we are on the
        // 2nd `if` statement of `if (condition) 1 else if (condition) 2`.
        let braced_expressions = match self.braced_expressions {
            Some(braced_expressions) => braced_expressions,
            None => compute_braced_expressions(node)?,
        };

        // It's important that the `else_clause` end up in the same `group()` as the rest
        // of the if statement, so we `format_once()` it to be evaluated once we are
        // inside the `group()`
        let else_clause = format_once(|f| {
            if let Some(else_clause) = else_clause {
                write!(
                    f,
                    [
                        space(),
                        else_clause
                            .format()
                            .with_options(FormatRElseClauseOptions { braced_expressions })
                    ]
                )?;
            }

            Ok(())
        });

        write!(
            f,
            [group(&format_args![
                if_token.format(),
                space(),
                l_paren_token.format(),
                group(&soft_block_indent(&condition.format())),
                r_paren_token.format(),
                space(),
                FormatIfBody::new_consequence(&consequence?, braced_expressions),
                else_clause
            ])]
        )
    }
}

/// Determine if braced expressions should be forced within this if statement
///
/// We force braced expressions using the following rules:
/// - A single line if statement must never have braces.
/// - A multiline if statement must have braced expressions.
///
/// That ends up resulting in the following scenarios:
///
/// ## The `consequence` or `alternative` has a leading newline
///
/// ```r
/// # Before
/// if (cond)
///   consequence
///
/// if (cond) consequence else
///   alternative
///
/// # After
/// if (cond) {
///   consequence
/// }
///
/// if (cond) {
///   consequence
/// } else {
///   alternative
/// }
/// ```
///
/// ## The `else` token has a leading newline
///
/// ```r
/// # Before
/// {
///   if (condition) 1
///   else 2
/// }
///
/// # After
/// {
///   if (condition) {
///     1
///   } else {
///     2
///   }
/// }
/// ```
///
/// ## The `consequence` or `alternative` has braced expressions
///
/// ```r
/// # Before
/// if (cond) { consequence } else alternative
/// if (cond) consequence else { alternative }
///
/// # After
/// if (cond) {
///   consequence
/// } else {
///   alternative
/// }
/// ```
///
/// ## The `consequence` or `alternative` is another if statement
///
/// ```r
/// # Before
/// if (cond) if (cond) consequence
/// #         |----consequence----|
///
/// # After
/// if (cond) {
///   if (cond) {
///     consequence
///   }
/// }
///
/// # Before
/// if (cond) consequence1 else if (cond) consequence2
/// #                           |-----alternative----|
///
/// # After
/// if (cond) {
///   consequence1
/// } else if (cond) {
///   consequence2
/// }
/// ```
fn compute_braced_expressions(node: &RIfStatement) -> SyntaxResult<BracedExpressions> {
    let consequence = node.consequence()?;

    if consequence.syntax().has_leading_newline() {
        return Ok(BracedExpressions::Force);
    }
    if matches!(consequence, AnyRExpression::RBracedExpressions(_)) {
        return Ok(BracedExpressions::Force);
    }
    if matches!(consequence, AnyRExpression::RIfStatement(_)) {
        // Disallow `if (condition) if (condition) 1` as that is too complex.
        // Also shortcircuits recursion nicely.
        return Ok(BracedExpressions::Force);
    }

    if let Some(else_clause) = node.else_clause() {
        let else_token = else_clause.else_token()?;

        if else_token.has_leading_newline() {
            return Ok(BracedExpressions::Force);
        }

        let alternative = else_clause.alternative()?;

        if alternative.syntax().has_leading_newline() {
            return Ok(BracedExpressions::Force);
        }
        if matches!(alternative, AnyRExpression::RBracedExpressions(_)) {
            return Ok(BracedExpressions::Force);
        }
        if matches!(alternative, AnyRExpression::RIfStatement(_)) {
            // Disallow `if (condition) 1 else if (condition) 2 else 3` as that is too complex.
            // Also shortcircuits recursion nicely.
            return Ok(BracedExpressions::Force);
        }
    };

    Ok(BracedExpressions::IfGroupBreaks)
}

pub(crate) struct FormatIfBody<'a> {
    node: &'a AnyRExpression,
    braced_expressions: BracedExpressions,
    if_body_kind: IfBodyKind,
}

impl<'a> FormatIfBody<'a> {
    pub(crate) fn new_consequence(
        node: &'a AnyRExpression,
        braced_expressions: BracedExpressions,
    ) -> Self {
        Self {
            node,
            braced_expressions,
            if_body_kind: IfBodyKind::Consequence,
        }
    }

    pub(crate) fn new_alternative(
        node: &'a AnyRExpression,
        braced_expressions: BracedExpressions,
    ) -> Self {
        Self {
            node,
            braced_expressions,
            if_body_kind: IfBodyKind::Alternative,
        }
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub(crate) enum BracedExpressions {
    Force,
    #[default]
    IfGroupBreaks,
}

pub(crate) enum IfBodyKind {
    /// The body attached to `if (condition)`
    Consequence,
    /// The body attached to `else`
    Alternative,
}

impl Format<RFormatContext> for FormatIfBody<'_> {
    fn fmt(&self, f: &mut Formatter<RFormatContext>) -> FormatResult<()> {
        match self.node {
            // Body already has braces, just format it
            AnyRExpression::RBracedExpressions(node) => {
                write!(f, [node.format()])
            }
            // Body is a `consequence` that is another if statement, pass through
            // pre computed `braced_expressions` (will be `BracedExpressions::Force`)
            // and force braces around the body
            //
            // ```r
            // if (TRUE) if (TRUE) 1
            // #        |-----------|
            // ```
            AnyRExpression::RIfStatement(node)
                if matches!(self.if_body_kind, IfBodyKind::Consequence) =>
            {
                debug_assert!(matches!(self.braced_expressions, BracedExpressions::Force));
                write!(
                    f,
                    [
                        text("{"),
                        block_indent(&format_args![&node.format().with_options(
                            FormatRIfStatementOptions {
                                braced_expressions: Some(self.braced_expressions)
                            }
                        )]),
                        text("}")
                    ]
                )
            }
            // Body is an `alternative` that is another if statement, pass through
            // pre computed `braced_expressions` (will be `BracedExpressions::Force`)
            //
            // ```r
            // if (TRUE) 1 else if (TRUE) 2
            // #               |-----------|
            // ```
            AnyRExpression::RIfStatement(node)
                if matches!(self.if_body_kind, IfBodyKind::Alternative) =>
            {
                debug_assert!(matches!(self.braced_expressions, BracedExpressions::Force));
                write!(
                    f,
                    [node.format().with_options(FormatRIfStatementOptions {
                        braced_expressions: Some(self.braced_expressions)
                    })]
                )
            }
            // Body does not have braces yet
            node => match self.braced_expressions {
                BracedExpressions::Force => write!(
                    f,
                    [
                        text("{"),
                        block_indent(&format_args![&node.format()]),
                        text("}")
                    ]
                ),
                BracedExpressions::IfGroupBreaks => write!(
                    f,
                    [
                        if_group_breaks(&text("{")),
                        soft_block_indent(&format_args![&node.format()]),
                        if_group_breaks(&text("}"))
                    ]
                ),
            },
        }
    }
}
