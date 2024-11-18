//
// handlers_format.rs
//
// Copyright (C) 2024 Posit Software, PBC. All rights reserved.
//
//

use air_r_formatter::{context::RFormatOptions, format_node};
use biome_formatter::{IndentStyle, LineWidth};
use biome_lsp_converters::from_proto;
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

    let edits = to_proto::replace_all_edit(&doc.line_index, &doc.contents, output)?;
    Ok(Some(edits))
}

#[tracing::instrument(level = "info", skip_all)]
pub(crate) fn document_range_formatting(
    params: lsp_types::DocumentRangeFormattingParams,
    state: &WorldState,
) -> anyhow::Result<Option<Vec<lsp_types::TextEdit>>> {
    let doc = state.get_document(&params.text_document.uri)?;

    let line_width = LineWidth::try_from(80).map_err(|err| anyhow::anyhow!("{err}"))?;
    let range =
        from_proto::text_range(&doc.line_index.index, params.range, doc.line_index.encoding)?;

    // WIP
    let options = RFormatOptions::default()
        .with_indent_style(IndentStyle::Space)
        .with_line_width(line_width);

    let output = biome_formatter::format_range(
        &doc.parse.syntax(),
        range,
        air_r_formatter::RFormatLanguage::new(options),
    )?
    .into_code();

    let edits = to_proto::replace_range_edit(&doc.line_index, range, output)?;
    Ok(Some(edits))
}
