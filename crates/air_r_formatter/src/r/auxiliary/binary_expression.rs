use crate::context::RFormatOptions;
use crate::either::Either;
use crate::is_suppressed_by_comment;
use crate::prelude::*;
use crate::r::auxiliary::call::is_table_by_comment;
use crate::r::auxiliary::call_arguments::FormatRCallArgumentsOptions;
use air_r_syntax::AnyRExpression;
use air_r_syntax::RBinaryExpression;
use air_r_syntax::RBinaryExpressionFields;
use air_r_syntax::RLanguage;
use air_r_syntax::RSyntaxKind;
use biome_formatter::FormatRuleWithOptions;
use biome_formatter::format_args;
use biome_formatter::write;
use biome_rowan::AstNode;
use biome_rowan::SyntaxResult;
use biome_rowan::SyntaxToken;

#[derive(Default, Debug, Clone, Copy)]
pub(crate) enum ChainAlignment {
    #[default]
    Indented,

    /// Used in the RHS of assign operators to align pipelines.
    LeftAligned,
}

#[derive(Default, Debug, Clone)]
pub(crate) struct FormatRBinaryExpression {
    /// Alignment to use with chained expressions
    ///
    /// Left alignment is used to prevent "double-indenting" an
    /// assigned pipeline, e.g.:
    ///
    /// ```r
    /// foo <-
    ///   bar |>
    ///   baz()
    ///
    /// foo <-
    ///   bar +
    ///   baz()
    /// ```
    ///
    /// See https://github.com/posit-dev/air/issues/220.
    pub(crate) alignment: ChainAlignment,
}

#[derive(Default, Debug, Clone)]
pub(crate) struct FormatRBinaryExpressionOptions {
    pub(crate) alignment: ChainAlignment,
}

impl FormatRuleWithOptions<RBinaryExpression> for FormatRBinaryExpression {
    type Options = FormatRBinaryExpressionOptions;

    fn with_options(mut self, options: Self::Options) -> Self {
        self.alignment = options.alignment;
        self
    }
}

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
            | RSyntaxKind::SUPER_ASSIGN_RIGHT => fmt_binary_assignment(node, left, operator, right, f),

            // Chainable (pipes, logical, arithmetic)
            kind if is_chainable_binary_operator(kind)  => fmt_binary_chain(left, operator, right, self.alignment, f),

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
///
/// # Persistent line breaks
///
/// The one exception to this is if a newline already exists between a left assignment
/// operator and the rhs, which is a persistent line break case, like:
///
/// ```r
/// response <-
///   if (condition) {
///     "yes"
///   } else {
///     "no"
///   }
///
/// resolved <-
///   this %||%
///     complex_that(a, b, c, d) %||%
///     complex_that(e, f, g, h) %||%
///     default()
/// ```
///
/// Walrus assignment is not considered when looking for persistent line breaks because we
/// don't want the `:=` case below to look like a request for expansion. While `:=` is
/// technically parsed as a binary operator, we format it more like a named argument with
/// a simple `space()` between the operator and the right hand side.
///
/// ```r
/// # `:=` here is technically a binary operator, `x := y` is technically an unnamed argument
/// fn(
///   x :=
///     y
/// )
///
/// # `=` here is not a binary operator, `x = y` is a named argument and `y` will be
/// # forced onto the same line as `=`. We want to treat `:=` like this.
/// fn(
///   x =
///     y
/// )
/// ```
///
/// Comment handling with persistent line breaks here is a bit tricky. Consider this example:
///
/// ```r
/// x <-
///   y # comment
/// ```
///
/// Note that `# comment` is actually attached to the whole binary expression node. When
/// determining default comment placement, the [DecoratedComment::enclosing_node()] is the
/// root node here, making the [DecoratedComment::preceding_node()] the binary expression
/// node.
///
/// This means we can't use a simple [block_indent()] on `right`. The `block_indent()`
/// would force a newline after the `y` but before the comment, moving the comment to the
/// next line.
///
/// Instead, we do the same trick that we do in [fmt_binary_chain()] (and in Biome with
/// binary chains) of using the "rarely needed" manual [indent()] and adding a leading
/// [hard_line_break()] but not a trailing one. By avoiding a trailing hard line break,
/// the trailing comment is allowed to be formatted on the same line as `y`.
fn fmt_binary_assignment(
    node: &RBinaryExpression,
    left: AnyRExpression,
    operator: SyntaxToken<RLanguage>,
    right: AnyRExpression,
    f: &mut Formatter<RFormatContext>,
) -> FormatResult<()> {
    // Check for table directive here to simplify lifetimes with
    // `format_assignment_rhs()`
    let table = is_table_by_comment(node, f);

    let right_formatted = format_with(|f| {
        if binary_assignment_has_persistent_line_break(&operator, &right, f.options()) {
            let right = match &right {
                AnyRExpression::RBinaryExpression(right) => {
                    Either::Left(right.format().with_options(FormatRBinaryExpressionOptions {
                        alignment: ChainAlignment::LeftAligned,
                    }))
                }
                right => Either::Right(format_assignment_rhs(right, table)),
            };
            write!(f, [indent(&format_args![hard_line_break(), right])])
        } else {
            write!(f, [space(), format_assignment_rhs(&right, table)])
        }
    });

    write!(
        f,
        [group(&format_args![
            left.format(),
            space(),
            operator.format(),
            right_formatted
        ])]
    )
}

fn format_assignment_rhs(expr: &AnyRExpression, table: bool) -> impl Format<RFormatContext> {
    format_with(move |f| {
        if table {
            if let AnyRExpression::RCall(call) = expr {
                let options = FormatRCallArgumentsOptions { table: true };
                return write!(f, [call.format().with_options(options)]);
            }
        }
        write!(f, [expr.format()])
    })
}

fn binary_assignment_has_persistent_line_break(
    operator: &SyntaxToken<RLanguage>,
    right: &AnyRExpression,
    options: &RFormatOptions,
) -> bool {
    if options.persistent_line_breaks().is_ignore() {
        return false;
    }

    // Only for these kinds of left assignment
    if !matches!(
        operator.kind(),
        RSyntaxKind::EQUAL | RSyntaxKind::ASSIGN | RSyntaxKind::SUPER_ASSIGN
    ) {
        return false;
    }

    right.syntax().has_leading_newline()
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

#[derive(Debug)]
struct TailPiece {
    operator: SyntaxToken<RLanguage>,
    right: AnyRExpression,
    enclosing: Option<RBinaryExpression>,
}

/// Format a binary expression chain
///
/// # Chain inversion
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
///
/// # Deciding when to "break" the chain
///
/// We use the guiding principle that we want the reading order of
/// a chain to match its execution order. That means that we never want
/// to generate a chain with indentation like this:
///
/// ```r
/// # Bad output
/// a +
///   b -
///   c *
///   d
/// ```
///
/// The above indentation implies a top to bottom reading order of
/// "a plus b, then minus c, then times d", but that isn't what happens. It's
/// really more like "a plus b, then minus (c times d)".
///
/// A better indentation for this is:
///
/// ```r
/// # Good output
/// a +
///   b -
///   c *
///     d
///
/// # Also allowed, since it fits on one line
/// # (depends on persistent line breaks and object lengths)
/// a +
///   b -
///   c * d
/// ```
///
/// This better conveys that `c * d` is grouped together, either by being on
/// the same line when possible, or by indenting to show that the binary chain
/// has broken.
///
/// Implementing this is complex to think about but simple in practice. Because
/// we only look down the left-hand side of the parse tree, this naturally
/// enforces that any left` candidate binary operator that we see has an
/// operator precedence that is `>=` its parent binary operator, making it
/// a good candidate for chaining because it binds tighter to its operators
/// than the parent does. When that candidate is written to the front of
/// the chain, reading order still matches execution order. With this example:
///
/// ```r
/// a + b - c * d
/// ```
///
/// ```text
///        -
///       /  \
///      /    \
///    +        *
///   / \      / \
///  a   b    c   d
/// ```
///
/// Note that `-` and `+` have the same precedence level, so `+` is a good
/// candidate for chaining. `*` is placed on the RHS of the tree during parsing
/// due to precedence rules, and is not considered for chaining, so it ends up
/// in its own separate group, causing the extra indent from above.
///
/// This works nicely with `%>%` chains too. Since a chain of `%>%` are all at
/// the same precedence level, you get a series of pipes on the `left` that you
/// can collect and chain.
///
/// ```r
/// a %>% b() %>% c()
/// ```
///
/// ```text
///        %>%
///       /  \
///      /    \
///    %>%     c()
///   / \
///  a   b()
/// ```
///
/// Mixing `|>` and `%>%` works fine because those are at the same precedence
/// level.
///
/// # Piping into ggplot2
///
/// Chaining in this way extends naturally to `%>%` and `+` with ggplot2 calls.
/// Consider this very common ggplot2 call:
///
/// ```r
/// df %>%
///   ggplot() +
///   geom_bar()
/// ```
///
/// ```text
///        +
///       /  \
///      /    \
///    %>%     geom_line()
///   / \
///  df  ggplot()
/// ```
///
/// The precedences of `%>%` and `+` don't match here, but note that the `+` is
/// the parent operator and the `%>%` is the child operator. The child `%>%` has
/// a higher precedence than the parent `+` does, which means that we can
/// continue chaining (i.e. reading order will match execution order if we keep
/// chaining because `df %>% ggplot()` executes before the `+ geom_bar()`).
///
/// If you try and pipe OUT of a ggplot2 chain, then you get an indent at the
/// end of the chain.
///
/// ```r
/// df %>%
///   ggplot() +
///   geom_bar() %>%
///     identity()
/// ```
///
/// ```text
///               +
///             /   \
///           /       \
///         /           \
///      %>%               %>%
///    /   \           /        \
///   /     \         /          \
/// df  ggplot()  geom_bar() identity()
/// ```
///
/// The final `%>%` ends up on the RHS of `+` in the parse tree,
/// so it isn't considered chainable. The result follows our guiding principle
/// that reading order should follow execution order. The
/// `geom_bar() %>% identity()` is executed first, and the extra indent helps you
/// see that.
///
/// As a final example, consider this line break, which we actually want to ignore:
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
/// The entire binary chain is chainable here, because every operator ends up
/// on the LHS in the parse tree, so it forms 1 group. We perform the user
/// requested line break check on the `|>`, see that there isn't a line break,
/// and flatten the whole chain. If `|>` and the `+`s were split into different
/// groups, then we'd have 2 places where the user could request a line break
/// and we'd end up with the bad result.
fn fmt_binary_chain(
    mut left: AnyRExpression,
    operator: SyntaxToken<RLanguage>,
    right: AnyRExpression,
    alignment: ChainAlignment,
    f: &mut Formatter<RFormatContext>,
) -> FormatResult<()> {
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
    while let Some(node) = as_chainable_binary_expression(&left)? {
        // It's only possible to suppress the formatting of the whole binary expression formatting OR
        // the formatting of the right hand side value but not of a nested binary expression.
        if is_suppressed_by_comment(node, f) {
            tracing::warn!("Can't use a suppression comment partway through a binary chain.");
        }

        tail.push(TailPiece {
            operator: node.operator()?,
            right: node.right()?,
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

    let chain = match alignment {
        ChainAlignment::Indented => Either::Left(indent(&chain)),
        ChainAlignment::LeftAligned => Either::Right(chain),
    };

    write!(
        f,
        [group(&format_args![left.format(), &chain])
            .should_expand(has_persistent_line_break(&tail, f.options()))]
    )
}

fn as_chainable_binary_expression(
    node: &AnyRExpression,
) -> SyntaxResult<Option<&RBinaryExpression>> {
    let Some(node) = node.as_r_binary_expression() else {
        return Ok(None);
    };

    let operator = node.operator()?;

    if !is_chainable_binary_operator(operator.kind()) {
        return Ok(None);
    }

    Ok(Some(node))
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

/// Check if the user has inserted a persistent line break before the very first `right`.
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
fn has_persistent_line_break(tail: &[TailPiece], options: &RFormatOptions) -> bool {
    if options.persistent_line_breaks().is_ignore() {
        return false;
    }

    tail.first()
        .is_some_and(|piece| piece.right.syntax().has_leading_newline())
}
