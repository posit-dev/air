use air_r_syntax::RSyntaxKind::STRING_CONTENT;
use air_r_syntax::RSyntaxToken;
use biome_formatter::Format;
use biome_formatter::FormatResult;
use biome_formatter::prelude::syntax_token_cow_slice;
use biome_formatter::trivia::format_replaced;
use std::borrow::Cow;

use crate::RFormatter;
use crate::context::RFormatContext;

/// Helper utility for formatting a string content token
///
/// The main job of this utility is to normalize the string content and handle the
/// complicated way we have to call [format_replaced] with that normalized result.
pub(crate) struct FormatStringContentToken<'token> {
    /// The string content token to format
    token: &'token RSyntaxToken,
}

impl<'token> FormatStringContentToken<'token> {
    pub(crate) fn new(token: &'token RSyntaxToken) -> Self {
        Self { token }
    }
}

impl Format<RFormatContext> for FormatStringContentToken<'_> {
    fn fmt(&self, f: &mut RFormatter) -> FormatResult<()> {
        format_replaced(
            self.token,
            &syntax_token_cow_slice(
                normalize_string_content_token(self.token),
                self.token,
                self.token.text_trimmed_range().start(),
            ),
        )
        .fmt(f)
    }
}

/// Normalize `STRING_CONTENT` text, returning a [`Cow::Borrowed`] if the text was already
/// normalized
///
/// Currently the only normalization this does is to convert `\r\n` to `\n`. We may do
/// more normalization in the future (like, `quote-style`).
///
/// This function is particularly useful for multiline strings, which capture the existing
/// line ending inside the string content token itself. We must normalize those line
/// endings to `\n` before the formatter -> printer stage, because the printer can't
/// handle other line endings and will panic on them. At the printer -> string stage at
/// the very end, the printer will replace all `\n` with the `LineEnding` requested by the
/// user.
/// https://github.com/biomejs/biome/blob/a658a294087c143b83350cbeb6b44f7a2e9afdd1/crates/biome_formatter/src/printer/mod.rs#L714-L718
///
/// https://github.com/posit-dev/air/pull/127
fn normalize_string_content_token(token: &RSyntaxToken) -> Cow<'_, str> {
    debug_assert!(
        matches!(token.kind(), STRING_CONTENT),
        "Found kind {:?}",
        token.kind()
    );
    debug_assert!(
        token.text() == token.text_trimmed(),
        "String content tokens should never have trivia. Trivia should be on string open or string close tokens instead."
    );
    line_ending::normalize_ref(token.text())
}
