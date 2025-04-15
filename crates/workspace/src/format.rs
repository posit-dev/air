use std::fmt::Display;
use std::fmt::Formatter;
use std::io;
use std::path::Path;
use std::path::PathBuf;
use thiserror::Error;

use air_r_formatter::context::RFormatOptions;
use air_r_parser::RParserOptions;
use colored::Colorize;
use fs::relativize_path;
use itertools::Either;
use itertools::Itertools;

use crate::discovery::discover_r_file_paths;
use crate::resolve::PathResolver;
use crate::settings::FormatSettings;
use crate::settings::Settings;

/// Representation of a formatted file
#[derive(Debug)]
pub enum FormattedFile {
    /// The file was formatted. the [`String`] contains the transformed source code,
    /// and the [PathBuf] contains the file name.
    Changed(PathBuf, String),
    /// The file was unchanged.
    Unchanged,
}

#[derive(Error, Debug)]
pub enum FormatFileError {
    Ignore(#[from] ignore::Error),
    Format(PathBuf, FormatSourceError),
    Read(PathBuf, io::Error),
}

#[derive(Debug)]
enum FormattedSource {
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

pub fn format_paths<P: AsRef<Path>>(
    paths: &[P],
    resolver: &PathResolver<Settings>,
) -> (Vec<FormattedFile>, Vec<FormatFileError>) {
    let paths = discover_r_file_paths(paths, resolver, true);

    paths
        .into_iter()
        .map(|path| match path {
            Ok(path) => {
                let settings = resolver.resolve_or_fallback(&path);
                format_file(path, &settings.format)
            }
            Err(err) => Err(err.into()),
        })
        .partition_map(|result| match result {
            Ok(result) => Either::Left(result),
            Err(err) => Either::Right(err),
        })
}

impl FormattedFile {
    pub fn is_changed(&self) -> bool {
        matches!(self, FormattedFile::Changed(_, _))
    }
}

impl Display for FormatFileError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ignore(err) => {
                if let ignore::Error::WithPath { path, .. } = err {
                    write!(
                        f,
                        "Failed to format {path}: {err}",
                        path = relativize_path(path).underline(),
                        err = err
                            .io_error()
                            .map_or_else(|| err.to_string(), std::string::ToString::to_string)
                    )
                } else {
                    write!(
                        f,
                        "Encountered error: {err}",
                        err = err
                            .io_error()
                            .map_or_else(|| err.to_string(), std::string::ToString::to_string)
                    )
                }
            }
            Self::Read(path, err) => {
                write!(
                    f,
                    "Failed to read {path}: {err}",
                    path = relativize_path(path).underline(),
                )
            }
            Self::Format(path, err) => {
                write!(
                    f,
                    "Failed to format {path}: {err}",
                    path = relativize_path(path).underline(),
                )
            }
        }
    }
}

/// Formats a single file
fn format_file(path: PathBuf, settings: &FormatSettings) -> Result<FormattedFile, FormatFileError> {
    tracing::trace!("Formatting {path}", path = path.display());

    let source = match std::fs::read_to_string(&path) {
        Ok(source) => source,
        Err(err) => {
            return Err(FormatFileError::Read(path, err));
        }
    };

    let options = settings.to_format_options(&source);

    let formatted = match format_source(source.as_str(), options) {
        Ok(formatted) => formatted,
        Err(err) => return Err(FormatFileError::Format(path, err)),
    };

    match formatted {
        FormattedSource::Changed(formatted) => Ok(FormattedFile::Changed(path, formatted)),
        FormattedSource::Unchanged => Ok(FormattedFile::Unchanged),
    }
}

/// Formats a vector of `source` code
fn format_source(
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
        Ok(FormattedSource::Changed(formatted))
    }
}
