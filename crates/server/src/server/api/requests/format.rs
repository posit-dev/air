use air_r_parser::RParserOptions;
use biome_formatter::LineEnding;
use lsp_types::{self as types, request as req};
use workspace::settings::FormatSettings;

use crate::document::TextEdit;
use crate::document::{PositionEncoding, TextDocument};
use crate::server::api::LSPResult;
use crate::server::{client::Notifier, Result};
use crate::session::{DocumentQuery, DocumentSnapshot};

type FormatResponse = Option<Vec<lsp_types::TextEdit>>;

pub(crate) struct Format;

impl super::RequestHandler for Format {
    type RequestType = req::Formatting;
}

impl super::BackgroundDocumentRequestHandler for Format {
    fn document_url(params: &types::DocumentFormattingParams) -> std::borrow::Cow<lsp_types::Url> {
        std::borrow::Cow::Borrowed(&params.text_document.uri)
    }

    fn run_with_snapshot(
        snapshot: DocumentSnapshot,
        _notifier: Notifier,
        _params: types::DocumentFormattingParams,
    ) -> Result<FormatResponse> {
        format_document(&snapshot)
    }
}

/// Formats a full text document
#[tracing::instrument(level = "info", skip_all)]
pub(super) fn format_document(snapshot: &DocumentSnapshot) -> Result<FormatResponse> {
    let text_document = snapshot.query().as_single_document();
    let query = snapshot.query();
    format_text_document(text_document, query, snapshot.encoding())
}

fn format_text_document(
    text_document: &TextDocument,
    query: &DocumentQuery,
    encoding: PositionEncoding,
) -> Result<FormatResponse> {
    let document_settings = query.settings();
    let formatter_settings = &document_settings.format;

    let source = text_document.source_file();
    let text = source.contents();
    let ending = text_document.ending();

    let new_text = format_source(text, formatter_settings)
        .with_failure_code(lsp_server::ErrorCode::InternalError)?;

    let Some(new_text) = new_text else {
        return Ok(None);
    };

    let text_edit = TextEdit::diff(text, &new_text);

    let edits = text_edit
        .into_proto(source, encoding, ending)
        .with_failure_code(lsp_server::ErrorCode::InternalError)?;

    Ok(Some(edits))
}

fn format_source(
    source: &str,
    formatter_settings: &FormatSettings,
) -> anyhow::Result<Option<String>> {
    let parse = air_r_parser::parse(source, RParserOptions::default());

    if parse.has_errors() {
        return Err(anyhow::anyhow!("Can't format when there are parse errors."));
    }

    // Do we need to check that `doc` is indeed an R file? What about special
    // files that don't have extensions like `NAMESPACE`, do we hard-code a
    // list? What about unnamed temporary files?

    // Always use `Lf` line endings on the way out from the formatter since we
    // internally store all LSP text documents with `Lf` endings
    let format_options = formatter_settings
        .to_format_options(source)
        .with_line_ending(LineEnding::Lf);

    let formatted = air_r_formatter::format_node(format_options, &parse.syntax())?;
    let code = formatted.print()?.into_code();

    Ok(Some(code))
}

#[cfg(test)]
mod tests {
    use crate::document::TextDocument;
    use crate::{test::init_test_client, test::TestClientExt};

    #[test]
    fn test_format() {
        let mut client = init_test_client();

        #[rustfmt::skip]
        let doc = TextDocument::doodle(
"
1
2+2
3 + 3 +
3",
        );

        let formatted = client.format_document(&doc);
        insta::assert_snapshot!(formatted);

        client.shutdown();
        client.exit();
    }

    // https://github.com/posit-dev/air/issues/61
    #[test]
    fn test_format_minimal_diff() {
        let mut client = init_test_client();

        #[rustfmt::skip]
        let doc = TextDocument::doodle(
"1
2+2
3
",
        );

        let edits = client.format_document_edits(&doc).unwrap();
        assert!(edits.len() == 1);

        let edit = &edits[0];
        assert_eq!(edit.new_text, " + ");

        client.shutdown();
        client.exit();
    }
}
