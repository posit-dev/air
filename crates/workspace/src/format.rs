use air_r_formatter::context::RFormatOptions;
use air_r_parser::Parse;
use air_r_parser::RParserOptions;
use thiserror::Error;

#[derive(Debug)]
pub enum FormattedSource {
    /// The source was formatted, the [`String`] contains the transformed source code.
    Changed(String),
    /// The source was unchanged.
    Unchanged,
}

#[derive(Error, Debug)]
pub enum FormatSourceError {
    #[error(transparent)]
    Parse(#[from] air_r_parser::ParseError),
    #[error(transparent)]
    Format(#[from] biome_formatter::FormatError),
    #[error(transparent)]
    Print(#[from] biome_formatter::PrintError),
}

#[derive(Error, Debug)]
pub enum FormatParseError {
    #[error(transparent)]
    Format(#[from] biome_formatter::FormatError),
    #[error(transparent)]
    Print(#[from] biome_formatter::PrintError),
}

/// Formats a vector of `source` code
///
/// Note that this does not normalize line endings! In the LSP we currently do normalize
/// line endings in `Document::new()` and on document updates because our protocol
/// converter functions expect them to be normalized to unix (we should look into relaxing
/// this). If you need to format an on-disk file from the LSP, make sure you also call
/// [line_ending::normalize()] as required.
pub fn format_source(
    source: &str,
    options: RFormatOptions,
) -> std::result::Result<FormattedSource, FormatSourceError> {
    let parse = air_r_parser::parse(source, RParserOptions::default());

    if parse.has_error() {
        let error = parse.into_error().unwrap();
        return Err(error.into());
    }

    format_source_with_parse(source, &parse, options).map_err(|err| match err {
        FormatParseError::Format(err) => FormatSourceError::Format(err),
        FormatParseError::Print(err) => FormatSourceError::Print(err),
    })
}

/// Formats a vector of `source` code using a preexisting `parse` result
///
/// # Invariants
///
/// It is a logic error to pass a `source` that does not exactly correspond to `parse`.
///
/// This function will panic if you provide a `parse` with parse errors, it is up to
/// the caller to handle that case appropriately.
pub fn format_source_with_parse(
    source: &str,
    parse: &Parse,
    options: RFormatOptions,
) -> std::result::Result<FormattedSource, FormatParseError> {
    if parse.has_error() {
        panic!("Can't supply a `parse` with known errors.");
    }

    let formatted = air_r_formatter::format_node(options, &parse.syntax())?;
    let formatted = formatted.print()?;
    let formatted = formatted.into_code();

    if source.len() == formatted.len() && source == formatted.as_str() {
        Ok(FormattedSource::Unchanged)
    } else {
        Ok(FormattedSource::Changed(formatted))
    }
}
