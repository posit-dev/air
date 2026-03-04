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
    let formatted = format_stdin(&settings.format)?;

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
    let formatted = format_stdin(&settings.format)?;

    match formatted {
        FormattedStdin::Changed(_) => Ok(true),
        FormattedStdin::Unchanged(_) => Ok(false),
    }
}

fn format_stdin(settings: &FormatSettings) -> Result<FormattedStdin, FormatStdinError> {
    tracing::trace!("Formatting stdin");

    let mut old = String::new();

    // Blocks until EOF is received!
    io::stdin()
        .lock()
        .read_to_string(&mut old)
        .map_err(FormatStdinError::Read)?;

    let options = settings.to_format_options(&old);

    let new = workspace::format::format_source(&old, options).map_err(FormatStdinError::Format)?;

    match new {
        FormattedSource::Changed(new) => Ok(FormattedStdin::Changed(new)),
        FormattedSource::Unchanged => Ok(FormattedStdin::Unchanged(old)),
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
