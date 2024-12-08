use std::fmt::Display;
use std::fmt::Formatter;

use air_r_formatter::context::RFormatOptions;
use air_r_parser::RParserOptions;
use biome_diagnostics::Diagnostic;
use biome_formatter::FormatError;
use biome_formatter::PrintError;
use biome_parser::prelude::ParseDiagnostic;

#[derive(Debug)]
pub enum FormattedSource {
    /// The source was formatted, and the [`String`] contains the transformed source code.
    Formatted(String),
    /// The source was unchanged.
    Unchanged,
}

#[derive(Debug, Diagnostic)]
pub enum FormatSourceError {
    Parse(ParseDiagnostic),
    Format(FormatError),
    Print(PrintError),
}

impl Display for FormatSourceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FormatSourceError::Parse(err) => err.description(f),
            FormatSourceError::Format(err) => err.description(f),
            FormatSourceError::Print(err) => err.description(f),
        }
    }
}

impl From<ParseDiagnostic> for FormatSourceError {
    fn from(value: ParseDiagnostic) -> Self {
        FormatSourceError::Parse(value)
    }
}

impl From<FormatError> for FormatSourceError {
    fn from(value: FormatError) -> Self {
        FormatSourceError::Format(value)
    }
}

impl From<PrintError> for FormatSourceError {
    fn from(value: PrintError) -> Self {
        FormatSourceError::Print(value)
    }
}

/// Formats a vector of `source` code
///
/// Safety: `source` should already be normalized to Unix line endings
pub fn format_source(
    source: &str,
    options: RFormatOptions,
) -> std::result::Result<FormattedSource, FormatSourceError> {
    let parsed = air_r_parser::parse(source, RParserOptions::default());

    if parsed.has_errors() {
        let diagnostic = parsed.into_diagnostics().into_iter().next().unwrap();
        return Err(diagnostic.into());
    }

    let formatted = air_r_formatter::format_node(options, &parsed.syntax())?;
    let formatted = formatted.print()?;
    let formatted = formatted.into_code();

    if source.len() == formatted.len() && source == formatted.as_str() {
        Ok(FormattedSource::Unchanged)
    } else {
        Ok(FormattedSource::Formatted(formatted))
    }
}
