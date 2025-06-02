use crate::document::PositionEncoding;
use biome_text_size::TextSize;
use lsp_types as types;
use source_file::LineNumber;
use source_file::LineOffset;
use source_file::LineOffsetEncoding;
use source_file::{SourceFile, SourceLocation};

// We don't own this type so we need a helper trait
pub(crate) trait TextSizeExt {
    fn into_proto(self, source: &SourceFile, encoding: PositionEncoding) -> types::Position;

    fn from_proto(
        position: types::Position,
        source: &SourceFile,
        encoding: PositionEncoding,
    ) -> Self;
}

impl TextSizeExt for TextSize {
    fn into_proto(self, source: &SourceFile, encoding: PositionEncoding) -> types::Position {
        let source_location = source.source_location(self, remap_encoding(encoding));
        types::Position {
            line: source_location.line_number().into(),
            character: source_location.line_offset().raw(),
        }
    }

    fn from_proto(
        position: types::Position,
        source: &SourceFile,
        encoding: PositionEncoding,
    ) -> Self {
        let source_location = SourceLocation::new(
            LineNumber::from(position.line),
            LineOffset::new(position.character, remap_encoding(encoding)),
        );
        source.offset(source_location)
    }
}

/// Here's how to think about these conversions:
///
/// [lsp_types::Position] contains a location encoded as `row` and `character`,
/// where:
/// - `row` represents the 0-indexed line number
/// - `character` represents the 0-indexed column offset, with precise meaning decided
///   by [lsp_types::PositionEncodingKind]
///
/// `character` is interpreted as:
/// - With [lsp_types::PositionEncodingKind::UTF8], the number of UTF-8 code units.
/// - With [lsp_types::PositionEncodingKind::UTF16], the number of UTF-16 code units.
/// - With [lsp_types::PositionEncodingKind::UTF32], the number of UTF-32 code units.
///
/// Now, for some definitions:
///
/// - Code unit: The minimal bit combination that can represent a single character.
///   - UTF-8:
///     - 1 code unit = 1 byte = 8 bits
///   - UTF-16:
///     - 1 code unit = 2 bytes = 16 bits
///   - UTF-32:
///     - 1 code unit = 4 bytes = 32 bits
///
/// - Character: A combination of code units that construct a single UTF element.
///   - UTF-8:
///     - 1 character = 1,2,3,4 code units = 1,2,3,4 bytes = 8,16,24,32 bits
///   - UTF-16:
///     - 1 character = 1,2 code units = 2,4 bytes = 16,32 bits
///   - UTF-16:
///     - 1 character = 1 code units = 4 bytes = 32 bits
///
/// - Unicode Scalar Value: Any Unicode Code Point other than a Surrogate Code Point (
///   which are only used by UTF-16). Technically this means any value in the range of
///   [0 to 0x10FFFF] excluding the slice of [0xD800 to 0xDFFF].
///
/// - Unicode Code Point: Any value in the Unicode code space of [0 to 0x10FFFF]. This
///   means that something representing an arbitrary code point must be 4 bytes, implying
///   that something representing a Unicode Scalar Value must also be 4 bytes.
///
/// In Rust, [String] and [str] are in UTF-8. Figuring out how to go from the, say,
/// 8th column `Position.character` of a line to the byte offset on that line requires
/// knowing both the UTF-8 content of that line and the `PositionEncodingKind` that
/// `Position.character` is encoded in.
///
/// Note that `chars()` returns an iterator over the individual `char` contained within a
/// string. And each `char` is a Unicode Scalar Value. This means that each `char` is
/// internally represented as a `u32` of exactly 4 bytes. It also means that you can
/// think of iterating over `chars()` as equivalent to iterating over UTF-32 Characters
/// or UTF-32 Code Points.
///
/// Also relevant is that [char::len_utf16] returns the number of UTF-16 code units that
/// would be required to represent the `char`, and [char::len_utf8] returns the number
/// of UTF-8 code units (and therefore bytes) that would be required to represent the
/// `char`.
///
/// # Converting `character` UTF-8/16/32 code points -> UTF-8 String byte offset
///
/// An arbitrary algorithm to find the number of UTF-8 bytes required to represent `character` column offset would be:
/// - Iterate over `chars()`
/// - Figure out how many `char`s are required TODO??
///
/// - With [lsp_types::PositionEncodingKind::UTF8]:
///   - `character` is the number of UTF-8 code units
///   - 1 UTF-8 code unit is just 1 UTF-8 byte, so just return `character`
/// - With [lsp_types::PositionEncodingKind::UTF16]:
///   - `character` is the number of UTF-16 code units
///   - 1 UTF-16 code unit must
/// - With [lsp_types::PositionEncodingKind::UTF32]:
///
fn remap_encoding(encoding: PositionEncoding) -> LineOffsetEncoding {
    match encoding {
        PositionEncoding::UTF16 => LineOffsetEncoding::UTF16,
        PositionEncoding::UTF32 => LineOffsetEncoding::UTF32,
        PositionEncoding::UTF8 => LineOffsetEncoding::UTF8,
    }
}
