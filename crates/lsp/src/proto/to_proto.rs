//
// to_proto.rs
//
// Copyright (C) 2024 Posit Software, PBC. All rights reserved.
//
//

// Utilites for converting internal types to LSP types

use anyhow::Context;
use biome_line_index::LineIndex;
use biome_text_size::TextRange;
use biome_text_size::TextSize;
use settings::LineEnding;
use tower_lsp::lsp_types;

use crate::proto::PositionEncoding;
use crate::text_edit::Indel;
use crate::text_edit::TextEdit;

/// The function is used to convert TextSize to a LSP position.
/// From `biome_lsp_converters::to_proto::position()`.
pub(crate) fn position(
    offset: TextSize,
    line_index: &LineIndex,
    position_encoding: PositionEncoding,
) -> anyhow::Result<lsp_types::Position> {
    let line_col = line_index
        .line_col(offset)
        .with_context(|| format!("Could not convert offset {offset:?} into a line-column index"))?;

    let position = match position_encoding {
        PositionEncoding::Utf8 => lsp_types::Position::new(line_col.line, line_col.col),
        PositionEncoding::Wide(enc) => {
            let line_col = line_index
                .to_wide(enc, line_col)
                .with_context(|| format!("Could not convert {line_col:?} into wide line column"))?;
            lsp_types::Position::new(line_col.line, line_col.col)
        }
    };

    Ok(position)
}

/// The function is used to convert TextRange to a LSP range.
/// From `biome_lsp_converters::to_proto::range()`.
pub(crate) fn range(
    range: TextRange,
    line_index: &LineIndex,
    position_encoding: PositionEncoding,
) -> anyhow::Result<lsp_types::Range> {
    let start = position(range.start(), line_index, position_encoding)?;
    let end = position(range.end(), line_index, position_encoding)?;
    Ok(lsp_types::Range::new(start, end))
}

pub(crate) fn text_edit(
    indel: Indel,
    line_index: &LineIndex,
    position_encoding: PositionEncoding,
    endings: LineEnding,
) -> anyhow::Result<lsp_types::TextEdit> {
    let range = range(indel.delete, line_index, position_encoding)?;
    let new_text = match endings {
        LineEnding::Lf => indel.insert,
        LineEnding::Crlf => indel.insert.replace('\n', "\r\n"),
    };
    Ok(lsp_types::TextEdit { range, new_text })
}

pub(crate) fn text_edit_vec(
    text_edit: TextEdit,
    line_index: &LineIndex,
    position_encoding: PositionEncoding,
    endings: LineEnding,
) -> anyhow::Result<Vec<lsp_types::TextEdit>> {
    text_edit
        .into_iter()
        .map(|indel| self::text_edit(indel, line_index, position_encoding, endings))
        .collect()
}

#[cfg(test)]
pub(crate) fn doc_edit_vec(
    text_edit: TextEdit,
    line_index: &LineIndex,
    position_encoding: PositionEncoding,
    endings: LineEnding,
) -> anyhow::Result<Vec<lsp_types::TextDocumentContentChangeEvent>> {
    let edits = text_edit_vec(text_edit, line_index, position_encoding, endings)?;

    Ok(edits
        .into_iter()
        .map(|edit| lsp_types::TextDocumentContentChangeEvent {
            range: Some(edit.range),
            range_length: None,
            text: edit.new_text,
        })
        .collect())
}

pub(crate) fn replace_range_edit(
    range: TextRange,
    replace_with: String,
    line_index: &LineIndex,
    position_encoding: PositionEncoding,
    endings: LineEnding,
) -> anyhow::Result<Vec<lsp_types::TextEdit>> {
    let edit = TextEdit::replace(range, replace_with);
    text_edit_vec(edit, line_index, position_encoding, endings)
}

pub(crate) fn replace_all_edit(
    text: &str,
    replace_with: &str,
    line_index: &LineIndex,
    position_encoding: PositionEncoding,
    endings: LineEnding,
) -> anyhow::Result<Vec<lsp_types::TextEdit>> {
    let edit = crate::diff::diff(text, replace_with);
    text_edit_vec(edit, line_index, position_encoding, endings)
}
