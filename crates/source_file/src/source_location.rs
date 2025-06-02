use std::fmt::Debug;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SourceLocation {
    line_number: LineNumber,
    line_offset: LineOffset,
}

impl SourceLocation {
    pub fn new(line_number: LineNumber, line_offset: LineOffset) -> Self {
        Self {
            line_number,
            line_offset,
        }
    }

    pub fn line_number(&self) -> LineNumber {
        self.line_number
    }

    pub fn line_offset(&self) -> LineOffset {
        self.line_offset
    }

    pub fn into_fields(self) -> (LineNumber, LineOffset) {
        (self.line_number, self.line_offset)
    }
}

/// A 0-indexed line number
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct LineNumber(u32);

impl From<u32> for LineNumber {
    fn from(value: u32) -> Self {
        LineNumber(value)
    }
}

impl TryFrom<usize> for LineNumber {
    type Error = std::num::TryFromIntError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        Ok(LineNumber(u32::try_from(value)?))
    }
}

impl From<LineNumber> for u32 {
    fn from(value: LineNumber) -> Self {
        value.0
    }
}

impl From<LineNumber> for usize {
    fn from(value: LineNumber) -> Self {
        value.0 as usize
    }
}

/// A 0-indexed offset into a line, represented as a number of code units under one of the
/// three possible [LineOffsetEncoding]s
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct LineOffset {
    raw: u32,
    encoding: LineOffsetEncoding,
}

impl LineOffset {
    pub fn new(raw: u32, encoding: LineOffsetEncoding) -> Self {
        Self { raw, encoding }
    }

    pub fn raw(&self) -> u32 {
        self.raw
    }

    pub fn encoding(&self) -> LineOffsetEncoding {
        self.encoding
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum LineOffsetEncoding {
    /// Preferred encoding, as Rust [String]s are UTF-8
    UTF8,

    /// UTF-16 is the encoding supported by all LSP clients, but is most expensive to translate
    UTF16,

    /// Second choice because UTF-32 uses a fixed 4 byte encoding for each character (makes conversion relatively easy)
    UTF32,
}
