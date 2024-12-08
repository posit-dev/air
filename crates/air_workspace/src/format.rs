use std::fmt::Debug;

use air_r_formatter::context::RFormatOptions;
use air_r_parser::ParseError;
use air_r_parser::RParserOptions;
use biome_formatter::FormatError;
use biome_formatter::PrintError;
use thiserror::Error;

#[derive(Debug)]
pub enum FormattedSource {
    /// The source was formatted, and the [`String`] contains the transformed source code.
    Formatted(String),
    /// The source was unchanged.
    Unchanged,
}

#[derive(Error, Debug)]
pub enum FormatSourceError {
    #[error(transparent)]
    Parse(#[from] ParseError),
    #[error(transparent)]
    Format(#[from] FormatError),
    #[error(transparent)]
    Print(#[from] PrintError),
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
        let error = parsed.into_errors().into_iter().next().unwrap();
        return Err(error.into());
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
