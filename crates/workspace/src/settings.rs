mod indent_style;
mod indent_width;
mod line_ending;
mod line_length;
mod magic_line_break;

pub use indent_style::*;
pub use indent_width::*;
pub use line_ending::*;
pub use line_length::*;
pub use magic_line_break::*;

use air_r_formatter::context::RFormatOptions;

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
    pub line_length: LineLength,
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
            LineEnding::Auto => match ruff_source_file::find_newline(source) {
                Some((_, ruff_source_file::LineEnding::Lf)) => biome_formatter::LineEnding::Lf,
                Some((_, ruff_source_file::LineEnding::Crlf)) => biome_formatter::LineEnding::Crlf,
                Some((_, ruff_source_file::LineEnding::Cr)) => biome_formatter::LineEnding::Cr,
                None => biome_formatter::LineEnding::Lf,
            },
        };

        RFormatOptions::new()
            .with_indent_style(self.indent_style.into())
            .with_indent_width(self.indent_width.into())
            .with_line_ending(line_ending)
            .with_line_width(self.line_length.into())
            .with_magic_line_break(self.magic_line_break.into())
    }
}
