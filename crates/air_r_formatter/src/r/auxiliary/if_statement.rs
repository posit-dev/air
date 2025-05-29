use crate::prelude::*;
use crate::r::auxiliary::else_clause::FormatRElseClauseOptions;
use air_r_syntax::AnyRExpression;
use air_r_syntax::RExpressionList;
use air_r_syntax::RIfStatement;
use air_r_syntax::RIfStatementFields;
use air_r_syntax::RRoot;
use air_r_syntax::RSyntaxNode;
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
/// We must decide if the if statement is simple enough to potentially stay on a single
/// line:
/// - It must be in a [SyntaxPosition::Value] position
/// - Any existing newline forces multiline
/// - A braced `consequence` or `alternative` forces multiline
/// - Nested if statements force multiline
///
/// That ends up resulting in the following scenarios:
///
/// ## The if statement is not in a [SyntaxPosition::Value] position
///
/// ```r
/// # Before (top level, considered an `Effect`)
/// if (cond) consequence else alternative
///
/// # After
/// if (cond) {
///   consequence
/// } else {
///   alternative
/// }
/// ```
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
    if node.syntax().position() != SyntaxPosition::Value {
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

// NOTE: If this ends up being useful for more than if statements, we can move it to
// `air_r_syntax` as `syntax_node_ext.rs`
trait SyntaxPositionExt {
    /// Compute the [SyntaxPosition] of a node
    ///
    /// [SyntaxPosition] is an attempt to take a very simple syntax based approach to
    /// determine whether a node is in a _value_ or an _effect_ position.
    ///
    /// - [SyntaxPosition::Value]s are assigned, are used as inputs to functions, and
    ///   are returned from `RBracedExpression` scopes.
    ///
    /// - [SyntaxPosition::Effect]s are statements that are typically used for their side
    ///   effects, such as calling `cat()`, or performing an assignment like `x <- 5`.
    ///
    /// An [SyntaxPosition::Effect] is either:
    ///
    /// - A direct child of `RRoot`
    /// - A direct child of `RBracedExpression`, with the exception of the last child,
    ///   which is considered a [SyntaxPosition::Value], as it is the returned value
    ///   of the braced expression
    ///
    /// Everything else is considered a [SyntaxPosition::Value].
    ///
    /// ## Examples
    ///
    /// At top level
    ///
    /// ```r
    /// 1 + 1 # Effect
    /// ```
    ///
    /// Direct children of `{}`
    ///
    /// ```r
    /// fn <- function() {
    ///   fn() # Effect
    ///   cat("hi") # Effect
    ///   fn() # Value, last child of `{}`
    /// }
    /// ```
    ///
    /// Additional examples of [SyntaxPosition::Value] positions
    ///
    /// ```r
    /// x <- <value>
    /// fun(<value>)
    /// function(x = <value>)
    /// map(xs, function() <value>)
    /// ```
    fn position(&self) -> SyntaxPosition;
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum SyntaxPosition {
    Value,
    Effect,
}

impl SyntaxPositionExt for RSyntaxNode {
    fn position(&self) -> SyntaxPosition {
        let Some(expressions) = self.parent().and_then(RExpressionList::cast) else {
            // If our direct parent isn't an `RExpressionList`, we are definitely a `Value`
            return SyntaxPosition::Value;
        };

        // Direct child of `RRoot`, this is an `Effect`
        if expressions.parent::<RRoot>().is_some() {
            return SyntaxPosition::Effect;
        }

        // Otherwise, direct child of `RBracedExpressions`. If this is the last
        // expression, it is a `Value` (the value returned from a scope), otherwise it is
        // an `Effect`.
        let Some(last) = expressions.last() else {
            // Would be unexpected since we just checked that `self` is a child of this,
            // but we'd rather not crash if we are wrong
            return SyntaxPosition::Value;
        };

        if self == last.syntax() {
            SyntaxPosition::Value
        } else {
            SyntaxPosition::Effect
        }
    }
}
