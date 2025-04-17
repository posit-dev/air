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
    FormatFile(#[from] FormatFileError),
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
        writeln!(f, "Would reformat: {}", path.display())?;
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
                match format_path(path, &settings.format).and_then(write_path) {
                    Ok(()) => None,
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
                match format_path(path, &settings.format) {
                    Ok(file) => check_path(file).map(Ok),
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

fn format_path(
    path: PathBuf,
    settings: &FormatSettings,
) -> std::result::Result<FormattedFile, FormatPathError> {
    tracing::trace!("Formatting {path}", path = path.display());
    format_file(path, settings).map_err(Into::into)
}

/// Returns `Ok(())` if the format results were successfully written back, otherwise
/// returns an error
fn write_path(file: FormattedFile) -> std::result::Result<(), FormatPathError> {
    match file {
        FormattedFile::Changed(file) => match std::fs::write(file.path(), file.new()) {
            Ok(()) => Ok(()),
            Err(err) => Err(FormatPathError::Write(file.into_path(), err)),
        },
        FormattedFile::Unchanged => Ok(()),
    }
}

/// Returns `Some(path)` if a change occurred, otherwise returns `None`
fn check_path(file: FormattedFile) -> Option<PathBuf> {
    match file {
        FormattedFile::Changed(file) => Some(file.into_path()),
        FormattedFile::Unchanged => None,
    }
}

impl Display for FormatPathError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FormatFile(err) => err.fmt(f),
            Self::Write(path, err) => {
                write!(
                    f,
                    "Failed to write {path}: {err}",
                    path = relativize_path(path).underline(),
                )
            }
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
        }
    }
}

#[cfg(test)]
mod test {
    use crate::args::Args;
    use crate::run;
    use crate::status::ExitStatus;
    use clap::Parser;
    use std::path::Path;
    use std::path::PathBuf;
    use tempfile::TempDir;

    fn path_fixtures() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("fixtures")
    }

    #[test]
    fn default_options() -> anyhow::Result<()> {
        let tempdir = TempDir::new()?;
        let temppath = tempdir.path().join("test.R");
        std::fs::write(
            &temppath,
            r#"
1 + 1
"#,
        )?;

        let args = Args::parse_from(["", "format", temppath.to_str().unwrap()]);
        let err = run(args)?;

        assert_eq!(err, ExitStatus::Success);
        Ok(())
    }

    #[test]
    fn test_default_exclude_patterns() -> anyhow::Result<()> {
        let tempdir = TempDir::new()?;

        let test_path = tempdir.path().join("test.R");
        let test_contents = r#"
1+1
"#;

        let cpp11_path = tempdir.path().join("cpp11.R");
        let cpp11_contents = r#"
1+1
"#;

        std::fs::write(&test_path, test_contents)?;
        std::fs::write(&cpp11_path, cpp11_contents)?;

        // Only `test.R` should be formatted, `cpp11.R` is a default exclude
        let args = Args::parse_from(["", "format", tempdir.path().to_str().unwrap()]);
        let err = run(args)?;
        assert_eq!(err, ExitStatus::Success);
        assert!(test_contents != std::fs::read_to_string(&test_path)?);
        assert_eq!(cpp11_contents, std::fs::read_to_string(&cpp11_path)?);

        Ok(())
    }

    #[test]
    fn test_default_exclude_patterns_with_explicit_format_file_request() -> anyhow::Result<()> {
        let tempdir = TempDir::new()?;

        let cpp11_path = tempdir.path().join("cpp11.R");
        let cpp11_contents = r#"
1+1
"#;
        std::fs::write(&cpp11_path, cpp11_contents)?;

        // Formatting on `cpp11.R` is explicitly requested, since this does not require
        // any file discovery, we format it even though it matches a default `exclude`.
        let args = Args::parse_from(["", "format", cpp11_path.to_str().unwrap()]);
        let err = run(args)?;
        assert_eq!(err, ExitStatus::Success);
        assert!(cpp11_contents != std::fs::read_to_string(&cpp11_path)?);

        Ok(())
    }

    #[test]
    fn test_default_exclude_patterns_with_explicit_format_folder_request() -> anyhow::Result<()> {
        let tempdir = TempDir::new()?;

        let renv = tempdir.path().join("renv");
        std::fs::create_dir(&renv)?;

        let activate_path = renv.join("activate.R");
        let activate_contents = r#"
1+1
"#;
        std::fs::write(&activate_path, activate_contents)?;

        // Formatting on `air format renv` is explicitly requested. We don't auto
        // accept folders provided on the command line, so this goes through the standard
        // path and ends up excluding everything in `renv/`.
        let args = Args::parse_from(["", "format", renv.to_str().unwrap()]);
        let err = run(args)?;
        assert_eq!(err, ExitStatus::Success);
        assert_eq!(activate_contents, std::fs::read_to_string(&activate_path)?);

        Ok(())
    }

    #[test]
    fn test_modified_exclude_patterns() -> anyhow::Result<()> {
        let tempdir = TempDir::new()?;

        let test_path = tempdir.path().join("test.R");
        let test_contents = r#"
1+1
"#;

        let cpp11_path = tempdir.path().join("cpp11.R");
        let cpp11_contents = r#"
1+1
"#;

        let air_path = tempdir.path().join("air.toml");
        let air_contents = r#"
[format]
exclude = ["test.R"]
default-exclude = false
"#;

        // Turn off `default-exclude`, turn on the custom `exclude`
        std::fs::write(&test_path, test_contents)?;
        std::fs::write(&cpp11_path, cpp11_contents)?;
        std::fs::write(&air_path, air_contents)?;

        // Only `cpp11.R` should be formatted
        let args = Args::parse_from(["", "format", tempdir.path().to_str().unwrap()]);
        let err = run(args)?;
        assert_eq!(err, ExitStatus::Success);
        assert_eq!(test_contents, std::fs::read_to_string(&test_path)?);
        assert!(cpp11_contents != std::fs::read_to_string(&cpp11_path)?);

        Ok(())
    }

    #[test]
    fn test_check_returns_cleanly_for_multiline_strings_with_crlf_line_endings(
    ) -> anyhow::Result<()> {
        let fixtures = path_fixtures();
        let path = fixtures.join("crlf").join("multiline_string_value.R");
        let path = path.to_str().unwrap();

        let args = Args::parse_from(["", "format", path, "--check"]);
        let err = run(args)?;

        assert_eq!(err, ExitStatus::Success);
        Ok(())
    }
}
