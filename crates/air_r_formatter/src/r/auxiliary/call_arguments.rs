// TODO: (c) Biome

use crate::comments::RComments;
use crate::prelude::*;
use crate::r::auxiliary::function_definition::FormatFunctionOptions;
use crate::separated::FormatAstSeparatedListExtension;
use air_r_syntax::AnyRArgument;
use air_r_syntax::AnyRExpression;
use air_r_syntax::RArgumentList;
use air_r_syntax::RCall;
use air_r_syntax::RCallArguments;
use air_r_syntax::RCallArgumentsFields;
use air_r_syntax::RLanguage;
use air_r_syntax::RSyntaxToken;
use biome_formatter::separated::TrailingSeparator;
use biome_formatter::{format_args, format_element, write, VecBuffer};
use biome_rowan::{AstSeparatedElement, AstSeparatedList, SyntaxResult};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRCallArguments;
impl FormatNodeRule<RCallArguments> for FormatRCallArguments {
    fn fmt_fields(&self, node: &RCallArguments, f: &mut RFormatter) -> FormatResult<()> {
        // TODO: Special handling for comments? See `handle_array_holes` for JS.

        let RCallArgumentsFields {
            l_paren_token,
            items,
            r_paren_token,
        } = node.as_fields();

        // Special case where the dangling comment has no node to attach to:
        //
        // ```r
        // fn(
        //   # dangling comment
        // )
        // ```
        //
        // If we don't handle it specially, it can break idempotence
        if items.is_empty() {
            return write!(
                f,
                [
                    l_paren_token.format(),
                    format_dangling_comments(node.syntax()).with_soft_block_indent(),
                    r_paren_token.format()
                ]
            );
        }

        // Special case where we have a test call where we never want to break
        // even if we exceed the line length
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

            return write!(f, [l_paren_token.format(), &items, r_paren_token.format()]);
        }

        FormatRCallLikeArguments::new(l_paren_token, items, r_paren_token).fmt(f)
    }

    fn fmt_dangling_comments(&self, _: &RCallArguments, _: &mut RFormatter) -> FormatResult<()> {
        // Formatted inside of `fmt_fields`
        // Only applicable for the empty arguments case
        Ok(())
    }
}

pub(crate) struct FormatRCallLikeArguments {
    l_token: SyntaxResult<RSyntaxToken>,
    items: RArgumentList,
    r_token: SyntaxResult<RSyntaxToken>,
}

impl FormatRCallLikeArguments {
    pub(crate) fn new(
        l_token: SyntaxResult<RSyntaxToken>,
        items: RArgumentList,
        r_token: SyntaxResult<RSyntaxToken>,
    ) -> Self {
        Self {
            l_token,
            items,
            r_token,
        }
    }
}

impl Format<RFormatContext> for FormatRCallLikeArguments {
    fn fmt(&self, f: &mut Formatter<RFormatContext>) -> FormatResult<()> {
        let last_index = self.items.len().saturating_sub(1);
        let mut has_empty_line = false;

        // Wrap `RArgumentList` elements in a `FormatCallArgument` type that
        // knows how to cache itself when we use `will_break()` to check if
        // the argument breaks
        let arguments: Vec<_> = self
            .items
            .elements()
            .enumerate()
            .map(|(index, element)| {
                let leading_lines = element
                    .node()
                    .map_or(0, |node| get_lines_before(node.syntax()));
                has_empty_line = has_empty_line || leading_lines > 1;

                FormatCallArgument::Default {
                    element,
                    is_last: index == last_index,
                    leading_lines,
                }
            })
            .collect();

        // Special case where the user has requested a fully empty line between
        // some of their arguments. Let's respect that and use it as an
        // indicator to short circuit here.
        if has_empty_line {
            return write!(
                f,
                [FormatAllArgsBrokenOut {
                    l_token: &self.l_token.format(),
                    args: &arguments,
                    r_token: &self.r_token.format(),
                    expand: true,
                }]
            );
        }

        if let Some(group_layout) = arguments_grouped_layout(&self.items, f.comments()) {
            write_grouped_arguments(&self.l_token, &self.r_token, arguments, group_layout, f)
        } else {
            write!(
                f,
                [FormatAllArgsBrokenOut {
                    l_token: &self.l_token.format(),
                    args: &arguments,
                    r_token: &self.r_token.format(),
                    expand: false,
                }]
            )
        }
    }
}

/// Helper for formatting a call argument
enum FormatCallArgument {
    /// Argument that has not been inspected if its formatted content breaks.
    Default {
        element: AstSeparatedElement<RLanguage, AnyRArgument>,

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
        element: AstSeparatedElement<RLanguage, AnyRArgument>,

        /// The lines before this element
        leading_lines: usize,
    },
}

impl FormatCallArgument {
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
                    if *is_last {
                        write!(f, [format_removed(separator)])
                    } else {
                        write!(f, [separator.format()])
                    }
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
    fn element(&self) -> &AstSeparatedElement<RLanguage, AnyRArgument> {
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
/// ```r
/// # NOTE: Not currently possible, as the `{}` always force a break right now,
/// # but this would be an example if `{}` didn't force a break.
/// map(xs, function(x) {})
/// ```
///
/// This variant is removed from the set if we detect that the grouped argument
/// contains a forced break in the body (if a forced break is found in the
/// parameters, we bail entirely and use the most expanded form, as noted
/// at the beginning of this documentation page).
///
/// Note that because `{}` currently unconditionally force a break, and because
/// we only go down this path when we have a `{}` to begin with, that means that
/// currently the most flat variant is always removed. There is an
/// `unreachable!()` in the code to assert this. We can't simply remove the
/// `most_flat` code path though, because it is also where we detect if a
/// parameter forces a break, triggering one of our early exists. Additionally,
/// in the future we may allow `{}` to not force a break, meaning this variant
/// may come back into play.
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
    l_token: &SyntaxResult<RSyntaxToken>,
    r_token: &SyntaxResult<RSyntaxToken>,
    mut arguments: Vec<FormatCallArgument>,
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
                    l_token: &l_token.format(),
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
                l_token: &l_token,
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
    let _most_flat = {
        let snapshot = f.state_snapshot();
        let mut buffer = VecBuffer::new(f.state_mut());
        buffer.write_element(FormatElement::Tag(Tag::StartEntry))?;

        let result = write!(
            buffer,
            [
                l_token,
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

    // If the grouped content breaks, then we can skip the most_flat variant,
    // since we already know that it won't be fitting on a single line.
    let variants = if grouped_breaks {
        write!(f, [expand_parent()])?;
        vec![middle_variant, most_expanded.into_boxed_slice()]
    } else {
        unreachable!("`grouped_breaks` is currently always `true`.");
        // vec![most_flat, middle_variant, most_expanded.into_boxed_slice()]
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
        let node = element.node()?;

        let argument_expression = match node {
            AnyRArgument::RNamedArgument(node) => node.value(),
            AnyRArgument::RUnnamedArgument(node) => node.value().ok(),
            AnyRArgument::RBogusArgument(_)
            | AnyRArgument::RDotsArgument(_)
            | AnyRArgument::RHoleArgument(_) => None,
        };

        // For inline functions, re-format the node and pass the argument that it is the
        // last grouped argument. This changes the formatting of parameters to remove any
        // soft line breaks. When the inline function is the only argument, we want it
        // to hug the `()` of the function call and breaking in the parameters is okay.
        match argument_expression {
            Some(RFunctionDefinition(function)) if !self.is_only => {
                with_token_tracking_disabled(f, |f| {
                    write!(
                        f,
                        [function.format().with_options(FormatFunctionOptions {
                            call_argument_layout: Some(
                                GroupedCallArgumentLayout::GroupedLastArgument
                            ),
                        })]
                    )?;

                    if let Some(separator) = element.trailing_separator()? {
                        write!(f, [format_removed(separator)])?;
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
    l_token: &'a dyn Format<RFormatContext>,
    args: &'a [FormatCallArgument],
    r_token: &'a dyn Format<RFormatContext>,
    expand: bool,
}

impl<'a> Format<RFormatContext> for FormatAllArgsBrokenOut<'a> {
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

        write!(
            f,
            [group(&format_args![
                self.l_token,
                soft_block_indent(&args),
                self.r_token,
            ])
            .should_expand(self.expand)]
        )
    }
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

    if comments.has_leading_comments(last.syntax()) || comments.has_trailing_comments(last.syntax())
    {
        return Ok(false);
    }

    let argument_expression = |arg| match arg {
        AnyRArgument::RNamedArgument(arg) => arg.value(),
        AnyRArgument::RUnnamedArgument(arg) => arg.value().ok(),
        AnyRArgument::RBogusArgument(_)
        | AnyRArgument::RDotsArgument(_)
        | AnyRArgument::RHoleArgument(_) => None,
    };

    let Some(last) = argument_expression(last) else {
        return Ok(false);
    };

    argument_is_groupable(&last)
}

/// Checks if `argument` benefits from grouping in call arguments.
fn argument_is_groupable(argument: &AnyRExpression) -> SyntaxResult<bool> {
    use air_r_syntax::AnyRExpression::*;

    let result = match argument {
        // ```r
        // with(data, {
        //   col
        // })
        // ```
        //
        // Empty braces always expand, so they benefit from grouping
        //
        // ```r
        // with(data, {
        // })
        // ```
        //
        // Empty braces that have comments are still groupable
        //
        // ```r
        // with(data, {
        //   // comment
        // })
        // ```
        //
        // NOTE: If we ever allow empty `{}` to NOT forcibly expand, then empty
        // braced expressions won't benefit from grouping unless there is a
        // comment in there, i.e. we'd change the match arm to this (used by
        // biome in the JS implementation):
        //
        // ```rust
        // !node.expressions().is_empty() || comments.has_comments(node.syntax())
        // ```
        RBracedExpressions(_) => true,

        // ```r
        // map(a, function(x) {
        //   x
        // })
        // ```
        RFunctionDefinition(node) => {
            let body = node.body()?;
            matches!(&body, AnyRExpression::RBracedExpressions(_))
        }

        // Nothing else benefits from grouping
        _ => false,
    };

    Ok(result)
}
