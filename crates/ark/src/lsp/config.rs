use serde::Deserialize;
use serde::Serialize;
use struct_field_names_as_array::FieldNamesAsArray;

use crate::lsp;

/// Configuration of a document.
///
/// The naming follows <https://editorconfig.org/> where possible.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DocumentConfig {
    /// Whether to insert spaces of tabs for one level of indentation.
    pub indent_style: IndentStyle,

    /// The number of spaces for one level of indentation.
    pub indent_size: u8,

    /// The width of a tab. There may be projects with an `indent_size` of 4 and
    /// a `tab_width` of 8 (e.g. GNU R).
    pub tab_width: u8,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum IndentStyle {
    Tab,
    Space,
}

/// VS Code representation of a document configuration
#[derive(Serialize, Deserialize, FieldNamesAsArray, Clone, Debug)]
pub(crate) struct VscDocumentConfig {
    // DEV NOTE: Update `section_from_key()` method after adding a field
    pub insert_spaces: bool,
    pub indent_size: VscIndentSize,
    pub tab_size: u8,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub(crate) enum VscIndentSize {
    Alias(String),
    Size(u8),
}

impl Default for DocumentConfig {
    fn default() -> Self {
        Self {
            indent_style: IndentStyle::Space,
            indent_size: 2,
            tab_width: 2,
        }
    }
}

impl VscDocumentConfig {
    pub(crate) fn section_from_key(key: &str) -> &str {
        match key {
            "insert_spaces" => "editor.insertSpaces",
            "indent_size" => "editor.indentSize",
            "tab_size" => "editor.tabSize",
            _ => "unknown", // To be caught via downstream errors
        }
    }
}

// Convert from VS Code representation of a document config to our own
// representation
impl From<VscDocumentConfig> for DocumentConfig {
    fn from(x: VscDocumentConfig) -> Self {
        let indent_style = if x.insert_spaces {
            IndentStyle::Space
        } else {
            IndentStyle::Tab
        };

        let indent_size = match x.indent_size {
            VscIndentSize::Size(size) => size,
            VscIndentSize::Alias(var) => {
                if var == "tabSize" {
                    x.tab_size
                } else {
                    lsp::log_warn!("Unknown indent alias {var}, using default");
                    2
                }
            },
        };

        Self {
            indent_style,
            indent_size,
            tab_width: x.tab_size,
        }
    }
}
