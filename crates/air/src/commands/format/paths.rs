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
use workspace::discovery;
use workspace::discovery::discover_r_file_paths;
use workspace::resolve::PathResolver;
use workspace::settings::FormatSettings;
use workspace::settings::Settings;

use crate::ExitStatus;
use crate::color;
use crate::commands::format::ConfigurationMode;
use crate::commands::format::FormatMode;
use crate::commands::format::diagnostics::ParseFailure;
use crate::commands::format::diagnostics::print_parse_failures;
use crate::commands::format::resolve_settings;
use crate::commands::format::source;
use crate::commands::format::source::FormatSourceError;
use crate::commands::format::source::FormattedSource;

#[derive(Error, Debug)]
enum FormatPathError {
    Parse(ParseFailure),
    Format(PathBuf, biome_formatter::FormatError),
    Print(PathBuf, biome_formatter::PrintError),
    Read(PathBuf, io::Error),
    Write(PathBuf, io::Error),
    Ignore(#[from] ignore::Error),
}

pub(crate) fn format(
    paths: Vec<PathBuf>,
    mode: FormatMode,
    configuration: ConfigurationMode,
    exclude: discovery::Exclude,
    include: discovery::Include,
) -> anyhow::Result<ExitStatus> {
    let (resolver, default_settings) = resolve_settings(&paths, configuration)?;

    match mode {
        FormatMode::Write => {
            let errors = format_paths_write(&paths, &resolver, &default_settings, exclude, include);

            let has_errors = !errors.is_empty();
            report_errors(errors)?;

            if has_errors {
                Ok(ExitStatus::Error)
            } else {
                Ok(ExitStatus::Success)
            }
        }
        FormatMode::Check => {
            let (paths, errors) =
                format_paths_check(&paths, &resolver, &default_settings, exclude, include);

            let has_errors = !errors.is_empty();
            report_errors(errors)?;

            inform_changed(&paths, &mut stderr().lock())?;

            if has_errors {
                Ok(ExitStatus::Error)
            } else {
                if paths.is_empty() {
                    Ok(ExitStatus::Success)
                } else {
                    Ok(ExitStatus::Failure)
                }
            }
        }
    }
}

/// Renders parse failures as code frames, then logs everything else
fn report_errors(errors: Vec<FormatPathError>) -> io::Result<()> {
    let (mut failures, errors): (Vec<ParseFailure>, Vec<FormatPathError>) =
        errors.into_iter().partition_map(|error| match error {
            FormatPathError::Parse(failure) => Either::Left(failure),
            FormatPathError::Format(..) => Either::Right(error),
            FormatPathError::Print(..) => Either::Right(error),
            FormatPathError::Read(..) => Either::Right(error),
            FormatPathError::Write(..) => Either::Right(error),
            FormatPathError::Ignore(..) => Either::Right(error),
        });

    // Discovery runs in parallel so order isn't deterministic
    failures.sort_unstable_by(|lhs, rhs| lhs.path.cmp(&rhs.path));

    print_parse_failures(&failures, &mut color::stderr().lock())?;

    match failures.len() {
        0 => {}
        1 => tracing::error!("Failed to parse 1 file."),
        n => tracing::error!("Failed to parse {n} files."),
    }

    for error in &errors {
        tracing::error!("{error}");
    }

    Ok(())
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
    default_settings: &Settings,
    exclude: discovery::Exclude,
    include: discovery::Include,
) -> Vec<FormatPathError> {
    let paths = discover_r_file_paths(
        paths,
        resolver,
        default_settings,
        discovery::Mode::Format,
        exclude,
        include,
    );

    paths
        .into_iter()
        .filter_map(|path| match path {
            Ok(path) => {
                let settings = resolver
                    .resolve(&path)
                    .map_or(default_settings, |item| item.value());

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
    default_settings: &Settings,
    exclude: discovery::Exclude,
    include: discovery::Include,
) -> (Vec<PathBuf>, Vec<FormatPathError>) {
    let paths = discover_r_file_paths(
        paths,
        resolver,
        default_settings,
        discovery::Mode::Format,
        exclude,
        include,
    );

    paths
        .into_iter()
        .filter_map(|path| match path {
            Ok(path) => {
                let settings = resolver
                    .resolve(&path)
                    .map_or(default_settings, |item| item.value());

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

    let source = std::fs::read_to_string(path)
        .map_err(|error| FormatPathError::Read(path.to_path_buf(), error))?;

    let options = settings.to_format_options(&source);

    match source::format_source(&source, options) {
        Ok(formatted) => Ok(formatted),
        Err(FormatSourceError::Parse(diagnostics)) => Err(FormatPathError::Parse(ParseFailure {
            path: path.to_path_buf(),
            source,
            diagnostics,
        })),
        Err(FormatSourceError::Format(error)) => {
            Err(FormatPathError::Format(path.to_path_buf(), error))
        }
        Err(FormatSourceError::Print(error)) => {
            Err(FormatPathError::Print(path.to_path_buf(), error))
        }
    }
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
            // TODO(diagnostics): This is not actually used, and is a code smell!
            Self::Parse(failure) => {
                write!(
                    f,
                    "Failed to parse {path}",
                    path = relativize_path(&failure.path).underline()
                )
            }
            Self::Format(path, err) => write!(
                f,
                "Failed to format {path}: {err}",
                path = relativize_path(path).underline(),
            ),
            Self::Print(path, err) => write!(
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
