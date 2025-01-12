use assert_matches::assert_matches;
use lsp::start_lsp;
use tower_lsp::lsp_types::PositionEncodingKind;
use tower_lsp::lsp_types::ServerCapabilities;
use tower_lsp::lsp_types::ServerInfo;
use tower_lsp::lsp_types::TextDocumentSyncCapability;
use tower_lsp::lsp_types::TextDocumentSyncKind;

// Normal usage of `with_client()` handles client initialization, so to test it we have
// to run this particular test in its own process and manually start up and initialize
// the client. This also gives us a chance to test the shutdown/exit procedure.

#[tokio::test]
async fn test_initialization_and_shutdown() {
    let mut client = lsp_test::lsp_client::TestClient::new(|server_rx, client_tx| async {
        start_lsp(server_rx, client_tx).await
    });

    let id = client.initialize().await;

    let value = client.recv_response().await;
    assert_eq!(&id, value.id());
    let value: tower_lsp::lsp_types::InitializeResult =
        serde_json::from_value(value.result().unwrap().clone()).unwrap();

    client.initialized().await;

    assert_matches!(
        value,
        tower_lsp::lsp_types::InitializeResult {
            capabilities,
            server_info
        } => {
            assert_matches!(capabilities, ServerCapabilities {
                position_encoding,
                text_document_sync,
                ..
            } => {
                assert_eq!(position_encoding, Some(PositionEncodingKind::UTF16));
                assert_eq!(text_document_sync, Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::INCREMENTAL)));
            });

            assert_matches!(server_info, Some(ServerInfo { name, version }) => {
                assert!(name.contains("Air Language Server"));
                assert!(version.is_some());
            });
        }
    );

    client.shutdown().await;
    client.exit().await;
}
