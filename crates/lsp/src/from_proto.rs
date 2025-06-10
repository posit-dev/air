use anyhow::Context;
use biome_line_index::LineCol;
use biome_line_index::LineIndex;
use biome_line_index::WideLineCol;
use tower_lsp::lsp_types;

use crate::documents::Document;
use crate::proto::PositionEncoding;

/// The function is used to convert a LSP position to TextSize.
/// From `biome_lsp_converters::from_proto::offset()`.
pub(crate) fn offset(
    line_index: &LineIndex,
    position: lsp_types::Position,
    position_encoding: PositionEncoding,
) -> anyhow::Result<biome_text_size::TextSize> {
    let line_col = match position_encoding {
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
    };

    line_index
        .offset(line_col)
        .with_context(|| format!("Position {position:?} is out of range"))
}

/// The function is used to convert a LSP range to TextRange.
/// From `biome_lsp_converters::from_proto::text_range()`.
pub(crate) fn text_range(
    line_index: &LineIndex,
    range: lsp_types::Range,
    position_encoding: PositionEncoding,
) -> anyhow::Result<biome_text_size::TextRange> {
    let start = offset(line_index, range.start, position_encoding)?;
    let end = offset(line_index, range.end, position_encoding)?;
    Ok(biome_text_size::TextRange::new(start, end))
}

pub fn apply_text_edits(
    doc: &Document,
    mut edits: Vec<lsp_types::TextEdit>,
) -> anyhow::Result<String> {
    let mut text = doc.contents.clone();

    // Apply edits from bottom to top to avoid inserted newlines to invalidate
    // positions in earlier parts of the doc (they are sent in reading order
    // accorder to the LSP protocol)
    edits.reverse();

    for edit in edits {
        let start: usize = offset(
            &doc.line_index.index,
            edit.range.start,
            doc.line_index.encoding,
        )?
        .into();
        let end: usize = offset(
            &doc.line_index.index,
            edit.range.end,
            doc.line_index.encoding,
        )?
        .into();

        text.replace_range(start..end, &edit.new_text);
    }

    Ok(text)
}
