// --- source
// authors = ["rust-analyzer team"]
// license = "MIT OR Apache-2.0"
// origin = "https://github.com/rust-lang/rust-analyzer/blob/master/crates/rust-analyzer/src/lsp/to_proto.rs"
// ---

//! Conversion of rust-analyzer specific types to lsp_types equivalents.

use super::{
    line_index::LineIndex,
    text_edit::{Indel, TextEdit},
};
use settings::LineEnding;
use tower_lsp::lsp_types;

pub(crate) fn text_edit(
    line_index: &LineIndex,
    indel: Indel,
) -> anyhow::Result<lsp_types::TextEdit> {
    let range = biome_lsp_converters::to_proto::range(
        &line_index.index,
        indel.delete,
        line_index.encoding,
    )?;
    let new_text = match line_index.endings {
        LineEnding::Lf => indel.insert,
        LineEnding::Crlf => indel.insert.replace('\n', "\r\n"),
    };
    Ok(lsp_types::TextEdit { range, new_text })
}

pub(crate) fn text_edit_vec(
    line_index: &LineIndex,
    text_edit: TextEdit,
) -> anyhow::Result<Vec<lsp_types::TextEdit>> {
    text_edit
        .into_iter()
        .map(|indel| self::text_edit(line_index, indel))
        .collect()
}
