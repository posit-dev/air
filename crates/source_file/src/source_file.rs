use biome_text_size::TextRange;
use biome_text_size::TextSize;

use crate::line_index::LineIndex;
use crate::OneIndexed;
use crate::SourceLocation;

/// Manager of a single source file
///
/// Builds a [LineIndex] on creation, and associates that index with the source it
/// was created from for future method calls.
#[derive(Debug, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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

    /// Returns the row and column index for an offset.
    ///
    /// ## Examples
    ///
    /// ```
    /// # use biome_text_size::TextSize;
    /// # use source_file::{SourceFile, OneIndexed, SourceLocation};
    /// let source = "def a():\n    pass".to_string();
    /// let source = SourceFile::new(source);
    ///
    /// assert_eq!(
    ///     source.source_location(TextSize::from(0)),
    ///     SourceLocation { row: OneIndexed::from_zero_indexed(0), column: OneIndexed::from_zero_indexed(0) }
    /// );
    ///
    /// assert_eq!(
    ///     source.source_location(TextSize::from(4)),
    ///     SourceLocation { row: OneIndexed::from_zero_indexed(0), column: OneIndexed::from_zero_indexed(4) }
    /// );
    /// assert_eq!(
    ///     source.source_location(TextSize::from(13)),
    ///     SourceLocation { row: OneIndexed::from_zero_indexed(1), column: OneIndexed::from_zero_indexed(4) }
    /// );
    /// ```
    ///
    /// ## Panics
    ///
    /// If the offset is out of bounds.
    pub fn source_location(&self, offset: TextSize) -> SourceLocation {
        self.index.source_location(offset, self.contents())
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
    /// # use biome_text_size::TextSize;
    /// # use source_file::{SourceFile, OneIndexed, SourceLocation};
    /// let source = "def a():\n    pass".to_string();
    /// let source = SourceFile::new(source);
    ///
    /// assert_eq!(source.line_index(TextSize::from(0)), OneIndexed::from_zero_indexed(0));
    /// assert_eq!(source.line_index(TextSize::from(4)), OneIndexed::from_zero_indexed(0));
    /// assert_eq!(source.line_index(TextSize::from(13)), OneIndexed::from_zero_indexed(1));
    /// ```
    ///
    /// ## Panics
    ///
    /// If the offset is out of bounds.
    pub fn line_index(&self, offset: TextSize) -> OneIndexed {
        self.index.line_index(offset)
    }

    /// Returns the [byte offset](TextSize) for the `line` with the given index.
    pub fn line_start(&self, line: OneIndexed) -> TextSize {
        self.index.line_start(line, self.contents())
    }

    /// Returns the [byte offset](TextSize) of the `line`'s end.
    /// The offset is the end of the line, up to and including the newline character ending the line (if any).
    pub fn line_end(&self, line: OneIndexed) -> TextSize {
        self.index.line_end(line, self.contents())
    }

    /// Returns the [byte offset](TextSize) of the `line`'s end.
    /// The offset is the end of the line, excluding the newline character ending the line (if any).
    pub fn line_end_exclusive(&self, line: OneIndexed) -> TextSize {
        self.index.line_end_exclusive(line, self.contents())
    }

    /// Returns the [`TextRange`] of the `line` with the given index.
    /// The start points to the first character's [byte offset](TextSize), the end up to, and including
    /// the newline character ending the line (if any).
    pub fn line_range(&self, line: OneIndexed) -> TextRange {
        self.index.line_range(line, self.contents())
    }

    /// Returns the [byte offset](TextSize) at `line` and `column`.
    ///
    /// ## Examples
    ///
    /// ### ASCII
    ///
    /// ```
    /// use source_file::{SourceFile, OneIndexed};
    /// use biome_text_size::TextSize;
    /// let source = r#"a = 4
    /// c = "some string"
    /// x = b"#.to_string();
    ///
    /// let source = SourceFile::new(source);
    ///
    /// // First line, first column
    /// assert_eq!(source.offset(OneIndexed::from_zero_indexed(0), OneIndexed::from_zero_indexed(0)), TextSize::from(0));
    ///
    /// // Second line, 4th column
    /// assert_eq!(source.offset(OneIndexed::from_zero_indexed(1), OneIndexed::from_zero_indexed(4)), TextSize::from(10));
    ///
    /// // Offset past the end of the first line
    /// assert_eq!(source.offset(OneIndexed::from_zero_indexed(0), OneIndexed::from_zero_indexed(10)), TextSize::from(6));
    ///
    /// // Offset past the end of the file
    /// assert_eq!(source.offset(OneIndexed::from_zero_indexed(3), OneIndexed::from_zero_indexed(0)), TextSize::from(29));
    /// ```
    ///
    /// ### UTF8
    ///
    /// ```
    /// use source_file::{SourceFile, OneIndexed};
    /// use biome_text_size::TextSize;
    /// let source = r#"a = 4
    /// c = "❤️"
    /// x = b"#.to_string();
    ///
    /// let source = SourceFile::new(source);
    ///
    /// // First line, first column
    /// assert_eq!(source.offset(OneIndexed::from_zero_indexed(0), OneIndexed::from_zero_indexed(0)), TextSize::from(0));
    ///
    /// // Third line, 2nd column, after emoji
    /// assert_eq!(source.offset(OneIndexed::from_zero_indexed(2), OneIndexed::from_zero_indexed(1)), TextSize::from(20));
    ///
    /// // Offset past the end of the second line
    /// assert_eq!(source.offset(OneIndexed::from_zero_indexed(1), OneIndexed::from_zero_indexed(10)), TextSize::from(19));
    ///
    /// // Offset past the end of the file
    /// assert_eq!(source.offset(OneIndexed::from_zero_indexed(3), OneIndexed::from_zero_indexed(0)), TextSize::from(24));
    /// ```
    ///
    pub fn offset(&self, line: OneIndexed, column: OneIndexed) -> TextSize {
        self.index.offset(line, column, self.contents())
    }

    /// Returns the [byte offsets](TextSize) for every line
    pub fn line_starts(&self) -> &[TextSize] {
        self.index.line_starts()
    }
}
