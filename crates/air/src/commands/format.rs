use std::fmt::Display;
use std::fmt::Formatter;
use std::io;
use std::io::stderr;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use colored::Colorize;
use fs::relativize_path;
use itertools::Either;
use itertools::Itertools;
use thiserror::Error;
use workspace::discovery::discover_r_file_paths;
use workspace::discovery::discover_settings;
use workspace::discovery::DiscoveredSettings;
use workspace::format::format_file;
use workspace::format::FormatFileError;
use workspace::format::FormattedFile;
use workspace::resolve::PathResolver;
use workspace::settings::FormatSettings;
use workspace::settings::Settings;

use crate::args::FormatCommand;
use crate::ExitStatus;

#[derive(Copy, Clone, Debug)]
enum FormatMode {
    Write,
    Check,
}

#[derive(Error, Debug)]
enum FormatPathError {
    FormatFile(PathBuf, FormatFileError),
    Write(PathBuf, io::Error),
    Ignore(#[from] ignore::Error),
}

pub(crate) fn format(command: FormatCommand) -> anyhow::Result<ExitStatus> {
    let mode = FormatMode::from_command(&command);

    let mut resolver = PathResolver::new(Settings::default());

    for DiscoveredSettings {
        directory,
        settings,
    } in discover_settings(&command.paths)?
    {
        resolver.add(&directory, settings);
    }

    match mode {
        FormatMode::Write => {
            let errors = format_paths_write(&command.paths, &resolver);

            for error in &errors {
                tracing::error!("{error}");
            }

            if errors.is_empty() {
                Ok(ExitStatus::Success)
            } else {
                Ok(ExitStatus::Error)
            }
        }
        FormatMode::Check => {
            let (paths, errors) = format_paths_check(&command.paths, &resolver);

            for error in &errors {
                tracing::error!("{error}");
            }

            inform_changed(&paths, &mut stderr().lock())?;

            if errors.is_empty() {
                if paths.is_empty() {
                    Ok(ExitStatus::Success)
                } else {
                    Ok(ExitStatus::Failure)
                }
            } else {
                Ok(ExitStatus::Error)
            }
        }
    }
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

fn inform_changed(paths: &[PathBuf], f: &mut impl Write) -> io::Result<()> {
    for path in paths.iter().sorted_unstable() {
        writeln!(
            f,
            "Would reformat: {path}",
            path = relativize_path(path).underline()
        )?;
    }
    Ok(())
}

fn format_paths_write<P: AsRef<Path>>(
    paths: &[P],
    resolver: &PathResolver<Settings>,
) -> Vec<FormatPathError> {
    let paths = discover_r_file_paths(paths, resolver, true);

    paths
        .into_iter()
        .filter_map(|path| match path {
            Ok(path) => {
                let settings = resolver.resolve_or_fallback(&path);
                match format_path(&path, &settings.format) {
                    Ok(file) => match write_path(&path, file) {
                        Ok(()) => None,
                        Err(err) => Some(FormatPathError::Write(path, err)),
                    },
                    Err(err) => Some(FormatPathError::FormatFile(path, err)),
                }
            }
            Err(err) => Some(err.into()),
        })
        .collect()
}

fn format_paths_check<P: AsRef<Path>>(
    paths: &[P],
    resolver: &PathResolver<Settings>,
) -> (Vec<PathBuf>, Vec<FormatPathError>) {
    let paths = discover_r_file_paths(paths, resolver, true);

    paths
        .into_iter()
        .filter_map(|path| match path {
            Ok(path) => {
                let settings = resolver.resolve_or_fallback(&path);
                match format_path(&path, &settings.format) {
                    Ok(file) => check_path(path, file).map(Ok),
                    Err(err) => Some(Err(FormatPathError::FormatFile(path, err))),
                }
            }
            Err(err) => Some(Err(err.into())),
        })
        .partition_map(|result| match result {
            Ok(result) => Either::Left(result),
            Err(err) => Either::Right(err),
        })
}

fn format_path<P: AsRef<Path>>(
    path: P,
    settings: &FormatSettings,
) -> std::result::Result<FormattedFile, FormatFileError> {
    let path = path.as_ref();
    tracing::trace!("Formatting {path}", path = path.display());
    format_file(path, settings)
}

/// Returns `Ok(())` if the format results were successfully written back, otherwise
/// returns an error
fn write_path<P: AsRef<Path>>(path: P, file: FormattedFile) -> io::Result<()> {
    match file {
        FormattedFile::Changed(file) => std::fs::write(path, file.new()),
        FormattedFile::Unchanged => Ok(()),
    }
}

/// Returns `Some(path)` if a change occurred, otherwise returns `None`
fn check_path(path: PathBuf, file: FormattedFile) -> Option<PathBuf> {
    match file {
        FormattedFile::Changed(_) => Some(path),
        FormattedFile::Unchanged => None,
    }
}

impl Display for FormatPathError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FormatFile(path, err) => match err {
                FormatFileError::Format(err) => write!(
                    f,
                    "Failed to format {path}: {err}",
                    path = relativize_path(path).underline(),
                ),
                FormatFileError::Read(err) => write!(
                    f,
                    "Failed to read {path}: {err}",
                    path = relativize_path(path).underline(),
                ),
            },
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
            Self::Write(path, err) => {
                write!(
                    f,
                    "Failed to write {path}: {err}",
                    path = relativize_path(path).underline(),
                )
            }
        }
    }
}
