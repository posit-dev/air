//
// settings.rs
//
// Copyright (C) 2025 Posit Software, PBC. All rights reserved.
//
//

mod line_ending;

pub(crate) use line_ending::LineEnding;

use air_r_formatter::context::RFormatOptions;
use settings::IndentStyle;
use settings::IndentWidth;
use settings::LineWidth;
use settings::MagicLineBreak;

/// Resolved configuration settings used within air
///
/// May still require a source document to finalize some options, such as
/// `LineEnding::Auto` in the formatter.
#[derive(Clone, Debug, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
pub struct Settings {
    /// Settings to configure code formatting.
    pub format: FormatSettings,
}

#[derive(Clone, Debug, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
pub struct FormatSettings {
    pub indent_style: IndentStyle,
    pub indent_width: IndentWidth,
    pub line_ending: LineEnding,
    pub line_width: LineWidth,
    pub magic_line_break: MagicLineBreak,
}

impl FormatSettings {
    // Finalize `RFormatOptions` in preparation for a formatting operation on `source`
    pub fn to_format_options(&self, source: &str) -> RFormatOptions {
        RFormatOptions::new()
            .with_indent_style(self.indent_style)
            .with_indent_width(self.indent_width)
            .with_line_ending(self.line_ending.finalize(source))
            .with_line_width(self.line_width)
            .with_magic_line_break(self.magic_line_break)
    }
}
