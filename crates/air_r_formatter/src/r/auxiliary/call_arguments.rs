use std::cell::Cell;

use crate::comments::RComments;
use crate::context::RFormatOptions;
use crate::either::Either;
use crate::prelude::*;
use crate::r::auxiliary::braced_expressions::BracedExpressionsVariant;
use crate::r::auxiliary::braced_expressions::braced_expressions_variant;
use crate::r::auxiliary::function_definition::FormatFunctionOptions;
use crate::separated::FormatAstSeparatedListExtension;
use air_r_syntax::AnyRExpression;
use air_r_syntax::RArgument;
use air_r_syntax::RArgumentList;
use air_r_syntax::RBracedExpressions;
use air_r_syntax::RCall;
use air_r_syntax::RCallArguments;
use air_r_syntax::RLanguage;
use air_r_syntax::RSubset2Arguments;
use air_r_syntax::RSubsetArguments;
use air_r_syntax::RSyntaxNode;
use air_r_syntax::RSyntaxToken;

use biome_formatter::FormatRuleWithOptions;
use biome_formatter::separated::TrailingSeparator;
use biome_formatter::{VecBuffer, format_args, format_element, write};
use biome_rowan::{AstSeparatedElement, AstSeparatedList, SyntaxResult};
use itertools::Itertools;

#[derive(Debug, Clone, Default)]
pub struct FormatRCallArgumentsOptions {
    pub table: bool,
}

#[derive(Debug, Clone, Default)]
pub struct FormatRCallArguments {
    table: bool,
}

impl FormatRCallArguments {
    pub(crate) fn fmt_call_like(
        &self,
        node: &RCallArguments,
        f: &mut RFormatter,
    ) -> FormatResult<()> {
        RCallLikeArguments::Call(node.clone()).fmt(f)
    }
}

impl FormatNodeRule<RCallArguments> for FormatRCallArguments {
    fn fmt_fields(&self, node: &RCallArguments, f: &mut RFormatter) -> FormatResult<()> {
        if self.table {
            let snapshot = f.state_snapshot();

            if let Some(()) = self.fmt_table(node, f)? {
                Ok(())
            } else {
                f.restore_state_snapshot(snapshot);

                // Table formatting failed, fall back to verbatim. Ideally
                // we'd emit diagnostics about why tabular formatting failed
                // here.
                write!(f, [format_verbatim_node(node.syntax())])
            }
        } else {
            self.fmt_call_like(node, f)
        }
    }

    fn fmt_dangling_comments(&self, _: &RCallArguments, _: &mut RFormatter) -> FormatResult<()> {
        // Formatted inside of `fmt_fields`
        // Only applicable for the empty arguments case
        Ok(())
    }
}

impl FormatRuleWithOptions<RCallArguments> for FormatRCallArguments {
    type Options = FormatRCallArgumentsOptions;

    fn with_options(mut self, options: Self::Options) -> Self {
        self.table = options.table;
        self
    }
}

pub(crate) enum RCallLikeArguments {
    Call(RCallArguments),
    Subset(RSubsetArguments),
    Subset2(RSubset2Arguments),
}

impl RCallLikeArguments {
    fn l_token(&self) -> SyntaxResult<RSyntaxToken> {
        match self {
            Self::Call(node) => node.l_paren_token(),
            Self::Subset(node) => node.l_brack_token(),
            Self::Subset2(node) => node.l_brack2_token(),
        }
    }

    fn r_token(&self) -> SyntaxResult<RSyntaxToken> {
        match self {
            Self::Call(node) => node.r_paren_token(),
            Self::Subset(node) => node.r_brack_token(),
            Self::Subset2(node) => node.r_brack2_token(),
        }
    }

    fn items(&self) -> RArgumentList {
        match self {
            Self::Call(node) => node.items(),
            Self::Subset(node) => node.items(),
            Self::Subset2(node) => node.items(),
        }
    }

    fn syntax(&self) -> &RSyntaxNode {
        match self {
            Self::Call(node) => node.syntax(),
            Self::Subset(node) => node.syntax(),
            Self::Subset2(node) => node.syntax(),
        }
    }

    /// Writes the function arguments
    ///
    /// The "grouped" argument is either the first or last argument depending on the
    /// `group_layout`, but currently it is always the last one.
    ///
    /// - If any arguments that aren't the grouped argument *force* a break, then we
    ///   print in fully expanded mode.
    ///
    /// - If the grouped argument is an inline function with `parameters` that would
    ///   *force* a break, then we print in fully expanded mode. We only want to
    ///   allow forced breaks in a braced expression body.
    ///
    /// If neither of those trigger fully expanded mode, we best-fit between three
    /// possible forms:
    ///
    /// ## Most expanded
    ///
    /// The `(`, `)`, and all arguments are within a single `group()`, and that
    /// group is marked with `should_expand(true)`. The arguments are wrapped in
    /// `soft_block_indent()`, and each argument is separated by a
    /// `soft_line_break_or_space()`. Due to the forced expansion, these all
    /// become hard indents / line breaks, i.e. the "most expanded" form.
    ///
    /// Example:
    ///
    /// ```r
    /// map(
    ///   xs,
    ///   function(x) {
    ///     x + 1
    ///   }
    /// )
    /// ```
    ///
    /// ## Most flat
    ///
    /// Arguments are not grouped, each argument is separated by a
    /// `soft_line_break_or_space()`, no forced expansion is done.
    ///
    /// Special formatting is done for a grouped argument that is an inline
    /// function. We remove any soft line breaks in the `parameters`, which
    /// practically means the only place it is allowed to break is in the function
    /// body (but the break is not forced).
    ///
    /// Example:
    ///
    /// An inline function without braces is an example where "most flat" is considered.
    /// We consider this as groupable because it is possible that a long function body
    /// or long function signature can cause a break, which may benefit from the middle
    /// variant before expanding all the way to the most expanded variant.
    ///
    /// ```r
    /// # Most flat wins
    /// map(xs, function(x) x + 1)
    ///
    /// # Middle variant wins
    /// map(xs, function(x) x + a_really_long_thing_here + a_really_long_thing_there)
    /// # ->
    /// map(xs, function(x) {
    ///   x + a_really_long_thing_here + a_really_long_thing_there
    /// })
    ///
    /// # Most expanded wins (middle variant wouldn't fit within line length)
    /// map(xs_are_extremely_long_too, function(this_argument_is_really_long, option = "so is this") this_argument_is_really_long)
    /// # ->
    /// map(
    ///   xs_are_extremely_long_too,
    ///   function(this_argument_is_really_long, option = "so is this") {
    ///     this_argument_is_really_long
    ///   }
    /// )
    /// ```
    ///
    /// This variant is removed from the set if we detect that the grouped argument
    /// contains a forced break in the body (if a forced break is found in the
    /// parameters, we bail entirely and use the most expanded form, as noted
    /// at the beginning of this documentation page).
    ///
    /// ```r
    /// # Forced break in the body
    /// map(xs, function(x) {
    ///   x
    /// })
    ///
    /// # This is a forced break in the body too, and triggers autobracing and
    /// # results in the middle variant winning
    /// map(xs, function(x)
    ///   x)
    /// ```
    ///
    /// An empty `{}` never expands, so this case is not considered groupable, and best
    /// fitting is not actually used.
    ///
    /// ```r
    /// map(xs, function(x) {})
    /// ```
    ///
    /// Like `{}`, we explicitly disallow curly-curly as a groupable argument, so this
    /// case is never considered grouped, and is therefore not an example of "most flat".
    ///
    /// ```r
    /// group_by(df, {{ by }})
    /// ```
    ///
    /// ## Middle variant
    ///
    /// Exactly the same as "most flat", except that the grouped argument is put
    /// in its own `group()` marked with `should_expand(true)`. The soft line breaks
    /// are removed from any grouped argument parameters, like with most flat.
    ///
    /// Example:
    ///
    /// ```r
    /// map(xs, function(x) {
    ///   x + 1
    /// })
    /// ```
    ///
    /// ```r
    /// # The soft line breaks are removed from the `parameters`, meaning that this...
    /// map(xs, function(x, a_long_secondary_argument = "with a default", and_another_one_here) {
    ///   x + 1
    /// })
    ///
    /// # ...is not allowed to be formatted as...
    /// map(xs, function(
    ///   x,
    ///   a_long_secondary_argument = "with a default",
    ///   and_another_one_here
    /// ) {
    ///   x + 1
    /// })
    ///
    /// # ...and instead the most expanded form is chosen by best-fitting:
    /// map(
    ///   xs,
    ///   function(
    ///     x,
    ///     a_long_secondary_argument = "with a default",
    ///     and_another_one_here
    ///   ) {
    ///     x + 1
    ///   }
    /// )
    /// ```
    fn write_grouped_arguments(
        &self,
        l_token: RSyntaxToken,
        leading_holes: Vec<FormatCallArgumentHole>,
        mut arguments: Vec<FormatCallArgument>,
        r_token: RSyntaxToken,
        group_layout: GroupedCallArgumentLayout,
        f: &mut RFormatter,
    ) -> FormatResult<()> {
        let grouped_breaks = {
            let (grouped_arg, other_args) = match group_layout {
                GroupedCallArgumentLayout::GroupedFirstArgument => {
                    let (first, tail) = arguments.split_at_mut(1);
                    (&mut first[0], tail)
                }
                GroupedCallArgumentLayout::GroupedLastArgument => {
                    let end_index = arguments.len().saturating_sub(1);
                    let (head, last) = arguments.split_at_mut(end_index);
                    (&mut last[0], head)
                }
            };

            let non_grouped_breaks = other_args.iter_mut().any(|arg| arg.will_break(f));

            // If any of the not grouped elements break, then fall back to the variant where
            // all arguments are printed in expanded mode.
            if non_grouped_breaks {
                return write!(
                    f,
                    [FormatAllArgsBrokenOut {
                        call: self,
                        l_token: &l_token.format(),
                        leading_holes: &leading_holes,
                        args: &arguments,
                        r_token: &r_token.format(),
                        expand: true,
                    }]
                );
            }

            grouped_arg.will_break(f)
        };

        // We now cache the delimiters tokens. This is needed because `[biome_formatter::best_fitting]` will try to
        // print each version first
        // tokens on the left
        let l_token = l_token.format().memoized();

        // tokens on the right
        let r_token = r_token.format().memoized();

        // First write the most expanded variant because it needs `arguments`.
        let most_expanded = {
            let mut buffer = VecBuffer::new(f.state_mut());
            buffer.write_element(FormatElement::Tag(Tag::StartEntry))?;

            write!(
                buffer,
                [FormatAllArgsBrokenOut {
                    call: self,
                    l_token: &l_token,
                    leading_holes: &leading_holes,
                    args: &arguments,
                    r_token: &r_token,
                    expand: true,
                }]
            )?;

            buffer.write_element(FormatElement::Tag(Tag::EndEntry))?;
            buffer.into_vec()
        };

        // Now reformat the first or last argument if they happen to be an inline function.
        // Inline functions in this context apply a custom formatting that removes soft line breaks
        // from the parameters.
        //
        // TODO: The JS approach caches the function body of the "normal" (before soft line breaks
        // are removed) formatted function to avoid quadratic complexity if the function's body contains
        // another call expression with an inline function as first or last argument. We may want to
        // consider this if we have issues here.
        let last_index = arguments.len() - 1;
        let grouped = arguments
            .into_iter()
            .enumerate()
            .map(|(index, argument)| {
                let layout = match group_layout {
                    GroupedCallArgumentLayout::GroupedFirstArgument if index == 0 => {
                        Some(GroupedCallArgumentLayout::GroupedFirstArgument)
                    }
                    GroupedCallArgumentLayout::GroupedLastArgument if index == last_index => {
                        Some(GroupedCallArgumentLayout::GroupedLastArgument)
                    }
                    _ => None,
                };

                FormatGroupedArgument {
                    argument,
                    single_argument_list: last_index == 0,
                    layout,
                }
                .memoized()
            })
            .collect::<Vec<_>>();

        // Write the most flat variant with the first or last argument grouped
        // (but not forcibly expanded)
        let most_flat = {
            let snapshot = f.state_snapshot();
            let mut buffer = VecBuffer::new(f.state_mut());
            buffer.write_element(FormatElement::Tag(Tag::StartEntry))?;

            let result = write!(
                buffer,
                [
                    l_token,
                    format_with(|f| { f.join().entries(leading_holes.iter()).finish() }),
                    maybe_space(!leading_holes.is_empty() && !grouped.is_empty()),
                    format_with(|f| {
                        f.join_with(soft_line_break_or_space())
                            .entries(grouped.iter())
                            .finish()
                    }),
                    r_token
                ]
            );

            // Turns out, using the grouped layout isn't a good fit because some parameters of the
            // grouped inline function break. In that case, fall back to the all args expanded
            // formatting.
            // This back tracking is required because testing if the grouped argument breaks in general
            // would also return `true` if any content of the function BODY breaks. But, as far as this
            // is concerned, it's only interested if any content in just the function SIGNATURE breaks.
            if matches!(result, Err(FormatError::PoorLayout)) {
                drop(buffer);
                f.restore_state_snapshot(snapshot);

                let mut most_expanded_iter = most_expanded.into_iter();
                // Skip over the Start/EndEntry items.
                most_expanded_iter.next();
                most_expanded_iter.next_back();

                return f.write_elements(most_expanded_iter);
            }

            buffer.write_element(FormatElement::Tag(Tag::EndEntry))?;
            buffer.into_vec().into_boxed_slice()
        };

        // Write the second most flat variant that now forces the group of the first/last argument to expand.
        let middle_variant = {
            let mut buffer = VecBuffer::new(f.state_mut());
            buffer.write_element(FormatElement::Tag(Tag::StartEntry))?;

            write!(
                buffer,
                [
                    l_token,
                    format_with(|f| { f.join().entries(leading_holes.iter()).finish() }),
                    maybe_space(!leading_holes.is_empty() && !grouped.is_empty()),
                    format_with(|f| {
                        let mut joiner = f.join_with(soft_line_break_or_space());

                        match group_layout {
                            GroupedCallArgumentLayout::GroupedFirstArgument => {
                                joiner.entry(&group(&grouped[0]).should_expand(true));
                                joiner.entries(&grouped[1..]).finish()
                            }
                            GroupedCallArgumentLayout::GroupedLastArgument => {
                                let last_index = grouped.len() - 1;
                                joiner.entries(&grouped[..last_index]);
                                joiner
                                    .entry(&group(&grouped[last_index]).should_expand(true))
                                    .finish()
                            }
                        }
                    }),
                    r_token
                ]
            )?;

            buffer.write_element(FormatElement::Tag(Tag::EndEntry))?;
            buffer.into_vec().into_boxed_slice()
        };

        let variants = if grouped_breaks {
            // If the grouped content breaks, then we can skip the most_flat variant,
            // since we already know that it won't be fitting on a single line. We can
            // also go ahead and signal that we will be expanding.
            write!(f, [expand_parent()])?;
            vec![middle_variant, most_expanded.into_boxed_slice()]
        } else {
            // Otherwise we best fit between all variants
            vec![most_flat, middle_variant, most_expanded.into_boxed_slice()]
        };

        // SAFETY: Safe because variants is guaranteed to contain >=2 entries:
        // * most flat (optional)
        // * middle
        // * most expanded
        // ... and best fitting only requires the most flat/and expanded.
        unsafe {
            f.write_element(FormatElement::BestFitting(
                format_element::BestFittingElement::from_vec_unchecked(variants),
            ))
        }
    }
}

impl Format<RFormatContext> for RCallLikeArguments {
    fn fmt(&self, f: &mut Formatter<RFormatContext>) -> FormatResult<()> {
        let l_token = self.l_token()?;
        let items = self.items();
        let r_token = self.r_token()?;

        // Special case where the dangling comment has no node to attach to:
        //
        // ```r
        // fn(
        //   # dangling comment
        // )
        // ```
        //
        // If we don't handle it specially, it can break idempotence.
        // Same as `RParameters`.
        if items.is_empty() {
            return write!(
                f,
                [
                    l_token.format(),
                    format_dangling_comments(self.syntax()).with_soft_block_indent(),
                    r_token.format()
                ]
            );
        }

        // Special case where we have a test call where we never want to break
        // even if we exceed the line length. Only applies to `Call` variants.
        if let RCallLikeArguments::Call(node) = self {
            let is_test_call = node
                .parent::<RCall>()
                .map_or(Ok(false), |call| call.is_test_call())?;

            if is_test_call {
                let items = format_with(|f| {
                    f.join_with(space())
                        .entries(
                            items
                                .format_separated(",")
                                .with_trailing_separator(TrailingSeparator::Disallowed),
                        )
                        .finish()
                });

                return write!(f, [l_token.format(), &items, r_token.format()]);
            }
        }

        let comments = f.comments();

        let mut iter_elements = items.elements();

        // Split leading holes out from the remainder of the arguments.
        // Leading holes tightly hug the `l_token` no matter what.
        // Because they are intended to tightly hug, a node is only considered
        // a leading hole if there isn't a comment attached.
        let leading_holes: Vec<_> = iter_elements
            .take_while_ref(|element| {
                element
                    .node()
                    .is_ok_and(|node| node.is_hole() && !comments.has_comments(node.syntax()))
            })
            .map(FormatCallArgumentHole::new)
            .collect();

        let last_index = (items.len() - leading_holes.len()).saturating_sub(1);

        // Wrap remaining `RArgumentList` elements in a `FormatCallArgument` type that
        // knows how to cache itself when we use `will_break()` to check if
        // the argument breaks
        let arguments: Vec<_> = iter_elements
            .enumerate()
            .map(|(index, element)| FormatCallArgument::new(element, index == last_index))
            .collect();

        let has_empty_line = leading_holes
            .iter()
            .any(|leading_hole| leading_hole.leading_lines() > 1)
            || arguments
                .iter()
                .any(|argument| argument.leading_lines() > 1);

        // Special case where the user has requested a fully empty line between
        // some of their arguments. Let's respect that and use it as an
        // indicator to short circuit here.
        if has_empty_line {
            return write!(
                f,
                [FormatAllArgsBrokenOut {
                    call: self,
                    l_token: &l_token.format(),
                    leading_holes: &leading_holes,
                    args: &arguments,
                    r_token: &r_token.format(),
                    expand: true,
                }]
            );
        }

        // Special case where a persistent line break exists between the `l_token` and the
        // first non-hole argument. Treat this as a user request to expand.
        if has_persistent_line_break(&leading_holes, &arguments, f.options()) {
            return write!(
                f,
                [FormatAllArgsBrokenOut {
                    call: self,
                    l_token: &l_token.format(),
                    leading_holes: &leading_holes,
                    args: &arguments,
                    r_token: &r_token.format(),
                    expand: true,
                }]
            );
        }

        if let Some(group_layout) = arguments_grouped_layout(&items, comments) {
            self.write_grouped_arguments(
                l_token,
                leading_holes,
                arguments,
                r_token,
                group_layout,
                f,
            )
        } else {
            write!(
                f,
                [FormatAllArgsBrokenOut {
                    call: self,
                    l_token: &l_token.format(),
                    leading_holes: &leading_holes,
                    args: &arguments,
                    r_token: &r_token.format(),
                    expand: false,
                }]
            )
        }
    }
}

/// Check if the user has inserted a persistent line break before the very first non-hole
/// `argument`. If so, we respect that and treat it as a request to break ALL of the
/// arguments. Note this is a case of irreversible formatting!
///
/// ```r
/// # Fits on one line, but newline before `bob` forces ALL arguments to break, so it
/// # stays as is
///
/// # Input
/// dictionary <- list(
///   bob = "burger",
///   dina = "dairy",
///   john = "juice"
/// )
///
/// # Output
/// dictionary <- list(
///   bob = "burger",
///   dina = "dairy",
///   john = "juice"
/// )
/// ```
///
/// Note that removing this line break is a request to flatten if possible. By only having
/// this special behavior on the very first argument, we make it easy to request flattening.
///
/// ```r
/// # Remove the first line break and run air
/// dictionary <- list(bob = "burger",
///   dina = "dairy",
///   john = "juice"
/// )
///
/// # Output
/// dictionary <- list(bob = "burger", dina = "dairy", john = "juice")
/// ```
///
/// The persistent line break check is done on the first non-hole argument, so this
/// is considered a persistent line break and stays as is because there
/// is a leading newline before the `j` argument node.
///
/// ```r
/// dt[,
///   j = complex + things,
///   by = col
/// ]
/// ```
///
/// This is also considered a persistent line break. We treat holes as
/// "invisible" for this check, so if you squint and remove the leading `,`
/// and there are any leading lines before the first non-hole argument,
/// that is still considered a persistent line break, but the `,`s attached
/// to the hole will get moved to hug the `[`.
///
/// ```r
/// dt[
///   , j = complex + things,
///   by = col
/// ]
/// ```
fn has_persistent_line_break(
    leading_holes: &[FormatCallArgumentHole],
    arguments: &[FormatCallArgument],
    options: &RFormatOptions,
) -> bool {
    if options.persistent_line_breaks().is_ignore() {
        return false;
    }

    // Do any leading holes have leading lines?
    // We treat leading holes as "invisible" so a leading line in the hole
    // implies a leading line in the first argument.
    if leading_holes.iter().any(|hole| hole.leading_lines() > 0) {
        return true;
    }

    // Does the first non-hole argument have leading lines?
    if arguments
        .first()
        .is_some_and(|argument| argument.leading_lines() > 0)
    {
        return true;
    }

    false
}

/// Helper for formatting a call argument hole
///
/// We cache the result at `fmt()` time. This is necessary because
/// using `BestFitting` will try and print the hole multiple times as it
/// tries out the different variants, which would be an error if it wasn't
/// cached.
struct FormatCallArgumentHole {
    /// The element to format
    element: AstSeparatedElement<RLanguage, RArgument>,

    /// The formatted element
    ///
    /// Cached using interior mutability the first time `fmt()` is called.
    content: Cell<Option<FormatResult<Option<FormatElement>>>>,

    /// The number of lines before this node
    leading_lines: usize,
}

impl FormatCallArgumentHole {
    fn new(element: AstSeparatedElement<RLanguage, RArgument>) -> Self {
        // Note that holes by their very nature don't have any physical nodes
        // to attach trivia to, so we can't use `get_lines_before()` on the
        // node. Instead we look at the attached `,` token and look for lines
        // before that!
        let leading_lines = element
            .trailing_separator()
            .unwrap_or(None)
            .map_or(0, get_lines_before_token);

        Self {
            element,
            content: Cell::new(None),
            leading_lines,
        }
    }

    fn leading_lines(&self) -> usize {
        self.leading_lines
    }
}

impl Format<RFormatContext> for FormatCallArgumentHole {
    fn fmt(&self, f: &mut Formatter<RFormatContext>) -> FormatResult<()> {
        // If we've formatted this hole before, reuse the content.
        // Otherwise `intern()` the hole and cache it.
        let content = match self.content.take() {
            Some(content) => content,
            None => f.intern(&format_with(|f| {
                write!(
                    f,
                    [
                        self.element.node()?.format(),
                        self.element.trailing_separator()?.format()
                    ]
                )
            })),
        };

        // Set before writing in case there is an error at write time
        self.content.set(Some(content.clone()));

        if let Some(element) = content? {
            f.write_element(element)?;
        }

        Ok(())
    }
}

/// Helper for formatting a call argument
enum FormatCallArgument {
    /// Argument that has not been inspected if its formatted content breaks.
    Default {
        element: AstSeparatedElement<RLanguage, RArgument>,

        /// Whether this is the last element.
        is_last: bool,

        /// The number of lines before this node
        leading_lines: usize,
    },

    /// The argument has been formatted because a caller inspected if it [Self::will_break].
    ///
    /// Allows to re-use the formatted output rather than having to call into the formatting again.
    Inspected {
        /// The formatted element
        content: FormatResult<Option<FormatElement>>,

        /// The separated element
        element: AstSeparatedElement<RLanguage, RArgument>,

        /// The lines before this element
        leading_lines: usize,
    },
}

impl FormatCallArgument {
    fn new(element: AstSeparatedElement<RLanguage, RArgument>, is_last: bool) -> Self {
        let leading_lines = element
            .node()
            .map_or(0, |node| get_lines_before(node.syntax()));

        FormatCallArgument::Default {
            element,
            is_last,
            leading_lines,
        }
    }

    /// Returns `true` if this argument contains any content that forces a group to [`break`](FormatElements::will_break).
    ///
    /// Caches the formatted content after we check, so we can utilize it later
    /// on in `fmt_with_cache()`
    fn will_break(&mut self, f: &mut RFormatter) -> bool {
        match &self {
            FormatCallArgument::Default {
                element,
                leading_lines,
                ..
            } => {
                // Handles the separator
                let interned = f.intern(&self);

                let breaks = match &interned {
                    Ok(Some(element)) => element.will_break(),
                    _ => false,
                };

                *self = FormatCallArgument::Inspected {
                    content: interned,
                    element: element.clone(),
                    leading_lines: *leading_lines,
                };
                breaks
            }
            FormatCallArgument::Inspected {
                content: Ok(Some(result)),
                ..
            } => result.will_break(),
            FormatCallArgument::Inspected { .. } => false,
        }
    }

    fn fmt_with_cache(&self, f: &mut RFormatter) -> FormatResult<()> {
        match self {
            // Re-use the cached formatted output if there is any.
            FormatCallArgument::Inspected { content, .. } => match content.clone()? {
                Some(element) => {
                    f.write_element(element)?;
                    Ok(())
                }
                None => Ok(()),
            },
            FormatCallArgument::Default {
                element, is_last, ..
            } => {
                write!(f, [element.node()?.format()])?;

                if let Some(separator) = element.trailing_separator()? {
                    // Are we correctly stripping trailing whitespace?
                    write!(f, [separator.format()])
                } else if !is_last {
                    Err(FormatError::SyntaxError)
                } else {
                    Ok(())
                }
            }
        }
    }

    /// Returns the number of leading lines before the argument's node
    fn leading_lines(&self) -> usize {
        match self {
            FormatCallArgument::Default { leading_lines, .. } => *leading_lines,
            FormatCallArgument::Inspected { leading_lines, .. } => *leading_lines,
        }
    }

    /// Returns the [`separated element`](AstSeparatedElement) of this argument.
    fn element(&self) -> &AstSeparatedElement<RLanguage, RArgument> {
        match self {
            FormatCallArgument::Default { element, .. } => element,
            FormatCallArgument::Inspected { element, .. } => element,
        }
    }
}

impl Format<RFormatContext> for FormatCallArgument {
    fn fmt(&self, f: &mut Formatter<RFormatContext>) -> FormatResult<()> {
        self.fmt_with_cache(f)?;
        Ok(())
    }
}

/// Helper for formatting the first grouped argument (see [should_group_first_argument]).
struct FormatGroupedFirstArgument<'a> {
    argument: &'a FormatCallArgument,

    /// Whether this is the only argument in the argument list.
    _is_only: bool,
}

impl Format<RFormatContext> for FormatGroupedFirstArgument<'_> {
    fn fmt(&self, f: &mut Formatter<RFormatContext>) -> FormatResult<()> {
        // TODO: Add special handling if we decide we want first argument formatting
        self.argument.fmt(f)
    }
}

/// Helper for formatting the last grouped argument (see [should_group_last_argument]).
struct FormatGroupedLastArgument<'a> {
    argument: &'a FormatCallArgument,
    /// Is this the only argument in the arguments list
    is_only: bool,
}

impl Format<RFormatContext> for FormatGroupedLastArgument<'_> {
    fn fmt(&self, f: &mut Formatter<RFormatContext>) -> FormatResult<()> {
        use air_r_syntax::AnyRExpression::*;
        let element = self.argument.element();
        let argument = element.node()?;

        // For inline functions, re-format the node and pass the argument that it is the
        // last grouped argument. This changes the formatting of parameters to remove any
        // soft line breaks. When the inline function is the only argument, we want it
        // to hug the `()` of the function call and breaking in the parameters is okay.
        match argument.value() {
            Some(RFunctionDefinition(function)) if !self.is_only => {
                with_token_tracking_disabled(f, |f| {
                    if let Some(name_clause) = argument.name_clause() {
                        write!(f, [name_clause.format(), space()])?;
                    }

                    write!(
                        f,
                        [function.format().with_options(FormatFunctionOptions {
                            call_argument_layout: Some(
                                GroupedCallArgumentLayout::GroupedLastArgument
                            ),
                        })]
                    )?;

                    if let Some(separator) = element.trailing_separator()? {
                        write!(f, [separator.format()])?;
                    }

                    Ok(())
                })
            }

            // Functions that are the only argument, and anything else
            _ => self.argument.fmt(f),
        }
    }
}

/// Disable the token tracking because it is necessary to format inline functions
/// with soft line breaks removed.
fn with_token_tracking_disabled<F: FnOnce(&mut RFormatter) -> R, R>(
    f: &mut RFormatter,
    callback: F,
) -> R {
    let was_disabled = f.state().is_token_tracking_disabled();
    f.state_mut().set_token_tracking_disabled(true);

    let result = callback(f);

    f.state_mut().set_token_tracking_disabled(was_disabled);

    result
}

/// Helper for formatting a grouped call argument (see [should_group_first_argument] and [should_group_last_argument]).
struct FormatGroupedArgument {
    argument: FormatCallArgument,

    /// Whether this argument is the only argument in the argument list.
    single_argument_list: bool,

    /// The layout to use for this argument.
    layout: Option<GroupedCallArgumentLayout>,
}

impl Format<RFormatContext> for FormatGroupedArgument {
    fn fmt(&self, f: &mut Formatter<RFormatContext>) -> FormatResult<()> {
        match self.layout {
            Some(GroupedCallArgumentLayout::GroupedFirstArgument) => FormatGroupedFirstArgument {
                argument: &self.argument,
                _is_only: self.single_argument_list,
            }
            .fmt(f),
            Some(GroupedCallArgumentLayout::GroupedLastArgument) => FormatGroupedLastArgument {
                argument: &self.argument,
                is_only: self.single_argument_list,
            }
            .fmt(f),
            None => self.argument.fmt(f),
        }
    }
}

struct FormatAllArgsBrokenOut<'a> {
    call: &'a RCallLikeArguments,
    l_token: &'a dyn Format<RFormatContext>,
    leading_holes: &'a [FormatCallArgumentHole],
    args: &'a [FormatCallArgument],
    r_token: &'a dyn Format<RFormatContext>,
    expand: bool,
}

impl Format<RFormatContext> for FormatAllArgsBrokenOut<'_> {
    fn fmt(&self, f: &mut Formatter<RFormatContext>) -> FormatResult<()> {
        let args = format_with(|f| {
            for (index, entry) in self.args.iter().enumerate() {
                // Respect 1 full empty line between sequential arguments
                // if the user requested it (similar to top level expressions)
                if index > 0 {
                    match entry.leading_lines() {
                        0 | 1 => write!(f, [soft_line_break_or_space()])?,
                        _ => write!(f, [empty_line()])?,
                    }
                }

                write!(f, [entry])?;
            }

            Ok(())
        });

        let args = if !self.expand && is_hugging_call(self.args, self.call.r_token())? {
            Either::Left(args)
        } else {
            Either::Right(soft_block_indent(&args))
        };

        write!(
            f,
            [group(&format_args![
                self.l_token,
                format_with(|f| f.join().entries(self.leading_holes.iter()).finish()),
                maybe_space(!self.leading_holes.is_empty() && !self.args.is_empty()),
                &args,
                self.r_token,
            ])
            .should_expand(self.expand)]
        )
    }
}

fn is_hugging_call(
    items: &[FormatCallArgument],
    r_token: SyntaxResult<RSyntaxToken>,
) -> SyntaxResult<bool> {
    // We only consider calls with one argument
    let Some(item) = items.first() else {
        return Ok(false);
    };
    if items.len() != 1 {
        return Ok(false);
    }

    let arg = item.element().node()?;

    // We only consider hugging for an unnamed argument
    // (Particularly meaningful for `mutate(key = value)` where you don't expect hugging)
    if arg.name_clause().is_some() {
        return Ok(false);
    }

    let Some(arg) = arg.value() else {
        return Ok(false);
    };

    // Bail on hugging if the argument has comments attached. In practice only
    // trailing comments get reordered so we don't need to check for comments on
    // the argument name.
    if arg.syntax().has_comments_direct() || r_token.is_ok_and(|tok| tok.has_leading_comments()) {
        return Ok(false);
    }

    Ok(matches!(
        arg,
        AnyRExpression::RCall(_) | AnyRExpression::RSubset(_) | AnyRExpression::RSubset2(_)
    ))
}

#[derive(Copy, Clone, Debug)]
pub enum GroupedCallArgumentLayout {
    /// Group the first call argument.
    GroupedFirstArgument,

    /// Group the last call argument.
    GroupedLastArgument,
}

fn arguments_grouped_layout(
    args: &RArgumentList,
    comments: &RComments,
) -> Option<GroupedCallArgumentLayout> {
    if should_group_first_argument(args, comments).unwrap_or(false) {
        Some(GroupedCallArgumentLayout::GroupedFirstArgument)
    } else if should_group_last_argument(args, comments).unwrap_or(false) {
        Some(GroupedCallArgumentLayout::GroupedLastArgument)
    } else {
        None
    }
}

/// Checks if the first argument requires grouping
fn should_group_first_argument(_list: &RArgumentList, _comments: &RComments) -> SyntaxResult<bool> {
    // TODO: Do we ever have cases where we'd want special behavior in the first argument?
    Ok(false)
}

/// Checks if the last argument should be grouped.
fn should_group_last_argument(list: &RArgumentList, comments: &RComments) -> SyntaxResult<bool> {
    let Some(last) = list.last() else {
        return Ok(false);
    };
    let last = last?;

    // If the entire argument node has comments attached, not groupable
    if comments.has_comments(last.syntax()) {
        return Ok(false);
    }

    // If this is a named argument and the `name_clause` has comments, not
    // groupable. This avoids idempotence issues. Plus, the comments by
    // definition make it non groupable. The `comments.rs` handlers should
    // ensure that the underlying `name` node inside `name_clause` does not
    // have any comments (they get put on `name_clause` instead), so we should
    // not need to check that.
    if let Some(name_clause) = last.name_clause() {
        if comments.has_comments(name_clause.syntax()) {
            return Ok(false);
        }
    }

    let Some(last) = last.value() else {
        return Ok(false);
    };

    argument_is_groupable(&last, comments)
}

/// Checks if `argument` benefits from grouping in call arguments.
fn argument_is_groupable(argument: &AnyRExpression, comments: &RComments) -> SyntaxResult<bool> {
    use air_r_syntax::AnyRExpression::*;

    let result = match argument {
        // Braces with any expression inside are groupable
        //
        // ```r
        // with(data, {
        //   col
        // })
        // ```
        //
        // Empty braces with dangling comments are groupable
        //
        // ```r
        // with(data, {
        //   # comment
        // })
        // ```
        //
        // Empty braces never expand, so they are not groupable
        //
        // ```r
        // with(data, {})
        // ```
        //
        // Curly-curly expressions are NOT groupable, we want this to fall through
        // to using the "normal" `FormatAllArgsBrokenOut` variant.
        //
        // ```r
        // group_by(df, {{ vars }})
        // ```
        RBracedExpressions(node) => braced_expressions_is_groupable(node, comments),

        // Almost every kind of function definition can benefit from grouping:
        //
        // - If the body does not have `{}`, we may keep the function definition
        //   completely flat on one line, or we may expand it over multiple lines if
        //   it exceeds the line length or a persistent line break forces a break. If
        //   it were to break, it would benefit from grouping, so we always consider
        //   this case a candidate for grouping.
        //
        // - If the body has `{}`, we treat it like `RBracedExpressions` above. It's only
        //   ever not groupable if the body is a totally empty `{}`, because that would
        //   never spread across multiple lines.
        //
        // Here are some examples:
        //
        // Trailing function definition with braces is the classic example of something
        // that benefits from grouping
        //
        // ```r
        // map(xs, function(x) {
        //   x
        // })
        // ```
        //
        // When the braces are empty, it won't benefit from grouping because the `{}`
        // are always kept collapsed
        //
        // ```r
        // map(xs, function(x) {})
        // ```
        //
        // With a dangling comment in empty braces, we always benefit from grouping
        //
        // ```r
        // map(xs, function(x) {
        //   # comment
        // })
        // ```
        //
        // Long function definition that would be split over multiple lines, triggering
        // autobracing, and would benefit from grouping
        //
        // ```r
        // map(xs, function(x) x + a_really_really_long_name + another_really_long_name)
        //
        // # Becomes:
        // map(xs, function(x) {
        //   x + a_really_really_long_name + another_really_long_name
        // })
        //
        // # Which is preferred over fully expanding:
        // map(
        //   xs,
        //   function(x) {
        //     x + a_really_really_long_name + another_really_long_name
        //   }
        // )
        // ```
        //
        // Line break in the function definition body, triggering autobracing, and would
        // benefit from grouping
        //
        // ```r
        // map(xs, function(x)
        //  x + 1)
        //
        // # Becomes:
        // map(xs, function(x) {
        //   x + 1
        // })
        //
        // # Which is preferred over fully expanding:
        // map(
        //   xs,
        //   function(x) {
        //     x + 1
        //   }
        // )
        // ```
        RFunctionDefinition(node) => node.body().is_ok_and(|body| match body {
            RBracedExpressions(ref body) => braced_expressions_is_groupable(body, comments),
            _ => true,
        }),

        // Nothing else benefits from grouping
        _ => false,
    };

    Ok(result)
}

fn braced_expressions_is_groupable(node: &RBracedExpressions, comments: &RComments) -> bool {
    match braced_expressions_variant(node, comments) {
        // These two stay collapsed as `{}` or `{{ x }}`, and don't benefit from grouping
        BracedExpressionsVariant::Empty => false,
        BracedExpressionsVariant::CurlyCurly(_) => false,
        // These two expand across multiple lines, and benefit from grouping
        BracedExpressionsVariant::EmptyWithDanglingComments => true,
        BracedExpressionsVariant::NotEmpty => true,
    }
}
