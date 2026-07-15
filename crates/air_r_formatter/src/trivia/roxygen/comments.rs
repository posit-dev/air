//! Exposes roxygen specific newtype wrappers
//!
//! - [RoxygenComments] wraps an array of [SourceComment]
//! - [RoxygenComment] wraps a single [SourceComment]
//! - [RoxygenSyntaxTriviaPieceComments] wraps [SyntaxTriviaPieceComments]
//!
//! These wrappers are completely stateless and allocation free on purpose. The point is
//! to make it "feel" like you are working with the underlying types by reexposing any
//! required methods, while also enhancing them with roxygen specific knowledge, most
//! importantly, [RoxygenSyntaxTriviaPieceComments::text_without_prefix()] and
//! [RoxygenSyntaxTriviaPieceComments::hash_count()].
//!
//! We "propagate" the lifetime in a number of places to make this maximally ergonomic.
//! For example, [RoxygenSyntaxTriviaPieceComments::text()]'s return value is `&'a str` to
//! tie it all the way back up to the lifetime of the original [SourceComment], rather
//! than just `&str`, which ties it to the lifetime of [RoxygenSyntaxTriviaPieceComments].
//! This allows the return value of [RoxygenSyntaxTriviaPieceComments::text()] to outlive
//! the [RoxygenSyntaxTriviaPieceComments] itself, which is quite useful!

use std::ops::Range;

use air_r_syntax::RLanguage;
use biome_formatter::comments::SourceComment;

use biome_rowan::SyntaxTriviaPiece;
use biome_rowan::SyntaxTriviaPieceComments;
use biome_rowan::TextRange;
use biome_rowan::TextSize;

/// A block of roxygen2 comments
///
/// Constructed via [RoxygenComments::from_partition()], which performs the one time
/// validation that the comment text matches `#+'`. That invariant allows
/// [RoxygenSyntaxTriviaPieceComments]'s methods to be infallible.
#[derive(Clone, Copy)]
pub(crate) struct RoxygenComments<'a>(&'a [SourceComment<RLanguage>]);

impl<'a> RoxygenComments<'a> {
    /// Splits an array of `comments` into `(before_roxygen, roxygen, after_roxygen)`
    pub(crate) fn from_partition(
        comments: &'a [SourceComment<RLanguage>],
    ) -> (
        &'a [SourceComment<RLanguage>],
        RoxygenComments<'a>,
        &'a [SourceComment<RLanguage>],
    ) {
        let start = comments
            .iter()
            .take_while(|comment| !is_roxygen(comment.piece().text()))
            .count();

        // Find the run of contiguous `#'`:
        // - First line is known to be `#'`
        // - Line that doesn't start with `#'` ends the run
        // - Roxygen comment with a blank line before it ends the run
        let len = comments[start..]
            .iter()
            .enumerate()
            .take_while(|(i, comment)| {
                is_roxygen(comment.piece().text()) && (*i == 0 || comment.lines_before() <= 1)
            })
            .count();
        let end = start + len;

        (
            &comments[..start],
            Self(&comments[start..end]),
            &comments[end..],
        )
    }

    pub(crate) fn len(&self) -> usize {
        self.0.len()
    }

    pub(crate) fn first(&self) -> Option<RoxygenComment<'a>> {
        self.0.first().map(RoxygenComment)
    }

    pub(crate) fn last(&self) -> Option<RoxygenComment<'a>> {
        self.0.last().map(RoxygenComment)
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = RoxygenComment<'a>> {
        self.0.iter().map(RoxygenComment)
    }

    pub(crate) fn slice(&self, range: Range<usize>) -> RoxygenComments<'a> {
        RoxygenComments(&self.0[range])
    }
}

/// A roxygen2 comment
#[derive(Clone, Copy)]
pub(crate) struct RoxygenComment<'a>(&'a SourceComment<RLanguage>);

impl<'a> RoxygenComment<'a> {
    pub(crate) fn lines_after(&self) -> u32 {
        self.0.lines_after()
    }

    pub(crate) fn mark_formatted(&self) {
        self.0.mark_formatted();
    }

    pub(crate) fn piece(&self) -> RoxygenSyntaxTriviaPieceComments<'a> {
        RoxygenSyntaxTriviaPieceComments(self.0.piece())
    }
}

/// A roxygen2 comment syntax trivia piece
#[derive(Clone, Copy)]
pub(crate) struct RoxygenSyntaxTriviaPieceComments<'a>(&'a SyntaxTriviaPieceComments<RLanguage>);

impl<'a> RoxygenSyntaxTriviaPieceComments<'a> {
    pub(crate) fn text(&self) -> &'a str {
        self.0.text()
    }

    pub(crate) fn text_len(&self) -> TextSize {
        self.0.text_len()
    }

    pub(crate) fn text_range(&self) -> TextRange {
        self.0.text_range()
    }

    pub(crate) fn as_piece(&self) -> &'a SyntaxTriviaPiece<RLanguage> {
        self.0.as_piece()
    }

    /// The text with the roxygen prefix and at most one following space stripped
    pub(crate) fn text_without_prefix(&self) -> &'a str {
        roxygen_strip_prefix(self.text())
    }

    /// The number of `#` characters in the roxygen prefix
    pub(crate) fn hash_count(&self) -> usize {
        roxygen_hash_count(self.text())
    }
}

/// Is this comment a roxygen2 comment, i.e. does it start with `#'`, `##'`, etc.?
///
/// # Safety
///
/// Since `text` comes from a [SourceComment], we can assume it starts with at least 1 `#`
/// and don't have to check for that before trimming.
#[inline]
fn is_roxygen(text: &str) -> bool {
    text.trim_start_matches('#').starts_with('\'')
}

/// The number of `#` characters in the roxygen prefix
#[inline]
fn roxygen_hash_count(text: &str) -> usize {
    text.bytes().take_while(|&b| b == b'#').count()
}

/// Strips the roxygen prefix and at most one space from `text`
///
/// # Safety
///
/// Construction of [RoxygenSyntaxTriviaPieceComments] is guarded by [is_roxygen()],
/// so we can expect a `#'` prefix
#[inline]
fn roxygen_strip_prefix(text: &str) -> &str {
    let rest = text.trim_start_matches('#');
    let rest = rest
        .strip_prefix('\'')
        .expect("`RoxygenSyntaxTriviaPieceComments` is guaranteed to have a `'`");
    rest.strip_prefix(' ').unwrap_or(rest)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn accepts_roxygen() {
        assert!(is_roxygen("#' foo"));
        assert!(is_roxygen("##' foo"));
        assert!(is_roxygen("#'foo"));
        assert!(is_roxygen("##'foo"));
    }

    #[test]
    fn rejects_non_roxygen() {
        assert!(!is_roxygen("# foo"));
        assert!(!is_roxygen("## foo"));
        assert!(!is_roxygen("#"));
        assert!(!is_roxygen("# ' foo"));
    }

    #[test]
    fn hash_count_matches_one_or_more_hashes() {
        assert_eq!(roxygen_hash_count("#' foo"), 1);
        assert_eq!(roxygen_hash_count("##' foo"), 2);
        assert_eq!(roxygen_hash_count("###'x"), 3);
        assert_eq!(roxygen_hash_count("#'"), 1);
    }

    #[test]
    fn strips_prefix_and_at_most_one_space() {
        assert_eq!(roxygen_strip_prefix("#' foo"), "foo");
        assert_eq!(roxygen_strip_prefix("##'foo"), "foo");
        assert_eq!(roxygen_strip_prefix("##'  foo"), " foo");
        assert_eq!(roxygen_strip_prefix("#'"), "");
        assert_eq!(roxygen_strip_prefix("##'"), "");
    }
}
