use biome_rowan::FileSourceError;
use biome_string_case::StrLikeExtension;
use std::{ffi::OsStr, path::Path};

#[derive(
    Debug, Clone, Default, Copy, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize,
)]
pub struct RFileSource {
    // ?? Options
}

impl RFileSource {
    // Well-known R files
    // This list should be SORTED!
    // Source: https://github.com/github-linguist/linguist/blob/4ac734c15a96f9e16fd12330d0cb8de82274f700/lib/linguist/languages.yml#L5682-L5701
    // Note: we shouldn't include machine generated files
    const WELL_KNOWN_R_FILES: &'static [&'static str] = &[".R", ".r"];

    pub fn r() -> Self {
        Self {
            // ?? Options
        }
    }

    pub fn is_well_known_r_file(file_name: &str) -> bool {
        Self::WELL_KNOWN_R_FILES.binary_search(&file_name).is_ok()
    }

    /// Try to return the R file source corresponding to this file name from well-known files
    pub fn try_from_well_known(path: &Path) -> Result<Self, FileSourceError> {
        let file_name = path
            .file_name()
            .and_then(OsStr::to_str)
            .ok_or_else(|| FileSourceError::MissingFileName)?;
        if Self::is_well_known_r_file(file_name) {
            return Ok(Self::r());
        }
        Err(FileSourceError::UnknownFileName)
    }

    /// Try to return the R file source corresponding to this file extension
    pub fn try_from_extension(extension: &OsStr) -> Result<Self, FileSourceError> {
        // We assume the file extension is normalized to lowercase
        match extension.as_encoded_bytes() {
            b"r" => Ok(Self::r()),
            _ => Err(FileSourceError::UnknownExtension),
        }
    }

    /// Try to return the R file source corresponding to this language ID
    ///
    /// See the [LSP spec] and [VS Code spec] for a list of language identifiers
    ///
    /// The language ID for code snippets is registered by [VS Code built-in extensions]
    ///
    /// [LSP spec]: https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#textDocumentItem
    /// [VS Code spec]: https://code.visualstudio.com/docs/languages/identifiers
    pub fn try_from_language_id(language_id: &str) -> Result<Self, FileSourceError> {
        match language_id {
            "r" => Ok(Self::r()),
            _ => Err(FileSourceError::UnknownLanguageId),
        }
    }
}

impl TryFrom<&Path> for RFileSource {
    type Error = FileSourceError;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        if let Ok(file_source) = Self::try_from_well_known(path) {
            return Ok(file_source);
        }

        let Some(extension) = path.extension() else {
            return Err(FileSourceError::MissingFileExtension);
        };
        // We assume the file extensions are case-insensitive
        // and we use the lowercase form of them for pattern matching
        Self::try_from_extension(&extension.to_ascii_lowercase_cow())
    }
}

#[test]
fn test_order() {
    for items in RFileSource::WELL_KNOWN_R_FILES.windows(2) {
        assert!(items[0] < items[1], "{} < {}", items[0], items[1]);
    }
}
