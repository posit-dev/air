// +------------------------------------------------------------+
// | Code adopted from:                                         |
// | Repository: https://github.com/astral-sh/ruff.git          |
// | Commit: 5bc9d6d3aa694ab13f38dd5cf91b713fd3844380           |
// +------------------------------------------------------------+

use crate::edit::PositionEncoding;
use crate::proto::TextSizeExt;
use biome_text_size::{TextRange, TextSize};
use lsp_types as types;
use source_file::LineIndex;

// We don't own this type so we need a helper trait
pub(crate) trait TextRangeExt {
    fn into_proto(self, text: &str, index: &LineIndex, encoding: PositionEncoding) -> types::Range;

    fn from_proto(
        range: types::Range,
        text: &str,
        index: &LineIndex,
        encoding: PositionEncoding,
    ) -> Self;
}

impl TextRangeExt for TextRange {
    fn into_proto(self, text: &str, index: &LineIndex, encoding: PositionEncoding) -> types::Range {
        types::Range {
            start: self.start().into_proto(text, index, encoding),
            end: self.end().into_proto(text, index, encoding),
        }
    }

    fn from_proto(
        range: types::Range,
        text: &str,
        index: &LineIndex,
        encoding: PositionEncoding,
    ) -> Self {
        TextRange::new(
            TextSize::from_proto(range.start, text, index, encoding),
            TextSize::from_proto(range.end, text, index, encoding),
        )
    }
}
