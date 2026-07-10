use air_r_syntax::RSyntaxNode;
use biome_formatter::CstFormatContext;
use biome_formatter::FormatContext;

use crate::prelude::*;
use crate::trivia::roxygen::RoxygenComments;

/// Air's replacement for [biome_formatter::trivia::format_leading_comments()]
///
/// Splits a `node`'s leading comments into runs of:
/// - [crate::trivia::roxygen::FormatRoxygenComments]
/// - [biome_formatter::trivia::FormatLeadingComments]
pub(crate) fn format_leading_comments(node: &RSyntaxNode) -> FormatLeadingComments<'_> {
    FormatLeadingComments { node }
}

pub(crate) struct FormatLeadingComments<'a> {
    node: &'a RSyntaxNode,
}

impl Format<RFormatContext> for FormatLeadingComments<'_> {
    fn fmt(&self, f: &mut RFormatter) -> FormatResult<()> {
        let comments = f.context().comments().clone();
        let mut comments = comments.leading_comments(self.node);

        // User hasn't opted in to roxygen2 formatting
        if f.context().options().roxygen_examples().is_disabled() {
            return biome_formatter::trivia::FormatLeadingComments::Comments(comments).fmt(f);
        }

        // Continually split leading comments into runs of standard comments and roxygen2
        // comments until we run out of leading comments
        while !comments.is_empty() {
            let (before_roxygen, roxygen, after_roxygen) =
                RoxygenComments::from_partition(comments);

            if !before_roxygen.is_empty() {
                biome_formatter::trivia::FormatLeadingComments::Comments(before_roxygen).fmt(f)?;
            }

            if !roxygen.is_empty() {
                crate::trivia::roxygen::FormatRoxygenComments::new(roxygen).fmt(f)?;
            }

            comments = after_roxygen;
        }

        Ok(())
    }
}
