mod examples;
mod verbatim;

use biome_formatter::Format;
use biome_formatter::FormatResult;
use biome_rowan::TextLen;
use biome_rowan::TextRange;

use crate::RFormatter;
use crate::context::RFormatContext;
use crate::trivia::roxygen::comments::RoxygenComment;
use crate::trivia::roxygen::comments::RoxygenComments;
use crate::trivia::roxygen::prefix::RoxygenPrefix;
use crate::trivia::roxygen::section::examples::ExamplesKind;
use crate::trivia::roxygen::section::examples::FormatRoxygenExamples;
use crate::trivia::roxygen::section::verbatim::FormatRoxygenVerbatim;

/// Formatter for one section of a roxygen block
pub(crate) enum FormatRoxygenSection<'a> {
    /// Leading description before any roxygen tags
    Introduction(verbatim::FormatRoxygenVerbatim<'a>),

    /// `@examples` and `@examplesIf`
    Examples(examples::FormatRoxygenExamples<'a>),

    /// An unhandled roxygen tag
    Unknown(verbatim::FormatRoxygenVerbatim<'a>),
}

impl Format<RFormatContext> for FormatRoxygenSection<'_> {
    fn fmt(&self, f: &mut RFormatter) -> FormatResult<()> {
        match self {
            FormatRoxygenSection::Introduction(comments) => comments.fmt(f),
            FormatRoxygenSection::Examples(comments) => comments.fmt(f),
            FormatRoxygenSection::Unknown(comments) => comments.fmt(f),
        }
    }
}

/// Splits a roxygen block into ordered [FormatRoxygenSection]
pub(crate) fn sections<'a>(
    comments: RoxygenComments<'a>,
    prefix: &'a RoxygenPrefix,
    indent: u8,
) -> Vec<FormatRoxygenSection<'a>> {
    // Locate the first roxygen tag
    let Some((mut start, mut tag)) = comments
        .iter()
        .enumerate()
        .find_map(|(i, comment)| parse_tag(comment).map(|tag| (i, tag)))
    else {
        // No tags
        return vec![introduction_section(comments, prefix)];
    };

    let mut sections = Vec::new();

    // Leading comments before the first tag form the introduction
    if start > 0 {
        sections.push(introduction_section(comments.slice(0..start), prefix));
    }

    // Subsequent tags close the current section and open a new one
    for (i, comment) in comments.iter().enumerate().skip(start + 1) {
        let Some(next_tag) = parse_tag(comment) else {
            continue;
        };
        sections.push(tag_section(tag, comments.slice(start..i), prefix, indent));
        start = i;
        tag = next_tag;
    }

    // Collect the final section
    sections.push(tag_section(
        tag,
        comments.slice(start..comments.len()),
        prefix,
        indent,
    ));

    sections
}

/// A single roxygen section
///
/// A section corresponds to the tag comment opening the section, i.e. the one starting
/// with `@tag`, and all of its body comments, i.e. with:
///
/// ```r
/// #' @examplesIf is_true()
/// #' 1 + 1
/// #' 2 + 2
/// ```
///
/// - All 3 lines together make up the section
/// - `@exampleIf` is the tag
/// - `is_true()` is the tag text
/// - `#' @examplesIf is_true()` is the tag comment
/// - `#' 1 + 1` and `#' 2 + 2` are body comments
///
/// Because the text directly following the `@tag` is a bit tricky to extract, we do it
/// once at the same time we parse the `@tag` kind and expose its value as
/// [RoxygenSection::tag_text()].
pub(crate) struct RoxygenSection<'a> {
    /// The tag line text, with `@tag` and all leading and trailing whitespace removed
    tag_text: &'a str,

    /// The range that the tag line text is at in the original source
    tag_text_range: TextRange,

    /// The comments associated with this section
    ///
    /// # Safety
    ///
    /// Guaranteed to contain at least the tag comment by construction
    comments: RoxygenComments<'a>,
}

impl<'a> RoxygenSection<'a> {
    pub(crate) fn tag_text(&self) -> &'a str {
        self.tag_text
    }

    pub(crate) fn tag_text_range(&self) -> TextRange {
        self.tag_text_range
    }

    pub(crate) fn tag_comment(&self) -> RoxygenComment<'a> {
        self.comments
            .first()
            .expect("`RoxygenSection` is guaranteed to have a tag comment")
    }

    pub(crate) fn body_comments(&self) -> RoxygenComments<'a> {
        self.comments.slice(1..self.comments.len())
    }

    /// All comments in the section
    pub(crate) fn comments(&self) -> RoxygenComments<'a> {
        self.comments
    }
}

struct Tag<'a> {
    kind: TagKind,
    text: &'a str,
    text_range: TextRange,
}

enum TagKind {
    Examples,
    ExamplesIf,
    Unknown,
}

fn introduction_section<'a>(
    comments: RoxygenComments<'a>,
    prefix: &'a RoxygenPrefix,
) -> FormatRoxygenSection<'a> {
    FormatRoxygenSection::Introduction(FormatRoxygenVerbatim::new(comments, prefix))
}

fn tag_section<'a>(
    tag: Tag<'a>,
    comments: RoxygenComments<'a>,
    prefix: &'a RoxygenPrefix,
    indent: u8,
) -> FormatRoxygenSection<'a> {
    match tag.kind {
        TagKind::Examples => FormatRoxygenSection::Examples(FormatRoxygenExamples::new(
            ExamplesKind::Examples,
            RoxygenSection {
                tag_text: tag.text,
                tag_text_range: tag.text_range,
                comments,
            },
            prefix,
            indent,
        )),
        TagKind::ExamplesIf => FormatRoxygenSection::Examples(FormatRoxygenExamples::new(
            ExamplesKind::ExamplesIf,
            RoxygenSection {
                tag_text: tag.text,
                tag_text_range: tag.text_range,
                comments,
            },
            prefix,
            indent,
        )),
        TagKind::Unknown => {
            FormatRoxygenSection::Unknown(FormatRoxygenVerbatim::new(comments, prefix))
        }
    }
}

/// Parse a roxygen tag comment
///
/// ```text
/// #' @tag some stuff
///    |--| |--------|
///    ^    ^
///    |    | tag text
///    | tag kind
/// ```
///
/// Returns the tag kind and tag text
fn parse_tag<'a>(comment: RoxygenComment<'a>) -> Option<Tag<'a>> {
    // Remove the leading roxygen prefix (`#'`) and at most one following space
    let text = comment.piece().text_without_prefix();

    // Remove any additional leading spaces
    let text = text.trim_start();

    // Remove the `@`
    let text = text.strip_prefix('@')?;

    // Next character must be alphanumeric, otherwise we don't have a tag at all.
    // This rejects things like `@ foo`, but also `@@` the escaped `@` symbol.
    if !text
        .chars()
        .next()
        .is_some_and(|c| c.is_ascii_alphanumeric())
    {
        return None;
    }

    // Ok, now we know we have a tag, find its end
    let end = text
        .find(|c: char| !c.is_ascii_alphanumeric())
        .unwrap_or(text.len());

    let kind = match &text[..end] {
        "examples" => TagKind::Examples,
        "examplesIf" => TagKind::ExamplesIf,
        _ => TagKind::Unknown,
    };

    // Tag text
    let text = &text[end..];

    // Remove all leading whitespace after the `@tag`, we normalize to just 1 space
    // between it and any tag text
    let text = text.trim_start();

    let text_len = comment.piece().text_len();
    let text_without_prefix_len = text.text_len();
    let prefix_width = text_len - text_without_prefix_len;

    // Remove all trailing whitespace after the text
    let text = text.trim_end();
    let text_without_prefix_and_suffix_len = text.text_len();
    let suffix_width = text_without_prefix_len - text_without_prefix_and_suffix_len;

    // Compute reduced range
    let text_range = comment
        .piece()
        .text_range()
        .add_start(prefix_width)
        .sub_end(suffix_width);

    Some(Tag {
        kind,
        text,
        text_range,
    })
}
