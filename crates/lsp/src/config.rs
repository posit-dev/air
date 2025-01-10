//
// config.rs
//
// Copyright (C) 2024 Posit Software, PBC. All rights reserved.
//
//

use struct_field_names_as_array::FieldNamesAsArray;

use crate::logging::LogLevel;

/// Configuration of the LSP
#[derive(Clone, Debug, Default)]
pub(crate) struct LspConfig {}

/// Configuration of a document.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct DocumentConfig {
    pub indent_style: Option<settings::IndentStyle>,
    pub indent_width: Option<settings::IndentWidth>,
    pub line_width: Option<settings::LineWidth>,
}

/// VS Code representation of a document configuration
#[derive(Clone, Debug, FieldNamesAsArray, serde::Serialize, serde::Deserialize)]
pub(crate) struct VscDocumentConfig {
    // DEV NOTE: Update `section_from_key()` method after adding a field
    pub insert_spaces: bool,
    pub indent_size: VscIndentSize,
    pub tab_size: usize,
}

#[derive(Clone, Debug, FieldNamesAsArray, serde::Serialize, serde::Deserialize)]
pub(crate) struct VscDiagnosticsConfig {
    // DEV NOTE: Update `section_from_key()` method after adding a field
    pub enable: bool,
}

#[derive(Clone, Debug, FieldNamesAsArray, serde::Deserialize)]
pub(crate) struct VscLogConfig {
    // DEV NOTE: Update `section_from_key()` method after adding a field
    pub log_level: Option<LogLevel>,
    pub dependency_log_levels: Option<String>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub(crate) enum VscIndentSize {
    Alias(String),
    Size(usize),
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
        let indent_style = indent_style_from_vsc(x.insert_spaces);
        let indent_width = indent_width_from_vsc(x);

        Self {
            indent_style: Some(indent_style),
            indent_width: Some(indent_width),
            line_width: None, // TODO!
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

pub(crate) fn indent_width_from_vsc(config: VscDocumentConfig) -> settings::IndentWidth {
    let indent_width = match config.indent_size {
        VscIndentSize::Size(size) => size,
        VscIndentSize::Alias(var) => {
            if var != "tabSize" {
                tracing::warn!("Unknown indent alias {var}, using default");
                return settings::IndentWidth::default();
            }
            config.tab_size
        }
    };

    indent_width_from_usize(indent_width)
}

pub(crate) fn indent_width_from_usize(indent_width: usize) -> settings::IndentWidth {
    indent_width.try_into().unwrap_or_else(|err| {
        tracing::warn!("Invalid indent width: {err:?}");
        settings::IndentWidth::default()
    })
}

pub(crate) fn indent_style_from_vsc(insert_spaces: bool) -> settings::IndentStyle {
    if insert_spaces {
        settings::IndentStyle::Space
    } else {
        settings::IndentStyle::Tab
    }
}

impl VscLogConfig {
    pub(crate) fn section_from_key(key: &str) -> &str {
        match key {
            "log_level" => "air.logLevel",
            "dependency_log_levels" => "air.dependencyLogLevels",
            _ => "unknown", // To be caught via downstream errors
        }
    }
}
