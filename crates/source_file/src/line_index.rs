use std::fmt;
use std::fmt::{Debug, Formatter};
use std::sync::Arc;

use biome_text_size::{TextLen, TextRange, TextSize};

use crate::source_location::LineNumber;
use crate::source_location::LineOffset;
use crate::source_location::LineOffsetEncoding;
use crate::SourceLocation;

/// Index for fast [byte offset](TextSize) to [`SourceLocation`] conversions.
///
/// Cloning a [`LineIndex`] is cheap because it only requires bumping a reference count.
#[derive(Clone, Eq, PartialEq)]
pub(crate) struct LineIndex {
    inner: Arc<LineIndexInner>,
}

#[derive(Eq, PartialEq)]
struct LineIndexInner {
    line_starts: Vec<TextSize>,
    kind: IndexKind,
}

impl LineIndex {
    /// Builds the [`LineIndex`] from the source text of a file.
    pub(crate) fn from_source_text(text: &str) -> Self {
        let mut line_starts: Vec<TextSize> = Vec::with_capacity(text.len() / 88);
        line_starts.push(TextSize::from(0));

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

    /// Returns the [SourceLocation] for an offset.
    pub(crate) fn source_location(
        &self,
        offset: TextSize,
        content: &str,
        encoding: LineOffsetEncoding,
    ) -> SourceLocation {
        let line_number = self.line_number(offset);
        let line_start = self.line_start(line_number, content);

        let line_offset = if self.is_ascii() {
            LineOffset::new((offset - line_start).into(), encoding)
        } else {
            match encoding {
                LineOffsetEncoding::UTF8 => LineOffset::new((offset - line_start).into(), encoding),
                LineOffsetEncoding::UTF16 => {
                    let line_contents_up_to_offset = &content[TextRange::new(line_start, offset)];
                    let offset = line_contents_up_to_offset
                        .encode_utf16()
                        .count()
                        .try_into()
                        .expect("A single line's offset should fit in u32");
                    LineOffset::new(offset, encoding)
                }
                LineOffsetEncoding::UTF32 => {
                    let line_contents_up_to_offset = &content[TextRange::new(line_start, offset)];
                    let offset = line_contents_up_to_offset
                        .chars()
                        .count()
                        .try_into()
                        .expect("A single line's offset should fit in u32");
                    LineOffset::new(offset, encoding)
                }
            }
        };

        SourceLocation::new(line_number, line_offset)
    }

    /// Return the number of lines in the source code.
    pub(crate) fn line_count(&self) -> usize {
        self.line_starts().len()
    }

    /// Returns `true` if the text only consists of ASCII characters.
    pub(crate) fn is_ascii(&self) -> bool {
        self.kind().is_ascii()
    }

    /// Returns the line number for a given offset.
    pub(crate) fn line_number(&self, offset: TextSize) -> LineNumber {
        let line = match self.line_starts().binary_search(&offset) {
            // `offset` is at the start of a line
            Ok(row) => row,
            Err(next_row) => {
                // SAFETY: Safe because the index always contains an entry for the offset 0
                next_row - 1
            }
        };

        LineNumber::try_from(line).expect("Number of line starts should fit in a `LineNumber`")
    }

    /// Returns the [byte offset](TextSize) for the `line`'s start.
    pub(crate) fn line_start(&self, line_number: LineNumber, contents: &str) -> TextSize {
        let line_number = usize::from(line_number);
        let starts = self.line_starts();

        // If start-of-line position after last line
        if line_number >= starts.len() {
            contents.text_len()
        } else {
            starts[line_number]
        }
    }

    /// Returns the [byte offset](TextSize) of the `line`'s end.
    /// The offset is the end of the line, up to and including the newline character ending the line (if any).
    pub(crate) fn line_end(&self, line_number: LineNumber, contents: &str) -> TextSize {
        let line_number = usize::from(line_number);
        let starts = self.line_starts();

        // If start-of-line position after last line
        if line_number.saturating_add(1) >= starts.len() {
            contents.text_len()
        } else {
            starts[line_number + 1]
        }
    }

    /// Returns the [`TextRange`] of the `line`.
    /// The start points to the first character's [byte offset](TextSize), the end up to, and including
    /// the newline character ending the line (if any).
    pub(crate) fn line_range(&self, line_number: LineNumber, contents: &str) -> TextRange {
        TextRange::new(
            self.line_start(line_number, contents),
            self.line_end(line_number, contents),
        )
    }

    /// Returns the [byte offset](TextSize) at this [SourceLocation].
    pub(crate) fn offset(&self, source_location: SourceLocation, contents: &str) -> TextSize {
        let line_number = source_location.line_number();
        let line_offset = source_location.line_offset();

        let line_range = self.line_range(line_number, contents);

        let offset = if self.is_ascii() {
            TextSize::from(line_offset.raw())
        } else {
            match line_offset.encoding() {
                LineOffsetEncoding::UTF8 => TextSize::from(line_offset.raw()),
                LineOffsetEncoding::UTF16 => {
                    let n_code_units = line_offset.raw();
                    let line_contents = &contents[line_range];

                    let mut i = 0;
                    let mut offset = 0;

                    for c in line_contents.chars() {
                        if i >= n_code_units {
                            break;
                        }
                        i += c.len_utf16() as u32;
                        offset += c.len_utf8() as u32;
                    }

                    TextSize::from(offset)
                }
                LineOffsetEncoding::UTF32 => {
                    let n_code_units = line_offset.raw();
                    let line_contents = &contents[line_range];

                    let mut offset: u32 = 0;

                    for c in line_contents.chars().take(n_code_units as usize) {
                        offset += c.len_utf8() as u32;
                    }

                    TextSize::from(offset)
                }
            }
        };

        line_range.start() + offset.clamp(TextSize::from(0), line_range.len())
    }

    /// Returns the [byte offsets](TextSize) for every line
    pub fn line_starts(&self) -> &[TextSize] {
        &self.inner.line_starts
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
    use crate::source_location::LineNumber;
    use crate::source_location::LineOffset;
    use crate::source_location::LineOffsetEncoding;
    use crate::SourceLocation;

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
        let loc = index.source_location(TextSize::from(2), contents, LineOffsetEncoding::UTF8);
        assert_eq!(
            loc,
            SourceLocation::new(
                LineNumber::from(0),
                LineOffset::new(2, LineOffsetEncoding::UTF8)
            )
        );

        // Second row.
        let loc = index.source_location(TextSize::from(6), contents, LineOffsetEncoding::UTF8);
        assert_eq!(
            loc,
            SourceLocation::new(
                LineNumber::from(1),
                LineOffset::new(0, LineOffsetEncoding::UTF8)
            )
        );

        let loc = index.source_location(TextSize::from(11), contents, LineOffsetEncoding::UTF8);
        assert_eq!(
            loc,
            SourceLocation::new(
                LineNumber::from(1),
                LineOffset::new(5, LineOffsetEncoding::UTF8)
            )
        );
    }

    #[test]
    fn ascii_carriage_return() {
        let contents = "x = 4\ry = 3";
        let index = LineIndex::from_source_text(contents);
        assert_eq!(index.line_starts(), &[TextSize::from(0), TextSize::from(6)]);

        assert_eq!(
            index.source_location(TextSize::from(4), contents, LineOffsetEncoding::UTF8),
            SourceLocation::new(
                LineNumber::from(0),
                LineOffset::new(4, LineOffsetEncoding::UTF8)
            )
        );
        assert_eq!(
            index.source_location(TextSize::from(6), contents, LineOffsetEncoding::UTF8),
            SourceLocation::new(
                LineNumber::from(1),
                LineOffset::new(0, LineOffsetEncoding::UTF8)
            )
        );
        assert_eq!(
            index.source_location(TextSize::from(7), contents, LineOffsetEncoding::UTF8),
            SourceLocation::new(
                LineNumber::from(1),
                LineOffset::new(1, LineOffsetEncoding::UTF8)
            )
        );
    }

    #[test]
    fn ascii_carriage_return_newline() {
        let contents = "x = 4\r\ny = 3";
        let index = LineIndex::from_source_text(contents);
        assert_eq!(index.line_starts(), &[TextSize::from(0), TextSize::from(7)]);

        assert_eq!(
            index.source_location(TextSize::from(4), contents, LineOffsetEncoding::UTF8),
            SourceLocation::new(
                LineNumber::from(0),
                LineOffset::new(4, LineOffsetEncoding::UTF8)
            )
        );
        assert_eq!(
            index.source_location(TextSize::from(7), contents, LineOffsetEncoding::UTF8),
            SourceLocation::new(
                LineNumber::from(1),
                LineOffset::new(0, LineOffsetEncoding::UTF8)
            )
        );
        assert_eq!(
            index.source_location(TextSize::from(8), contents, LineOffsetEncoding::UTF8),
            SourceLocation::new(
                LineNumber::from(1),
                LineOffset::new(1, LineOffsetEncoding::UTF8)
            )
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

        // Second ', UTF8
        assert_eq!(
            index.source_location(TextSize::from(9), contents, LineOffsetEncoding::UTF8),
            SourceLocation::new(
                LineNumber::from(0),
                LineOffset::new(9, LineOffsetEncoding::UTF8)
            )
        );
        assert_eq!(
            index.source_location(TextSize::from(11), contents, LineOffsetEncoding::UTF8),
            SourceLocation::new(
                LineNumber::from(1),
                LineOffset::new(0, LineOffsetEncoding::UTF8)
            )
        );
        assert_eq!(
            index.source_location(TextSize::from(12), contents, LineOffsetEncoding::UTF8),
            SourceLocation::new(
                LineNumber::from(1),
                LineOffset::new(1, LineOffsetEncoding::UTF8)
            )
        );

        // Second ', UTF16
        assert_eq!(
            index.source_location(TextSize::from(9), contents, LineOffsetEncoding::UTF16),
            SourceLocation::new(
                LineNumber::from(0),
                LineOffset::new(7, LineOffsetEncoding::UTF16)
            )
        );
        assert_eq!(
            index.source_location(TextSize::from(11), contents, LineOffsetEncoding::UTF16),
            SourceLocation::new(
                LineNumber::from(1),
                LineOffset::new(0, LineOffsetEncoding::UTF16)
            )
        );
        assert_eq!(
            index.source_location(TextSize::from(12), contents, LineOffsetEncoding::UTF16),
            SourceLocation::new(
                LineNumber::from(1),
                LineOffset::new(1, LineOffsetEncoding::UTF16)
            )
        );

        // Second ', UTF32
        assert_eq!(
            index.source_location(TextSize::from(9), contents, LineOffsetEncoding::UTF32),
            SourceLocation::new(
                LineNumber::from(0),
                LineOffset::new(6, LineOffsetEncoding::UTF32)
            )
        );
        assert_eq!(
            index.source_location(TextSize::from(11), contents, LineOffsetEncoding::UTF32),
            SourceLocation::new(
                LineNumber::from(1),
                LineOffset::new(0, LineOffsetEncoding::UTF32)
            )
        );
        assert_eq!(
            index.source_location(TextSize::from(12), contents, LineOffsetEncoding::UTF32),
            SourceLocation::new(
                LineNumber::from(1),
                LineOffset::new(1, LineOffsetEncoding::UTF32)
            )
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
            index.source_location(TextSize::from(9), contents, LineOffsetEncoding::UTF32),
            SourceLocation::new(
                LineNumber::from(0),
                LineOffset::new(6, LineOffsetEncoding::UTF32)
            )
        );
        assert_eq!(
            index.source_location(TextSize::from(12), contents, LineOffsetEncoding::UTF32),
            SourceLocation::new(
                LineNumber::from(1),
                LineOffset::new(0, LineOffsetEncoding::UTF32)
            )
        );
        assert_eq!(
            index.source_location(TextSize::from(13), contents, LineOffsetEncoding::UTF32),
            SourceLocation::new(
                LineNumber::from(1),
                LineOffset::new(1, LineOffsetEncoding::UTF32)
            )
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

        // First row, start
        let loc = index.source_location(TextSize::from(0), contents, LineOffsetEncoding::UTF8);
        assert_eq!(
            loc,
            SourceLocation::new(
                LineNumber::from(0),
                LineOffset::new(0, LineOffsetEncoding::UTF8)
            )
        );
        let loc = index.source_location(TextSize::from(0), contents, LineOffsetEncoding::UTF16);
        assert_eq!(
            loc,
            SourceLocation::new(
                LineNumber::from(0),
                LineOffset::new(0, LineOffsetEncoding::UTF16)
            )
        );
        let loc = index.source_location(TextSize::from(0), contents, LineOffsetEncoding::UTF32);
        assert_eq!(
            loc,
            SourceLocation::new(
                LineNumber::from(0),
                LineOffset::new(0, LineOffsetEncoding::UTF32)
            )
        );

        // First row, right before
        let loc = index.source_location(TextSize::from(5), contents, LineOffsetEncoding::UTF8);
        assert_eq!(
            loc,
            SourceLocation::new(
                LineNumber::from(0),
                LineOffset::new(5, LineOffsetEncoding::UTF8)
            )
        );
        let loc = index.source_location(TextSize::from(5), contents, LineOffsetEncoding::UTF16);
        assert_eq!(
            loc,
            SourceLocation::new(
                LineNumber::from(0),
                LineOffset::new(5, LineOffsetEncoding::UTF16)
            )
        );
        let loc = index.source_location(TextSize::from(5), contents, LineOffsetEncoding::UTF32);
        assert_eq!(
            loc,
            SourceLocation::new(
                LineNumber::from(0),
                LineOffset::new(5, LineOffsetEncoding::UTF32)
            )
        );

        // First row, right after
        let loc = index.source_location(TextSize::from(8), contents, LineOffsetEncoding::UTF8);
        assert_eq!(
            loc,
            SourceLocation::new(
                LineNumber::from(0),
                LineOffset::new(8, LineOffsetEncoding::UTF8)
            )
        );
        let loc = index.source_location(TextSize::from(8), contents, LineOffsetEncoding::UTF16);
        assert_eq!(
            loc,
            SourceLocation::new(
                LineNumber::from(0),
                LineOffset::new(6, LineOffsetEncoding::UTF16)
            )
        );
        let loc = index.source_location(TextSize::from(8), contents, LineOffsetEncoding::UTF32);
        assert_eq!(
            loc,
            SourceLocation::new(
                LineNumber::from(0),
                LineOffset::new(6, LineOffsetEncoding::UTF32)
            )
        );

        // Second row, start
        let loc = index.source_location(TextSize::from(10), contents, LineOffsetEncoding::UTF8);
        assert_eq!(
            loc,
            SourceLocation::new(
                LineNumber::from(1),
                LineOffset::new(0, LineOffsetEncoding::UTF8)
            )
        );
        let loc = index.source_location(TextSize::from(10), contents, LineOffsetEncoding::UTF16);
        assert_eq!(
            loc,
            SourceLocation::new(
                LineNumber::from(1),
                LineOffset::new(0, LineOffsetEncoding::UTF16)
            )
        );
        let loc = index.source_location(TextSize::from(10), contents, LineOffsetEncoding::UTF32);
        assert_eq!(
            loc,
            SourceLocation::new(
                LineNumber::from(1),
                LineOffset::new(0, LineOffsetEncoding::UTF32)
            )
        );

        // One-past-the-end.
        let loc = index.source_location(TextSize::from(15), contents, LineOffsetEncoding::UTF8);
        assert_eq!(
            loc,
            SourceLocation::new(
                LineNumber::from(1),
                LineOffset::new(5, LineOffsetEncoding::UTF8)
            )
        );
        let loc = index.source_location(TextSize::from(15), contents, LineOffsetEncoding::UTF16);
        assert_eq!(
            loc,
            SourceLocation::new(
                LineNumber::from(1),
                LineOffset::new(5, LineOffsetEncoding::UTF16)
            )
        );
        let loc = index.source_location(TextSize::from(15), contents, LineOffsetEncoding::UTF32);
        assert_eq!(
            loc,
            SourceLocation::new(
                LineNumber::from(1),
                LineOffset::new(5, LineOffsetEncoding::UTF32)
            )
        );
    }
}
