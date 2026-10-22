use crate::RStringValue;
use crate::RSyntaxKind::R_DOUBLE_QUOTED_STRING_LITERAL;
use crate::RSyntaxKind::R_RAW_STRING_LITERAL;
use crate::RSyntaxKind::R_SINGLE_QUOTED_STRING_LITERAL;
use crate::RSyntaxToken;
use biome_rowan::SyntaxResult;
use biome_rowan::TextRange;
use biome_rowan::TextSize;
use biome_rowan::TokenText;

impl RStringValue {
    /// The text between the delimiters
    ///
    /// `"foo"` and `r"---(foo)---"` both give `foo`
    pub fn inner_string_text(&self) -> SyntaxResult<TokenText> {
        let token = self.value()?;
        Ok(token.token_text_trimmed().slice(inner_string_range(&token)))
    }
}

/// Range of the inner text
///
/// # Safety
///
/// String literals always carry both of their delimiters. Broken strings are
/// returned from the lexer as `ERROR_TOKEN`s, making this safe on any string we
/// see.
fn inner_string_range(token: &RSyntaxToken) -> TextRange {
    let text = token.text_trimmed();

    let (open, close) = match token.kind() {
        R_SINGLE_QUOTED_STRING_LITERAL | R_DOUBLE_QUOTED_STRING_LITERAL => (1, 1),
        // `r`, the quote, the hyphens, and the bracket
        R_RAW_STRING_LITERAL => {
            let hyphens = text.as_bytes()[2..]
                .iter()
                .take_while(|byte| **byte == b'-')
                .count();
            (3 + hyphens, 2 + hyphens)
        }
        kind => unreachable!("`{kind:?}` is not a string"),
    };

    TextRange::new(
        TextSize::from(open as u32),
        TextSize::from((text.len() - close) as u32),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::RSyntaxKind;
    use crate::RSyntaxKind::R_STRING_VALUE;
    use crate::RSyntaxNode;
    use biome_rowan::AstNode;
    use biome_rowan::TriviaPiece;

    /// The parser cannot be reached from here, so the token is built by hand.
    /// `lexer/tests/strings.rs` is what pins that the lexer really produces it.
    fn string_value(kind: RSyntaxKind, text: &str, leading: usize) -> RStringValue {
        let leading = (leading > 0).then(|| TriviaPiece::whitespace(leading as u32));
        let token = RSyntaxToken::new_detached(kind, text, leading, []);
        let node = RSyntaxNode::new_detached(R_STRING_VALUE, [Some(token.into())]);

        RStringValue::cast(node).expect("a string value")
    }

    #[track_caller]
    fn assert_inner(kind: RSyntaxKind, text: &str, expected: &str, range: TextRange) {
        let value = string_value(kind, text, 0);

        assert_eq!(value.inner_string_text().unwrap().text(), expected);
        assert_eq!(inner_string_range(&value.value().unwrap()), range);
        assert_eq!(&text[range], expected);
    }

    fn range(start: u32, end: u32) -> TextRange {
        TextRange::new(TextSize::from(start), TextSize::from(end))
    }

    #[test]
    fn quoted_strings() {
        assert_inner(
            R_DOUBLE_QUOTED_STRING_LITERAL,
            "\"foo\"",
            "foo",
            range(1, 4),
        );
        assert_inner(R_SINGLE_QUOTED_STRING_LITERAL, "'foo'", "foo", range(1, 4));
    }

    #[test]
    fn empty_strings() {
        assert_inner(R_SINGLE_QUOTED_STRING_LITERAL, "''", "", range(1, 1));
        assert_inner(R_DOUBLE_QUOTED_STRING_LITERAL, "\"\"", "", range(1, 1));
        assert_inner(R_RAW_STRING_LITERAL, "r\"()\"", "", range(3, 3));
    }

    #[test]
    fn raw_strings() {
        assert_inner(R_RAW_STRING_LITERAL, "r\"(a)\"", "a", range(3, 4));
        assert_inner(R_RAW_STRING_LITERAL, "r\"---(a)---\"", "a", range(6, 7));
        assert_inner(R_RAW_STRING_LITERAL, "R'--[a]--'", "a", range(5, 6));
        assert_inner(R_RAW_STRING_LITERAL, "r\"-|a|-\"", "a", range(4, 5));
    }

    /// A closing bracket that does not close belongs to the content
    #[test]
    fn a_raw_string_containing_its_own_bracket() {
        assert_inner(R_RAW_STRING_LITERAL, "r\"(())\"", "()", range(3, 5));
        assert_inner(R_RAW_STRING_LITERAL, "r\"|a|b|\"", "a|b", range(3, 6));
    }

    #[test]
    fn multiline_strings() {
        assert_inner(
            R_DOUBLE_QUOTED_STRING_LITERAL,
            "\"a\nb\"",
            "a\nb",
            range(1, 4),
        );
        assert_inner(R_RAW_STRING_LITERAL, "r\"(a\nb)\"", "a\nb", range(3, 6));
    }

    /// Escapes are handed back exactly as written
    #[test]
    fn nothing_is_unescaped() {
        assert_inner(
            R_DOUBLE_QUOTED_STRING_LITERAL,
            "\"a\\nb\"",
            "a\\nb",
            range(1, 5),
        );
        assert_inner(
            R_DOUBLE_QUOTED_STRING_LITERAL,
            "\"a\\\"b\"",
            "a\\\"b",
            range(1, 5),
        );
    }

    /// The range is off the trimmed token, so leading trivia does not shift it
    #[test]
    fn trivia_is_not_content() {
        let value = string_value(R_DOUBLE_QUOTED_STRING_LITERAL, "  \"foo\"", 2);
        let token = value.value().unwrap();

        assert_eq!(token.text_trimmed(), "\"foo\"");
        assert_eq!(value.inner_string_text().unwrap().text(), "foo");
        assert_eq!(inner_string_range(&token), range(1, 4));
    }
}
