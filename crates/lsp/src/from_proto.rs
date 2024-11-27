pub(crate) use biome_lsp_converters::from_proto::offset;
pub(crate) use biome_lsp_converters::from_proto::text_range;

use tower_lsp::lsp_types;

use crate::documents::Document;

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
