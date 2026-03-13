use std::fmt::Display;
use std::fmt::Formatter;
use std::io;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use thiserror::Error;
use workspace::discovery::DiscoveredSettings;
use workspace::discovery::discover_settings;
use workspace::format::FormatSourceError;
use workspace::format::FormattedSource;
use workspace::resolve::PathResolver;
use workspace::settings::FormatSettings;
use workspace::settings::Settings;

use crate::ExitStatus;
use crate::commands::format::FormatMode;

#[derive(Debug)]
enum FormattedStdin {
    /// Stdin was formatted.
    Changed(String),
    /// Stdin was unchanged.
    Unchanged(String),
}

#[derive(Error, Debug)]
enum FormatStdinError {
    Format(FormatSourceError),
    Read(io::Error),
    Write(io::Error),
}

pub(crate) fn format(path: PathBuf, mode: FormatMode) -> anyhow::Result<ExitStatus> {
    // Normalize up front, relative to current working directory
    let path = fs::normalize_path(path);

    let mut resolver = PathResolver::new(Settings::default());

    for DiscoveredSettings {
        directory,
        settings,
    } in discover_settings(&[&path])?
    {
        resolver.add(&directory, settings);
    }

    match mode {
        FormatMode::Write => match format_stdin_write(&path, &resolver) {
            Ok(()) => Ok(ExitStatus::Success),
            Err(error) => {
                tracing::error!("{error}");
                Ok(ExitStatus::Error)
            }
        },
        FormatMode::Check => match format_stdin_check(&path, &resolver) {
            Ok(changed) => {
                if changed {
                    Ok(ExitStatus::Failure)
                } else {
                    Ok(ExitStatus::Success)
                }
            }
            Err(error) => {
                tracing::error!("{error}");
                Ok(ExitStatus::Error)
            }
        },
    }
}

fn format_stdin_write<P: AsRef<Path>>(
    path: P,
    resolver: &PathResolver<Settings>,
) -> Result<(), FormatStdinError> {
    let settings = resolver.resolve_or_fallback(&path);

    let formatted = if is_stdin_formattable(path, settings) {
        format_stdin(&settings.format)?
    } else {
        asis_stdin()?
    };

    let buffer = match formatted {
        FormattedStdin::Changed(changed) => changed,
        FormattedStdin::Unchanged(unchanged) => unchanged,
    };

    std::io::stdout()
        .lock()
        .write_all(buffer.as_bytes())
        .map_err(FormatStdinError::Write)
}

fn format_stdin_check<P: AsRef<Path>>(
    path: P,
    resolver: &PathResolver<Settings>,
) -> Result<bool, FormatStdinError> {
    let settings = resolver.resolve_or_fallback(&path);

    if !is_stdin_formattable(path, settings) {
        // Don't even attempt to read from stdin, we know nothing will change
        return Ok(false);
    }

    let formatted = format_stdin(&settings.format)?;

    match formatted {
        FormattedStdin::Changed(_) => Ok(true),
        FormattedStdin::Unchanged(_) => Ok(false),
    }
}

fn format_stdin(settings: &FormatSettings) -> Result<FormattedStdin, FormatStdinError> {
    tracing::trace!("Formatting stdin");

    let old = read_stdin().map_err(FormatStdinError::Read)?;
    let options = settings.to_format_options(&old);
    let new = workspace::format::format_source(&old, options).map_err(FormatStdinError::Format)?;

    match new {
        FormattedSource::Changed(new) => Ok(FormattedStdin::Changed(new)),
        FormattedSource::Unchanged => Ok(FormattedStdin::Unchanged(old)),
    }
}

fn asis_stdin() -> Result<FormattedStdin, FormatStdinError> {
    tracing::trace!("Passing stdin through unformatted");

    let old = read_stdin().map_err(FormatStdinError::Read)?;

    Ok(FormattedStdin::Unchanged(old))
}

/// Read from stdin
///
/// Blocks until EOF is received!
fn read_stdin() -> io::Result<String> {
    let mut out = String::new();
    io::stdin().lock().read_to_string(&mut out)?;
    Ok(out)
}

/// Determine if stdin is formattable
///
/// Should be aligned with the behavior of [workspace::discovery::discover_r_file_paths()]
/// with a directly specified file path. In other words, these should follow the same
/// rules:
///
/// ```bash
/// air format path/to/this.R
/// air format --stdin-file-path path/to/this.R
/// ```
fn is_stdin_formattable<P: AsRef<Path>>(path: P, settings: &Settings) -> bool {
    const IS_DIRECTORY: bool = false;

    let path = path.as_ref();

    // `exclude` or `default_exclude` may exclude it.
    // Like `discover_r_file_paths()`, matches against the path and its parents, because
    // using stdin to format `renv/activate.R` should refuse to format due to our
    // `default_exclude` of `**/renv/`.
    if let Some(glob) = workspace::discovery::any_exclude_matched_path_or_any_parents(
        path,
        IS_DIRECTORY,
        settings.format.exclude.as_ref(),
        settings.format.default_exclude.as_ref(),
    ) {
        tracing::trace!(
            "Excluded due to '{glob}': {path}",
            glob = glob.original(),
            path = path.display()
        );
        return false;
    }

    // `default_include` must include it.
    // No need for `IS_DIRECTORY` since includes are only applicable for files.
    match workspace::discovery::any_include_matched_path(
        path,
        settings.format.default_include.as_ref(),
    ) {
        Some(glob) => {
            tracing::trace!(
                "Included due to '{glob}': {path}",
                glob = glob.original(),
                path = path.display()
            );
            true
        }
        None => {
            tracing::trace!(
                "Excluded due to not matching an include: {path}",
                path = path.display()
            );
            false
        }
    }
}

impl Display for FormatStdinError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Format(error) => write!(f, "Failed to format stdin: {error}"),
            Self::Read(error) => write!(f, "Failed to read from stdin: {error}"),
            Self::Write(error) => write!(f, "Failed to write to stdout: {error}"),
        }
    }
}

#[cfg(test)]
mod test {
    use tempfile::TempDir;
    use workspace::settings::Settings;

    use crate::commands::format::stdin::is_stdin_formattable;

    #[test]
    fn test_default_includes_respected_for_directly_supplied_paths() -> anyhow::Result<()> {
        let tempdir = TempDir::new()?;
        let tempdir = tempdir.path();

        let test_big_r = tempdir.join("test1.R");
        let test_little_r = tempdir.join("test2.r");
        let test_qmd = tempdir.join("test3.qmd");

        let settings = Settings::default();

        {
            // `{tempdir}/test1.R`
            // Part of default includes, so accepted
            assert!(is_stdin_formattable(test_big_r, &settings));
        }

        {
            // `{tempdir}/test2.r`
            // Part of default includes, so accepted
            assert!(is_stdin_formattable(test_little_r, &settings));
        }

        {
            // `{tempdir}/test3.qmd`
            // Not part of default includes, so rejected even though the user supplied it
            // directly.
            assert!(!is_stdin_formattable(test_qmd, &settings));
        }

        Ok(())
    }

    #[test]
    fn test_default_excludes_respected_for_directly_supplied_paths() -> anyhow::Result<()> {
        let tempdir = TempDir::new()?;
        let tempdir = tempdir.path();

        std::fs::create_dir(tempdir.join("R"))?;
        std::fs::create_dir(tempdir.join("renv"))?;

        // Exclude all of these
        std::fs::write(tempdir.join("R").join("cpp11.R"), b"")?;
        std::fs::write(tempdir.join("renv").join("activate.R"), b"")?;

        let settings = Settings::default();

        {
            // `{tempdir}/R/cpp11.R`
            // Directly supplied file matching default excludes is excluded
            let start = tempdir.join("R").join("cpp11.R");
            assert!(!is_stdin_formattable(start.as_path(), &settings));
        }

        {
            // `{tempdir}/renv/activate.R`
            // Directly supplied file with parent matching default excludes is excluded
            let start = tempdir.join("renv").join("activate.R");
            assert!(!is_stdin_formattable(start.as_path(), &settings));
        }

        Ok(())
    }
}
