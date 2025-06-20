use air_r_syntax::RSyntaxKind::R_STRING_LITERAL;
use air_r_syntax::RSyntaxToken;
use biome_formatter::Format;
use biome_formatter::FormatResult;
use biome_formatter::prelude::Formatter;
use biome_formatter::prelude::syntax_token_cow_slice;
use biome_formatter::trivia::format_replaced;
use std::borrow::Cow;

use crate::RFormatter;
use crate::context::RFormatContext;

/// Helper utility for formatting a string literal token
///
/// The main job of this utility is to `normalize()` the string and handle the
/// complicated way we have to call [format_replaced] with that normalized result.
pub(crate) struct FormatStringLiteralToken<'token> {
    /// The string literal token to format
    token: &'token RSyntaxToken,
}

impl<'token> FormatStringLiteralToken<'token> {
    pub(crate) fn new(token: &'token RSyntaxToken) -> Self {
        Self { token }
    }

    fn normalize(&self) -> FormatNormalizedStringLiteralToken {
        let token = self.token;

        debug_assert!(
            matches!(token.kind(), R_STRING_LITERAL),
            "Found kind {:?}",
            token.kind()
        );

        let text = token.text_trimmed();
        let text = normalize_string(text);

        FormatNormalizedStringLiteralToken { token, text }
    }
}

impl Format<RFormatContext> for FormatStringLiteralToken<'_> {
    fn fmt(&self, f: &mut RFormatter) -> FormatResult<()> {
        self.normalize().fmt(f)
    }
}

struct FormatNormalizedStringLiteralToken<'token> {
    /// The original string literal token before normalization
    token: &'token RSyntaxToken,

    /// The normalized text
    text: Cow<'token, str>,
}

impl Format<RFormatContext> for FormatNormalizedStringLiteralToken<'_> {
    fn fmt(&self, f: &mut Formatter<RFormatContext>) -> FormatResult<()> {
        format_replaced(
            self.token,
            &syntax_token_cow_slice(
                // Cloning the `Cow<str>` is cheap since 99% of the time it will be the
                // `Borrowed` variant. Only with multiline strings on Windows will it
                // ever actually clone the underlying string.
                self.text.clone(),
                self.token,
                self.token.text_trimmed_range().start(),
            ),
        )
        .fmt(f)
    }
}

/// Normalize a string, returning a [`Cow::Borrowed`] if the input was already normalized
///
/// This function:
/// - Normalizes all line endings to `\n`
///
/// We may perform more normalization in the future. We don't use utilities from the
/// `line_ending` crate because we don't own the string.
///
/// This function is particularly useful for multiline strings, which capture the existing
/// line ending inside the string token itself. We must normalize those line endings to
/// `\n` before the formatter -> printer stage, because the printer can't handle other
/// line endings and will panic on them. At the printer -> string stage at the very end,
/// the printer will replace all `\n` with the `LineEnding` requested by the user.
/// https://github.com/biomejs/biome/blob/a658a294087c143b83350cbeb6b44f7a2e9afdd1/crates/biome_formatter/src/printer/mod.rs#L714-L718
fn normalize_string(input: &str) -> Cow<str> {
    // The normalized string if `input` is not yet normalized.
    // `output` must remain empty if `input` is already normalized.
    let mut output = String::new();

    // Tracks the last index of `input` that has been written to `output`.
    // If `last_loc` is `0` at the end, then the input is already normalized and can be returned as is.
    let mut last_loc = 0;

    let mut iter = input.char_indices().peekable();

    while let Some((loc, char)) = iter.next() {
        if char == '\r' {
            output.push_str(&input[last_loc..loc]);

            if iter.peek().is_some_and(|(_, next)| next == &'\n') {
                // CRLF support - skip over the '\r' character, keep the `\n`
                iter.next();
            } else {
                // CR support - Replace the `\r` with a `\n`
                output.push('\n');
            }

            last_loc = loc + '\r'.len_utf8();
        }
    }

    if last_loc == 0 {
        Cow::Borrowed(input)
    } else {
        output.push_str(&input[last_loc..]);
        Cow::Owned(output)
    }
}

#[cfg(test)]
mod tests {
    use crate::string_literal::normalize_string;
    use std::borrow::Cow;

    #[test]
    fn normalize_empty() {
        let x = "";
        assert_eq!(normalize_string(x), Cow::Borrowed(x));
    }

    #[test]
    fn normalize_newlines() {
        let x = "abcd";
        assert_eq!(normalize_string(x), Cow::Borrowed(x));

        let x = "a\nb\nc\nd\n";
        assert_eq!(normalize_string(x), Cow::Borrowed(x));

        let x = "a\nb\rc\r\nd\n";
        assert_eq!(
            normalize_string(x),
            Cow::Owned::<str>(String::from("a\nb\nc\nd\n"))
        );
    }
}
