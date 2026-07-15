use air_r_parser::RParserOptions;
use biome_formatter::FormatContext;
use biome_formatter::FormatOptions;
use biome_formatter::write;
use settings::IndentStyle;
use settings::LineEnding;
use settings::LineWidth;
use settings::RoxygenExamples;

use crate::prelude::*;
use crate::trivia::roxygen::builder::hard_line_break_or_empty_line;
use crate::trivia::roxygen::prefix::RoxygenPrefix;
use crate::trivia::roxygen::section::RoxygenSection;
use crate::trivia::roxygen::section::verbatim::FormatRoxygenVerbatim;

#[derive(Debug, Clone, Copy)]
pub(crate) enum ExamplesKind {
    Examples,
    ExamplesIf,
}

/// Formatter for `@examples` and `@examplesIf` sections
pub(crate) struct FormatRoxygenExamples<'a> {
    kind: ExamplesKind,
    section: RoxygenSection<'a>,
    prefix: &'a RoxygenPrefix,
    indent: u8,
}

impl Format<RFormatContext> for FormatRoxygenExamples<'_> {
    fn fmt(&self, f: &mut RFormatter) -> FormatResult<()> {
        if self.try_fmt(f)? {
            return Ok(());
        }

        // Something failed, like a parse error. Totally fine and expected. Fall back to
        // verbatim formatter.
        FormatRoxygenVerbatim::new(self.section.comments(), self.prefix).fmt(f)
    }
}

impl<'a> FormatRoxygenExamples<'a> {
    pub(crate) fn new(
        kind: ExamplesKind,
        section: RoxygenSection<'a>,
        prefix: &'a RoxygenPrefix,
        indent: u8,
    ) -> Self {
        Self {
            section,
            kind,
            prefix,
            indent,
        }
    }

    fn try_fmt(&self, f: &mut RFormatter) -> FormatResult<bool> {
        let mut lines: Vec<&str> = Vec::new();

        // This
        //
        // ```
        // #' @examples 1 + 1
        // #' 2 + 2
        // ```
        //
        // is normalized to
        //
        // ```
        // #' @examples
        // #' 1 + 1
        // #' 2 + 2
        // ```
        match self.kind {
            ExamplesKind::Examples => {
                if !self.section.tag_text().is_empty() {
                    lines.push(self.section.tag_text());
                }
            }
            ExamplesKind::ExamplesIf => {
                // Tag line is left verbatim
            }
        }

        for comment in self.section.body_comments().iter() {
            lines.push(comment.piece().text_without_prefix());
        }

        // It's somewhat common to interleave multiple `@examples` and `@examplesIf`
        // sections into a single roxygen block. When this happens, you often need to put
        // a blank line after `@examplesIf` to ensure the final result is readable with or
        // without the `@examplesIf` body (i.e., you can't put the blank line before the
        // `@examplesIf` to achieve the same result). We allow that by writing a single
        // empty roxygen line if the first line of the body was empty. Air's formatter
        // will otherwise strip all leading blank lines out.
        //
        // ```r
        // #' @examples
        // #' fn(1)
        // #' fn(2)
        // #' @examplesIf has_pkg()
        // #'
        // #' # `fn(2)` works with pkg
        // #' pkg::this(fn(2))
        // #' @examples
        // #'
        // #' # Another feature
        // #' another_demo(fn(3))
        // fn <- function(x) { x }
        // ```
        let needs_leading_empty_line = lines.first().is_some_and(|line| line.trim().is_empty());

        // It's also common to have an empty roxygen comment between sections for
        // readability. We preserve that by writing a single empty roxygen line if the
        // last body line of this section was empty.
        //
        // ```r
        // #' @param x A number.
        // #'
        // #' @examples
        // #' fn(1)
        // #' fn(2)
        // #'
        // #' @returns
        // #' Something
        // fn <- function(x) { x }
        // ```
        //
        // The extra `lines.len() != 1` check is to avoid writing two empty roxygen lines
        // (one from `needs_leading_empty_line` and one from `needs_trailing_empty_line`)
        // when the entire section is just a single blank line, i.e.:
        //
        // ```r
        // #' @examples
        // #'
        // #' @returns
        // #' Something
        // ```
        let needs_trailing_empty_line =
            lines.last().is_some_and(|line| line.trim().is_empty()) && lines.len() != 1;

        let options = f.context().options();

        // To compute the actual line width used for the roxygen examples:
        // - Start with user requested line width
        // - Subtract leading indent before the `#`
        // - Subtract prefix width, i.e. the `#'` or `##'` size
        // - Subtract 1, for a space following the prefix
        let line_width = (options.line_width().value())
            .saturating_sub(self.indent.into())
            .saturating_sub(self.prefix.len().into())
            .saturating_sub(1)
            .max(1);

        let Ok(line_width) = LineWidth::try_from(line_width) else {
            // Should never happen
            return Ok(false);
        };

        // Start with user's formatting options and apply overrides
        //
        // The reconstructed lines are emitted via `dynamic_text()`, so the text must
        // contain `\n` line endings. The printer will rewrite them if required.
        //
        // Example sections always follow `#'`, i.e. they aren't the first thing on a
        // line. This means they should always be indented with spaces, even if the user
        // requests tabs for the surrounding file. But we do respect the user's
        // `IndentWidth`.
        let options = options
            .clone()
            .with_line_width(line_width)
            .with_line_ending(LineEnding::Lf)
            .with_indent_style(IndentStyle::Space)
            .with_roxygen_examples(RoxygenExamples::Disabled);

        let text = lines.join("\n");

        // Any parse errors or `\dontrun{}` style content results in a silent no-op
        let parsed = air_r_parser::parse(&text, RParserOptions::default());
        if parsed.has_error() {
            return Ok(false);
        }
        let Ok(formatted) = crate::format_node(options, &parsed.syntax()) else {
            return Ok(false);
        };
        let Ok(printed) = formatted.print() else {
            return Ok(false);
        };

        let text = printed.into_code();

        // TODO: Can we make `InsertFinalNewline` an air option for library style usage?
        // This isn't the first time we've had to strip it back off.
        //
        // Air currently strips off all user trailing line breaks and then unconditionally
        // appends a single final `hard_line_break()` in `FormatRRoot`. We want to be in
        // charge of trailing line breaks, so we strip that off here.
        let text = text.strip_suffix('\n').unwrap_or(&text);

        let lines: Vec<&str> = if text.is_empty() {
            Vec::new()
        } else {
            text.split('\n').collect()
        };

        match self.kind {
            ExamplesKind::Examples => {
                // Any tag text was normalized onto its own line
                write!(
                    f,
                    [
                        self.prefix,
                        space(),
                        biome_formatter::prelude::text("@examples")
                    ]
                )?;
            }
            ExamplesKind::ExamplesIf => {
                write!(
                    f,
                    [
                        self.prefix,
                        space(),
                        biome_formatter::prelude::text("@examplesIf")
                    ]
                )?;
                // `located_token_text()` is more efficient than `dynamic_text()` because
                // it comes straight from the original source and doesn't allocate
                if !self.section.tag_text().is_empty() {
                    write!(
                        f,
                        [
                            space(),
                            located_token_text(
                                &self.section.tag_comment().piece().as_piece().token(),
                                self.section.tag_text_range()
                            )
                        ]
                    )?;
                }
            }
        }

        // Reconstructed lines are synthetic, so we anchor their source position to the
        // tag comment
        let position = self.section.tag_comment().piece().text_range().start();

        if needs_leading_empty_line {
            write!(f, [hard_line_break(), self.prefix])?;
        }

        for line in lines {
            write!(f, [hard_line_break(), self.prefix])?;
            if !line.is_empty() {
                write!(f, [space(), dynamic_text(line, position)])?;
            }
        }

        if needs_trailing_empty_line {
            write!(f, [hard_line_break(), self.prefix])?;
        }

        // Handle special line break for the last comment in the block, which may need
        // to be followed by a full empty line if there was one in the original source
        let last = self
            .section
            .body_comments()
            .last()
            .unwrap_or(self.section.tag_comment());
        write!(f, [hard_line_break_or_empty_line(&last)])?;

        // Mark every comment as formatted
        for comment in self.section.comments().iter() {
            comment.mark_formatted();
        }

        Ok(true)
    }
}
