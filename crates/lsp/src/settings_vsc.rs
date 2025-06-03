//
// settings_vsc.rs
//
// Copyright (C) 2024 Posit Software, PBC. All rights reserved.
//
//

// Should we rename this to `_lsp` or move to `from_proto`? We'll probably use
// the same settings sections for all IDEs, even if the naming and organization is
// from VS Code?

use crate::{
    logging::LogLevel,
    settings::{DocumentSettings, GlobalSettings},
};
use struct_field_names_as_array::FieldNamesAsArray;

#[derive(Clone, Debug, FieldNamesAsArray, serde::Deserialize)]
pub(crate) struct VscGlobalSettings {
    // DEV NOTE: Update `section_from_key()` method after adding a field
    pub log_level: Option<LogLevel>,
    pub dependency_log_levels: Option<String>,
    pub sync_file_settings_with_client: Option<bool>,
}

/// VS Code representation of a document settings
#[derive(Clone, Debug, FieldNamesAsArray, serde::Serialize, serde::Deserialize)]
pub(crate) struct VscDocumentSettings {
    // DEV NOTE: Update `section_from_key()` method after adding a field
    pub insert_spaces: Option<bool>,
    pub indent_size: Option<VscIndentSize>,
    pub tab_size: Option<usize>,
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
    pub enable: Option<bool>,
}

impl From<VscGlobalSettings> for GlobalSettings {
    fn from(value: VscGlobalSettings) -> Self {
        Self {
            sync_file_settings_with_client: value
                .sync_file_settings_with_client
                .unwrap_or_default(),
        }
    }
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
        // Conversion is all or nothing to avoid sending, say, the `indent_size` without
        // also sending the `tab_size`
        let (indent_style, indent_width) =
            match (value.insert_spaces, value.indent_size, value.tab_size) {
                (Some(insert_spaces), Some(indent_size), Some(tab_size)) => (
                    Some(indent_style_from_vsc(insert_spaces)),
                    Some(indent_width_from_vsc(indent_size, tab_size)),
                ),
                _ => (None, None),
            };

        Self {
            indent_style,
            indent_width,
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
            "sync_file_settings_with_client" => "air.syncFileSettingsWithClient",
            _ => "unknown", // To be caught via downstream errors
        }
    }
}

pub(crate) fn indent_width_from_vsc(
    indent_size: VscIndentSize,
    tab_size: usize,
) -> settings::IndentWidth {
    let indent_width = match indent_size {
        VscIndentSize::Size(ref size) => *size,
        VscIndentSize::Alias(ref var) => {
            if var != "tabSize" {
                tracing::warn!("Unknown indent alias {var}, using default");
                return settings::IndentWidth::default();
            }
            tab_size
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

#[allow(dead_code)]
pub(crate) fn line_width_from_vsc(rulers: &[usize]) -> Option<settings::LineWidth> {
    rulers.first().and_then(|w| (*w as u16).try_into().ok())
}
