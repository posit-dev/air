//
// handlers_format.rs
//
// Copyright (C) 2024 Posit Software, PBC. All rights reserved.
//
//

use air_r_formatter::{context::RFormatOptions, format_node};
use biome_formatter::{IndentStyle, LineWidth};
use tower_lsp::lsp_types;

use crate::state::WorldState;
use crate::to_proto;

#[tracing::instrument(level = "info", skip_all)]
pub(crate) fn document_formatting(
    params: lsp_types::DocumentFormattingParams,
    state: &WorldState,
) -> anyhow::Result<Option<Vec<lsp_types::TextEdit>>> {
    let doc = state.get_document(&params.text_document.uri)?;

    let line_width = LineWidth::try_from(80).map_err(|err| anyhow::anyhow!("{err}"))?;

    // TODO: Handle FormattingOptions
    let options = RFormatOptions::default()
        .with_indent_style(IndentStyle::Space)
        .with_line_width(line_width);

    if doc.parse.has_errors() {
        return Err(anyhow::anyhow!("Can't format when there are parse errors."));
    }

    let formatted = format_node(options.clone(), &doc.parse.syntax())?;
    let output = formatted.print()?.into_code();

    // Do we need to check that `doc` is indeed an R file? What about special
    // files that don't have extensions like `NAMESPACE`, do we hard-code a
    // list? What about unnamed temporary files?

    let edits = to_proto::replace_all_edit(&doc.line_index, &doc.contents, &output)?;
    Ok(Some(edits))
}
