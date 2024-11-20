use tower_lsp::lsp_types;

use crate::state::WorldState;

pub(crate) fn view_file(
    params: lsp_types::TextDocumentPositionParams,
    state: &WorldState,
) -> anyhow::Result<String> {
    let doc = state.get_document(&params.text_document.uri)?;
    let syntax = doc.syntax();
    Ok(format!("{syntax:#?}"))
}
