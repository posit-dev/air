//
// tower_lsp.rs
//
// Copyright (C) 2022-2024 Posit Software, PBC. All rights reserved.
//
//

#![allow(deprecated)]

use tokio::io::{AsyncRead, AsyncWrite};
use tokio::sync::mpsc::unbounded_channel as tokio_unbounded_channel;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::Client;
use tower_lsp::LanguageServer;
use tower_lsp::LspService;
use tower_lsp::Server;
use tower_lsp::{jsonrpc, ClientSocket};

use crate::main_loop::Event;
use crate::main_loop::GlobalState;
use crate::main_loop::TokioUnboundedSender;

// Based on https://stackoverflow.com/a/69324393/1725177
macro_rules! cast_response {
    ($target:expr, $pat:path) => {{
        match $target {
            Ok($pat(resp)) => Ok(resp),
            Err(err) => Err(new_jsonrpc_error(format!("{err:?}"))),
            _ => panic!("Unexpected variant while casting to {}", stringify!($pat)),
        }
    }};
}

#[derive(Debug)]
pub(crate) enum LspMessage {
    Notification(LspNotification),
    Request(
        LspRequest,
        TokioUnboundedSender<anyhow::Result<LspResponse>>,
    ),
}

#[derive(Debug)]
pub(crate) enum LspNotification {
    Initialized(InitializedParams),
    DidChangeWorkspaceFolders(DidChangeWorkspaceFoldersParams),
    DidChangeConfiguration(DidChangeConfigurationParams),
    DidChangeWatchedFiles(DidChangeWatchedFilesParams),
    DidOpenTextDocument(DidOpenTextDocumentParams),
    DidChangeTextDocument(DidChangeTextDocumentParams),
    DidSaveTextDocument(DidSaveTextDocumentParams),
    DidCloseTextDocument(DidCloseTextDocumentParams),
}

#[derive(Debug)]
pub(crate) enum LspRequest {
    Initialize(InitializeParams),
    Shutdown(),
}

#[derive(Debug)]
pub(crate) enum LspResponse {
    Initialize(InitializeResult),
    Shutdown(()),
}

#[derive(Debug)]
struct Backend {
    /// Channel for communication with the main loop.
    events_tx: TokioUnboundedSender<Event>,

    /// Handle to main loop. Drop it to cancel the loop, all associated tasks,
    /// and drop all owned state.
    _main_loop: tokio::task::JoinSet<()>,
}

impl Backend {
    async fn request(&self, request: LspRequest) -> anyhow::Result<LspResponse> {
        let (response_tx, mut response_rx) =
            tokio_unbounded_channel::<anyhow::Result<LspResponse>>();

        // Relay request to main loop
        self.events_tx
            .send(Event::Lsp(LspMessage::Request(request, response_tx)))
            .unwrap();

        // Wait for response from main loop
        response_rx.recv().await.unwrap()
    }

    fn notify(&self, notif: LspNotification) {
        // Relay notification to main loop
        self.events_tx
            .send(Event::Lsp(LspMessage::Notification(notif)))
            .unwrap();
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, params: InitializeParams) -> Result<InitializeResult> {
        cast_response!(
            self.request(LspRequest::Initialize(params)).await,
            LspResponse::Initialize
        )
    }

    async fn initialized(&self, params: InitializedParams) {
        self.notify(LspNotification::Initialized(params));
    }

    async fn shutdown(&self) -> Result<()> {
        cast_response!(
            self.request(LspRequest::Shutdown()).await,
            LspResponse::Shutdown
        )
    }

    async fn did_change_workspace_folders(&self, params: DidChangeWorkspaceFoldersParams) {
        self.notify(LspNotification::DidChangeWorkspaceFolders(params));
    }

    async fn did_change_configuration(&self, params: DidChangeConfigurationParams) {
        self.notify(LspNotification::DidChangeConfiguration(params));
    }

    async fn did_change_watched_files(&self, params: DidChangeWatchedFilesParams) {
        self.notify(LspNotification::DidChangeWatchedFiles(params));
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        self.notify(LspNotification::DidOpenTextDocument(params));
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        self.notify(LspNotification::DidChangeTextDocument(params));
    }

    async fn did_save(&self, params: DidSaveTextDocumentParams) {
        self.notify(LspNotification::DidSaveTextDocument(params));
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        self.notify(LspNotification::DidCloseTextDocument(params));
    }
}

pub async fn start_lsp<I, O>(read: I, write: O)
where
    I: AsyncRead + Unpin,
    O: AsyncWrite,
{
    log::trace!("Starting LSP");

    let (service, socket) = new_lsp();
    let server = Server::new(read, write, socket);
    server.serve(service).await;

    log::trace!("LSP exiting gracefully.",);
}

fn new_lsp() -> (LspService<Backend>, ClientSocket) {
    log::trace!("Starting LSP");

    let init = |client: Client| {
        let state = GlobalState::new(client);
        let events_tx = state.events_tx();

        // Start main loop and hold onto the handle that keeps it alive
        let main_loop = state.start();

        Backend {
            events_tx,
            _main_loop: main_loop,
        }
    };

    LspService::new(init)
}

fn new_jsonrpc_error(message: String) -> jsonrpc::Error {
    jsonrpc::Error {
        code: jsonrpc::ErrorCode::ServerError(-1),
        message: message.into(),
        data: None,
    }
}

#[cfg(test)]
mod tests {
    use futures::{executor::block_on, StreamExt};
    use tokio::io::{ReadHalf, SimplexStream, WriteHalf};
    use tower_lsp::lsp_types;

    use super::*;
    use crate::tests::codec::LanguageServerCodec;
    use crate::tests::request::Request;

    use futures_util::sink::SinkExt;
    use tokio_util::codec::{FramedRead, FramedWrite};

    struct TestClient {
        pub rx: FramedRead<ReadHalf<SimplexStream>, LanguageServerCodec<jsonrpc::Response>>,
        pub tx: FramedWrite<WriteHalf<SimplexStream>, LanguageServerCodec<Request>>,

        server_handle: Option<tokio::task::JoinHandle<()>>,
        id_counter: i64,
    }

    impl TestClient {
        pub fn new() -> Self {
            let (client_rx, mut client_tx) = tokio::io::simplex(1024);
            let (mut server_rx, server_tx) = tokio::io::simplex(1024);

            let server_handle =
                tokio::spawn(async move { start_lsp(&mut server_rx, &mut client_tx).await });

            let rx = FramedRead::new(client_rx, LanguageServerCodec::default());
            let tx = FramedWrite::new(server_tx, LanguageServerCodec::default());

            Self {
                rx,
                tx,
                server_handle: Some(server_handle),
                id_counter: 0,
            }
        }

        // `jsonrpc::Id` requires i64 IDs
        fn id(&mut self) -> i64 {
            let id = self.id_counter;
            self.id_counter = id + 1;
            id
        }

        pub async fn recv_response(&mut self) -> jsonrpc::Response {
            // Unwrap: Option (None if stream closed), then Result (Err if codec fails).
            self.rx.next().await.unwrap().unwrap()
        }

        pub async fn request<R>(&mut self, params: R::Params) -> i64
        where
            R: lsp_types::request::Request,
        {
            let id = self.id();
            let req = Request::from_request::<R>(jsonrpc::Id::Number(id), params);

            // Unwrap: For this test client it's fine to panic if we can't send
            self.tx.send(req).await.unwrap();

            id
        }

        async fn initialize(&mut self) -> i64 {
            let params = lsp_types::InitializeParams::default();
            self.request::<lsp_types::request::Initialize>(params).await
        }

        async fn shutdown(&mut self) -> i64 {
            self.request::<lsp_types::request::Shutdown>(()).await
        }
    }

    impl Drop for TestClient {
        fn drop(&mut self) {
            // TODO: Check that no messages are pending

            // Unwrap: We drop only once, so handle must be Some
            let _handle = std::mem::take(&mut self.server_handle).unwrap();

            block_on(async {
                self.shutdown().await;

                // TODO: Implement Shutdown
                // Unwrap: Panics if task can't shut down as expected
                // handle.await.unwrap();
            })
        }
    }

    #[tokio::test]
    async fn test_init() {
        let mut client = TestClient::new();

        client.initialize().await;

        let value = client.recv_response().await;
        println!("{value:?}");
    }
}
