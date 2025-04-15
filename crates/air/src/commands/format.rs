use std::io;
use std::io::stderr;
use std::io::Write;

use itertools::Itertools;
use workspace::discovery::discover_settings;
use workspace::discovery::DiscoveredSettings;
use workspace::format::FormattedFile;
use workspace::format::OnChangedAction;
use workspace::resolve::PathResolver;
use workspace::settings::Settings;

use crate::args::FormatCommand;
use crate::ExitStatus;

pub(crate) fn format(command: FormatCommand) -> anyhow::Result<ExitStatus> {
    let mode = FormatMode::from_command(&command);

    let on_changed_action = match mode {
        FormatMode::Write => OnChangedAction::Write,
        FormatMode::Check => OnChangedAction::None,
    };

    let mut resolver = PathResolver::new(Settings::default());

    for DiscoveredSettings {
        directory,
        settings,
    } in discover_settings(&command.paths)?
    {
        resolver.add(&directory, settings);
    }

    let (files, errors) =
        workspace::format::format_paths(&command.paths, &resolver, on_changed_action);

    for error in &errors {
        tracing::error!("{error}");
    }

    match mode {
        FormatMode::Write => {}
        FormatMode::Check => {
            write_changed(&files, &mut stderr().lock())?;
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
                let any_changed = files.iter().any(FormattedFile::is_changed);

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

fn write_changed(files: &[FormattedFile], f: &mut impl Write) -> io::Result<()> {
    for path in files
        .iter()
        .filter_map(|result| match result {
            FormattedFile::Changed(path) => Some(path),
            FormattedFile::Unchanged => None,
        })
        .sorted_unstable()
    {
        writeln!(f, "Would reformat: {}", path.display())?;
    }

    Ok(())
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
