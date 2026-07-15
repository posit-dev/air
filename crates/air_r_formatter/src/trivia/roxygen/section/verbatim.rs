use biome_formatter::Format;
use biome_formatter::FormatResult;
use biome_formatter::write;
use biome_rowan::TextLen;

use crate::RFormatter;
use crate::context::RFormatContext;
use crate::prelude::*;
use crate::trivia::roxygen::RoxygenComments;
use crate::trivia::roxygen::builder::hard_line_break_or_empty_line;
use crate::trivia::roxygen::prefix::RoxygenPrefix;

/// Verbatim(ish) fallback formatter for unhandled sections
///
/// Prints the content of the roxygen comment mostly verbatim, but:
/// - Handles roxygen prefix normalization
/// - Handles trailing whitespace trimming (same as `FormatLeadingComments`)
///
/// A simpler form of [biome_formatter::trivia::FormatLeadingComments]
pub(crate) struct FormatRoxygenVerbatim<'a> {
    comments: RoxygenComments<'a>,
    prefix: &'a RoxygenPrefix,
}

impl<'a> FormatRoxygenVerbatim<'a> {
    pub(crate) fn new(comments: RoxygenComments<'a>, prefix: &'a RoxygenPrefix) -> Self {
        Self { comments, prefix }
    }
}

impl Format<RFormatContext> for FormatRoxygenVerbatim<'_> {
    fn fmt(&self, f: &mut RFormatter) -> FormatResult<()> {
        for comment in self.comments.iter() {
            write!(f, [self.prefix])?;

            let text_len = comment.piece().text_len();

            // Strip off `#+' ` and compute its width
            let text_without_prefix = comment.piece().text_without_prefix();
            let prefix_width = text_len - text_without_prefix.text_len();

            // Strip off trailing whitespace and compute its width
            let text_without_prefix_and_suffix = text_without_prefix.trim_end();
            let suffix_width =
                text_without_prefix.text_len() - text_without_prefix_and_suffix.text_len();

            // Compute reduced range
            let range = comment
                .piece()
                .text_range()
                .add_start(prefix_width)
                .sub_end(suffix_width);

            if !range.is_empty() {
                // Write via zero copy `located_token_text()`
                let token = comment.piece().as_piece().token();
                write!(f, [space(), located_token_text(&token, range)])?;
            }

            write!(f, [hard_line_break_or_empty_line(&comment)])?;

            comment.mark_formatted();
        }

        Ok(())
    }
}
