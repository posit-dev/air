use biome_formatter::FormatContext;
use biome_formatter::FormatOptions;
use biome_formatter::IndentWidth;
use biome_rowan::TriviaPieceKind;

use crate::prelude::*;
use crate::trivia::roxygen::comments::RoxygenComment;
use crate::trivia::roxygen::comments::RoxygenComments;
use crate::trivia::roxygen::prefix::RoxygenPrefix;
use crate::trivia::roxygen::section;

/// Formatter for a block of roxygen2 comments
pub(crate) struct FormatRoxygenComments<'a> {
    comments: RoxygenComments<'a>,
}

impl<'a> FormatRoxygenComments<'a> {
    pub(crate) fn new(comments: RoxygenComments<'a>) -> Self {
        Self { comments }
    }
}

impl Format<RFormatContext> for FormatRoxygenComments<'_> {
    fn fmt(&self, f: &mut RFormatter) -> FormatResult<()> {
        let Some(first) = self.comments.first() else {
            // It would be odd to not have any here. We only get called after creating
            // them.
            return Ok(());
        };

        // Derive roxygen prefix to normalize to from the first line of the block. We
        // allow either `#'` or `##'`, with anything else being normalized to `#'`.
        let prefix = match first.piece().hash_count() {
            1 => RoxygenPrefix::Single,
            2 => RoxygenPrefix::Double,
            _ => RoxygenPrefix::Single,
        };

        // Derive leading whitespace indent from the original source. Used to compute the
        // best effort adjusted line width when formatting `@examples`.
        let indent = leading_comment_indent(first, f.context().options().indent_width());

        // Split into sections and format each section
        for section in section::sections(self.comments, &prefix, indent) {
            section.fmt(f)?;
        }

        Ok(())
    }
}

/// Measures the leading indentation of a roxygen comment from the original source
///
/// This value is used to compute the adjusted line width that R code in `@examples`
/// and `@examplesIf` sections are wrapped at.
///
/// This adjusted line width is computed on a best effort basis, and should be correct for
/// nearly all real world usage. However, it relies on the pre-format indent width of the
/// node that the comment is attached to, because the post-format indent width is a print
/// time decision, and is unknowable at this time.
///
/// Consider the following snippet from an R6 class:
///
/// ```r
/// public = list(
/// #' @examples
/// #' fn(something_really_long_here)
/// fn = function() {}
/// )
/// ```
///
/// Here we'd compute the pre-format leading indentation as 0, so the adjusted line width
/// would be 80 - `#' `.len() - 0 = 77. If `fn(something_really_long_here)` was 76
/// characters wide, then it would be left as is. But post-format, we'd indent `fn` and
/// get this:
///
/// ```r
/// public = list(
///   #' @examples
///   #' fn(something_really_long_here)
///   fn = function() {}
/// )
/// ```
///
/// Recomputing the adjusted line width now gives `80 - `#' `.len() - 2 = 75`, and on a
/// second pass that would cause a 76 character wide `fn(something_really_long_here)` to
/// break. This goes against idempotence, but is such a rare case that we don't worry
/// about it.
fn leading_comment_indent(comment: RoxygenComment<'_>, indent_width: IndentWidth) -> u8 {
    let comment = comment.piece().as_piece();
    let comment_start = comment.text_range().start();

    // The token that the comment is attached to. We iterate its leading trivia (which
    // includes the comment itself).
    let token = comment.token();

    let mut previous = None;

    // Iterate the leading trivia of the `token` that the leading comment is attached to,
    // in order. Store the most recent `previous` piece of trivia as we go, and stop once
    // we've reached the leading comment. This gives us the comment's previous sibling.
    for piece in token.leading_trivia().pieces() {
        if piece.text_range().start() == comment_start {
            break;
        }
        previous = Some(piece);
    }

    // If the `previous` piece of trivia right before our leading comment was a run of
    // whitespace, compute its width, otherwise it's 0. Practically, I think the only
    // other option that can come right before a leading comment is
    // `TriviaPieceKind::Newline`.
    match previous {
        Some(previous) => match previous.kind() {
            TriviaPieceKind::Whitespace => whitespace_width(previous.text(), indent_width),
            TriviaPieceKind::Newline
            | TriviaPieceKind::SingleLineComment
            | TriviaPieceKind::MultiLineComment
            | TriviaPieceKind::Skipped => 0,
        },
        None => 0,
    }
}

fn whitespace_width(text: &str, indent_width: IndentWidth) -> u8 {
    text.chars()
        .map(|c| if c == '\t' { indent_width.value() } else { 1 })
        .fold(0, |acc, next| acc.saturating_add(next))
}
