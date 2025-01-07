use std::fmt::Display;
use std::fmt::Formatter;
use std::io;
use std::io::stdout;
use std::io::Write;
use std::path::PathBuf;

use air_r_formatter::context::RFormatOptions;
use air_r_parser::RParserOptions;
use fs::relativize_path;
use itertools::Either;
use itertools::Itertools;
use thiserror::Error;
use workspace::discovery::discover_r_file_paths;
use workspace::discovery::discover_settings;
use workspace::discovery::DiscoveredSettings;
use workspace::resolve::PathResolver;
use workspace::settings::FormatSettings;
use workspace::settings::Settings;

use crate::args::FormatCommand;
use crate::ExitStatus;

pub(crate) fn format(command: FormatCommand) -> anyhow::Result<ExitStatus> {
    let mode = FormatMode::from_command(&command);

    let paths = discover_r_file_paths(&command.paths);

    let mut resolver = PathResolver::new(Settings::default());

    for DiscoveredSettings {
        directory,
        settings,
    } in discover_settings(&command.paths)?
    {
        resolver.add(&directory, settings);
    }

    let (actions, errors): (Vec<_>, Vec<_>) = paths
        .into_iter()
        .map(|path| match path {
            Ok(path) => {
                let settings = resolver.resolve_or_fallback(&path);
                format_file(path, mode, &settings.format)
            }
            Err(err) => Err(err.into()),
        })
        .partition_map(|result| match result {
            Ok(result) => Either::Left(result),
            Err(err) => Either::Right(err),
        });

    for error in &errors {
        // TODO: Hook up a tracing subscriber!
        tracing::error!("{error}");
    }

    match mode {
        FormatMode::Write => {}
        FormatMode::Check => {
            write_changed(&actions, &mut stdout().lock())?;
        }
    }

    match mode {
        FormatMode::Write => {
            if errors.is_empty() {
                Ok(ExitStatus::Success)
            } else {
                Ok(ExitStatus::Error)
            }
        }
        FormatMode::Check => {
            if errors.is_empty() {
                let any_changed = actions.iter().any(FormatFileAction::is_changed);

                if any_changed {
                    Ok(ExitStatus::Failure)
                } else {
                    Ok(ExitStatus::Success)
                }
            } else {
                Ok(ExitStatus::Error)
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum FormatMode {
    Write,
    Check,
}

impl FormatMode {
    fn from_command(command: &FormatCommand) -> Self {
        if command.check {
            FormatMode::Check
        } else {
            FormatMode::Write
        }
    }
}

fn write_changed(actions: &[FormatFileAction], f: &mut impl Write) -> io::Result<()> {
    for path in actions
        .iter()
        .filter_map(|result| match result {
            FormatFileAction::Formatted(path) => Some(path),
            FormatFileAction::Unchanged => None,
        })
        .sorted_unstable()
    {
        writeln!(f, "Would reformat: {}", path.display())?;
    }

    Ok(())
}

pub(crate) enum FormatFileAction {
    Formatted(PathBuf),
    Unchanged,
}

impl FormatFileAction {
    fn is_changed(&self) -> bool {
        matches!(self, FormatFileAction::Formatted(_))
    }
}

fn format_file(
    path: PathBuf,
    mode: FormatMode,
    settings: &FormatSettings,
) -> Result<FormatFileAction, FormatCommandError> {
    let source = std::fs::read_to_string(&path)
        .map_err(|err| FormatCommandError::Read(path.clone(), err))?;

    let options = settings.to_format_options(&source);

    let formatted = match format_source(source.as_str(), options) {
        Ok(formatted) => formatted,
        Err(err) => return Err(FormatCommandError::Format(path.clone(), err)),
    };

    match formatted {
        FormattedSource::Formatted(new) => {
            match mode {
                FormatMode::Write => {
                    std::fs::write(&path, new)
                        .map_err(|err| FormatCommandError::Write(path.clone(), err))?;
                }
                FormatMode::Check => {}
            }
            Ok(FormatFileAction::Formatted(path))
        }
        FormattedSource::Unchanged => Ok(FormatFileAction::Unchanged),
    }
}

#[derive(Debug)]
pub(crate) enum FormattedSource {
    /// The source was formatted, and the [`String`] contains the transformed source code.
    Formatted(String),
    /// The source was unchanged.
    Unchanged,
}

#[derive(Error, Debug)]
pub(crate) enum FormatSourceError {
    #[error(transparent)]
    Parse(#[from] air_r_parser::ParseError),
    #[error(transparent)]
    Format(#[from] biome_formatter::FormatError),
    #[error(transparent)]
    Print(#[from] biome_formatter::PrintError),
}

/// Formats a vector of `source` code
pub(crate) fn format_source(
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

#[derive(Error, Debug)]
pub(crate) enum FormatCommandError {
    Ignore(#[from] ignore::Error),
    Format(PathBuf, FormatSourceError),
    Read(PathBuf, io::Error),
    Write(PathBuf, io::Error),
}

impl Display for FormatCommandError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ignore(err) => {
                if let ignore::Error::WithPath { path, .. } = err {
                    write!(
                        f,
                        "Failed to format {path}: {err}",
                        path = relativize_path(path),
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
                    path = relativize_path(path),
                )
            }
            Self::Write(path, err) => {
                write!(
                    f,
                    "Failed to write {path}: {err}",
                    path = relativize_path(path),
                )
            }
            Self::Format(path, err) => {
                write!(
                    f,
                    "Failed to format {path}: {err}",
                    path = relativize_path(path),
                )
            }
        }
    }
}
