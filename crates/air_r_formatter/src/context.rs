use std::fmt;
use std::rc::Rc;

use air_r_syntax::RLanguage;
use biome_formatter::CstFormatContext;
use biome_formatter::FormatContext;
use biome_formatter::FormatOptions;
use biome_formatter::TransformSourceMap;
use biome_formatter::printer::PrinterOptions;
use settings::IndentStyle;
use settings::IndentWidth;
use settings::LineEnding;
use settings::LineWidth;
use settings::PersistentLineBreaks;
use settings::Skip;

use crate::comments::FormatRLeadingComment;
use crate::comments::RCommentStyle;
use crate::comments::RComments;

#[derive(Clone, Debug)]
pub struct RFormatContext {
    options: RFormatOptions,

    /// The comments of the nodes and tokens in the program.
    comments: Rc<RComments>,

    source_map: Option<TransformSourceMap>,
}

impl RFormatContext {
    pub fn new(options: RFormatOptions, comments: RComments) -> Self {
        Self {
            options,
            comments: Rc::new(comments),
            source_map: None,
        }
    }

    pub fn with_source_map(mut self, source_map: Option<TransformSourceMap>) -> Self {
        self.source_map = source_map;
        self
    }
}

impl FormatContext for RFormatContext {
    type Options = RFormatOptions;

    fn options(&self) -> &Self::Options {
        &self.options
    }

    fn source_map(&self) -> Option<&TransformSourceMap> {
        self.source_map.as_ref()
    }
}

impl CstFormatContext for RFormatContext {
    type Language = RLanguage;
    type Style = RCommentStyle;
    type CommentRule = FormatRLeadingComment;

    fn comments(&self) -> &RComments {
        &self.comments
    }
}

#[derive(Debug, Default, Clone)]
pub struct RFormatOptions {
    /// The indent style.
    indent_style: IndentStyle,

    /// The indent width.
    indent_width: IndentWidth,

    /// The type of line ending.
    line_ending: LineEnding,

    /// The max width of a line.
    line_width: LineWidth,

    /// The behavior of persistent line breaks.
    persistent_line_breaks: PersistentLineBreaks,

    /// The set of functions that are skipped without requiring a `# fmt: skip` comment.
    skip: Option<Skip>,
}

impl RFormatOptions {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn with_indent_style(mut self, indent_style: IndentStyle) -> Self {
        self.indent_style = indent_style;
        self
    }

    pub fn with_indent_width(mut self, indent_width: IndentWidth) -> Self {
        self.indent_width = indent_width;
        self
    }

    pub fn with_line_ending(mut self, line_ending: LineEnding) -> Self {
        self.line_ending = line_ending;
        self
    }

    pub fn with_line_width(mut self, line_width: LineWidth) -> Self {
        self.line_width = line_width;
        self
    }

    pub fn with_persistent_line_breaks(
        mut self,
        persistent_line_breaks: PersistentLineBreaks,
    ) -> Self {
        self.persistent_line_breaks = persistent_line_breaks;
        self
    }

    pub fn with_skip(mut self, skip: Option<Skip>) -> Self {
        self.skip = skip;
        self
    }

    pub fn set_indent_style(&mut self, indent_style: IndentStyle) {
        self.indent_style = indent_style;
    }

    pub fn set_indent_width(&mut self, indent_width: IndentWidth) {
        self.indent_width = indent_width;
    }

    pub fn set_line_ending(&mut self, line_ending: LineEnding) {
        self.line_ending = line_ending;
    }

    pub fn set_line_width(&mut self, line_width: LineWidth) {
        self.line_width = line_width;
    }

    pub fn set_persistent_line_breaks(&mut self, persistent_line_breaks: PersistentLineBreaks) {
        self.persistent_line_breaks = persistent_line_breaks;
    }

    pub fn set_skip(&mut self, skip: Option<Skip>) {
        self.skip = skip;
    }

    pub fn persistent_line_breaks(&self) -> PersistentLineBreaks {
        self.persistent_line_breaks
    }

    pub fn skip(&self) -> Option<&Skip> {
        self.skip.as_ref()
    }
}

impl FormatOptions for RFormatOptions {
    fn indent_style(&self) -> biome_formatter::IndentStyle {
        self.indent_style.into()
    }

    fn indent_width(&self) -> biome_formatter::IndentWidth {
        self.indent_width.into()
    }

    fn line_width(&self) -> biome_formatter::LineWidth {
        self.line_width.into()
    }

    fn line_ending(&self) -> biome_formatter::LineEnding {
        self.line_ending.into()
    }

    fn as_print_options(&self) -> PrinterOptions {
        PrinterOptions::from(self)
    }
}

impl fmt::Display for RFormatOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Indent style: {}", self.indent_style)?;
        writeln!(f, "Indent width: {}", self.indent_width.value())?;
        writeln!(f, "Line ending: {}", self.line_ending)?;
        writeln!(f, "Line width: {}", self.line_width.value())?;
        writeln!(f, "Persistent line breaks: {}", self.persistent_line_breaks)?;
        writeln!(
            f,
            "Skip: {}",
            match &self.skip {
                Some(skip) => format!("{skip}"),
                None => String::from("None"),
            }
        )
    }
}
