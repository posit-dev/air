use std::fmt::Display;
use std::fmt::Formatter;
use std::io;
use std::io::stdout;
use std::io::Write;
use std::path::PathBuf;

use air_r_formatter::context::RFormatOptions;
use air_workspace::format::format_source;
use air_workspace::format::FormatSourceError;
use air_workspace::format::FormattedSource;
use ignore::DirEntry;
use itertools::Either;
use itertools::Itertools;
use line_ending::LineEnding;

use crate::args::FormatCommand;
use crate::ExitStatus;

pub(crate) fn format(command: FormatCommand) -> anyhow::Result<ExitStatus> {
    let mode = FormatMode::from_command(&command);
    let paths = resolve_paths(&command.paths);

    let (results, errors): (Vec<_>, Vec<_>) = paths
        .into_iter()
        .map(|path| match path {
            Ok(path) => format_file(path, mode),
            Err(err) => Err(FormatCommandError::Ignore(err)),
        })
        .partition_map(|result| match result {
            Ok(result) => Either::Left(result),
            Err(err) => Either::Right(err),
        });

    for error in &errors {
        tracing::error!("{error}");
    }

    match mode {
        FormatMode::Write => {}
        FormatMode::Check => {
            write_changed(&results, &mut stdout().lock())?;
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
                let any_would_format = results
                    .iter()
                    .any(|result| matches!(result, FormatFileResult::Formatted(_)));

                if any_would_format {
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

fn write_changed(results: &[FormatFileResult], f: &mut impl Write) -> io::Result<()> {
    for path in results
        .iter()
        .filter_map(|result| match result {
            FormatFileResult::Formatted(path) => Some(path),
            FormatFileResult::Unchanged => None,
        })
        .sorted_unstable()
    {
        writeln!(f, "Would reformat: {}", path.display())?;
    }

    Ok(())
}

fn resolve_paths(paths: &[PathBuf]) -> Vec<Result<PathBuf, ignore::Error>> {
    let paths: Vec<PathBuf> = paths.iter().map(air_fs::normalize_path).collect();

    let (first_path, paths) = paths
        .split_first()
        .expect("Clap should ensure at least 1 path is supplied.");

    // TODO: Parallel directory visitor
    let mut builder = ignore::WalkBuilder::new(first_path);

    for path in paths {
        builder.add(path);
    }

    let mut out = Vec::new();

    for path in builder.build() {
        match path {
            Ok(entry) => {
                if let Some(path) = judge_entry(entry) {
                    out.push(Ok(path));
                }
            }
            Err(err) => {
                out.push(Err(err));
            }
        }
    }

    out
}

// Decide whether or not to accept an `entry` based on include/exclude rules.
// Non-R files are filtered out later on, this blindly accepts those.
fn judge_entry(entry: DirEntry) -> Option<PathBuf> {
    // Ignore directories
    if entry.file_type().map_or(true, |ft| ft.is_dir()) {
        return None;
    }

    // Accept all files that are passed-in directly as long as it is an R file
    if entry.depth() == 0 {
        let path = entry.into_path();

        if air_fs::has_r_extension(&path) {
            return Some(path);
        } else {
            return None;
        }
    }

    // Otherwise check if we should accept this entry
    // TODO: Many other checks based on user exclude/includes
    let path = entry.into_path();

    if !air_fs::has_r_extension(&path) {
        return None;
    }

    Some(path)
}

pub(crate) enum FormatFileResult {
    Formatted(PathBuf),
    Unchanged,
}

// TODO: Take workspace `FormatOptions` that get resolved to `RFormatOptions`
// for the formatter here. Respect user specified `LineEnding` option too, and
// only use inferred endings when `FormatOptions::LineEnding::Auto` is used.
fn format_file(path: PathBuf, mode: FormatMode) -> Result<FormatFileResult, FormatCommandError> {
    let source = std::fs::read_to_string(&path)
        .map_err(|err| FormatCommandError::Read(path.clone(), err))?;

    let line_ending = match line_ending::infer(&source) {
        LineEnding::Lf => biome_formatter::LineEnding::Lf,
        LineEnding::Crlf => biome_formatter::LineEnding::Crlf,
    };
    let options = RFormatOptions::default().with_line_ending(line_ending);

    let source = line_ending::normalize(source);
    let formatted = match format_source(source.as_str(), options) {
        Ok(formatted) => formatted,
        Err(err) => return Err(FormatCommandError::Format(path.clone(), err)),
    };

    // TODO: We rarely ever take advantage of this optimization on Windows right
    // now. We always normalize on entry but we apply the requested line ending
    // on exit (so on Windows we often infer CRLF on entry and normalize to
    // LF, but apply CRLF on exit so `source` and `new` always have different
    // line endings). We probably need to compare pre-normalized against
    // post-formatted output?
    match formatted {
        FormattedSource::Formatted(new) => {
            match mode {
                FormatMode::Write => {
                    std::fs::write(&path, new)
                        .map_err(|err| FormatCommandError::Write(path.clone(), err))?;
                }
                FormatMode::Check => {}
            }
            Ok(FormatFileResult::Formatted(path))
        }
        FormattedSource::Unchanged => Ok(FormatFileResult::Unchanged),
    }
}

pub(crate) enum FormatCommandError {
    Ignore(ignore::Error),
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
                        "Failed to format {}: {}",
                        path.display(),
                        err.io_error()
                            .map_or_else(|| err.to_string(), std::string::ToString::to_string)
                    )
                } else {
                    write!(
                        f,
                        "Encountered error: {error}",
                        error = err
                            .io_error()
                            .map_or_else(|| err.to_string(), std::string::ToString::to_string)
                    )
                }
            }
            Self::Read(path, err) => {
                write!(f, "Failed to read {}: {err}", path.display())
            }
            Self::Write(path, err) => {
                write!(f, "Failed to write {}: {err}", path.display())
            }
            Self::Format(path, err) => {
                write!(f, "Failed to format {}: {err}", path.display())
            }
        }
    }
}
