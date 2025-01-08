//
// settings.rs
//
// Copyright (C) 2025 Posit Software, PBC. All rights reserved.
//
//

mod indent_style;
mod indent_width;
// TODO: Can we pick a better crate name for `line_ending` so these don't collide?
#[path = "settings/line_ending.rs"]
mod line_ending_setting;
mod line_width;
mod magic_line_break;

pub use indent_style::*;
pub use indent_width::*;
pub use line_ending_setting::*;
pub use line_width::*;
pub use magic_line_break::*;

use air_r_formatter::context::RFormatOptions;
use line_ending;

/// Resolved configuration settings used within air
///
/// May still require a source document to finalize some options, such as
/// `LineEnding::Auto` in the formatter.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct Settings {
    /// Settings to configure code formatting.
    pub format: FormatSettings,
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
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
        let line_ending = match self.line_ending {
            LineEnding::Lf => biome_formatter::LineEnding::Lf,
            LineEnding::Crlf => biome_formatter::LineEnding::Crlf,
            #[cfg(target_os = "windows")]
            LineEnding::Native => biome_formatter::LineEnding::Crlf,
            #[cfg(not(target_os = "windows"))]
            LineEnding::Native => biome_formatter::LineEnding::Lf,
            LineEnding::Auto => match line_ending::infer(source) {
                line_ending::LineEnding::Lf => biome_formatter::LineEnding::Lf,
                line_ending::LineEnding::Crlf => biome_formatter::LineEnding::Crlf,
            },
        };

        RFormatOptions::new()
            .with_indent_style(self.indent_style.into())
            .with_indent_width(self.indent_width.into())
            .with_line_ending(line_ending)
            .with_line_width(self.line_width.into())
            .with_magic_line_break(self.magic_line_break.into())
    }
}
