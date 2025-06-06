//
// to_proto.rs
//
// Copyright (C) 2024 Posit Software, PBC. All rights reserved.
//
//

// Utilites for converting internal types to LSP types

use anyhow::Context;
pub(crate) use rust_analyzer::to_proto::text_edit_vec;

use crate::proto::PositionEncoding;
use crate::rust_analyzer::{self, line_index::LineIndex, text_edit::TextEdit};
use biome_text_size::TextRange;
use biome_text_size::TextSize;
use tower_lsp::lsp_types;

// TODO!: We use `rust_analyzer::LineIndex` here, but `biome_line_index::LineIndex`
// in `from_proto.rs`. We should use `biome_line_index::LineIndex` everywhere, and
// consider getting rid of `rust_analyzer::LineIndex` entirely.

/// The function is used to convert TextSize to a LSP position.
/// From `biome_lsp_converters::to_proto::position()`.
pub fn position(
    line_index: &LineIndex,
    offset: TextSize,
    position_encoding: PositionEncoding,
) -> anyhow::Result<lsp_types::Position> {
    let line_col = line_index
        .index
        .line_col(offset)
        .with_context(|| format!("Could not convert offset {offset:?} into a line-column index"))?;

    let position = match position_encoding {
        PositionEncoding::Utf8 => lsp_types::Position::new(line_col.line, line_col.col),
        PositionEncoding::Wide(enc) => {
            let line_col = line_index
                .index
                .to_wide(enc, line_col)
                .with_context(|| format!("Could not convert {line_col:?} into wide line column"))?;
            lsp_types::Position::new(line_col.line, line_col.col)
        }
    };

    Ok(position)
}

/// The function is used to convert TextRange to a LSP range.
/// From `biome_lsp_converters::to_proto::range()`.
pub fn range(
    line_index: &LineIndex,
    range: TextRange,
    position_encoding: PositionEncoding,
) -> anyhow::Result<lsp_types::Range> {
    let start = position(line_index, range.start(), position_encoding)?;
    let end = position(line_index, range.end(), position_encoding)?;
    Ok(lsp_types::Range::new(start, end))
}

#[cfg(test)]
pub(crate) fn doc_edit_vec(
    line_index: &LineIndex,
    text_edit: TextEdit,
) -> anyhow::Result<Vec<lsp_types::TextDocumentContentChangeEvent>> {
    let edits = text_edit_vec(line_index, text_edit)?;

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
    line_index: &LineIndex,
    range: TextRange,
    replace_with: String,
) -> anyhow::Result<Vec<lsp_types::TextEdit>> {
    let edit = TextEdit::replace(range, replace_with);
    text_edit_vec(line_index, edit)
}

pub(crate) fn replace_all_edit(
    line_index: &LineIndex,
    text: &str,
    replace_with: &str,
) -> anyhow::Result<Vec<lsp_types::TextEdit>> {
    let edit = TextEdit::diff(text, replace_with);
    text_edit_vec(line_index, edit)
}
