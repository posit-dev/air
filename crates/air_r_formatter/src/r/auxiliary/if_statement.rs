use crate::prelude::*;
use crate::r::auxiliary::else_clause::FormatRElseClauseOptions;
use air_r_syntax::AnyRExpression;
use air_r_syntax::RArgument;
use air_r_syntax::RBinaryExpression;
use air_r_syntax::RIfStatement;
use air_r_syntax::RIfStatementFields;
use air_r_syntax::RParameterDefault;
use air_r_syntax::RSyntaxKind;
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
/// Single line if statements are only allowed in a few specific contexts:
/// - The right hand side of a `=`, `<-`, or `<<-` assignment
/// - A function call argument
/// - A function signature parameter
///
/// If we are within one of those contexts, we must also decide if the if statement
/// is simple enough to stay on a single line.
/// - Any existing newline forces multiline
/// - A braced `consequence` or `alternative` forces multiline
/// - Nested if statements force multiline
///
/// That ends up resulting in the following scenarios:
///
/// ## The `consequence` or `alternative` has a leading newline
///
/// ```r
/// # Before
/// x <- if (cond)
///   consequence
///
/// x <- if (cond) consequence else
///   alternative
///
/// # After
/// x <- if (cond) {
///   consequence
/// }
///
/// x <- if (cond) {
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
///   x <- if (condition) 1
///   else 2
/// }
///
/// # After
/// {
///   x <- if (condition) {
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
/// x <- if (cond) { consequence } else alternative
/// x <- if (cond) consequence else { alternative }
///
/// # After
/// x <- if (cond) {
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
/// x <- if (cond) if (cond) consequence
/// #              |----consequence----|
///
/// # After
/// # Note that we don't `Force` the inner if to be braced, because short
/// # ifs would be allowed there if the user wants to write it like that to begin with.
/// x <- if (cond) {
///   if (cond) consequence
/// }
///
/// # Before
/// x <- if (cond) consequence1 else if (cond) consequence2
/// #                                |-----alternative----|
///
/// # After
/// x <- if (cond) {
///   consequence1
/// } else if (cond) {
///   consequence2
/// }
/// ```
fn compute_braced_expressions(node: &RIfStatement) -> SyntaxResult<BracedExpressions> {
    if !in_allowed_one_line_if_statement_context(node)? {
        return Ok(BracedExpressions::Force);
    }

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
        // Notably we don't pass through `Force` to the inner if statement as well,
        // it gets to compute its own `BracedExpressions` value.
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

fn in_allowed_one_line_if_statement_context(node: &RIfStatement) -> SyntaxResult<bool> {
    // ```r
    // # Allowed
    // x = if (a) else b
    // x <- if (a) else b
    // x <<- if (a) else b
    // ```
    if let Some(parent) = node.parent::<RBinaryExpression>() {
        // Check for `=`, `<-`, or `<<-` as the operator
        if !matches!(
            parent.operator()?.kind(),
            RSyntaxKind::EQUAL | RSyntaxKind::ASSIGN | RSyntaxKind::SUPER_ASSIGN
        ) {
            return Ok(false);
        }

        // Ensure we were the right hand side
        if parent.right()?.syntax() != node.syntax() {
            return Ok(false);
        }

        return Ok(true);
    };

    // ```r
    // # Allowed (unnamed)
    // fn(if (a) 1)
    // fn(if (a) 1 else 2)
    //
    // # Allowed (named)
    // fn(x = if (a) 1 else 2)
    //
    // # Allowed here, rejected later for being too complex
    // fn(if (a) 1 else if (b) 2)
    // ```
    if node.parent::<RArgument>().is_some() {
        return Ok(true);
    };

    // ```r
    // # Allowed (parameter with default)
    // function(x = if (a) 1) {}
    // function(x = if (a) 1 else 2) {}
    //
    // # Allowed here, rejected later for being too complex
    // function(x = if (a) 1 else if (b) 2) {}
    // ```
    if node.parent::<RParameterDefault>().is_some() {
        return Ok(true);
    }

    // Otherwise one line if statements are not allowed
    Ok(false)
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
