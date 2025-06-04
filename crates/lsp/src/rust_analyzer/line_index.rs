// --- source
// authors = ["rust-analyzer team"]
// license = "MIT OR Apache-2.0"
// origin = "https://github.com/rust-lang/rust-analyzer/blob/master/crates/rust-analyzer/src/line_index.rs"
// ---

//! Enhances `ide::LineIndex` with additional info required to convert offsets
//! into lsp positions.

use settings::LineEnding;
use triomphe::Arc;

use crate::proto::PositionEncoding;

#[derive(Debug, Clone)]
pub struct LineIndex {
    pub index: Arc<biome_line_index::LineIndex>,
    pub endings: LineEnding,
    pub encoding: PositionEncoding,
}
