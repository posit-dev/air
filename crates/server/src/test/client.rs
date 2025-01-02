use std::sync::LazyLock;
use std::sync::Mutex;

use server_test::TestClient;

use crate::Server;

/// Global test client used by all unit tests
///
/// The language [Server] has per-process global state, such as the global `tracing`
/// subscriber and a global `MESSENGER` to send `ShowMessage` notifications to the client.
/// Because of this, we cannot just repeatedly call [test_client()] to start up a new
/// client/server pair per unit test. Instead, unit tests use [with_client()] to access
/// the global test client, which they can then manipulate. Synchronization is managed
/// through a [Mutex], ensuring that multiple unit tests that need to mutate the client
/// can't run simultaneously (while still allowing other unit tests to run in parallel).
/// Unit tests should be careful not to put the client/server pair into a state that
/// prevents other unit tests from running successfully.
///
/// If you need to modify a client/server pair in such a way that no other unit tests will
/// be able to use it, create an integration test instead, which runs in its own process.
static TEST_CLIENT: LazyLock<Mutex<TestClient>> = LazyLock::new(|| Mutex::new(test_client()));

pub(crate) fn with_client<F>(f: F)
where
    F: FnOnce(&mut server_test::TestClient),
{
    let mut client = TEST_CLIENT.lock().unwrap();
    f(&mut client)
}

fn test_client() -> server_test::TestClient {
    let mut client =
        server_test::TestClient::new(|worker_threads, connection, connection_threads| {
            let server = Server::new(worker_threads, connection, connection_threads).unwrap();
            server.run().unwrap();
        });

    // Initialize and wait for the server response
    let id = client.initialize();
    let response = client.recv_response();
    assert_eq!(id, response.id);

    // Notify the server we have received its initialize response
    client.initialized();

    client
}
