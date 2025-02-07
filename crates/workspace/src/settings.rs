//
// settings.rs
//
// Copyright (C) 2025 Posit Software, PBC. All rights reserved.
//
//

mod ignore_patterns;
mod include_patterns;
mod line_ending;

pub use ignore_patterns::IgnorePatterns;
pub use include_patterns::IncludePatterns;
pub(crate) use line_ending::LineEnding;

use air_r_formatter::context::RFormatOptions;
use settings::IndentStyle;
use settings::IndentWidth;
use settings::LineWidth;
use settings::PersistentLineBreaks;

/// Resolved configuration settings used within air
///
/// May still require a source document to finalize some options, such as
/// `LineEnding::Auto` in the formatter.
#[derive(Debug, Default)]
pub struct Settings {
    /// Settings to configure code formatting.
    pub format: FormatSettings,
}

#[derive(Debug, Default)]
pub struct FormatSettings {
    pub indent_style: IndentStyle,
    pub indent_width: IndentWidth,
    pub line_ending: LineEnding,
    pub line_width: LineWidth,
    pub persistent_line_breaks: PersistentLineBreaks,
    pub ignore: IgnorePatterns,
    pub include: IncludePatterns,
}

impl FormatSettings {
    // Finalize `RFormatOptions` in preparation for a formatting operation on `source`
    pub fn to_format_options(&self, source: &str) -> RFormatOptions {
        RFormatOptions::new()
            .with_indent_style(self.indent_style)
            .with_indent_width(self.indent_width)
            .with_line_ending(self.line_ending.finalize(source))
            .with_line_width(self.line_width)
            .with_persistent_line_breaks(self.persistent_line_breaks)
    }
}
