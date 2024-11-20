use air_r_formatter::{context::RFormatOptions, format_node};
use biome_formatter::{IndentStyle, LineWidth};
use tower_lsp::lsp_types;

use crate::state::WorldState;

#[derive(Debug, Eq, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ViewFileParams {
    /// From `lsp_types::TextDocumentPositionParams`
    pub(crate) text_document: lsp_types::TextDocumentIdentifier,
    pub(crate) position: lsp_types::Position,

    /// Viewer type
    pub(crate) kind: ViewFileKind,
}

#[derive(Debug, Eq, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) enum ViewFileKind {
    SyntaxTree,
    FormatTree,
}

pub(crate) fn view_file(params: ViewFileParams, state: &WorldState) -> anyhow::Result<String> {
    let doc = state.get_document(&params.text_document.uri)?;

    match params.kind {
        ViewFileKind::SyntaxTree => {
            let syntax = doc.syntax();
            Ok(format!("{syntax:#?}"))
        }

        ViewFileKind::FormatTree => {
            let line_width = LineWidth::try_from(80).map_err(|err| anyhow::anyhow!("{err}"))?;

            let options = RFormatOptions::default()
                .with_indent_style(IndentStyle::Space)
                .with_line_width(line_width);

            let formatted = format_node(options.clone(), &doc.parse.syntax())?;
            Ok(format!("{}", formatted.into_document()))
        }
    }
}
