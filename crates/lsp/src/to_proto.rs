//
// to_proto.rs
//
// Copyright (C) 2024 Posit Software, PBC. All rights reserved.
//
//

// Utilites for converting internal types to LSP types

pub(crate) use rust_analyzer::to_proto::text_edit_vec;

#[cfg(test)]
pub(crate) use biome_lsp_converters::to_proto::range;

use crate::rust_analyzer::{self, line_index::LineIndex, text_edit::TextEdit};
use biome_text_size::TextRange;
use tower_lsp::lsp_types;

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
