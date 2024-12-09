mod fixtures;

use assert_matches::assert_matches;
use tower_lsp::lsp_types;
use tower_lsp::lsp_types::ServerCapabilities;
use tower_lsp::lsp_types::ServerInfo;
use tower_lsp::lsp_types::TextDocumentSyncCapability;
use tower_lsp::lsp_types::TextDocumentSyncKind;

use fixtures::test_client::start_test_client;

#[tests_macros::lsp_test]
async fn test_init() {
    let mut client = start_test_client().await;

    client.initialize().await;

    let value = client.recv_response().await;
    let value: lsp_types::InitializeResult =
        serde_json::from_value(value.result().unwrap().clone()).unwrap();

    assert_matches!(
        value,
        lsp_types::InitializeResult {
            capabilities,
            server_info
        } => {
            assert_matches!(capabilities, ServerCapabilities {
                position_encoding,
                text_document_sync,
                ..
            } => {
                assert_eq!(position_encoding, None);
                assert_eq!(text_document_sync, Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::INCREMENTAL)));
            });

            assert_matches!(server_info, Some(ServerInfo { name, version }) => {
                assert!(name.contains("Air Language Server"));
                assert!(version.is_some());
            });
        }
    );

    client
}
