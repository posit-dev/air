//! This top level documentation details the algorithms and terminology behind
//! [SourceFile::offset] and [SourceFile::source_location]. The primary goal of
//! these functions (and really this whole module) is to handle conversion between
//! a [byte offset](TextSize) and a [line number + line offset](SourceLocation),
//! including treatment of various UTF encodings.
//!
//! Both [TextSize] and [SourceLocation] are ways of pointing at a location within a
//! file:
//!
//! - A [TextSize] is a simple byte offset into a UTF-8 encoded [String].
//!
//! - A [SourceLocation] contains a location encoded as `line_number` and `line_offset`,
//!   where:
//!   - `line_number` ([LineNumber]) represents the 0-indexed line number
//!   - `line_offset` ([LineOffset]) represents the 0-indexed offset from the start of the
//!     line represented by `line_number`. The offset itself is (critically) measured in a
//!     UTF concept known as "code units", and is meaningless without the corresponding
//!     [LineOffsetEncoding].
//!
//! A [SourceLocation] is meant to map to an LSP `Position`, and a [LineOffsetEncoding] is
//! meant to map to an LSP `PositionEncodingKind`. We are typically handed an LSP
//! `Position`, must convert it to a [TextSize] (by going through [SourceLocation]), then
//! we use that [TextSize] to index into our [String] representing our document. On the
//! way out, we must convert from [TextSize] or [TextRange] back to LSP `Position` or LSP
//! `Range` by going back through [SourceLocation].
//!
//! Now, for some definitions:
//!
//! - Code unit: The minimal bit combination that can represent a single character,
//!   depending on the encoding used.
//!   - UTF-8:
//!     - 1 code unit = 1 byte = 8 bits
//!   - UTF-16:
//!     - 1 code unit = 2 bytes = 16 bits
//!   - UTF-32:
//!     - 1 code unit = 4 bytes = 32 bits
//!
//! - Character: A combination of code units that construct a single UTF element.
//!   - UTF-8:
//!     - 1 character = 1,2,3,4 code units = 1,2,3,4 bytes = 8,16,24,32 bits
//!   - UTF-16:
//!     - 1 character = 1,2 code units = 2,4 bytes = 16,32 bits
//!   - UTF-32:
//!     - 1 character = 1 code units = 4 bytes = 32 bits
//!
//! - Unicode Scalar Value: Any Unicode Code Point other than a Surrogate Code Point (
//!   which are only used by UTF-16). Technically, this means any value in the range of
//!   [0 to 0x10FFFF] excluding the slice of [0xD800 to 0xDFFF]. The [char] type
//!   represents these.
//!
//! - Unicode Code Point: Any value in the Unicode code space of [0 to 0x10FFFF]. This
//!   means that something representing an arbitrary code point must be 4 bytes, implying
//!   that something representing a Unicode Scalar Value must also be 4 bytes, and
//!   practically a [char] has the same memory layout as a [u32] under the hood.
//!
//! - Rust [String] and [str] are in UTF-8, and all [byte offsets](TextSize) into them
//!   assume the strings are encoded in UTF-8.
//!
//! One key thing to note is that `\n` (or `\r\n`) is the same in all encodings. This
//! means that finding the [LineNumber] you are on is easy, you are either given it
//! through [SourceLocation::line_number], or it can be easily computed from a UTF-8 [byte
//! offset](TextSize) by doing a binary search into an ordered vector of line start
//! locations. That isolates the "hard" details of encoding translation to the
//! [LineOffset], which is typically an extremely small slice of the overall file.
//!
//! # Implementing [SourceFile::offset]
//!
//! ## UTF-8 code units -> UTF-8 byte offset
//!
//! Easy! 1 UTF-8 code unit maps directly to 1 byte in a UTF-8 string, so counting the
//! code units is equivalent to finding the byte offset into the UTF-8 string.
//!
//! ## UTF-16 code units -> UTF-8 byte offset
//!
//! 1 UTF-16 code unit is always 2 bytes if the string is encoded in UTF-16. But if
//! the string is encoded in UTF-8 as ours is, we don't immediately know if the
//! UTF-16 code unit would be represented by 1 or 2 bytes in a UTF-8 string.
//!
//! To do this, we iterate over the [str::chars()] of the string, which are Unicode Scalar
//! Values, i.e. a UTF-32 character, the widest of them all. To figure out the correct
//! amount of UTF-16 code units to count up, we compute the [char::len_utf16()] of each
//! character, which returns the number of UTF-16 code units required if the [char]
//! were instead encoded in UTF-16. Once we've reached the [LineOffset] offset, we've
//! found all the [char]s we care about. To find the byte offset in UTF-8 encoded space,
//! we sum up the [char::len_utf8()] of each of those [char]s.
//!
//! ## UTF-32 code units -> UTF-8 byte offset
//!
//! Very similar to UTF-16, except 1 UTF-32 code unit is always 4 bytes if the string
//! itself is encoded in UTF-32.
//!
//! This is slightly easier than UTF-16. Because [str::chars()] already returns Unicode
//! Scalar Values, also known as UTF-32 characters, and because 1 UTF-32 character is
//! the same size as 1 UTF-32 code unit, we just iterate over the [str::chars()] up to
//! the [LineOffset] value, summing the [char::len_utf8()] of each [char] along the way.
//!
//! # Implementing [SourceFile::source_location]
//!
//! ## UTF-8 byte offset -> UTF-8 code units
//!
//! Easy! Like with the other direction, UTF-8 byte offsets can be directly translated
//! to UTF-8 code units, so there is nothing to do.
//!
//! ## UTF-8 byte offset -> UTF-16 code units
//!
//! This is actually pretty easy. All we do is slice the [String] from its start up to
//! the UTF-8 byte offset in question, then call [str::encode_utf16()] and count the
//! number of UTF-16 code units it returns.
//!
//! This would be expensive if we had to reencode as UTF-16 from the beginning of the
//! file, but we actually just need to reencode as UTF-16 from the beginning of the
//! line that the offset is on, up to the offset position itself, which is a very small
//! slice. This works because the line number itself is not dependent on the encoding,
//! only the line offset into that line is.
//!
//! ## UTF-8 byte offset -> UTF-32 code units
//!
//! Same as with UTF-16, but rather than [str::encode_utf16()], we can use [str::chars()]
//! because it already returns Unicode Scalar Values, which are UTF-32 characters, which
//! are equivalent to UTF-32 code units.

use biome_text_size::TextLen;
use biome_text_size::TextRange;
use biome_text_size::TextSize;

use crate::source_location::LineNumber;
use crate::source_location::LineOffsetEncoding;
use crate::LineOffset;
use crate::SourceLocation;

/// Manager of a single source file
///
/// Builds a vector of line start locations on creation, for use with
/// [TextSize] <-> [SourceLocation] conversions. In particular, see:
///
/// - [Self::offset()] for [SourceLocation] -> [TextSize]
/// - [Self::source_location()] for [TextSize] -> [SourceLocation]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SourceFile {
    contents: String,
    line_starts: Vec<TextSize>,
    kind: SourceKind,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum SourceKind {
    /// Optimized for an ASCII only document
    Ascii,

    /// Document containing UTF-8
    Utf8,
}

impl SourceKind {
    const fn is_ascii(self) -> bool {
        matches!(self, SourceKind::Ascii)
    }
}

impl SourceFile {
    /// Builds the [`SourceFile`] from the contents of a file.
    pub fn new(contents: String) -> Self {
        let (line_starts, kind) = Self::analyze(&contents);
        Self {
            contents,
            line_starts,
            kind,
        }
    }

    fn analyze(contents: &str) -> (Vec<TextSize>, SourceKind) {
        let mut line_starts: Vec<TextSize> = Vec::with_capacity(contents.len() / 88);

        // Always push a start for an offset of `0`, needed for an invariant in `line_number()`
        line_starts.push(TextSize::from(0));

        let mut utf8 = false;

        let bytes = contents.as_bytes();
        assert!(u32::try_from(bytes.len()).is_ok());

        for (i, byte) in bytes.iter().enumerate() {
            utf8 |= !byte.is_ascii();

            match byte {
                // Only track one line break for `\r\n`.
                b'\r' if bytes.get(i + 1) == Some(&b'\n') => continue,
                b'\n' | b'\r' => {
                    // Safety: Assertion above guarantees `i <= u32::MAX`
                    #[allow(clippy::cast_possible_truncation)]
                    line_starts.push(TextSize::from(i as u32) + TextSize::from(1));
                }
                _ => {}
            }
        }

        let kind = if utf8 {
            SourceKind::Utf8
        } else {
            SourceKind::Ascii
        };

        (line_starts, kind)
    }

    /// Returns a reference to the contents in the source file.
    pub fn contents(&self) -> &str {
        &self.contents
    }

    /// Consumes the source file, returning only the contents.
    pub fn into_contents(self) -> String {
        self.contents
    }

    /// Replace text in the source file and reanalyze afterwards.
    pub fn replace_range<R>(&mut self, range: R, replace_with: &str)
    where
        R: std::ops::RangeBounds<usize>,
    {
        self.contents.replace_range(range, replace_with);
        let (line_starts, kind) = Self::analyze(&self.contents);
        self.line_starts = line_starts;
        self.kind = kind;
    }

    /// Returns `true` if the text only consists of ASCII characters
    pub fn is_ascii(&self) -> bool {
        self.kind.is_ascii()
    }

    /// Return the number of lines in the source file.
    pub fn line_count(&self) -> usize {
        self.line_starts().len()
    }

    /// Returns the row number for a given offset.
    ///
    /// ## Examples
    ///
    /// ```
    /// use biome_text_size::TextSize;
    /// use source_file::{SourceFile, SourceLocation, LineNumber};
    ///
    /// let source = "def a():\n    pass".to_string();
    /// let source = SourceFile::new(source);
    ///
    /// assert_eq!(source.line_number(TextSize::from(0)), LineNumber::from(0));
    /// assert_eq!(source.line_number(TextSize::from(4)), LineNumber::from(0));
    /// assert_eq!(source.line_number(TextSize::from(13)), LineNumber::from(1));
    /// ```
    pub fn line_number(&self, offset: TextSize) -> LineNumber {
        let line_number = match self.line_starts().binary_search(&offset) {
            // `offset` is at the start of a line
            Ok(line_number) => line_number,
            Err(next_line_number) => {
                // SAFETY: Safe because the line starts always contain an entry for the offset 0
                next_line_number - 1
            }
        };

        LineNumber::try_from(line_number)
            .expect("Number of line starts should fit in a `LineNumber`")
    }

    /// Returns the [byte offset](TextSize) for the line's start.
    pub fn line_start(&self, line_number: LineNumber) -> TextSize {
        let line_number = usize::from(line_number);

        if line_number >= self.line_count() {
            // If asking for a line number past the last line, return last byte
            self.contents().text_len()
        } else {
            self.line_starts()[line_number]
        }
    }

    /// Returns the [byte offset](TextSize) of the line's end.
    ///
    /// The offset is the end of the line, up to and including the newline character
    /// ending the line (if any), making it equivalent to the next line's start.
    pub(crate) fn line_end(&self, line_number: LineNumber) -> TextSize {
        let line_number = usize::from(line_number);

        if line_number.saturating_add(1) >= self.line_count() {
            // If asking for a line number past the last line, return last byte
            self.contents().text_len()
        } else {
            self.line_starts()[line_number + 1]
        }
    }

    /// Returns the [`TextRange`] of the line.
    ///
    /// The start points to the first character's [byte offset](TextSize). The end points
    /// up to, and including, the newline character ending the line (if any). This makes
    /// the range a `[)` range.
    pub fn line_range(&self, line_number: LineNumber) -> TextRange {
        TextRange::new(self.line_start(line_number), self.line_end(line_number))
    }

    /// Returns the [byte offsets](TextSize) for every line
    pub fn line_starts(&self) -> &[TextSize] {
        &self.line_starts
    }

    /// Returns the [SourceLocation] at the [byte offset](TextSize).
    ///
    /// ## Examples
    ///
    /// ```
    /// use biome_text_size::TextSize;
    /// use source_file::{SourceFile, SourceLocation, LineNumber, LineOffset, LineOffsetEncoding};
    ///
    /// let source = "x <- function()\n  NULL".to_string();
    /// let source = SourceFile::new(source);
    ///
    /// assert_eq!(
    ///     source.source_location(TextSize::from(0), LineOffsetEncoding::UTF8),
    ///     SourceLocation::new(
    ///         LineNumber::from(0),
    ///         LineOffset::new(0, LineOffsetEncoding::UTF8)
    ///     )
    /// );
    /// assert_eq!(
    ///     source.source_location(TextSize::from(4), LineOffsetEncoding::UTF8),
    ///     SourceLocation::new(
    ///         LineNumber::from(0),
    ///         LineOffset::new(4, LineOffsetEncoding::UTF8)
    ///     )
    /// );
    /// assert_eq!(
    ///     source.source_location(TextSize::from(20), LineOffsetEncoding::UTF8),
    ///     SourceLocation::new(
    ///         LineNumber::from(1),
    ///         LineOffset::new(4, LineOffsetEncoding::UTF8)
    ///     )
    /// );
    /// ```
    pub fn source_location(
        &self,
        offset: TextSize,
        encoding: LineOffsetEncoding,
    ) -> SourceLocation {
        let line_number = self.line_number(offset);
        let line_range_up_to_offset = TextRange::new(self.line_start(line_number), offset);

        let line_offset = if self.is_ascii() {
            LineOffset::new(line_range_up_to_offset.len().into(), encoding)
        } else {
            match encoding {
                LineOffsetEncoding::UTF8 => {
                    LineOffset::new(line_range_up_to_offset.len().into(), encoding)
                }
                LineOffsetEncoding::UTF16 => {
                    let line_contents_up_to_offset = &self.contents()[line_range_up_to_offset];
                    let offset = line_contents_up_to_offset
                        .encode_utf16()
                        .count()
                        .try_into()
                        .expect("A single line's `offset` should fit in a `u32`");
                    LineOffset::new(offset, encoding)
                }
                LineOffsetEncoding::UTF32 => {
                    let line_contents_up_to_offset = &self.contents()[line_range_up_to_offset];
                    let offset = line_contents_up_to_offset
                        .chars()
                        .count()
                        .try_into()
                        .expect("A single line's `offset` should fit in a `u32`");
                    LineOffset::new(offset, encoding)
                }
            }
        };

        SourceLocation::new(line_number, line_offset)
    }

    /// Returns the [byte offset](TextSize) at the [SourceLocation].
    ///
    /// ## Examples
    ///
    /// ### ASCII
    ///
    /// ```
    /// use source_file::{SourceFile, SourceLocation, LineNumber, LineOffset, LineOffsetEncoding};
    /// use biome_text_size::TextSize;
    ///
    /// let source = r#"a = 4
    /// c = "some string"
    /// x = b"#.to_string();
    ///
    /// let source = SourceFile::new(source);
    ///
    /// // First line, first column
    /// assert_eq!(
    ///     source.offset(SourceLocation::new(
    ///         LineNumber::from(0),
    ///         LineOffset::new(0, LineOffsetEncoding::UTF8)
    ///     )),
    ///     TextSize::from(0)
    /// );
    ///
    /// // Second line, 4th column
    /// assert_eq!(
    ///     source.offset(SourceLocation::new(
    ///         LineNumber::from(1),
    ///         LineOffset::new(4, LineOffsetEncoding::UTF8)
    ///     )),
    ///     TextSize::from(10)
    /// );
    ///
    /// // Offset past the end of the first line
    /// assert_eq!(
    ///     source.offset(SourceLocation::new(
    ///         LineNumber::from(0),
    ///         LineOffset::new(10, LineOffsetEncoding::UTF8)
    ///     )),
    ///     TextSize::from(6)
    /// );
    ///
    /// // Offset past the end of the file
    /// assert_eq!(
    ///     source.offset(SourceLocation::new(
    ///         LineNumber::from(3),
    ///         LineOffset::new(0, LineOffsetEncoding::UTF8)
    ///     )),
    ///     TextSize::from(29)
    /// );
    /// ```
    ///
    /// ### UTF8
    ///
    /// ```
    /// use source_file::{SourceFile, SourceLocation, LineNumber, LineOffset, LineOffsetEncoding};
    /// use biome_text_size::TextSize;
    ///
    /// let source = r#"a = 4
    /// c = "❤️"
    /// x = b"#.to_string();
    ///
    /// let source = SourceFile::new(source);
    ///
    /// // First line, first column
    /// assert_eq!(
    ///     source.offset(SourceLocation::new(
    ///         LineNumber::from(0),
    ///         LineOffset::new(0, LineOffsetEncoding::UTF8)
    ///     )),
    ///     TextSize::from(0)
    /// );
    ///
    /// // Third line, 2nd column, after emoji, UTF8
    /// assert_eq!(
    ///     source.offset(SourceLocation::new(
    ///         LineNumber::from(2),
    ///         LineOffset::new(1, LineOffsetEncoding::UTF8)
    ///     )),
    ///     TextSize::from(20)
    /// );
    ///
    /// // Third line, 2nd column, after emoji, UTF16
    /// assert_eq!(
    ///     source.offset(SourceLocation::new(
    ///         LineNumber::from(2),
    ///         LineOffset::new(1, LineOffsetEncoding::UTF16)
    ///     )),
    ///     TextSize::from(20)
    /// );
    ///
    /// // Offset past the end of the second line, UTF8
    /// assert_eq!(
    ///     source.offset(SourceLocation::new(
    ///         LineNumber::from(1),
    ///         LineOffset::new(10, LineOffsetEncoding::UTF8)
    ///     )),
    ///     TextSize::from(16)
    /// );
    ///
    /// // Offset past the end of the second line, UTF32
    /// assert_eq!(
    ///     source.offset(SourceLocation::new(
    ///         LineNumber::from(1),
    ///         LineOffset::new(10, LineOffsetEncoding::UTF32)
    ///     )),
    ///     TextSize::from(19)
    /// );
    ///
    /// // Offset past the end of the file
    /// assert_eq!(
    ///     source.offset(SourceLocation::new(
    ///         LineNumber::from(3),
    ///         LineOffset::new(0, LineOffsetEncoding::UTF32)
    ///     )),
    ///     TextSize::from(24)
    /// );
    /// ```
    ///
    pub fn offset(&self, source_location: SourceLocation) -> TextSize {
        let (line_number, line_offset) = source_location.into_fields();

        let line_range = self.line_range(line_number);

        let offset = if self.is_ascii() {
            TextSize::from(line_offset.raw())
        } else {
            match line_offset.encoding() {
                LineOffsetEncoding::UTF8 => TextSize::from(line_offset.raw()),
                LineOffsetEncoding::UTF16 => {
                    let n_code_units = line_offset.raw();
                    let line_contents = &self.contents()[line_range];

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
                    let line_contents = &self.contents()[line_range];

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
}

#[cfg(test)]
mod tests {
    use biome_text_size::TextSize;

    use crate::source_location::LineNumber;
    use crate::source_location::LineOffset;
    use crate::source_location::LineOffsetEncoding;
    use crate::SourceFile;
    use crate::SourceLocation;

    #[test]
    fn ascii_source_file() {
        let source = SourceFile::new(String::new());
        assert_eq!(source.line_starts(), &[TextSize::from(0)]);

        let source = SourceFile::new("x = 1".to_string());
        assert_eq!(source.line_starts(), &[TextSize::from(0)]);

        let source = SourceFile::new("x = 1\n".to_string());
        assert_eq!(
            source.line_starts(),
            &[TextSize::from(0), TextSize::from(6)]
        );

        let source = SourceFile::new("x = 1\ny = 2\nz = x + y\n".to_string());
        assert_eq!(
            source.line_starts(),
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
        let contents = "x = 1\ny = 2".to_string();
        let source = SourceFile::new(contents);

        // First row.
        let loc = source.source_location(TextSize::from(2), LineOffsetEncoding::UTF8);
        assert_eq!(
            loc,
            SourceLocation::new(
                LineNumber::from(0),
                LineOffset::new(2, LineOffsetEncoding::UTF8)
            )
        );

        // Second row.
        let loc = source.source_location(TextSize::from(6), LineOffsetEncoding::UTF8);
        assert_eq!(
            loc,
            SourceLocation::new(
                LineNumber::from(1),
                LineOffset::new(0, LineOffsetEncoding::UTF8)
            )
        );

        let loc = source.source_location(TextSize::from(11), LineOffsetEncoding::UTF8);
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
        let contents = "x = 4\ry = 3".to_string();
        let source = SourceFile::new(contents);
        assert_eq!(
            source.line_starts(),
            &[TextSize::from(0), TextSize::from(6)]
        );

        assert_eq!(
            source.source_location(TextSize::from(4), LineOffsetEncoding::UTF8),
            SourceLocation::new(
                LineNumber::from(0),
                LineOffset::new(4, LineOffsetEncoding::UTF8)
            )
        );
        assert_eq!(
            source.source_location(TextSize::from(6), LineOffsetEncoding::UTF8),
            SourceLocation::new(
                LineNumber::from(1),
                LineOffset::new(0, LineOffsetEncoding::UTF8)
            )
        );
        assert_eq!(
            source.source_location(TextSize::from(7), LineOffsetEncoding::UTF8),
            SourceLocation::new(
                LineNumber::from(1),
                LineOffset::new(1, LineOffsetEncoding::UTF8)
            )
        );
    }

    #[test]
    fn ascii_carriage_return_newline() {
        let contents = "x = 4\r\ny = 3".to_string();
        let source = SourceFile::new(contents);
        assert_eq!(
            source.line_starts(),
            &[TextSize::from(0), TextSize::from(7)]
        );

        assert_eq!(
            source.source_location(TextSize::from(4), LineOffsetEncoding::UTF8),
            SourceLocation::new(
                LineNumber::from(0),
                LineOffset::new(4, LineOffsetEncoding::UTF8)
            )
        );
        assert_eq!(
            source.source_location(TextSize::from(7), LineOffsetEncoding::UTF8),
            SourceLocation::new(
                LineNumber::from(1),
                LineOffset::new(0, LineOffsetEncoding::UTF8)
            )
        );
        assert_eq!(
            source.source_location(TextSize::from(8), LineOffsetEncoding::UTF8),
            SourceLocation::new(
                LineNumber::from(1),
                LineOffset::new(1, LineOffsetEncoding::UTF8)
            )
        );
    }

    #[test]
    fn utf8_source_file() {
        let source = SourceFile::new("x = '🫣'".to_string());
        assert_eq!(source.line_count(), 1);
        assert_eq!(source.line_starts(), &[TextSize::from(0)]);

        let source = SourceFile::new("x = '🫣'\n".to_string());
        assert_eq!(source.line_count(), 2);
        assert_eq!(
            source.line_starts(),
            &[TextSize::from(0), TextSize::from(11)]
        );

        let source = SourceFile::new("x = '🫣'\ny = 2\nz = x + y\n".to_string());
        assert_eq!(source.line_count(), 4);
        assert_eq!(
            source.line_starts(),
            &[
                TextSize::from(0),
                TextSize::from(11),
                TextSize::from(17),
                TextSize::from(27)
            ]
        );

        let source = SourceFile::new("# 🫣\nclass Foo:\n    \"\"\".\"\"\"".to_string());
        assert_eq!(source.line_count(), 3);
        assert_eq!(
            source.line_starts(),
            &[TextSize::from(0), TextSize::from(7), TextSize::from(18)]
        );
    }

    #[test]
    fn utf8_carriage_return() {
        let contents = "x = '🫣'\ry = 3".to_string();
        let source = SourceFile::new(contents);
        assert_eq!(source.line_count(), 2);
        assert_eq!(
            source.line_starts(),
            &[TextSize::from(0), TextSize::from(11)]
        );

        // Second ', UTF8
        assert_eq!(
            source.source_location(TextSize::from(9), LineOffsetEncoding::UTF8),
            SourceLocation::new(
                LineNumber::from(0),
                LineOffset::new(9, LineOffsetEncoding::UTF8)
            )
        );
        assert_eq!(
            source.source_location(TextSize::from(11), LineOffsetEncoding::UTF8),
            SourceLocation::new(
                LineNumber::from(1),
                LineOffset::new(0, LineOffsetEncoding::UTF8)
            )
        );
        assert_eq!(
            source.source_location(TextSize::from(12), LineOffsetEncoding::UTF8),
            SourceLocation::new(
                LineNumber::from(1),
                LineOffset::new(1, LineOffsetEncoding::UTF8)
            )
        );

        // Second ', UTF16
        assert_eq!(
            source.source_location(TextSize::from(9), LineOffsetEncoding::UTF16),
            SourceLocation::new(
                LineNumber::from(0),
                LineOffset::new(7, LineOffsetEncoding::UTF16)
            )
        );
        assert_eq!(
            source.source_location(TextSize::from(11), LineOffsetEncoding::UTF16),
            SourceLocation::new(
                LineNumber::from(1),
                LineOffset::new(0, LineOffsetEncoding::UTF16)
            )
        );
        assert_eq!(
            source.source_location(TextSize::from(12), LineOffsetEncoding::UTF16),
            SourceLocation::new(
                LineNumber::from(1),
                LineOffset::new(1, LineOffsetEncoding::UTF16)
            )
        );

        // Second ', UTF32
        assert_eq!(
            source.source_location(TextSize::from(9), LineOffsetEncoding::UTF32),
            SourceLocation::new(
                LineNumber::from(0),
                LineOffset::new(6, LineOffsetEncoding::UTF32)
            )
        );
        assert_eq!(
            source.source_location(TextSize::from(11), LineOffsetEncoding::UTF32),
            SourceLocation::new(
                LineNumber::from(1),
                LineOffset::new(0, LineOffsetEncoding::UTF32)
            )
        );
        assert_eq!(
            source.source_location(TextSize::from(12), LineOffsetEncoding::UTF32),
            SourceLocation::new(
                LineNumber::from(1),
                LineOffset::new(1, LineOffsetEncoding::UTF32)
            )
        );
    }

    #[test]
    fn utf8_carriage_return_newline() {
        let contents = "x = '🫣'\r\ny = 3".to_string();
        let source = SourceFile::new(contents);
        assert_eq!(source.line_count(), 2);
        assert_eq!(
            source.line_starts(),
            &[TextSize::from(0), TextSize::from(12)]
        );

        // Second '
        assert_eq!(
            source.source_location(TextSize::from(9), LineOffsetEncoding::UTF32),
            SourceLocation::new(
                LineNumber::from(0),
                LineOffset::new(6, LineOffsetEncoding::UTF32)
            )
        );
        assert_eq!(
            source.source_location(TextSize::from(12), LineOffsetEncoding::UTF32),
            SourceLocation::new(
                LineNumber::from(1),
                LineOffset::new(0, LineOffsetEncoding::UTF32)
            )
        );
        assert_eq!(
            source.source_location(TextSize::from(13), LineOffsetEncoding::UTF32),
            SourceLocation::new(
                LineNumber::from(1),
                LineOffset::new(1, LineOffsetEncoding::UTF32)
            )
        );
    }

    #[test]
    fn utf8_byte_offset() {
        let contents = "x = '☃'\ny = 2".to_string();
        let source = SourceFile::new(contents);
        assert_eq!(
            source.line_starts(),
            &[TextSize::from(0), TextSize::from(10)]
        );

        // First row, start
        let loc = source.source_location(TextSize::from(0), LineOffsetEncoding::UTF8);
        assert_eq!(
            loc,
            SourceLocation::new(
                LineNumber::from(0),
                LineOffset::new(0, LineOffsetEncoding::UTF8)
            )
        );
        let loc = source.source_location(TextSize::from(0), LineOffsetEncoding::UTF16);
        assert_eq!(
            loc,
            SourceLocation::new(
                LineNumber::from(0),
                LineOffset::new(0, LineOffsetEncoding::UTF16)
            )
        );
        let loc = source.source_location(TextSize::from(0), LineOffsetEncoding::UTF32);
        assert_eq!(
            loc,
            SourceLocation::new(
                LineNumber::from(0),
                LineOffset::new(0, LineOffsetEncoding::UTF32)
            )
        );

        // First row, right before
        let loc = source.source_location(TextSize::from(5), LineOffsetEncoding::UTF8);
        assert_eq!(
            loc,
            SourceLocation::new(
                LineNumber::from(0),
                LineOffset::new(5, LineOffsetEncoding::UTF8)
            )
        );
        let loc = source.source_location(TextSize::from(5), LineOffsetEncoding::UTF16);
        assert_eq!(
            loc,
            SourceLocation::new(
                LineNumber::from(0),
                LineOffset::new(5, LineOffsetEncoding::UTF16)
            )
        );
        let loc = source.source_location(TextSize::from(5), LineOffsetEncoding::UTF32);
        assert_eq!(
            loc,
            SourceLocation::new(
                LineNumber::from(0),
                LineOffset::new(5, LineOffsetEncoding::UTF32)
            )
        );

        // First row, right after
        let loc = source.source_location(TextSize::from(8), LineOffsetEncoding::UTF8);
        assert_eq!(
            loc,
            SourceLocation::new(
                LineNumber::from(0),
                LineOffset::new(8, LineOffsetEncoding::UTF8)
            )
        );
        let loc = source.source_location(TextSize::from(8), LineOffsetEncoding::UTF16);
        assert_eq!(
            loc,
            SourceLocation::new(
                LineNumber::from(0),
                LineOffset::new(6, LineOffsetEncoding::UTF16)
            )
        );
        let loc = source.source_location(TextSize::from(8), LineOffsetEncoding::UTF32);
        assert_eq!(
            loc,
            SourceLocation::new(
                LineNumber::from(0),
                LineOffset::new(6, LineOffsetEncoding::UTF32)
            )
        );

        // Second row, start
        let loc = source.source_location(TextSize::from(10), LineOffsetEncoding::UTF8);
        assert_eq!(
            loc,
            SourceLocation::new(
                LineNumber::from(1),
                LineOffset::new(0, LineOffsetEncoding::UTF8)
            )
        );
        let loc = source.source_location(TextSize::from(10), LineOffsetEncoding::UTF16);
        assert_eq!(
            loc,
            SourceLocation::new(
                LineNumber::from(1),
                LineOffset::new(0, LineOffsetEncoding::UTF16)
            )
        );
        let loc = source.source_location(TextSize::from(10), LineOffsetEncoding::UTF32);
        assert_eq!(
            loc,
            SourceLocation::new(
                LineNumber::from(1),
                LineOffset::new(0, LineOffsetEncoding::UTF32)
            )
        );

        // One-past-the-end.
        let loc = source.source_location(TextSize::from(15), LineOffsetEncoding::UTF8);
        assert_eq!(
            loc,
            SourceLocation::new(
                LineNumber::from(1),
                LineOffset::new(5, LineOffsetEncoding::UTF8)
            )
        );
        let loc = source.source_location(TextSize::from(15), LineOffsetEncoding::UTF16);
        assert_eq!(
            loc,
            SourceLocation::new(
                LineNumber::from(1),
                LineOffset::new(5, LineOffsetEncoding::UTF16)
            )
        );
        let loc = source.source_location(TextSize::from(15), LineOffsetEncoding::UTF32);
        assert_eq!(
            loc,
            SourceLocation::new(
                LineNumber::from(1),
                LineOffset::new(5, LineOffsetEncoding::UTF32)
            )
        );
    }
}
