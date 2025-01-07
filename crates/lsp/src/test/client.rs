use std::future::Future;
use std::sync::LazyLock;

use tokio::sync::Mutex;

use lsp_test::lsp_client::TestClient;
use tokio::sync::OnceCell;

use crate::start_lsp;

/// Global test client used by all unit tests
///
/// The language server has per-process global state, such as the global `tracing`
/// subscriber that sends log messages to the client. Because of this, we cannot just
/// repeatedly call [new_test_client()] to start up a new client/server pair per unit
/// test. Instead, unit tests use [with_client()] to access the global test client, which
/// they can then manipulate. Synchronization is managed through a [Mutex], ensuring that
/// multiple unit tests that need to mutate the client can't run simultaneously (while
/// still allowing other unit tests to run in parallel). Unit tests should be careful not
/// to put the client/server pair into a state that prevents other unit tests from running
/// successfully.
///
/// If you need to modify a client/server pair in such a way that no other unit tests will
/// be able to use it, create an integration test instead, which runs in its own process.
///
/// TODO: When we switch off async tower-lsp, make this a simpler `LazyLock` around a
/// `std::sync::Mutex`, and then actually `lock()` in `with_client()`, only providing
/// a `&mut TestClient` to the unit test.
static TEST_CLIENT: OnceCell<Mutex<TestClient>> = OnceCell::const_new();

/// `#[tokio::test]` creates a new runtime for each individual test. That doesn't work
/// for us because we want access to the `TEST_CLIENT` to be synchronized within a single
/// tokio runtime. The only way to do this seems to be to create a shared tokio runtime
/// as well.
static TEST_RUNTIME: LazyLock<tokio::runtime::Runtime> = LazyLock::new(|| {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("Should create a tokio runtime")
});

pub(crate) fn with_client<F, Fut>(f: F)
where
    F: FnOnce(&'static Mutex<TestClient>) -> Fut,
    Fut: Future<Output = ()> + Send + 'static,
{
    TEST_RUNTIME.block_on(async {
        let client = get_client().await;
        f(client).await
    });
}

async fn get_client() -> &'static Mutex<TestClient> {
    TEST_CLIENT
        .get_or_init(|| async { Mutex::new(new_test_client().await) })
        .await
}

async fn new_test_client() -> TestClient {
    let mut client =
        TestClient::new(|server_rx, client_tx| async { start_lsp(server_rx, client_tx).await });

    // Initialize and wait for the server response
    let id = client.initialize().await;
    let response = client.recv_response().await;
    assert_eq!(&id, response.id());

    // Notify the server we have received its initialize response
    client.initialized().await;

    client
}
