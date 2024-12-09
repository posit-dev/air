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
use crate::{from_proto, to_proto};

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

#[tracing::instrument(level = "info", skip_all)]
pub(crate) fn document_range_formatting(
    params: lsp_types::DocumentRangeFormattingParams,
    state: &WorldState,
) -> anyhow::Result<Option<Vec<lsp_types::TextEdit>>> {
    let doc = state.get_document(&params.text_document.uri)?;

    let line_width = LineWidth::try_from(80).map_err(|err| anyhow::anyhow!("{err}"))?;
    let range =
        from_proto::text_range(&doc.line_index.index, params.range, doc.line_index.encoding)?;

    // TODO: Handle FormattingOptions
    let options = RFormatOptions::default()
        .with_indent_style(IndentStyle::Space)
        .with_line_width(line_width);

    let format_info = biome_formatter::format_range(
        &doc.parse.syntax(),
        range,
        air_r_formatter::RFormatLanguage::new(options),
    )?;

    let Some(format_range) = format_info.range() else {
        // Happens in edge cases when biome returns a `Printed::new_empty()`
        return Ok(None);
    };

    let format_text = format_info.into_code();
    let edits = to_proto::replace_range_edit(&doc.line_index, format_range, format_text)?;

    Ok(Some(edits))
}

#[cfg(test)]
mod tests {
    use biome_text_size::{TextRange, TextSize};

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

        let edit = &edits[0];
        assert_eq!(edit.new_text, " + ");

        client
    }

    #[tests_macros::lsp_test]
    async fn test_format_range_none() {
        let mut client = init_test_client().await;

        #[rustfmt::skip]
        let doc = Document::doodle(
"",
        );
        let range = TextRange::new(TextSize::from(0), TextSize::from(0));

        let output = client.format_document_range(&doc, range).await;
        insta::assert_snapshot!(output);

        #[rustfmt::skip]
        let doc = Document::doodle(
"
",
        );
        let range = TextRange::new(TextSize::from(0), TextSize::from(1));

        let output = client.format_document_range(&doc, range).await;
        insta::assert_snapshot!(output);

        #[rustfmt::skip]
        let doc = Document::doodle(
"1
",
        );
        let range = TextRange::new(TextSize::from(0), TextSize::from(1));

        let output = client.format_document_range(&doc, range).await;
        insta::assert_snapshot!(output);

        client
    }

    #[tests_macros::lsp_test]
    async fn test_format_range_minimal() {
        // FIXME: This currently fails. Line 0 should not be affected by formatting line 1.
        let mut client = init_test_client().await;

        #[rustfmt::skip]
        let doc = Document::doodle(
"1+1
2+2
",
        );
        let range = TextRange::new(TextSize::from(4), TextSize::from(7));

        let output = client.format_document_range(&doc, range).await;
        insta::assert_snapshot!(output);

        client
    }
}
