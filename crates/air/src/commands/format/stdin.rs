use std::io;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use biome_console::Console;
use biome_diagnostics::Error;
use workspace::discovery;
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
use crate::diagnostics::StdinReadDiagnostic;
use crate::diagnostics::StdoutWriteDiagnostic;
use crate::diagnostics::from_parse_diagnostics;
use crate::diagnostics::print_diagnostics;

#[derive(Debug)]
enum FormattedStdin {
    /// Stdin was formatted.
    Changed(String),
    /// Stdin was unchanged.
    Unchanged(String),
}

pub(crate) fn format(
    path: PathBuf,
    mode: FormatMode,
    configuration: ConfigurationMode,
    exclude: discovery::Exclude,
    include: discovery::Include,
    console: &mut dyn Console,
) -> anyhow::Result<ExitStatus> {
    // Normalize up front, relative to current working directory
    let path = fs::normalize_path(path);

    let (resolver, default_settings) = resolve_settings(&[&path], configuration)?;

    match mode {
        FormatMode::Write => {
            match format_stdin_write(&path, &resolver, &default_settings, exclude, include) {
                Ok(()) => Ok(ExitStatus::Success),
                Err(diagnostics) => {
                    print_diagnostics(console, diagnostics);
                    Ok(ExitStatus::Error)
                }
            }
        }
        FormatMode::Check => {
            match format_stdin_check(&path, &resolver, &default_settings, exclude, include) {
                Ok(changed) => {
                    if changed {
                        // Don't use `print_reformatted_diagnostic()`, it
                        // doesn't add any information in stdin mode
                        Ok(ExitStatus::Failure)
                    } else {
                        Ok(ExitStatus::Success)
                    }
                }
                Err(diagnostics) => {
                    print_diagnostics(console, diagnostics);
                    Ok(ExitStatus::Error)
                }
            }
        }
    }
}

fn format_stdin_write<P: AsRef<Path>>(
    path: P,
    resolver: &PathResolver<Settings>,
    default_settings: &Settings,
    exclude: discovery::Exclude,
    include: discovery::Include,
) -> Result<(), Vec<Error>> {
    let settings = resolver
        .resolve(&path)
        .map_or(default_settings, |item| item.value());

    let formatted = if is_stdin_formattable(&path, settings, exclude, include) {
        format_stdin(&path, &settings.format)?
    } else {
        asis_stdin()?
    };

    let buffer = match formatted {
        FormattedStdin::Changed(changed) => changed,
        FormattedStdin::Unchanged(unchanged) => unchanged,
    };

    // Never through the `console`, which would "sanitize" the formatted code
    std::io::stdout()
        .lock()
        .write_all(buffer.as_bytes())
        .map_err(|error| vec![StdoutWriteDiagnostic::new(error).into()])
}

fn format_stdin_check<P: AsRef<Path>>(
    path: P,
    resolver: &PathResolver<Settings>,
    default_settings: &Settings,
    exclude: discovery::Exclude,
    include: discovery::Include,
) -> Result<bool, Vec<Error>> {
    let settings = resolver
        .resolve(&path)
        .map_or(default_settings, |item| item.value());

    if !is_stdin_formattable(&path, settings, exclude, include) {
        // Don't even attempt to read from stdin, we know nothing will change
        return Ok(false);
    }

    let formatted = format_stdin(&path, &settings.format)?;

    match formatted {
        FormattedStdin::Changed(_) => Ok(true),
        FormattedStdin::Unchanged(_) => Ok(false),
    }
}

fn format_stdin<P: AsRef<Path>>(
    path: P,
    settings: &FormatSettings,
) -> Result<FormattedStdin, Vec<Error>> {
    tracing::trace!("Formatting stdin");

    let path = path.as_ref();

    let source = read_stdin().map_err(|error| vec![StdinReadDiagnostic::new(error).into()])?;

    let options = settings.to_format_options(&source);

    match source::format_source(&source, options) {
        Ok(FormattedSource::Changed(formatted)) => Ok(FormattedStdin::Changed(formatted)),
        Ok(FormattedSource::Unchanged) => Ok(FormattedStdin::Unchanged(source)),
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

fn asis_stdin() -> Result<FormattedStdin, Vec<Error>> {
    tracing::trace!("Passing stdin through unformatted");

    let old = read_stdin().map_err(|error| vec![StdinReadDiagnostic::new(error).into()])?;

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
fn is_stdin_formattable<P: AsRef<Path>>(
    path: P,
    settings: &Settings,
    exclude: discovery::Exclude,
    include: discovery::Include,
) -> bool {
    const IS_DIRECTORY: bool = false;

    let path = path.as_ref();

    match exclude {
        discovery::Exclude::Matched => {
            // `exclude` or `default_exclude` may exclude it. Like
            // `discover_r_file_paths()`, matches against the path and its parents,
            // because using stdin to format `renv/activate.R` should refuse to format due
            // to our `default_exclude` of `**/renv/`.
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
        }
        discovery::Exclude::Nothing => {
            // Exclusion patterns are not considered
        }
    }

    match include {
        discovery::Include::Matched => {
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
        discovery::Include::Everything => {
            tracing::trace!(
                "Included due to including everything: {path}",
                path = path.display()
            );
            true
        }
    }
}

#[cfg(test)]
mod test {
    use tempfile::TempDir;
    use workspace::discovery;
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
        let exclude = discovery::Exclude::Matched;
        let include = discovery::Include::Matched;

        {
            // `{tempdir}/test1.R`
            // Part of default includes, so accepted
            assert!(is_stdin_formattable(
                test_big_r, &settings, exclude, include
            ));
        }

        {
            // `{tempdir}/test2.r`
            // Part of default includes, so accepted
            assert!(is_stdin_formattable(
                test_little_r,
                &settings,
                exclude,
                include
            ));
        }

        {
            // `{tempdir}/test3.qmd`
            // Not part of default includes, so rejected even though the user supplied it
            // directly.
            assert!(!is_stdin_formattable(test_qmd, &settings, exclude, include));
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
        let exclude = discovery::Exclude::Matched;
        let include = discovery::Include::Matched;

        {
            // `{tempdir}/R/cpp11.R`
            // Directly supplied file matching default excludes is excluded
            let start = tempdir.join("R").join("cpp11.R");
            assert!(!is_stdin_formattable(
                start.as_path(),
                &settings,
                exclude,
                include
            ));
        }

        {
            // `{tempdir}/renv/activate.R`
            // Directly supplied file with parent matching default excludes is excluded
            let start = tempdir.join("renv").join("activate.R");
            assert!(!is_stdin_formattable(
                start.as_path(),
                &settings,
                exclude,
                include
            ));
        }

        Ok(())
    }

    #[test]
    fn test_exclude_nothing_allows_default_excluded_files() -> anyhow::Result<()> {
        let tempdir = TempDir::new()?;
        let tempdir = tempdir.path();

        std::fs::create_dir(tempdir.join("R"))?;
        std::fs::create_dir(tempdir.join("renv"))?;

        let settings = Settings::default();
        let exclude = discovery::Exclude::Nothing;
        let include = discovery::Include::Matched;

        {
            // `cpp11.R` is a default exclude, but `Exclude::Nothing` bypasses it
            let start = tempdir.join("R").join("cpp11.R");
            assert!(is_stdin_formattable(
                start.as_path(),
                &settings,
                exclude,
                include
            ));
        }

        {
            // `renv/activate.R` has a parent matching default excludes, but
            // `Exclude::Nothing` bypasses it
            let start = tempdir.join("renv").join("activate.R");
            assert!(is_stdin_formattable(
                start.as_path(),
                &settings,
                exclude,
                include
            ));
        }

        Ok(())
    }

    #[test]
    fn test_include_everything_allows_non_r_files() -> anyhow::Result<()> {
        let tempdir = TempDir::new()?;
        let tempdir = tempdir.path();

        let test_qmd = tempdir.join("test.qmd");

        let settings = Settings::default();
        let exclude = discovery::Exclude::Matched;
        let include = discovery::Include::Everything;

        // `.qmd` is not part of default includes, but `Include::Everything` bypasses that
        assert!(is_stdin_formattable(test_qmd, &settings, exclude, include));

        Ok(())
    }
}
