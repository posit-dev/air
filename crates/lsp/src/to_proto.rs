//
// to_proto.rs
//
// Copyright (C) 2024 Posit Software, PBC. All rights reserved.
//
//

// Utilites for converting internal types to LSP types

pub use crate::rust_analyzer::to_proto as rust_analyzer;

use tower_lsp::lsp_types;

use crate::rust_analyzer::{line_index::LineIndex, text_edit::TextEdit, to_proto};

pub(crate) fn doc_edit_vec(
    line_index: &LineIndex,
    text_edit: TextEdit,
) -> anyhow::Result<Vec<lsp_types::TextDocumentContentChangeEvent>> {
    let edits = to_proto::text_edit_vec(line_index, text_edit)?;

    Ok(edits
        .into_iter()
        .map(|edit| lsp_types::TextDocumentContentChangeEvent {
            range: Some(edit.range),
            range_length: None,
            text: edit.new_text,
        })
        .collect())
}

pub(crate) fn replace_all_edit(
    line_index: &LineIndex,
    text: &str,
    replace_with: String,
) -> anyhow::Result<Vec<lsp_types::TextEdit>> {
    let edit = TextEdit::replace_all(text, replace_with);
    to_proto::text_edit_vec(line_index, edit)
}
