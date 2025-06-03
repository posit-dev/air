//
// to_proto.rs
//
// Copyright (C) 2024 Posit Software, PBC. All rights reserved.
//
//

// Utilites for converting internal types to LSP types

use biome_rowan::TextSize;
use settings::LineEnding;
use source_file::LineOffsetEncoding;
use source_file::SourceFile;

use crate::text_edit::Indel;
use crate::text_edit::TextEdit;
use biome_text_size::TextRange;
use tower_lsp::lsp_types;

pub(crate) fn position(
    offset: TextSize,
    source_file: &SourceFile,
    encoding: LineOffsetEncoding,
) -> lsp_types::Position {
    let source_location = source_file.source_location(offset, encoding);
    lsp_types::Position {
        line: source_location.line_number().into(),
        character: source_location.line_offset().raw(),
    }
}

pub(crate) fn range(
    range: TextRange,
    source_file: &SourceFile,
    encoding: LineOffsetEncoding,
) -> lsp_types::Range {
    lsp_types::Range {
        start: self::position(range.start(), source_file, encoding),
        end: self::position(range.end(), source_file, encoding),
    }
}

pub(crate) fn text_edit(
    indel: Indel,
    source_file: &SourceFile,
    encoding: LineOffsetEncoding,
    endings: LineEnding,
) -> lsp_types::TextEdit {
    let range = self::range(indel.delete, source_file, encoding);
    let new_text = match endings {
        LineEnding::Lf => indel.insert,
        LineEnding::Crlf => indel.insert.replace('\n', "\r\n"),
    };
    lsp_types::TextEdit { range, new_text }
}

pub(crate) fn text_edit_vec(
    text_edit: TextEdit,
    source_file: &SourceFile,
    encoding: LineOffsetEncoding,
    endings: LineEnding,
) -> Vec<lsp_types::TextEdit> {
    text_edit
        .into_iter()
        .map(|indel| self::text_edit(indel, source_file, encoding, endings))
        .collect()
}

#[cfg(test)]
pub(crate) fn doc_change_vec(
    text_edit: TextEdit,
    source_file: &SourceFile,
    encoding: LineOffsetEncoding,
    endings: LineEnding,
) -> Vec<lsp_types::TextDocumentContentChangeEvent> {
    let edits = self::text_edit_vec(text_edit, source_file, encoding, endings);

    edits
        .into_iter()
        .map(|edit| lsp_types::TextDocumentContentChangeEvent {
            range: Some(edit.range),
            range_length: None,
            text: edit.new_text,
        })
        .collect()
}

pub(crate) fn replace_range_edit(
    range: TextRange,
    replace_with: String,
    source_file: &SourceFile,
    encoding: LineOffsetEncoding,
    endings: LineEnding,
) -> Vec<lsp_types::TextEdit> {
    let edit = TextEdit::replace(range, replace_with);
    self::text_edit_vec(edit, source_file, encoding, endings)
}

pub(crate) fn replace_all_edit(
    replace_with: &str,
    source_file: &SourceFile,
    encoding: LineOffsetEncoding,
    endings: LineEnding,
) -> Vec<lsp_types::TextEdit> {
    let edit = crate::diff::diff(source_file.contents(), replace_with);
    self::text_edit_vec(edit, source_file, encoding, endings)
}
