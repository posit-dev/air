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

fn remap_encoding(encoding: PositionEncoding) -> LineOffsetEncoding {
    match encoding {
        PositionEncoding::UTF16 => LineOffsetEncoding::UTF16,
        PositionEncoding::UTF32 => LineOffsetEncoding::UTF32,
        PositionEncoding::UTF8 => LineOffsetEncoding::UTF8,
    }
}
