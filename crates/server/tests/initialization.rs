use assert_matches::assert_matches;
use lsp_types::PositionEncodingKind;
use lsp_types::ServerCapabilities;
use lsp_types::ServerInfo;
use lsp_types::TextDocumentSyncCapability;
use lsp_types::TextDocumentSyncKind;
use lsp_types::TextDocumentSyncOptions;

// Normal usage of `with_client()` handles client initialization, so to test it we have
// to run this particular test in its own process and manually start up and initialize
// the client. This also gives us a chance to test the shutdown/exit procedure.

#[test]
fn test_initialization_and_shutdown() {
    let mut client =
        server_test::TestClient::new(|worker_threads, connection, connection_threads| {
            let server =
                server::Server::new(worker_threads, connection, connection_threads).unwrap();
            server.run().unwrap();
        });

    let id = client.initialize();

    let value = client.recv_response();
    assert_eq!(id, value.id);
    let value: lsp_types::InitializeResult =
        serde_json::from_value(value.result.unwrap().clone()).unwrap();

    client.initialized();

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
                assert_eq!(position_encoding, Some(PositionEncodingKind::UTF8));
                assert_eq!(text_document_sync, Some(TextDocumentSyncCapability::Options(
                    TextDocumentSyncOptions {
                        open_close: Some(true),
                        change: Some(TextDocumentSyncKind::INCREMENTAL),
                        will_save: Some(false),
                        will_save_wait_until: Some(false),
                        ..Default::default()
                    },
                )));
            });

            assert_matches!(server_info, Some(ServerInfo { name, version }) => {
                assert!(name.contains("Air Language Server"));
                assert!(version.is_some());
            });
        }
    );

    client.shutdown();
    client.exit();
}
