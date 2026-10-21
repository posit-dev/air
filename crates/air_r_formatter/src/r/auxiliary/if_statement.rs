use crate::prelude::*;
use crate::r::auxiliary::else_clause::FormatRElseClauseOptions;
use air_r_syntax::AnyRExpression;
use air_r_syntax::AnyRStatement;
use air_r_syntax::RExpressionStatement;
use air_r_syntax::RIfStatement;
use air_r_syntax::RIfStatementFields;
use air_r_syntax::RRoot;
use air_r_syntax::RStatementList;
use air_r_syntax::RSyntaxNode;
use biome_formatter::FormatRuleWithOptions;
use biome_formatter::format_args;
use biome_formatter::write;
use biome_rowan::SyntaxResult;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRIfStatement {
    pub(crate) braced_statements: Option<BracedStatements>,
}

#[derive(Debug)]
pub(crate) struct FormatRIfStatementOptions {
    pub(crate) braced_statements: Option<BracedStatements>,
}

impl FormatRuleWithOptions<RIfStatement> for FormatRIfStatement {
    type Options = FormatRIfStatementOptions;

    fn with_options(mut self, options: Self::Options) -> Self {
        self.braced_statements = options.braced_statements;
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

        // Compute `braced_statements` if we are on a top level if statement,
        // otherwise use the passed through `braced_statements`, like if we are on the
        // 2nd `if` statement of `if (condition) 1 else if (condition) 2`.
        let braced_statements = match self.braced_statements {
            Some(braced_statements) => braced_statements,
            None => compute_braced_statements(node)?,
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
                            .with_options(FormatRElseClauseOptions { braced_statements })
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
                FormatIfBody::new_consequence(&consequence?, braced_statements),
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
fn compute_braced_statements(node: &RIfStatement) -> SyntaxResult<BracedStatements> {
    if node.syntax().position() != SyntaxPosition::Value {
        return Ok(BracedStatements::Force);
    }

    let consequence = node.consequence()?;

    if consequence.syntax().has_leading_newline() {
        return Ok(BracedStatements::Force);
    }
    if matches!(consequence, AnyRExpression::RBracedStatements(_)) {
        return Ok(BracedStatements::Force);
    }
    if matches!(consequence, AnyRExpression::RIfStatement(_)) {
        // Disallow `if (condition) if (condition) 1` as that is too complex.
        // Also shortcircuits recursion nicely.
        // Notably we don't pass through `Force` to the inner if statement as well,
        // it gets to compute its own `BracedStatements` value.
        return Ok(BracedStatements::Force);
    }

    if let Some(else_clause) = node.else_clause() {
        let else_token = else_clause.else_token()?;

        if else_token.has_leading_newline() {
            return Ok(BracedStatements::Force);
        }

        let alternative = else_clause.alternative()?;

        if alternative.syntax().has_leading_newline() {
            return Ok(BracedStatements::Force);
        }
        if matches!(alternative, AnyRExpression::RBracedStatements(_)) {
            return Ok(BracedStatements::Force);
        }
        if matches!(alternative, AnyRExpression::RIfStatement(_)) {
            // Disallow `if (condition) 1 else if (condition) 2 else 3` as that is too complex.
            // Also shortcircuits recursion nicely.
            return Ok(BracedStatements::Force);
        }
    };

    Ok(BracedStatements::IfGroupBreaks)
}

pub(crate) struct FormatIfBody<'a> {
    node: &'a AnyRExpression,
    braced_statements: BracedStatements,
    if_body_kind: IfBodyKind,
}

impl<'a> FormatIfBody<'a> {
    pub(crate) fn new_consequence(
        node: &'a AnyRExpression,
        braced_statements: BracedStatements,
    ) -> Self {
        Self {
            node,
            braced_statements,
            if_body_kind: IfBodyKind::Consequence,
        }
    }

    pub(crate) fn new_alternative(
        node: &'a AnyRExpression,
        braced_statements: BracedStatements,
    ) -> Self {
        Self {
            node,
            braced_statements,
            if_body_kind: IfBodyKind::Alternative,
        }
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub(crate) enum BracedStatements {
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
            AnyRExpression::RBracedStatements(node) => {
                write!(f, [node.format()])
            }
            // Body is an `alternative` that is another if statement, pass through
            // pre computed `braced_statements` (will be `BracedStatements::Force`)
            //
            // ```r
            // if (TRUE) 1 else if (TRUE) 2
            // #               |-----------|
            // ```
            AnyRExpression::RIfStatement(node)
                if matches!(self.if_body_kind, IfBodyKind::Alternative) =>
            {
                debug_assert!(matches!(self.braced_statements, BracedStatements::Force));
                write!(
                    f,
                    [node.format().with_options(FormatRIfStatementOptions {
                        braced_statements: Some(self.braced_statements)
                    })]
                )
            }
            // Body does not have braces yet
            node => match self.braced_statements {
                BracedStatements::Force => write!(
                    f,
                    [
                        text("{"),
                        block_indent(&format_args![&node.format()]),
                        text("}")
                    ]
                ),
                BracedStatements::IfGroupBreaks => write!(
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
    ///   are returned from `RBracedStatements` scopes.
    ///
    /// - [SyntaxPosition::Effect]s are statements that are typically used for their side
    ///   effects, such as calling `cat()`, or performing an assignment like `x <- 5`.
    ///
    /// An [SyntaxPosition::Effect] is either:
    ///
    /// - A statement of `RRoot`
    /// - A statement of `RBracedStatements`, with the exception of the last
    ///   expression statement, which is considered a [SyntaxPosition::Value],
    ///   as it is the returned value of the braced expression
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
        let Some(statement) = self.parent().and_then(RExpressionStatement::cast) else {
            // If our direct parent isn't an `RExpressionStatement`, we are
            // definitely a `Value`
            return SyntaxPosition::Value;
        };

        let Some(statements) = statement.parent::<RStatementList>() else {
            // Would be unexpected since an `RExpressionStatement` always lives in an
            // `RStatementList`, but we'd rather not crash if we are wrong
            return SyntaxPosition::Value;
        };

        // Statement of `RRoot`, this is an `Effect`
        if statements.parent::<RRoot>().is_some() {
            return SyntaxPosition::Effect;
        }

        // Otherwise, statement of `RBracedStatements`. If this is the last
        // expression statement, it is a `Value`, otherwise it is an `Effect`.
        //
        // Note that we have to skip over trailing `REmptyStatement`s that we
        // expect will be removed, such as:
        //
        // ```r
        // {
        //   if (a) 1
        //   ;
        // }
        // ```
        let Some(last) = statements
            .iter()
            .rev()
            .find_map(|statement| match statement {
                AnyRStatement::REmptyStatement(_) => None,
                AnyRStatement::RExpressionStatement(statement) => Some(statement),
            })
        else {
            // Would be unexpected to not find any expression statements, since
            // we just checked that `statement` is a child of this, but we'd
            // rather not crash if we are wrong
            return SyntaxPosition::Value;
        };

        if statement == last {
            SyntaxPosition::Value
        } else {
            SyntaxPosition::Effect
        }
    }
}
