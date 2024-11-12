use serde::Deserialize;
use serde::Serialize;
use struct_field_names_as_array::FieldNamesAsArray;

use crate::lsp;
use crate::lsp::diagnostics::DiagnosticsConfig;

/// Configuration of the LSP
#[derive(Clone, Debug)]
pub(crate) struct LspConfig {
    pub(crate) diagnostics: DiagnosticsConfig,
}

/// Configuration of a document.
///
/// The naming follows <https://editorconfig.org/> where possible.
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct DocumentConfig {
    pub indent: IndentationConfig,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct IndentationConfig {
    /// Whether to insert spaces of tabs for one level of indentation.
    pub indent_style: IndentStyle,

    /// The number of spaces for one level of indentation.
    pub indent_size: usize,

    /// The width of a tab. There may be projects with an `indent_size` of 4 and
    /// a `tab_width` of 8 (e.g. GNU R).
    pub tab_width: usize,
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
    pub tab_size: usize,
}

#[derive(Serialize, Deserialize, FieldNamesAsArray, Clone, Debug)]
pub(crate) struct VscDiagnosticsConfig {
    // DEV NOTE: Update `section_from_key()` method after adding a field
    pub enable: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub(crate) enum VscIndentSize {
    Alias(String),
    Size(usize),
}

impl Default for LspConfig {
    fn default() -> Self {
        Self {
            diagnostics: Default::default(),
        }
    }
}

impl Default for IndentationConfig {
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

/// Convert from VS Code representation of a document config to our own
/// representation. Currently one-to-one.
impl From<VscDocumentConfig> for DocumentConfig {
    fn from(x: VscDocumentConfig) -> Self {
        let indent_style = indent_style_from_lsp(x.insert_spaces);

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
            indent: IndentationConfig {
                indent_style,
                indent_size,
                tab_width: x.tab_size,
            },
        }
    }
}

impl VscDiagnosticsConfig {
    pub(crate) fn section_from_key(key: &str) -> &str {
        match key {
            "enable" => "positron.r.diagnostics.enable",
            _ => "unknown", // To be caught via downstream errors
        }
    }
}

impl From<VscDiagnosticsConfig> for DiagnosticsConfig {
    fn from(value: VscDiagnosticsConfig) -> Self {
        Self {
            enable: value.enable,
        }
    }
}

pub(crate) fn indent_style_from_lsp(insert_spaces: bool) -> IndentStyle {
    if insert_spaces {
        IndentStyle::Space
    } else {
        IndentStyle::Tab
    }
}
