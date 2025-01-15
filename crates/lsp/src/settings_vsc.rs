//
// settings_vsc.rs
//
// Copyright (C) 2024 Posit Software, PBC. All rights reserved.
//
//

// Should we rename this to `_lsp` or move to `from_proto`? We'll probably use
// the same settings sections for all IDEs, even if the naming and organization is
// from VS Code?

use crate::{logging::LogLevel, settings::DocumentSettings};
use struct_field_names_as_array::FieldNamesAsArray;

/// VS Code representation of a document settings
#[derive(Clone, Debug, FieldNamesAsArray, serde::Serialize, serde::Deserialize)]
pub(crate) struct VscDocumentSettings {
    // DEV NOTE: Update `section_from_key()` method after adding a field
    pub insert_spaces: bool,
    pub indent_size: VscIndentSize,
    pub tab_size: usize,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub(crate) enum VscIndentSize {
    Alias(String),
    Size(usize),
}

#[derive(Clone, Debug, FieldNamesAsArray, serde::Serialize, serde::Deserialize)]
pub(crate) struct VscDiagnosticsSettings {
    // DEV NOTE: Update `section_from_key()` method after adding a field
    pub enable: bool,
}

#[derive(Clone, Debug, FieldNamesAsArray, serde::Deserialize)]
pub(crate) struct VscGlobalSettings {
    // DEV NOTE: Update `section_from_key()` method after adding a field
    pub log_level: Option<LogLevel>,
    pub dependency_log_levels: Option<String>,
}

impl VscDocumentSettings {
    pub(crate) fn section_from_key(key: &str) -> &str {
        match key {
            "insert_spaces" => "editor.insertSpaces",
            "indent_size" => "editor.indentSize",
            "tab_size" => "editor.tabSize",
            _ => "unknown", // To be caught via downstream errors
        }
    }
}

/// Convert from VS Code representation of document settings to our own
/// representation. Currently one-to-one.
impl From<VscDocumentSettings> for DocumentSettings {
    fn from(value: VscDocumentSettings) -> Self {
        let indent_style = indent_style_from_vsc(value.insert_spaces);
        let indent_width = indent_width_from_vsc(&value);

        Self {
            indent_style: Some(indent_style),
            indent_width: Some(indent_width),
            line_width: None, // We don't currently watch this setting
        }
    }
}

impl VscDiagnosticsSettings {
    pub(crate) fn section_from_key(key: &str) -> &str {
        match key {
            "enable" => "positron.r.diagnostics.enable",
            _ => "unknown", // To be caught via downstream errors
        }
    }
}

impl VscGlobalSettings {
    pub(crate) fn section_from_key(key: &str) -> &str {
        match key {
            "log_level" => "air.logLevel",
            "dependency_log_levels" => "air.dependencyLogLevels",
            _ => "unknown", // To be caught via downstream errors
        }
    }
}

pub(crate) fn indent_width_from_vsc(settings: &VscDocumentSettings) -> settings::IndentWidth {
    let indent_width = match settings.indent_size {
        VscIndentSize::Size(ref size) => *size,
        VscIndentSize::Alias(ref var) => {
            if var != "tabSize" {
                tracing::warn!("Unknown indent alias {var}, using default");
                return settings::IndentWidth::default();
            }
            settings.tab_size
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

pub(crate) fn line_width_from_vsc(rulers: &[usize]) -> Option<settings::LineWidth> {
    rulers.first().and_then(|w| (*w as u16).try_into().ok())
}
