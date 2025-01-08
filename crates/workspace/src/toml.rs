//
// toml.rs
//
// Copyright (C) 2025 Posit Software, PBC. All rights reserved.
//
//

//! Utilities for locating (and extracting configuration from) an air.toml.

use crate::toml_options::TomlOptions;
use std::fmt::Display;
use std::fmt::Formatter;
use std::io;
use std::path::{Path, PathBuf};

/// Parse an `air.toml` file.
pub fn parse_air_toml<P: AsRef<Path>>(path: P) -> Result<TomlOptions, ParseTomlError> {
    let contents = std::fs::read_to_string(path.as_ref())
        .map_err(|err| ParseTomlError::Read(path.as_ref().to_path_buf(), err))?;

    toml::from_str(&contents)
        .map_err(|err| ParseTomlError::Deserialize(path.as_ref().to_path_buf(), err))
}

#[derive(Debug)]
pub enum ParseTomlError {
    Read(PathBuf, io::Error),
    Deserialize(PathBuf, toml::de::Error),
}

impl std::error::Error for ParseTomlError {}

impl Display for ParseTomlError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            // It's nicer if we don't make these paths relative, so we can quickly
            // jump to the TOML file to see what is wrong
            Self::Read(path, err) => {
                write!(f, "Failed to read {path}:\n{err}", path = path.display())
            }
            Self::Deserialize(path, err) => {
                write!(f, "Failed to parse {path}:\n{err}", path = path.display())
            }
        }
    }
}

/// Return the path to the `air.toml` file in a given directory.
pub fn find_air_toml_in_directory<P: AsRef<Path>>(path: P) -> Option<PathBuf> {
    // Check for `air.toml`.
    let toml = path.as_ref().join("air.toml");

    if toml.is_file() {
        Some(toml)
    } else {
        None
    }
}

/// Find the path to the closest `air.toml` if one exists, walking up the filesystem
pub fn find_air_toml<P: AsRef<Path>>(path: P) -> Option<PathBuf> {
    for directory in path.as_ref().ancestors() {
        if let Some(toml) = find_air_toml_in_directory(directory) {
            return Some(toml);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use anyhow::{Context, Result};
    use std::fs;
    use tempfile::TempDir;

    use crate::settings::LineEnding;
    use crate::settings::LineWidth;
    use crate::toml::find_air_toml;
    use crate::toml::parse_air_toml;
    use crate::toml_options::GlobalTomlOptions;
    use crate::toml_options::TomlOptions;

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
        let options = parse_air_toml(toml)?;

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
}
