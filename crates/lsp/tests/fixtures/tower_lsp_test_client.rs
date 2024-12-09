use lsp_test::lsp_client::TestClient;
use tower_lsp::lsp_types;

use lsp::documents::Document;
use lsp::from_proto;

pub(crate) trait TestClientExt {
    async fn open_document(&mut self, doc: &Document) -> lsp_types::TextDocumentItem;
    async fn format_document(&mut self, doc: &Document) -> String;
    async fn format_document_edits(&mut self, doc: &Document) -> Option<Vec<lsp_types::TextEdit>>;
}

impl TestClientExt for TestClient {
    async fn open_document(&mut self, doc: &Document) -> lsp_types::TextDocumentItem {
        let path = format!("test://{}", uuid::Uuid::new_v4());
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

    async fn format_document(&mut self, doc: &Document) -> String {
        let edits = self.format_document_edits(doc).await.unwrap();
        from_proto::apply_text_edits(doc, edits).unwrap()
    }

    async fn format_document_edits(&mut self, doc: &Document) -> Option<Vec<lsp_types::TextEdit>> {
        let lsp_doc = self.open_document(doc).await;

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
        })
        .await;

        let response = self.recv_response().await;

        let value: Option<Vec<lsp_types::TextEdit>> =
            serde_json::from_value(response.result().unwrap().clone()).unwrap();

        self.close_document(lsp_doc.uri).await;

        value
    }
}
