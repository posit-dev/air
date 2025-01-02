use crate::edit::PositionEncoding;
use crate::proto::TextSizeExt;
use biome_text_size::{TextRange, TextSize};
use lsp_types as types;
use source_file::SourceFile;

// We don't own this type so we need a helper trait
pub(crate) trait TextRangeExt {
    fn into_proto(self, source: &SourceFile, encoding: PositionEncoding) -> types::Range;

    fn from_proto(range: types::Range, source: &SourceFile, encoding: PositionEncoding) -> Self;
}

impl TextRangeExt for TextRange {
    fn into_proto(self, source: &SourceFile, encoding: PositionEncoding) -> types::Range {
        types::Range {
            start: self.start().into_proto(source, encoding),
            end: self.end().into_proto(source, encoding),
        }
    }

    fn from_proto(range: types::Range, source: &SourceFile, encoding: PositionEncoding) -> Self {
        TextRange::new(
            TextSize::from_proto(range.start, source, encoding),
            TextSize::from_proto(range.end, source, encoding),
        )
    }
}
