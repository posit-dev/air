use tower_lsp::lsp_types::FoldingRange;
use tower_lsp::lsp_types::FoldingRangeKind;

use crate::documents::Document;

pub fn folding_range(_document: &Document) -> anyhow::Result<Vec<FoldingRange>> {
    // sample
    let example_range = FoldingRange {
        start_line: 0,
        start_character: None,
        end_line: 1,
        end_character: None,
        kind: Some(FoldingRangeKind::Region),
        collapsed_text: None,
    };
    Ok(vec![example_range])
}
