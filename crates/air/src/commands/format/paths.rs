use std::fmt::Display;
use std::fmt::Formatter;
use std::io;
use std::io::Write;
use std::io::stderr;
use std::path::Path;
use std::path::PathBuf;

use colored::Colorize;
use fs::relativize_path;
use itertools::Either;
use itertools::Itertools;
use thiserror::Error;
use workspace::discovery::DiscoveredSettings;
use workspace::discovery::discover_r_file_paths;
use workspace::discovery::discover_settings;
use workspace::format::FormatSourceError;
use workspace::format::FormattedSource;
use workspace::resolve::PathResolver;
use workspace::settings::FormatSettings;
use workspace::settings::Settings;

use crate::ExitStatus;
use crate::commands::format::FormatMode;

#[derive(Error, Debug)]
enum FormatPathError {
    Format(PathBuf, FormatSourceError),
    Read(PathBuf, io::Error),
    Write(PathBuf, io::Error),
    Ignore(#[from] ignore::Error),
}

pub(crate) fn format(paths: Vec<PathBuf>, mode: FormatMode) -> anyhow::Result<ExitStatus> {
    let mut resolver = PathResolver::new(Settings::default());

    for DiscoveredSettings {
        directory,
        settings,
    } in discover_settings(&paths)?
    {
        resolver.add(&directory, settings);
    }

    match mode {
        FormatMode::Write => {
            let errors = format_paths_write(&paths, &resolver);

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
            let (paths, errors) = format_paths_check(&paths, &resolver);

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
                    Ok(formatted) => match write_path(&path, formatted) {
                        Ok(()) => None,
                        Err(err) => Some(FormatPathError::Write(path, err)),
                    },
                    Err(err) => Some(err),
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
                    Ok(file) => check_path(&path, file).map(Ok),
                    Err(err) => Some(Err(err)),
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
) -> std::result::Result<FormattedSource, FormatPathError> {
    let path = path.as_ref();
    tracing::trace!("Formatting {path}", path = path.display());

    let old = std::fs::read_to_string(path)
        .map_err(|error| FormatPathError::Read(path.to_path_buf(), error))?;

    let options = settings.to_format_options(&old);

    let new = workspace::format::format_source(&old, options)
        .map_err(|error| FormatPathError::Format(path.to_path_buf(), error))?;

    Ok(new)
}

/// Returns `Ok(())` if the format results were successfully written back, otherwise
/// returns an error
fn write_path<P: AsRef<Path>>(path: P, formatted: FormattedSource) -> io::Result<()> {
    match formatted {
        FormattedSource::Changed(changed) => std::fs::write(path, &changed),
        FormattedSource::Unchanged => Ok(()),
    }
}

/// Returns `Some(path)` if a change occurred, otherwise returns `None`
fn check_path<P: AsRef<Path>>(path: P, formatted: FormattedSource) -> Option<PathBuf> {
    match formatted {
        FormattedSource::Changed(_) => Some(path.as_ref().to_path_buf()),
        FormattedSource::Unchanged => None,
    }
}

impl Display for FormatPathError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Format(path, err) => write!(
                f,
                "Failed to format {path}: {err}",
                path = relativize_path(path).underline(),
            ),
            Self::Read(path, err) => write!(
                f,
                "Failed to read {path}: {err}",
                path = relativize_path(path).underline(),
            ),
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
            Self::Write(path, err) => write!(
                f,
                "Failed to write {path}: {err}",
                path = relativize_path(path).underline(),
            ),
        }
    }
}
