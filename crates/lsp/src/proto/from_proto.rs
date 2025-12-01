use std::ops::Range;

use anyhow::Context;
use biome_line_index::LineCol;
use biome_line_index::LineIndex;
use biome_line_index::WideLineCol;
use tower_lsp::lsp_types;

use crate::proto::PositionEncoding;

/// The function is used to convert a LSP position to TextSize.
/// From `biome_lsp_converters::from_proto::offset()`.
pub(crate) fn offset(
    position: lsp_types::Position,
    line_index: &LineIndex,
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
    range: lsp_types::Range,
    line_index: &LineIndex,
    position_encoding: PositionEncoding,
) -> anyhow::Result<biome_text_size::TextRange> {
    let start = offset(range.start, line_index, position_encoding)?;
    let end = offset(range.end, line_index, position_encoding)?;
    Ok(biome_text_size::TextRange::new(start, end))
}

/// Apply text changes to document contents.
///
/// If at least one of the changes is a full document change, uses the last of them
/// as the starting point and ignores all previous changes. We then know that all
/// changes after this (if any!) are incremental changes.
///
/// Handles all incremental changes after a full document change. We don't
/// typically get >1 incremental change as the user types, but we do get them in a
/// batch after a find-and-replace, or after a format-on-save request.
///
/// Some editors like VS Code send the edits in reverse order (from the bottom of
/// file -> top of file). We can take advantage of this, because applying an edit
/// on, say, line 10, doesn't invalidate the `line_index` if we then need to apply
/// an additional edit on line 5. That said, we may still have edits that cross
/// lines, so rebuilding the `line_index` is not always unavoidable.
///
/// We also normalize line endings. Changing the line length of inserted or
/// replaced text can't invalidate the text change events since the location of the
/// change itself is specified with [line, col] coordinates, separate from the
/// actual contents of the change.
pub(crate) fn apply_text_changes(
    contents: &mut String,
    mut changes: Vec<lsp_types::TextDocumentContentChangeEvent>,
    line_index: &mut LineIndex,
    position_encoding: PositionEncoding,
) {
    // If we do have a full document change, that implies the `last_start_line`
    // corresponding to that change is line 0, which will correctly force a rebuild
    // of the line index before applying any incremental changes. We don't go ahead
    // and rebuild the line index here, because it is guaranteed to be rebuilt for
    // us on the way out.
    let (changes, mut last_start_line) =
        match changes.iter().rposition(|change| change.range.is_none()) {
            Some(idx) => {
                let incremental = changes.split_off(idx + 1);
                // Unwrap: `rposition()` confirmed this index contains a full document change
                let change = changes.pop().unwrap();
                *contents = line_ending::normalize(change.text);
                (incremental, 0)
            }
            None => (changes, u32::MAX),
        };

    for change in changes {
        let range = change
            .range
            .expect("`None` case already handled by finding the last full document change.");

        // If the end of this change is at or past the start of the last change, then
        // the `line_index` needed to apply this change is now invalid, so we have to
        // rebuild it.
        if range.end.line >= last_start_line {
            *line_index = biome_line_index::LineIndex::new(contents);
        }
        last_start_line = range.start.line;

        // This is a panic if we can't convert. It means we can't keep the document up
        // to date and something is very wrong.
        let range: Range<usize> = text_range(range, line_index, position_encoding)
            .expect("Can convert `range` from `Position` to `TextRange`.")
            .into();

        contents.replace_range(range, &line_ending::normalize(change.text));
    }
}
