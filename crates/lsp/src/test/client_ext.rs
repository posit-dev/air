use aether_lsp_utils::proto::{from_proto, to_proto};
use biome_text_size::TextRange;
use lsp_test::lsp_client::TestClient;
use tower_lsp::lsp_types;

use crate::documents::Document;

pub(crate) trait TestClientExt {
    async fn open_document(
        &mut self,
        doc: &Document,
        filename: FileName,
    ) -> lsp_types::TextDocumentItem;

    async fn format_document(&mut self, doc: &Document, filename: FileName) -> String;
    async fn format_document_range(
        &mut self,
        doc: &Document,
        filename: FileName,
        range: TextRange,
    ) -> String;
    async fn format_document_edits(
        &mut self,
        doc: &Document,
        filename: FileName,
    ) -> Option<Vec<lsp_types::TextEdit>>;
    async fn format_document_range_edits(
        &mut self,
        doc: &Document,
        filename: FileName,
        range: TextRange,
    ) -> Option<Vec<lsp_types::TextEdit>>;
}

pub(crate) enum FileName {
    Random,
    Url(String),
}

impl TestClientExt for TestClient {
    async fn open_document(
        &mut self,
        doc: &Document,
        filename: FileName,
    ) -> lsp_types::TextDocumentItem {
        let path = match filename {
            FileName::Random => format!("test://{}", uuid::Uuid::new_v4()),
            FileName::Url(filename) => filename,
        };

        let uri = url::Url::parse(&path).unwrap();

        let text_document = lsp_types::TextDocumentItem {
            uri,
            language_id: String::from("r"),
            version: 0,
            text: doc.contents.clone(),
        };

        let params = lsp_types::DidOpenTextDocumentParams {
            text_document: text_document.clone(),
        };
        self.did_open_text_document(params).await;

        text_document
    }

    async fn format_document(&mut self, doc: &Document, filename: FileName) -> String {
        match self.format_document_edits(doc, filename).await {
            Some(edits) => {
                let mut contents = doc.contents.clone();
                let mut line_index = doc.line_index.clone();
                from_proto::apply_text_edits(
                    &mut contents,
                    edits,
                    &mut line_index,
                    doc.position_encoding,
                );
                contents
            }
            None => doc.contents.clone(),
        }
    }

    async fn format_document_range(
        &mut self,
        doc: &Document,
        filename: FileName,
        range: TextRange,
    ) -> String {
        match self.format_document_range_edits(doc, filename, range).await {
            Some(edits) => {
                let mut contents = doc.contents.clone();
                let mut line_index = doc.line_index.clone();
                from_proto::apply_text_edits(
                    &mut contents,
                    edits,
                    &mut line_index,
                    doc.position_encoding,
                );
                contents
            }
            None => doc.contents.clone(),
        }
    }

    async fn format_document_edits(
        &mut self,
        doc: &Document,
        filename: FileName,
    ) -> Option<Vec<lsp_types::TextEdit>> {
        let lsp_doc = self.open_document(doc, filename).await;

        self.formatting(lsp_types::DocumentFormattingParams {
            text_document: lsp_types::TextDocumentIdentifier {
                uri: lsp_doc.uri.clone(),
            },
            options: formatting_options(doc),
            work_done_progress_params: Default::default(),
        })
        .await;

        let response = self.recv_response().await;

        if let Some(err) = response.error() {
            panic!("Unexpected error: {}", err.message);
        };

        let value: Option<Vec<lsp_types::TextEdit>> =
            serde_json::from_value(response.result().unwrap().clone()).unwrap();

        self.close_document(lsp_doc.uri).await;

        value
    }

    async fn format_document_range_edits(
        &mut self,
        doc: &Document,
        filename: FileName,
        range: TextRange,
    ) -> Option<Vec<lsp_types::TextEdit>> {
        let lsp_doc = self.open_document(doc, filename).await;

        let range = to_proto::range(range, &doc.line_index, doc.position_encoding).unwrap();

        self.range_formatting(lsp_types::DocumentRangeFormattingParams {
            text_document: lsp_types::TextDocumentIdentifier {
                uri: lsp_doc.uri.clone(),
            },
            range,
            options: formatting_options(doc),
            work_done_progress_params: Default::default(),
        })
        .await;

        let response = self.recv_response().await;

        if let Some(err) = response.error() {
            panic!("Unexpected error: {}", err.message);
        };

        let value: Option<Vec<lsp_types::TextEdit>> =
            serde_json::from_value(response.result().unwrap().clone()).unwrap();

        self.close_document(lsp_doc.uri).await;

        value
    }
}

fn formatting_options(doc: &Document) -> lsp_types::FormattingOptions {
    let tab_size = doc.settings.indent_width.unwrap_or_default();
    let indent_style = doc.settings.indent_style.unwrap_or_default();

    lsp_types::FormattingOptions {
        tab_size: tab_size.value() as u32,
        insert_spaces: matches!(indent_style, settings::IndentStyle::Space),
        ..Default::default()
    }
}
