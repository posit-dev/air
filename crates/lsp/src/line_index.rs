// --- source
// authors = ["rust-analyzer team"]
// license = "MIT OR Apache-2.0"
// origin = "https://github.com/rust-lang/rust-analyzer/blob/master/crates/rust-analyzer/src/line_index.rs"
// ---

use triomphe::Arc;

#[derive(Debug, Clone)]
pub struct LineIndex {
    pub index: Arc<biome_line_index::LineIndex>,
}

impl LineIndex {
    pub fn new(text: &str) -> Self {
        Self {
            index: Arc::new(biome_line_index::LineIndex::new(text)),
        }
    }
}
