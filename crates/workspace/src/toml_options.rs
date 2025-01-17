//
// toml_options.rs
//
// Copyright (C) 2025 Posit Software, PBC. All rights reserved.
//
//

use crate::settings::FormatSettings;
use crate::settings::LineEnding;
use crate::settings::Settings;
use settings::IndentStyle;
use settings::IndentWidth;
use settings::LineWidth;
use settings::PersistentLineBreaks;

/// The Rust representation of `air.toml`
///
/// The names and types of the fields in this struct determine the names and types
/// that can be specified in the `air.toml`.
///
/// Every field is optional at this point, nothing is "finalized".
/// Finalization is done in [TomlOptions::into_settings].
///
/// Global options are specified at top level in the TOML file.
/// All other options are nested within their own `[table]`.
#[derive(Clone, Debug, PartialEq, Eq, Default, serde::Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub struct TomlOptions {
    /// Global options affecting multiple commands.
    #[serde(flatten)]
    pub global: GlobalTomlOptions,

    /// Options to configure code formatting.
    pub format: Option<FormatTomlOptions>,
}

// NOTE: Just a placeholder for now, we don't currently have any global settings
#[derive(Clone, Debug, PartialEq, Eq, Default, serde::Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub struct GlobalTomlOptions {}

/// Configures the way air formats your code.
#[derive(Clone, Debug, PartialEq, Eq, Default, serde::Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub struct FormatTomlOptions {
    /// The line width at which the formatter prefers to wrap lines.
    ///
    /// The value must be greater than or equal to `1` and less than or equal to `320`.
    ///
    /// Note: While the formatter will attempt to format lines such that they remain
    /// within the `line-width`, it isn't a hard upper bound, and formatted lines may
    /// exceed the `line-width`.
    pub line_width: Option<LineWidth>,

    /// The number of spaces per indentation level (tab).
    ///
    /// The value must be greater than or equal to `1` and less than or equal to `24`. The
    /// default value is `2`.
    ///
    /// Used by the formatter to determine the visual width of a tab.
    ///
    /// This option changes the number of spaces the formatter inserts when
    /// using `indent-style = "space"`. It also represents the width of a tab when
    /// `indent-style = "tab"` for the purposes of computing the `line-width`.
    pub indent_width: Option<IndentWidth>,

    /// Whether to use spaces or tabs for indentation.
    ///
    /// `indent-style = "space"` (default):
    ///
    /// ```r
    /// fn <- function() {
    ///   cat("Hello") # Spaces indent the `cat()` call.
    /// }
    /// ```
    ///
    /// `indent-style = "tab"`:
    ///
    /// ```r
    /// fn <- function() {
    ///   cat("Hello") # A tab `\t` indents the `cat()` call.
    /// }
    /// ```
    ///
    /// Air defaults to spaces due to the overwhelming amount of existing R code written
    /// in this style, but consider using tabs for new projects to improve accessibility.
    ///
    /// See `indent-width` to configure the number of spaces per indentation and the tab width.
    pub indent_style: Option<IndentStyle>,

    /// The character air uses at the end of a line.
    ///
    /// * `auto`: The newline style is detected automatically on a file per file basis. Files with mixed line endings will be converted to the first detected line ending. Defaults to `\n` for files that contain no line endings.
    /// * `lf`: Line endings will be converted to `\n`. The default line ending on Unix.
    /// * `crlf`: Line endings will be converted to `\r\n`. The default line ending on Windows.
    /// * `native`: Line endings will be converted to `\n` on Unix and `\r\n` on Windows.
    pub line_ending: Option<LineEnding>,

    /// Air respects a small set of persistent line breaks as an indication that certain
    /// function calls or function signatures should be left expanded. If this option
    /// is set to `false`, persistent line breaks are ignored.
    ///
    /// It may be preferable to ignore persistent line breaks if you prefer that `line-width`
    /// should be the only value that influences line breaks.
    pub persistent_line_breaks: Option<bool>,
}

impl TomlOptions {
    pub fn into_settings(self) -> Settings {
        let format = self.format.unwrap_or_default();

        let format = FormatSettings {
            indent_style: format.indent_style.unwrap_or_default(),
            indent_width: format.indent_width.unwrap_or_default(),
            line_ending: format.line_ending.unwrap_or_default(),
            line_width: format.line_width.unwrap_or_default(),
            persistent_line_breaks: match format.persistent_line_breaks {
                Some(persistent_line_breaks) => {
                    if persistent_line_breaks {
                        PersistentLineBreaks::Respect
                    } else {
                        PersistentLineBreaks::Ignore
                    }
                }
                None => PersistentLineBreaks::Respect,
            },
        };

        Settings { format }
    }
}
