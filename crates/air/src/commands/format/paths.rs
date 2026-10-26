use std::io;
use std::path::Path;
use std::path::PathBuf;

use biome_console::Console;
use biome_diagnostics::Error;
use workspace::discovery;
use workspace::discovery::discover_r_file_paths;
use workspace::resolve::PathResolver;
use workspace::settings::FormatSettings;
use workspace::settings::Settings;

use crate::ExitStatus;
use crate::commands::format::ConfigurationMode;
use crate::commands::format::FormatMode;
use crate::commands::format::resolve_settings;
use crate::commands::format::source;
use crate::commands::format::source::FormatSourceError;
use crate::commands::format::source::FormattedSource;
use crate::diagnostics::FormatDiagnostic;
use crate::diagnostics::ReadDiagnostic;
use crate::diagnostics::WalkDiagnostic;
use crate::diagnostics::WriteDiagnostic;
use crate::diagnostics::from_discover_settings_error;
use crate::diagnostics::from_parse_diagnostics;
use crate::diagnostics::print_diagnostics;
use crate::diagnostics::print_reformatted_diagnostic;

pub(crate) fn format(
    paths: Vec<PathBuf>,
    mode: FormatMode,
    configuration: ConfigurationMode,
    exclude: discovery::Exclude,
    include: discovery::Include,
    console: &mut dyn Console,
) -> ExitStatus {
    let (resolver, default_settings) = match resolve_settings(&paths, configuration) {
        Ok((resolver, default_settings)) => (resolver, default_settings),
        Err(error) => {
            print_diagnostics(console, vec![from_discover_settings_error(error)]);
            return ExitStatus::Error;
        }
    };

    match mode {
        FormatMode::Write => {
            let diagnostics =
                format_paths_write(&paths, &resolver, &default_settings, exclude, include);

            let status = if !diagnostics.is_empty() {
                ExitStatus::Error
            } else {
                ExitStatus::Success
            };

            print_diagnostics(console, diagnostics);

            status
        }
        FormatMode::Check => {
            let (paths, diagnostics) =
                format_paths_check(&paths, &resolver, &default_settings, exclude, include);

            let status = if !diagnostics.is_empty() {
                ExitStatus::Error
            } else if !paths.is_empty() {
                ExitStatus::Failure
            } else {
                ExitStatus::Success
            };

            print_diagnostics(console, diagnostics);
            print_reformatted_diagnostic(console, &paths);

            status
        }
    }
}

fn format_paths_write<P: AsRef<Path>>(
    paths: &[P],
    resolver: &PathResolver<Settings>,
    default_settings: &Settings,
    exclude: discovery::Exclude,
    include: discovery::Include,
) -> Vec<Error> {
    let paths = discover_r_file_paths(
        paths,
        resolver,
        default_settings,
        discovery::Mode::Format,
        exclude,
        include,
    );

    let mut diagnostics = Vec::new();

    for path in paths {
        let path = match path {
            Ok(path) => path,
            Err(error) => {
                diagnostics.push(WalkDiagnostic::new(error).into());
                continue;
            }
        };

        let settings = resolver
            .resolve(&path)
            .map_or(default_settings, |item| item.value());

        let formatted = match format_path(&path, &settings.format) {
            Ok(formatted) => formatted,
            Err(errors) => {
                diagnostics.extend(errors);
                continue;
            }
        };

        if let Err(error) = write_path(&path, formatted) {
            diagnostics.push(WriteDiagnostic::new(&path, error).into());
        }
    }

    diagnostics
}

fn format_paths_check<P: AsRef<Path>>(
    paths: &[P],
    resolver: &PathResolver<Settings>,
    default_settings: &Settings,
    exclude: discovery::Exclude,
    include: discovery::Include,
) -> (Vec<PathBuf>, Vec<Error>) {
    let paths = discover_r_file_paths(
        paths,
        resolver,
        default_settings,
        discovery::Mode::Format,
        exclude,
        include,
    );

    let mut changed = Vec::new();
    let mut diagnostics = Vec::new();

    for path in paths {
        let path = match path {
            Ok(path) => path,
            Err(error) => {
                diagnostics.push(WalkDiagnostic::new(error).into());
                continue;
            }
        };

        let settings = resolver
            .resolve(&path)
            .map_or(default_settings, |item| item.value());

        let formatted = match format_path(&path, &settings.format) {
            Ok(formatted) => formatted,
            Err(errors) => {
                diagnostics.extend(errors);
                continue;
            }
        };

        if let Some(path) = check_path(&path, formatted) {
            changed.push(path);
        }
    }

    (changed, diagnostics)
}

fn format_path<P: AsRef<Path>>(
    path: P,
    settings: &FormatSettings,
) -> std::result::Result<FormattedSource, Vec<Error>> {
    let path = path.as_ref();
    tracing::trace!("Formatting {path}", path = path.display());

    let source = std::fs::read_to_string(path)
        .map_err(|error| vec![ReadDiagnostic::new(path, error).into()])?;

    let options = settings.to_format_options(&source);

    match source::format_source(&source, options) {
        Ok(formatted) => Ok(formatted),
        Err(FormatSourceError::Parse(diagnostics)) => {
            Err(from_parse_diagnostics(path, &source, diagnostics))
        }
        Err(FormatSourceError::Format(error)) => {
            Err(vec![FormatDiagnostic::new(path, error).into()])
        }
        Err(FormatSourceError::Print(error)) => {
            Err(vec![FormatDiagnostic::new(path, error).into()])
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
