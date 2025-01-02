use biome_text_size::TextRange;
use server_test::TestClient;

use crate::document::PositionEncoding;
use crate::document::TextDocument;
use crate::proto::TextRangeExt;

pub(crate) trait TestClientExt {
    fn open_document(&mut self, doc: &TextDocument) -> lsp_types::TextDocumentItem;

    fn format_document(&mut self, doc: &TextDocument) -> String;
    fn format_document_range(&mut self, doc: &TextDocument, range: TextRange) -> String;
    fn format_document_edits(&mut self, doc: &TextDocument) -> Option<Vec<lsp_types::TextEdit>>;
    fn format_document_range_edits(
        &mut self,
        doc: &TextDocument,
        range: TextRange,
    ) -> Option<Vec<lsp_types::TextEdit>>;

    fn position_encoding(&self) -> PositionEncoding;
}

impl TestClientExt for TestClient {
    fn open_document(&mut self, doc: &TextDocument) -> lsp_types::TextDocumentItem {
        let path = format!("test://{}", uuid::Uuid::new_v4());
        let uri = url::Url::parse(&path).unwrap();

        let text_document = lsp_types::TextDocumentItem {
            uri,
            language_id: String::from("r"),
            version: 0,
            text: doc.contents().to_string(),
        };

        let params = lsp_types::DidOpenTextDocumentParams {
            text_document: text_document.clone(),
        };
        self.did_open_text_document(params);

        text_document
    }

    fn format_document(&mut self, doc: &TextDocument) -> String {
        let edits = self.format_document_edits(doc).unwrap();
        apply_text_edits(edits, doc, self.position_encoding()).unwrap()
    }

    fn format_document_range(&mut self, doc: &TextDocument, range: TextRange) -> String {
        let Some(edits) = self.format_document_range_edits(doc, range) else {
            return doc.contents().to_string();
        };
        apply_text_edits(edits, doc, self.position_encoding()).unwrap()
    }

    fn format_document_edits(&mut self, doc: &TextDocument) -> Option<Vec<lsp_types::TextEdit>> {
        let lsp_doc = self.open_document(doc);

        let options = lsp_types::FormattingOptions {
            tab_size: 4,
            insert_spaces: false,
            ..Default::default()
        };

        self.formatting(lsp_types::DocumentFormattingParams {
            text_document: lsp_types::TextDocumentIdentifier {
                uri: lsp_doc.uri.clone(),
            },
            options,
            work_done_progress_params: Default::default(),
        });

        let response = self.recv_response();

        if let Some(err) = response.error {
            panic!("Unexpected error: {}", err.message);
        };

        let value: Option<Vec<lsp_types::TextEdit>> =
            serde_json::from_value(response.result.unwrap().clone()).unwrap();

        self.close_document(lsp_doc.uri);

        value
    }

    fn format_document_range_edits(
        &mut self,
        doc: &TextDocument,
        range: TextRange,
    ) -> Option<Vec<lsp_types::TextEdit>> {
        let lsp_doc = self.open_document(doc);

        let options = lsp_types::FormattingOptions {
            tab_size: 4,
            insert_spaces: false,
            ..Default::default()
        };

        let range = range.into_proto(doc.source_file(), self.position_encoding());

        self.range_formatting(lsp_types::DocumentRangeFormattingParams {
            text_document: lsp_types::TextDocumentIdentifier {
                uri: lsp_doc.uri.clone(),
            },
            range,
            options,
            work_done_progress_params: Default::default(),
        });

        let response = self.recv_response();

        if let Some(err) = response.error {
            panic!("Unexpected error: {}", err.message);
        };

        let value: Option<Vec<lsp_types::TextEdit>> =
            serde_json::from_value(response.result.unwrap().clone()).unwrap();

        self.close_document(lsp_doc.uri);

        value
    }

    fn position_encoding(&self) -> PositionEncoding {
        self.encoding().try_into().unwrap()
    }
}

fn apply_text_edits(
    mut edits: Vec<lsp_types::TextEdit>,
    doc: &crate::document::TextDocument,
    encoding: crate::document::PositionEncoding,
) -> anyhow::Result<String> {
    use std::ops::Range;

    let mut new_text = doc.contents().to_string();

    let source = doc.source_file();

    // Apply edits from bottom to top to avoid inserted newlines to invalidate
    // positions in earlier parts of the doc (they are sent in reading order
    // accorder to the LSP protocol)
    edits.reverse();

    for edit in edits {
        let range: Range<usize> = TextRange::from_proto(edit.range, source, encoding).into();
        new_text.replace_range(range, &edit.new_text);
    }

    Ok(new_text)
}
