//
// tower_lsp.rs
//
// Copyright (C) 2022-2024 Posit Software, PBC. All rights reserved.
//
//

#![allow(deprecated)]

use std::sync::Arc;

use crossbeam::channel::Sender;
use tokio::net::TcpListener;
use tokio::runtime::Runtime;
use tokio::sync::mpsc::unbounded_channel as tokio_unbounded_channel;
use tower_lsp::jsonrpc;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::Client;
use tower_lsp::LanguageServer;
use tower_lsp::LspService;
use tower_lsp::Server;

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

pub fn start_lsp(runtime: Arc<Runtime>, address: String, conn_init_tx: Sender<bool>) {
    runtime.block_on(async {
        log::trace!("Connecting to LSP at '{}'", &address);
        let listener = TcpListener::bind(&address).await.unwrap();

        // Notify frontend that we are ready to accept connections
        if let Err(err) = conn_init_tx.send(true) {
            log::warn!("Couldn't send LSP server init notification: {err:?}");
        }

        let (stream, _) = listener.accept().await.unwrap();
        log::trace!("Connected to LSP at '{}'", address);
        let (read, write) = tokio::io::split(stream);

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

        let (service, socket) = LspService::build(init).finish();

        let server = Server::new(read, write, socket);
        server.serve(service).await;

        log::trace!(
            "LSP thread exiting gracefully after connection closed ({:?}).",
            address
        );
    })
}

fn new_jsonrpc_error(message: String) -> jsonrpc::Error {
    jsonrpc::Error {
        code: jsonrpc::ErrorCode::ServerError(-1),
        message: message.into(),
        data: None,
    }
}
