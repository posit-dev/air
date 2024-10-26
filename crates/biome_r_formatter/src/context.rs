use std::fmt;
use std::rc::Rc;

use biome_formatter::printer::PrinterOptions;
use biome_formatter::AttributePosition;
use biome_formatter::BracketSpacing;
use biome_formatter::CstFormatContext;
use biome_formatter::FormatContext;
use biome_formatter::FormatOptions;
use biome_formatter::IndentStyle;
use biome_formatter::IndentWidth;
use biome_formatter::LineEnding;
use biome_formatter::LineWidth;
use biome_formatter::TransformSourceMap;
use biome_r_syntax::RLanguage;

use crate::comments::FormatRLeadingComment;
use crate::comments::RCommentStyle;
use crate::comments::RComments;

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

    /// The max width of a line. Defaults to 80.
    line_width: LineWidth,
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
}

impl FormatOptions for RFormatOptions {
    fn indent_style(&self) -> IndentStyle {
        self.indent_style
    }

    fn indent_width(&self) -> IndentWidth {
        self.indent_width
    }

    fn line_width(&self) -> LineWidth {
        self.line_width
    }

    fn line_ending(&self) -> LineEnding {
        self.line_ending
    }

    fn attribute_position(&self) -> AttributePosition {
        // TODO: Do we use this?
        AttributePosition::Auto
    }

    fn bracket_spacing(&self) -> BracketSpacing {
        // TODO: Do we use this?
        BracketSpacing::default()
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
        writeln!(f, "Line width: {}", self.line_width.value())
    }
}
