use anyhow::Context;
use biome_line_index::LineCol;
use biome_line_index::LineIndex;
use biome_line_index::WideLineCol;
use tower_lsp::lsp_types;

use crate::proto::PositionEncoding;

/// The function is used to convert a LSP position to LineCol.
pub(crate) fn line_col_from_position(
    position: lsp_types::Position,
    line_index: &LineIndex,
    position_encoding: PositionEncoding,
) -> LineCol {
    match position_encoding {
        PositionEncoding::Utf8 => LineCol {
            line: position.line,
            col: position.character,
        },
        PositionEncoding::Wide(enc) => {
            let line_col = WideLineCol {
                line: position.line,
                col: position.character,
            };
            line_index.to_utf8(enc, line_col)
        }
    }
}

/// The function is used to convert a LSP position to TextSize.
/// From `biome_lsp_converters::from_proto::offset()`.
pub(crate) fn offset_from_position(
    position: lsp_types::Position,
    line_index: &LineIndex,
    position_encoding: PositionEncoding,
) -> anyhow::Result<biome_text_size::TextSize> {
    let line_col = line_col_from_position(position, line_index, position_encoding);

    line_index
        .offset(line_col)
        .with_context(|| format!("Position {position:?} is out of range"))
}

/// The function is used to convert a LSP range to TextRange.
/// From `biome_lsp_converters::from_proto::text_range()`.
pub(crate) fn text_range(
    range: lsp_types::Range,
    line_index: &LineIndex,
    position_encoding: PositionEncoding,
) -> anyhow::Result<biome_text_size::TextRange> {
    let start = offset_from_position(range.start, line_index, position_encoding)?;
    let end = offset_from_position(range.end, line_index, position_encoding)?;
    Ok(biome_text_size::TextRange::new(start, end))
}
