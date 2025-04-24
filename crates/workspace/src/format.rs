use std::io;
use std::path::Path;

use air_r_formatter::context::RFormatOptions;
use air_r_parser::Parse;
use air_r_parser::RParserOptions;
use thiserror::Error;

use crate::settings::FormatSettings;

/// Representation of a formatted file
#[derive(Debug)]
pub enum FormattedFile {
    /// The file was formatted.
    Changed(ChangedFile),
    /// The file was unchanged.
    Unchanged,
}

#[derive(Debug)]
pub struct ChangedFile {
    old: String,
    new: String,
}

#[derive(Error, Debug)]
pub enum FormatFileError {
    #[error(transparent)]
    Format(#[from] FormatSourceError),
    #[error(transparent)]
    Read(#[from] io::Error),
}

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

impl FormattedFile {
    pub fn is_changed(&self) -> bool {
        matches!(self, FormattedFile::Changed(_))
    }
}

impl ChangedFile {
    pub fn old(&self) -> &str {
        &self.old
    }

    #[allow(clippy::new_ret_no_self)]
    pub fn new(&self) -> &str {
        &self.new
    }
}

/// Formats a single file
///
/// Note that this does not normalize line endings! In the LSP we currently do normalize
/// line endings in `Document::new()` and on document updates because our protocol
/// converter functions expect them to be normalized to unix (we should look into relaxing
/// this). If you need to format an on-disk file from the LSP, think hard about using this
/// vs calling the pieces yourself so you can also call [line_ending::normalize()].
pub fn format_file<P: AsRef<Path>>(
    path: P,
    settings: &FormatSettings,
) -> Result<FormattedFile, FormatFileError> {
    let old = match std::fs::read_to_string(path.as_ref()) {
        Ok(old) => old,
        Err(err) => {
            return Err(FormatFileError::Read(err));
        }
    };

    let options = settings.to_format_options(&old);

    let new = match format_source(&old, options) {
        Ok(new) => new,
        Err(err) => return Err(FormatFileError::Format(err)),
    };

    match new {
        FormattedSource::Changed(new) => Ok(FormattedFile::Changed(ChangedFile { old, new })),
        FormattedSource::Unchanged => Ok(FormattedFile::Unchanged),
    }
}

/// Formats a vector of `source` code
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
