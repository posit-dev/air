use crate::prelude::*;
use air_r_syntax::AnyRExpression;
use air_r_syntax::RFunctionDefinition;
use biome_formatter::format_args;
use biome_formatter::write;
use biome_formatter::FormatRuleWithOptions;
use biome_formatter::RemoveSoftLinesBuffer;
use biome_rowan::SyntaxResult;

use super::call_arguments::GroupedCallArgumentLayout;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRFunctionDefinition {
    options: FormatFunctionOptions,
}

impl FormatRuleWithOptions<RFunctionDefinition> for FormatRFunctionDefinition {
    type Options = FormatFunctionOptions;

    fn with_options(mut self, options: Self::Options) -> Self {
        self.options = options;
        self
    }
}

impl FormatNodeRule<RFunctionDefinition> for FormatRFunctionDefinition {
    fn fmt_fields(&self, node: &RFunctionDefinition, f: &mut RFormatter) -> FormatResult<()> {
        FormatFunction::new(node.clone()).fmt_with_options(f, &self.options)?;
        Ok(())
    }
}

struct FormatFunction {
    inner: RFunctionDefinition,
}

#[derive(Copy, Clone, Debug, Default)]
pub(crate) struct FormatFunctionOptions {
    pub call_argument_layout: Option<GroupedCallArgumentLayout>,
}

impl FormatFunction {
    fn new(node: RFunctionDefinition) -> Self {
        Self { inner: node }
    }

    /// Formats the function with the specified `options`.
    ///
    /// # Errors
    ///
    /// Returns [`FormatError::PoorLayout`] if [`call_argument_layout`](FormatFunctionOptions::call_argument_layout] is `Some`
    /// and the function parameters contain some content that [*force a group to break*](FormatElements::will_break).
    ///
    /// This error is handled by [FormatRCallArguments].
    pub(crate) fn fmt_with_options(
        &self,
        f: &mut RFormatter,
        options: &FormatFunctionOptions,
    ) -> FormatResult<()> {
        let node = &self.inner;

        let name = node.name()?;
        let parameters = node.parameters()?;
        let body = node.body()?;

        let format_parameters = format_with(|f: &mut RFormatter| {
            if options.call_argument_layout.is_some() {
                // If we remove all soft line breaks, would the `parameters` still break?
                // If so, return `PoorLayout` and let `FormatRCallArguments` revert
                // the state using a snapshot.
                let mut buffer = RemoveSoftLinesBuffer::new(f);

                let mut recording = buffer.start_recording();
                write!(recording, [parameters.format()])?;
                let recorded = recording.stop();

                if recorded.will_break() {
                    return Err(FormatError::PoorLayout);
                }
            } else {
                parameters.format().fmt(f)?;
            }

            Ok(())
        });

        // The `group()` should contain all elements of the function definition,
        // which allows `FormatFunctionBody` to autobrace if any element forces a
        // break
        write!(
            f,
            [group(&format_args!(
                name.format(),
                &format_parameters,
                space(),
                FormatFunctionBody::new(&body)
            ))]
        )
    }
}

pub(crate) struct FormatFunctionBody<'a> {
    node: &'a AnyRExpression,
}

impl<'a> FormatFunctionBody<'a> {
    pub fn new(node: &'a AnyRExpression) -> Self {
        Self { node }
    }
}

impl Format<RFormatContext> for FormatFunctionBody<'_> {
    fn fmt(&self, f: &mut Formatter<RFormatContext>) -> FormatResult<()> {
        match self.node {
            // Body already has braces, just format it
            AnyRExpression::RBracedExpressions(node) => {
                write!(f, [node.format()])
            }
            // Body does not have braces yet
            node => {
                if should_force_braced_expressions(node)? {
                    write!(
                        f,
                        [
                            text("{"),
                            block_indent(&format_args![&node.format()]),
                            text("}")
                        ]
                    )
                } else {
                    write!(
                        f,
                        [
                            if_group_breaks(&text("{")),
                            soft_block_indent(&format_args![&node.format()]),
                            if_group_breaks(&text("}"))
                        ]
                    )
                }
            }
        }
    }
}

fn should_force_braced_expressions(node: &AnyRExpression) -> SyntaxResult<bool> {
    // A kind of persistent line break, but not one we respect
    // with the `persistent_line_breaks` setting, because it is
    // more related to whether or not we autobrace
    //
    // ```r
    // function()
    //   a
    // ```
    if node.syntax().has_leading_newline() {
        return Ok(true);
    }

    Ok(false)
}
