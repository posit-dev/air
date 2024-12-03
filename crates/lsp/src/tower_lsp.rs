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
use tower_lsp::{jsonrpc, ClientSocket};

use crate::handlers_ext::ViewFileParams;
use crate::main_loop::AuxiliaryEventSender;
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
    DocumentFormatting(DocumentFormattingParams),
    Shutdown(),
    AirViewFile(ViewFileParams),
}

#[derive(Debug)]
pub(crate) enum LspResponse {
    Initialize(InitializeResult),
    DocumentFormatting(Option<Vec<TextEdit>>),
    Shutdown(()),
    AirViewFile(String),
}

#[derive(Debug)]
struct Backend {
    /// Channel for communication with the main loop.
    events_tx: TokioUnboundedSender<Event>,

    /// Channel for communication with the auxiliary loop.
    /// Used for latency sensitive logging.
    auxiliary_event_tx: AuxiliaryEventSender,

    /// Handle to main loop. Drop it to cancel the loop, all associated tasks,
    /// and drop all owned state.
    _main_loop: tokio::task::JoinSet<()>,
}

impl Backend {
    async fn request(&self, request: LspRequest) -> anyhow::Result<LspResponse> {
        self.log_info(format!("Incoming: {request:#?}"));

        let (response_tx, mut response_rx) =
            tokio_unbounded_channel::<anyhow::Result<LspResponse>>();

        // Relay request to main loop
        self.events_tx
            .send(Event::Lsp(LspMessage::Request(request, response_tx)))
            .unwrap();

        // Wait for response from main loop
        let out = response_rx.recv().await.unwrap()?;

        self.log_info(format!("Outgoing {out:#?}"));
        Ok(out)
    }

    fn notify(&self, notif: LspNotification) {
        self.log_info(format!("Incoming: {notif:#?}"));

        // Relay notification to main loop
        self.events_tx
            .send(Event::Lsp(LspMessage::Notification(notif)))
            .unwrap();
    }

    async fn air_view_file(&self, params: ViewFileParams) -> tower_lsp::jsonrpc::Result<String> {
        cast_response!(
            self.request(LspRequest::AirViewFile(params)).await,
            LspResponse::AirViewFile
        )
    }

    fn log_info(&self, message: String) {
        self.auxiliary_event_tx.log_info(message);
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

    async fn formatting(&self, params: DocumentFormattingParams) -> Result<Option<Vec<TextEdit>>> {
        cast_response!(
            self.request(LspRequest::DocumentFormatting(params)).await,
            LspResponse::DocumentFormatting
        )
    }
}

pub async fn start_lsp<I, O>(read: I, write: O)
where
    I: AsyncRead + Unpin,
    O: AsyncWrite,
{
    log::trace!("Starting LSP");

    let (service, socket) = new_lsp();
    let server = tower_lsp::Server::new(read, write, socket);
    server.serve(service).await;

    log::trace!("LSP exiting gracefully.",);
}

fn new_lsp() -> (LspService<Backend>, ClientSocket) {
    let init = |client: Client| {
        let state = GlobalState::new(client);
        let events_tx = state.events_tx();
        let auxiliary_event_tx = state.auxiliary_event_tx();

        // Start main loop and hold onto the handle that keeps it alive
        let main_loop = state.start();

        Backend {
            events_tx,
            auxiliary_event_tx,
            _main_loop: main_loop,
        }
    };

    LspService::build(init)
        .custom_method("air/viewFile", Backend::air_view_file)
        .finish()
}

fn new_jsonrpc_error(message: String) -> jsonrpc::Error {
    jsonrpc::Error {
        code: jsonrpc::ErrorCode::ServerError(-1),
        message: message.into(),
        data: None,
    }
}

#[cfg(test)]
pub(crate) async fn start_test_client() -> lsp_test::lsp_client::TestClient {
    lsp_test::lsp_client::TestClient::new(|server_rx, client_tx| async {
        start_lsp(server_rx, client_tx).await
    })
}

#[cfg(test)]
pub(crate) async fn init_test_client() -> lsp_test::lsp_client::TestClient {
    let mut client = start_test_client().await;

    client.initialize().await;
    client.recv_response().await;

    client
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_matches::assert_matches;
    use tower_lsp::lsp_types;

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
}
