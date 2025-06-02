use biome_text_size::TextRange;
use biome_text_size::TextSize;

use crate::line_index::LineIndex;
use crate::source_location::LineNumber;
use crate::source_location::LineOffsetEncoding;
use crate::SourceLocation;

/// Manager of a single source file
///
/// Builds a [LineIndex] on creation, and associates that index with the source it
/// was created from for future method calls.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SourceFile {
    contents: String,
    index: LineIndex,
}

impl SourceFile {
    /// Builds the [`SourceFile`] from the contents of a file.
    pub fn new(contents: String) -> Self {
        let index = LineIndex::from_source_text(&contents);
        Self { contents, index }
    }

    /// Returns a reference to the contents in the source file.
    pub fn contents(&self) -> &str {
        &self.contents
    }

    /// Consumes the source file, returning only the contents.
    pub fn into_contents(self) -> String {
        self.contents
    }

    /// Replace text in the source file and rebuild the line index afterwards.
    pub fn replace_range<R>(&mut self, range: R, replace_with: &str)
    where
        R: std::ops::RangeBounds<usize>,
    {
        self.contents.replace_range(range, replace_with);
        self.index = LineIndex::from_source_text(&self.contents);
    }

    /// Returns the [SourceLocation] for an offset.
    ///
    /// ## Examples
    ///
    /// ```
    /// use biome_text_size::TextSize;
    /// use source_file::{SourceFile, SourceLocation, LineNumber, LineOffset, Encoding};
    ///
    /// let source = "x <- function()\n  NULL".to_string();
    /// let source = SourceFile::new(source);
    ///
    /// assert_eq!(
    ///     source.source_location(TextSize::from(0), Encoding::UTF8),
    ///     SourceLocation::new(
    ///         LineNumber::from(0),
    ///         LineOffset::new(0, Encoding::UTF8)
    ///     )
    /// );
    /// assert_eq!(
    ///     source.source_location(TextSize::from(4), Encoding::UTF8),
    ///     SourceLocation::new(
    ///         LineNumber::from(0),
    ///         LineOffset::new(4, Encoding::UTF8)
    ///     )
    /// );
    /// assert_eq!(
    ///     source.source_location(TextSize::from(20), Encoding::UTF8),
    ///     SourceLocation::new(
    ///         LineNumber::from(1),
    ///         LineOffset::new(4, Encoding::UTF8)
    ///     )
    /// );
    /// ```
    pub fn source_location(
        &self,
        offset: TextSize,
        encoding: LineOffsetEncoding,
    ) -> SourceLocation {
        self.index
            .source_location(offset, self.contents(), encoding)
    }

    /// Return the number of lines in the source code.
    pub fn line_count(&self) -> usize {
        self.index.line_count()
    }

    /// Returns `true` if the text only consists of ASCII characters
    pub fn is_ascii(&self) -> bool {
        self.index.is_ascii()
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
        self.index.line_number(offset)
    }

    /// Returns the [byte offset](TextSize) for the `line_number`'s start.
    pub fn line_start(&self, line_number: LineNumber) -> TextSize {
        self.index.line_start(line_number, self.contents())
    }

    /// Returns the [byte offset](TextSize) of the `line_number`'s end.
    /// The offset is the end of the line, up to and including the newline character ending the line (if any).
    pub fn line_end(&self, line_number: LineNumber) -> TextSize {
        self.index.line_end(line_number, self.contents())
    }

    /// Returns the [`TextRange`] of the `line_number`.
    /// The start points to the first character's [byte offset](TextSize), the end up to, and including
    /// the newline character ending the line (if any).
    pub fn line_range(&self, line_number: LineNumber) -> TextRange {
        self.index.line_range(line_number, self.contents())
    }

    /// Returns the [byte offset](TextSize) at the [SourceLocation].
    ///
    /// ## Examples
    ///
    /// ### ASCII
    ///
    /// ```
    /// use source_file::{SourceFile, SourceLocation, LineNumber, LineOffset, Encoding};
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
    ///         LineOffset::new(0, Encoding::UTF8)
    ///     )),
    ///     TextSize::from(0)
    /// );
    ///
    /// // Second line, 4th column
    /// assert_eq!(
    ///     source.offset(SourceLocation::new(
    ///         LineNumber::from(1),
    ///         LineOffset::new(4, Encoding::UTF8)
    ///     )),
    ///     TextSize::from(10)
    /// );
    ///
    /// // Offset past the end of the first line
    /// assert_eq!(
    ///     source.offset(SourceLocation::new(
    ///         LineNumber::from(0),
    ///         LineOffset::new(10, Encoding::UTF8)
    ///     )),
    ///     TextSize::from(6)
    /// );
    ///
    /// // Offset past the end of the file
    /// assert_eq!(
    ///     source.offset(SourceLocation::new(
    ///         LineNumber::from(3),
    ///         LineOffset::new(0, Encoding::UTF8)
    ///     )),
    ///     TextSize::from(29)
    /// );
    /// ```
    ///
    /// ### UTF8
    ///
    /// ```
    /// use source_file::{SourceFile, SourceLocation, LineNumber, LineOffset, Encoding};
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
    ///         LineOffset::new(0, Encoding::UTF8)
    ///     )),
    ///     TextSize::from(0)
    /// );
    ///
    /// // Third line, 2nd column, after emoji, UTF8
    /// assert_eq!(
    ///     source.offset(SourceLocation::new(
    ///         LineNumber::from(2),
    ///         LineOffset::new(1, Encoding::UTF8)
    ///     )),
    ///     TextSize::from(20)
    /// );
    ///
    /// // Third line, 2nd column, after emoji, UTF16
    /// assert_eq!(
    ///     source.offset(SourceLocation::new(
    ///         LineNumber::from(2),
    ///         LineOffset::new(1, Encoding::UTF16)
    ///     )),
    ///     TextSize::from(20)
    /// );
    ///
    /// // Offset past the end of the second line, UTF8
    /// assert_eq!(
    ///     source.offset(SourceLocation::new(
    ///         LineNumber::from(1),
    ///         LineOffset::new(10, Encoding::UTF8)
    ///     )),
    ///     TextSize::from(16)
    /// );
    ///
    /// // Offset past the end of the second line, UTF32
    /// assert_eq!(
    ///     source.offset(SourceLocation::new(
    ///         LineNumber::from(1),
    ///         LineOffset::new(10, Encoding::UTF32)
    ///     )),
    ///     TextSize::from(19)
    /// );
    ///
    /// // Offset past the end of the file
    /// assert_eq!(
    ///     source.offset(SourceLocation::new(
    ///         LineNumber::from(3),
    ///         LineOffset::new(0, Encoding::UTF32)
    ///     )),
    ///     TextSize::from(24)
    /// );
    /// ```
    ///
    pub fn offset(&self, source_location: SourceLocation) -> TextSize {
        self.index.offset(source_location, self.contents())
    }

    /// Returns the [byte offsets](TextSize) for every line
    pub fn line_starts(&self) -> &[TextSize] {
        self.index.line_starts()
    }
}
