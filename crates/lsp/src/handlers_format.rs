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

#[cfg(test)]
mod tests {
    use crate::{
        documents::Document, tower_lsp::init_test_client, tower_lsp_test_client::TestClientExt,
    };

    #[tests_macros::lsp_test]
    async fn test_format() {
        let mut client = init_test_client().await;

        #[rustfmt::skip]
        let doc = Document::doodle(
"
1
2+2
3 + 3 +
3",
        );

        let formatted = client.format_document(&doc).await;
        insta::assert_snapshot!(formatted);

        client
    }

    // https://github.com/posit-dev/air/issues/61
    #[tests_macros::lsp_test]
    async fn test_format_minimal_diff() {
        let mut client = init_test_client().await;

        #[rustfmt::skip]
        let doc = Document::doodle(
"1
2+2
3
",
        );

        let edits = client.format_document_edits(&doc).await.unwrap();
        assert!(edits.len() == 1);

        let edit = edits.get(0).unwrap();
        assert_eq!(edit.new_text, " + ");

        client
    }
}
