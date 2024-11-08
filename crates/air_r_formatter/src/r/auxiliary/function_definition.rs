use crate::prelude::*;
use crate::statement_body::FormatStatementBody;
use air_r_syntax::RFunctionDefinition;
use biome_formatter::write;
use biome_formatter::FormatRuleWithOptions;
use biome_formatter::RemoveSoftLinesBuffer;

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

        write!(
            f,
            [
                name.format(),
                group(&format_parameters),
                group(&FormatStatementBody::new(&body))
            ]
        )
    }
}
