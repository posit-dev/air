use biome_formatter::prelude::Line;
use biome_formatter::prelude::empty_line;
use biome_formatter::prelude::hard_line_break;

use crate::trivia::roxygen::comments::RoxygenComment;

/// [Line] style to write after a roxygen comment
///
/// [biome_formatter::trivia::FormatLeadingComments]'s standard behavior for
/// [CommentKind::Line] comments is to insert:
/// - A single [hard_line_break()] if there are 0 or 1 line breaks after the comment in
///   the original source
/// - A single [empty_line()] if there are >1 lines breaks after the comment in the
///   original source
///
/// Because [crate::trivia::roxygen::RoxygenComments] blocks are constructed to be
/// sequentially separated by a single line break in the original source, this really only
/// applies to the final comment in the block, but it maintains the full empty line in
/// cases like this:
///
/// ```r
/// #' @examples
/// #' fn()
///
/// fn <- function() {}
// ```
///
/// Because we are in charge of formatting roxygen comments and don't call
/// [biome_formatter::trivia::FormatLeadingComments], we must reproduce this behavior
/// ourselves. We do this often enough that it justifies having its own helper.
pub(crate) fn hard_line_break_or_empty_line(comment: &RoxygenComment) -> Line {
    match comment.lines_after() {
        0 | 1 => hard_line_break(),
        _ => empty_line(),
    }
}
