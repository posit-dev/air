// --- source
// authors = ["rust-analyzer team"]
// license = "MIT OR Apache-2.0"
// origin = "https://github.com/rust-lang/rust-analyzer/blob/master/crates/rust-analyzer/src/line_index.rs"
// ---

//! Enhances `ide::LineIndex` with additional info required to convert offsets
//! into lsp positions.

use biome_lsp_converters::line_index;
use settings::LineEnding;
use triomphe::Arc;

#[derive(Debug, Clone)]
pub struct LineIndex {
    pub index: Arc<line_index::LineIndex>,
    pub endings: LineEnding,
    pub encoding: biome_lsp_converters::PositionEncoding,
}
