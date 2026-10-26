//
// toml.rs
//
// Copyright (C) 2025 Posit Software, PBC. All rights reserved.
//
//

//! Utilities for locating (and extracting configuration from) an air.toml.

use crate::toml_options::TomlOptions;
use std::io;
use std::path::{Path, PathBuf};

// No need to derive `Error`, we really just use this as a passthrough enum
// into `DiscoverSettingsError`
#[derive(Debug)]
pub enum ParseTomlError {
    Read {
        path: PathBuf,
        error: io::Error,
    },

    Deserialize {
        path: PathBuf,
        contents: String,
        // Clippy warned this this was a big `Error` object
        error: Box<toml::de::Error>,
    },
}

/// Parse an air configuration file.
pub fn parse_air_toml<P: AsRef<Path>>(path: P) -> Result<TomlOptions, ParseTomlError> {
    let path = path.as_ref();

    let contents = std::fs::read_to_string(path).map_err(|error| ParseTomlError::Read {
        path: path.to_path_buf(),
        error,
    })?;

    parse_air_inline_toml(&contents).map_err(|error| ParseTomlError::Deserialize {
        path: path.to_path_buf(),
        contents,
        error: Box::new(error),
    })
}

/// Parse inline toml
///
/// `pub` only for usage in tests
pub fn parse_air_inline_toml(contents: &str) -> Result<TomlOptions, toml::de::Error> {
    toml::from_str(contents)
}

/// Return the path to the `air.toml` or `.air.toml` file in a given directory.
pub fn find_air_toml_in_directory<P: AsRef<Path>>(path: P) -> Option<PathBuf> {
    // Check for `air.toml` first, as we prioritize the "visible" one.
    let toml = path.as_ref().join("air.toml");
    if toml.is_file() {
        return Some(toml);
    }

    // Now check for `.air.toml` as well
    let toml = path.as_ref().join(".air.toml");
    if toml.is_file() {
        return Some(toml);
    }

    // Didn't find a configuration file
    None
}

/// Find the path to the closest `air.toml` or `.air.toml` if one exists, walking up the filesystem
pub fn find_air_toml<P: AsRef<Path>>(path: P) -> Option<PathBuf> {
    for directory in path.as_ref().ancestors() {
        if let Some(toml) = find_air_toml_in_directory(directory) {
            return Some(toml);
        }
    }
    None
}

/// Check if a path is named like an `air.toml` or `.air.toml` file
///
/// Does not check if the path is an existing file on disk
pub fn is_air_toml<P: AsRef<Path>>(path: P) -> bool {
    let path = path.as_ref();
    path.ends_with("air.toml") || path.ends_with(".air.toml")
}

#[cfg(test)]
mod tests {
    use anyhow::{Context, Result};
    use std::fs;
    use tempfile::TempDir;

    use crate::settings::LineEnding;
    use crate::toml::find_air_toml;
    use crate::toml::parse_air_toml;
    use crate::toml_options::GlobalTomlOptions;
    use crate::toml_options::TomlOptions;
    use settings::LineWidth;

    #[test]
    fn deserialize_empty() -> Result<()> {
        let options: TomlOptions = toml::from_str(r"")?;
        assert_eq!(options.global, GlobalTomlOptions {});
        assert_eq!(options.format, None);
        Ok(())
    }

    #[test]
    fn find_and_parse_air_toml() -> Result<()> {
        let tempdir = TempDir::new()?;
        let toml = tempdir.path().join("air.toml");
        fs::write(
            toml,
            r#"
[format]
line-width = 88
line-ending = "auto"
"#,
        )?;

        let toml = find_air_toml(tempdir.path()).context("Failed to find air.toml")?;
        let options = parse_air_toml(toml).expect("Parses");

        let line_width = options
            .format
            .as_ref()
            .context("Expected to find [format] table")?
            .line_width
            .context("Expected to find `line-width` field")?;

        let line_ending = options
            .format
            .as_ref()
            .context("Expected to find [format] table")?
            .line_ending
            .context("Expected to find `line-ending` field")?;

        assert_eq!(line_width, LineWidth::try_from(88).unwrap());
        assert_eq!(line_ending, LineEnding::Auto);

        Ok(())
    }

    #[test]
    fn find_and_parse_dot_air_toml() -> Result<()> {
        let tempdir = TempDir::new()?;
        let toml = tempdir.path().join(".air.toml");
        fs::write(
            toml,
            r#"
[format]
persistent-line-breaks = false
"#,
        )?;

        let toml = find_air_toml(tempdir.path()).context("Failed to find air.toml")?;
        let options = parse_air_toml(toml).expect("Parses");

        let persistent_line_breaks = options
            .format
            .as_ref()
            .context("Expected to find [format] table")?
            .persistent_line_breaks
            .context("Expected to find `persistent-line-breaks` field")?;

        assert!(!persistent_line_breaks);

        Ok(())
    }

    #[test]
    fn test_air_toml_priority() -> Result<()> {
        let tempdir = TempDir::new()?;

        let toml = tempdir.path().join("air.toml");
        fs::write(
            toml.clone(),
            r#"
[format]
indent-width = 3
"#,
        )?;

        let dot_toml = tempdir.path().join(".air.toml");
        fs::write(
            dot_toml,
            r#"
[format]
indent-width = 4
"#,
        )?;

        // Finds `air.toml` over `.air.toml`
        let found_toml = find_air_toml(tempdir.path()).context("Failed to find air.toml")?;
        assert_eq!(found_toml, toml);

        Ok(())
    }
}
