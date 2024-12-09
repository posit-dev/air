use lsp::tower_lsp::start_test_server;
use lsp_test::lsp_client::TestClient;

pub async fn start_test_client() -> lsp_test::lsp_client::TestClient {
    TestClient::new(|server_rx, client_tx| async { start_test_server(server_rx, client_tx).await })
}

pub async fn init_test_client() -> TestClient {
    let mut client = start_test_client().await;

    client.initialize().await;
    client.recv_response().await;

    client
}
