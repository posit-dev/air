use std::fmt;
use std::fmt::{Debug, Formatter};
use std::ops::Deref;
use std::sync::Arc;

use biome_text_size::{TextLen, TextRange, TextSize};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::OneIndexed;
use crate::SourceLocation;

/// Index for fast [byte offset](TextSize) to [`SourceLocation`] conversions.
///
/// Cloning a [`LineIndex`] is cheap because it only requires bumping a reference count.
#[derive(Clone, Eq, PartialEq)]
pub struct LineIndex {
    inner: Arc<LineIndexInner>,
}

#[derive(Eq, PartialEq)]
struct LineIndexInner {
    line_starts: Vec<TextSize>,
    kind: IndexKind,
}

impl LineIndex {
    /// Builds the [`LineIndex`] from the source text of a file.
    pub fn from_source_text(text: &str) -> Self {
        let mut line_starts: Vec<TextSize> = Vec::with_capacity(text.len() / 88);
        line_starts.push(TextSize::default());

        let bytes = text.as_bytes();
        let mut utf8 = false;

        assert!(u32::try_from(bytes.len()).is_ok());

        for (i, byte) in bytes.iter().enumerate() {
            utf8 |= !byte.is_ascii();

            match byte {
                // Only track one line break for `\r\n`.
                b'\r' if bytes.get(i + 1) == Some(&b'\n') => continue,
                b'\n' | b'\r' => {
                    // SAFETY: Assertion above guarantees `i <= u32::MAX`
                    #[allow(clippy::cast_possible_truncation)]
                    line_starts.push(TextSize::from(i as u32) + TextSize::from(1));
                }
                _ => {}
            }
        }

        let kind = if utf8 {
            IndexKind::Utf8
        } else {
            IndexKind::Ascii
        };

        Self {
            inner: Arc::new(LineIndexInner { line_starts, kind }),
        }
    }

    fn kind(&self) -> IndexKind {
        self.inner.kind
    }

    /// Returns the row and column index for an offset.
    ///
    /// ## Examples
    ///
    /// ```
    /// # use biome_text_size::TextSize;
    /// # use source_file::{LineIndex, OneIndexed, SourceLocation};
    /// let source = "def a():\n    pass";
    /// let index = LineIndex::from_source_text(source);
    ///
    /// assert_eq!(
    ///     index.source_location(TextSize::from(0), source),
    ///     SourceLocation { row: OneIndexed::from_zero_indexed(0), column: OneIndexed::from_zero_indexed(0) }
    /// );
    ///
    /// assert_eq!(
    ///     index.source_location(TextSize::from(4), source),
    ///     SourceLocation { row: OneIndexed::from_zero_indexed(0), column: OneIndexed::from_zero_indexed(4) }
    /// );
    /// assert_eq!(
    ///     index.source_location(TextSize::from(13), source),
    ///     SourceLocation { row: OneIndexed::from_zero_indexed(1), column: OneIndexed::from_zero_indexed(4) }
    /// );
    /// ```
    ///
    /// ## Panics
    ///
    /// If the offset is out of bounds.
    pub fn source_location(&self, offset: TextSize, content: &str) -> SourceLocation {
        match self.line_starts().binary_search(&offset) {
            // Offset is at the start of a line
            Ok(row) => SourceLocation {
                row: OneIndexed::from_zero_indexed(row),
                column: OneIndexed::from_zero_indexed(0),
            },
            Err(next_row) => {
                // SAFETY: Safe because the index always contains an entry for the offset 0
                let row = next_row - 1;
                let mut line_start = self.line_starts()[row];

                let column = if self.kind().is_ascii() {
                    usize::from(offset) - usize::from(line_start)
                } else {
                    // Don't count the BOM character as a column.
                    if line_start == TextSize::from(0) && content.starts_with('\u{feff}') {
                        line_start = '\u{feff}'.text_len();
                    }

                    content[TextRange::new(line_start, offset)].chars().count()
                };

                SourceLocation {
                    row: OneIndexed::from_zero_indexed(row),
                    column: OneIndexed::from_zero_indexed(column),
                }
            }
        }
    }

    /// Return the number of lines in the source code.
    pub fn line_count(&self) -> usize {
        self.line_starts().len()
    }

    /// Returns `true` if the text only consists of ASCII characters
    pub fn is_ascii(&self) -> bool {
        self.kind().is_ascii()
    }

    /// Returns the row number for a given offset.
    ///
    /// ## Examples
    ///
    /// ```
    /// # use biome_text_size::TextSize;
    /// # use source_file::{LineIndex, OneIndexed, SourceLocation};
    /// let source = "def a():\n    pass";
    /// let index = LineIndex::from_source_text(source);
    ///
    /// assert_eq!(index.line_index(TextSize::from(0)), OneIndexed::from_zero_indexed(0));
    /// assert_eq!(index.line_index(TextSize::from(4)), OneIndexed::from_zero_indexed(0));
    /// assert_eq!(index.line_index(TextSize::from(13)), OneIndexed::from_zero_indexed(1));
    /// ```
    ///
    /// ## Panics
    ///
    /// If the offset is out of bounds.
    pub fn line_index(&self, offset: TextSize) -> OneIndexed {
        match self.line_starts().binary_search(&offset) {
            // Offset is at the start of a line
            Ok(row) => OneIndexed::from_zero_indexed(row),
            Err(row) => {
                // SAFETY: Safe because the index always contains an entry for the offset 0
                OneIndexed::from_zero_indexed(row - 1)
            }
        }
    }

    /// Returns the [byte offset](TextSize) for the `line` with the given index.
    pub fn line_start(&self, line: OneIndexed, contents: &str) -> TextSize {
        let row_index = line.to_zero_indexed();
        let starts = self.line_starts();

        // If start-of-line position after last line
        if row_index == starts.len() {
            contents.text_len()
        } else {
            starts[row_index]
        }
    }

    /// Returns the [byte offset](TextSize) of the `line`'s end.
    /// The offset is the end of the line, up to and including the newline character ending the line (if any).
    pub fn line_end(&self, line: OneIndexed, contents: &str) -> TextSize {
        let row_index = line.to_zero_indexed();
        let starts = self.line_starts();

        // If start-of-line position after last line
        if row_index.saturating_add(1) >= starts.len() {
            contents.text_len()
        } else {
            starts[row_index + 1]
        }
    }

    /// Returns the [byte offset](TextSize) of the `line`'s end.
    /// The offset is the end of the line, excluding the newline character ending the line (if any).
    pub fn line_end_exclusive(&self, line: OneIndexed, contents: &str) -> TextSize {
        let row_index = line.to_zero_indexed();
        let starts = self.line_starts();

        // If start-of-line position after last line
        if row_index.saturating_add(1) >= starts.len() {
            contents.text_len()
        } else {
            starts[row_index + 1] - TextSize::from(1)
        }
    }

    /// Returns the [`TextRange`] of the `line` with the given index.
    /// The start points to the first character's [byte offset](TextSize), the end up to, and including
    /// the newline character ending the line (if any).
    pub fn line_range(&self, line: OneIndexed, contents: &str) -> TextRange {
        let starts = self.line_starts();

        if starts.len() == line.to_zero_indexed() {
            TextRange::empty(contents.text_len())
        } else {
            TextRange::new(
                self.line_start(line, contents),
                self.line_start(line.saturating_add(1), contents),
            )
        }
    }

    /// Returns the [byte offset](TextSize) at `line` and `column`.
    ///
    /// ## Examples
    ///
    /// ### ASCII
    ///
    /// ```
    /// use source_file::{LineIndex, OneIndexed};
    /// use biome_text_size::TextSize;
    /// let source = r#"a = 4
    /// c = "some string"
    /// x = b"#;
    ///
    /// let index = LineIndex::from_source_text(source);
    ///
    /// // First line, first column
    /// assert_eq!(index.offset(OneIndexed::from_zero_indexed(0), OneIndexed::from_zero_indexed(0), source), TextSize::from(0));
    ///
    /// // Second line, 4th column
    /// assert_eq!(index.offset(OneIndexed::from_zero_indexed(1), OneIndexed::from_zero_indexed(4), source), TextSize::from(10));
    ///
    /// // Offset past the end of the first line
    /// assert_eq!(index.offset(OneIndexed::from_zero_indexed(0), OneIndexed::from_zero_indexed(10), source), TextSize::from(6));
    ///
    /// // Offset past the end of the file
    /// assert_eq!(index.offset(OneIndexed::from_zero_indexed(3), OneIndexed::from_zero_indexed(0), source), TextSize::from(29));
    /// ```
    ///
    /// ### UTF8
    ///
    /// ```
    /// use source_file::{LineIndex, OneIndexed};
    /// use biome_text_size::TextSize;
    /// let source = r#"a = 4
    /// c = "❤️"
    /// x = b"#;
    ///
    /// let index = LineIndex::from_source_text(source);
    ///
    /// // First line, first column
    /// assert_eq!(index.offset(OneIndexed::from_zero_indexed(0), OneIndexed::from_zero_indexed(0), source), TextSize::from(0));
    ///
    /// // Third line, 2nd column, after emoji
    /// assert_eq!(index.offset(OneIndexed::from_zero_indexed(2), OneIndexed::from_zero_indexed(1), source), TextSize::from(20));
    ///
    /// // Offset past the end of the second line
    /// assert_eq!(index.offset(OneIndexed::from_zero_indexed(1), OneIndexed::from_zero_indexed(10), source), TextSize::from(19));
    ///
    /// // Offset past the end of the file
    /// assert_eq!(index.offset(OneIndexed::from_zero_indexed(3), OneIndexed::from_zero_indexed(0), source), TextSize::from(24));
    /// ```
    ///
    pub fn offset(&self, line: OneIndexed, column: OneIndexed, contents: &str) -> TextSize {
        // If start-of-line position after last line
        if line.to_zero_indexed() > self.line_starts().len() {
            return contents.text_len();
        }

        let line_range = self.line_range(line, contents);

        match self.kind() {
            IndexKind::Ascii => {
                line_range.start()
                    + TextSize::try_from(column.to_zero_indexed())
                        .unwrap_or(line_range.len())
                        .clamp(TextSize::from(0), line_range.len())
            }
            IndexKind::Utf8 => {
                let rest = &contents[line_range];
                let column_offset: TextSize = rest
                    .chars()
                    .take(column.to_zero_indexed())
                    .map(biome_text_size::TextLen::text_len)
                    .sum();
                line_range.start() + column_offset
            }
        }
    }

    /// Returns the [byte offsets](TextSize) for every line
    pub fn line_starts(&self) -> &[TextSize] {
        &self.inner.line_starts
    }
}

impl Deref for LineIndex {
    type Target = [TextSize];

    fn deref(&self) -> &Self::Target {
        self.line_starts()
    }
}

impl Debug for LineIndex {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.line_starts()).finish()
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum IndexKind {
    /// Optimized index for an ASCII only document
    Ascii,

    /// Index for UTF8 documents
    Utf8,
}

impl IndexKind {
    const fn is_ascii(self) -> bool {
        matches!(self, IndexKind::Ascii)
    }
}

#[cfg(test)]
mod tests {
    use biome_text_size::TextSize;

    use crate::line_index::LineIndex;
    use crate::{OneIndexed, SourceLocation};

    #[test]
    fn ascii_index() {
        let index = LineIndex::from_source_text("");
        assert_eq!(index.line_starts(), &[TextSize::from(0)]);

        let index = LineIndex::from_source_text("x = 1");
        assert_eq!(index.line_starts(), &[TextSize::from(0)]);

        let index = LineIndex::from_source_text("x = 1\n");
        assert_eq!(index.line_starts(), &[TextSize::from(0), TextSize::from(6)]);

        let index = LineIndex::from_source_text("x = 1\ny = 2\nz = x + y\n");
        assert_eq!(
            index.line_starts(),
            &[
                TextSize::from(0),
                TextSize::from(6),
                TextSize::from(12),
                TextSize::from(22)
            ]
        );
    }

    #[test]
    fn ascii_source_location() {
        let contents = "x = 1\ny = 2";
        let index = LineIndex::from_source_text(contents);

        // First row.
        let loc = index.source_location(TextSize::from(2), contents);
        assert_eq!(
            loc,
            SourceLocation {
                row: OneIndexed::from_zero_indexed(0),
                column: OneIndexed::from_zero_indexed(2)
            }
        );

        // Second row.
        let loc = index.source_location(TextSize::from(6), contents);
        assert_eq!(
            loc,
            SourceLocation {
                row: OneIndexed::from_zero_indexed(1),
                column: OneIndexed::from_zero_indexed(0)
            }
        );

        let loc = index.source_location(TextSize::from(11), contents);
        assert_eq!(
            loc,
            SourceLocation {
                row: OneIndexed::from_zero_indexed(1),
                column: OneIndexed::from_zero_indexed(5)
            }
        );
    }

    #[test]
    fn ascii_carriage_return() {
        let contents = "x = 4\ry = 3";
        let index = LineIndex::from_source_text(contents);
        assert_eq!(index.line_starts(), &[TextSize::from(0), TextSize::from(6)]);

        assert_eq!(
            index.source_location(TextSize::from(4), contents),
            SourceLocation {
                row: OneIndexed::from_zero_indexed(0),
                column: OneIndexed::from_zero_indexed(4)
            }
        );
        assert_eq!(
            index.source_location(TextSize::from(6), contents),
            SourceLocation {
                row: OneIndexed::from_zero_indexed(1),
                column: OneIndexed::from_zero_indexed(0)
            }
        );
        assert_eq!(
            index.source_location(TextSize::from(7), contents),
            SourceLocation {
                row: OneIndexed::from_zero_indexed(1),
                column: OneIndexed::from_zero_indexed(1)
            }
        );
    }

    #[test]
    fn ascii_carriage_return_newline() {
        let contents = "x = 4\r\ny = 3";
        let index = LineIndex::from_source_text(contents);
        assert_eq!(index.line_starts(), &[TextSize::from(0), TextSize::from(7)]);

        assert_eq!(
            index.source_location(TextSize::from(4), contents),
            SourceLocation {
                row: OneIndexed::from_zero_indexed(0),
                column: OneIndexed::from_zero_indexed(4)
            }
        );
        assert_eq!(
            index.source_location(TextSize::from(7), contents),
            SourceLocation {
                row: OneIndexed::from_zero_indexed(1),
                column: OneIndexed::from_zero_indexed(0)
            }
        );
        assert_eq!(
            index.source_location(TextSize::from(8), contents),
            SourceLocation {
                row: OneIndexed::from_zero_indexed(1),
                column: OneIndexed::from_zero_indexed(1)
            }
        );
    }

    #[test]
    fn utf8_index() {
        let index = LineIndex::from_source_text("x = '🫣'");
        assert_eq!(index.line_count(), 1);
        assert_eq!(index.line_starts(), &[TextSize::from(0)]);

        let index = LineIndex::from_source_text("x = '🫣'\n");
        assert_eq!(index.line_count(), 2);
        assert_eq!(
            index.line_starts(),
            &[TextSize::from(0), TextSize::from(11)]
        );

        let index = LineIndex::from_source_text("x = '🫣'\ny = 2\nz = x + y\n");
        assert_eq!(index.line_count(), 4);
        assert_eq!(
            index.line_starts(),
            &[
                TextSize::from(0),
                TextSize::from(11),
                TextSize::from(17),
                TextSize::from(27)
            ]
        );

        let index = LineIndex::from_source_text("# 🫣\nclass Foo:\n    \"\"\".\"\"\"");
        assert_eq!(index.line_count(), 3);
        assert_eq!(
            index.line_starts(),
            &[TextSize::from(0), TextSize::from(7), TextSize::from(18)]
        );
    }

    #[test]
    fn utf8_carriage_return() {
        let contents = "x = '🫣'\ry = 3";
        let index = LineIndex::from_source_text(contents);
        assert_eq!(index.line_count(), 2);
        assert_eq!(
            index.line_starts(),
            &[TextSize::from(0), TextSize::from(11)]
        );

        // Second '
        assert_eq!(
            index.source_location(TextSize::from(9), contents),
            SourceLocation {
                row: OneIndexed::from_zero_indexed(0),
                column: OneIndexed::from_zero_indexed(6)
            }
        );
        assert_eq!(
            index.source_location(TextSize::from(11), contents),
            SourceLocation {
                row: OneIndexed::from_zero_indexed(1),
                column: OneIndexed::from_zero_indexed(0)
            }
        );
        assert_eq!(
            index.source_location(TextSize::from(12), contents),
            SourceLocation {
                row: OneIndexed::from_zero_indexed(1),
                column: OneIndexed::from_zero_indexed(1)
            }
        );
    }

    #[test]
    fn utf8_carriage_return_newline() {
        let contents = "x = '🫣'\r\ny = 3";
        let index = LineIndex::from_source_text(contents);
        assert_eq!(index.line_count(), 2);
        assert_eq!(
            index.line_starts(),
            &[TextSize::from(0), TextSize::from(12)]
        );

        // Second '
        assert_eq!(
            index.source_location(TextSize::from(9), contents),
            SourceLocation {
                row: OneIndexed::from_zero_indexed(0),
                column: OneIndexed::from_zero_indexed(6)
            }
        );
        assert_eq!(
            index.source_location(TextSize::from(12), contents),
            SourceLocation {
                row: OneIndexed::from_zero_indexed(1),
                column: OneIndexed::from_zero_indexed(0)
            }
        );
        assert_eq!(
            index.source_location(TextSize::from(13), contents),
            SourceLocation {
                row: OneIndexed::from_zero_indexed(1),
                column: OneIndexed::from_zero_indexed(1)
            }
        );
    }

    #[test]
    fn utf8_byte_offset() {
        let contents = "x = '☃'\ny = 2";
        let index = LineIndex::from_source_text(contents);
        assert_eq!(
            index.line_starts(),
            &[TextSize::from(0), TextSize::from(10)]
        );

        // First row.
        let loc = index.source_location(TextSize::from(0), contents);
        assert_eq!(
            loc,
            SourceLocation {
                row: OneIndexed::from_zero_indexed(0),
                column: OneIndexed::from_zero_indexed(0)
            }
        );

        let loc = index.source_location(TextSize::from(5), contents);
        assert_eq!(
            loc,
            SourceLocation {
                row: OneIndexed::from_zero_indexed(0),
                column: OneIndexed::from_zero_indexed(5)
            }
        );

        let loc = index.source_location(TextSize::from(8), contents);
        assert_eq!(
            loc,
            SourceLocation {
                row: OneIndexed::from_zero_indexed(0),
                column: OneIndexed::from_zero_indexed(6)
            }
        );

        // Second row.
        let loc = index.source_location(TextSize::from(10), contents);
        assert_eq!(
            loc,
            SourceLocation {
                row: OneIndexed::from_zero_indexed(1),
                column: OneIndexed::from_zero_indexed(0)
            }
        );

        // One-past-the-end.
        let loc = index.source_location(TextSize::from(15), contents);
        assert_eq!(
            loc,
            SourceLocation {
                row: OneIndexed::from_zero_indexed(1),
                column: OneIndexed::from_zero_indexed(5)
            }
        );
    }
}