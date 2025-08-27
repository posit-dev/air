//
// tower_lsp.rs
//
// Copyright (C) 2022-2024 Posit Software, PBC. All rights reserved.
//
//

#![allow(deprecated)]

use strum::IntoStaticStr;

use tokio::io::{AsyncRead, AsyncWrite};
use tokio::sync::mpsc::unbounded_channel as tokio_unbounded_channel;
use tower_lsp::Client;
use tower_lsp::LanguageServer;
use tower_lsp::LspService;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{ClientSocket, jsonrpc};

use crate::handlers_ext::ViewFileParams;
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

#[allow(clippy::large_enum_variant)]
#[derive(Debug)]
pub(crate) enum LspMessage {
    Notification(LspNotification),
    Request(
        LspRequest,
        TokioUnboundedSender<anyhow::Result<LspResponse>>,
    ),
}

#[derive(Debug, IntoStaticStr)]
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

#[allow(clippy::large_enum_variant)]
#[derive(Debug, IntoStaticStr)]
pub(crate) enum LspRequest {
    Initialize(InitializeParams),
    DocumentFormatting(DocumentFormattingParams),
    Shutdown,
    DocumentRangeFormatting(DocumentRangeFormattingParams),
    AirViewFile(ViewFileParams),
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, IntoStaticStr)]
pub(crate) enum LspResponse {
    Initialize(InitializeResult),
    DocumentFormatting(Option<Vec<TextEdit>>),
    DocumentRangeFormatting(Option<Vec<TextEdit>>),
    Shutdown(()),
    AirViewFile(String),
}

impl std::fmt::Display for LspNotification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.into())
    }
}
impl std::fmt::Display for LspRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.into())
    }
}
impl std::fmt::Display for LspResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.into())
    }
}

impl LspNotification {
    fn trace(&self) -> TraceLspNotification<'_> {
        TraceLspNotification { inner: self }
    }
}
impl LspRequest {
    fn trace(&self) -> TraceLspRequest<'_> {
        TraceLspRequest { inner: self }
    }
}
impl LspResponse {
    fn trace(&self) -> TraceLspResponse<'_> {
        TraceLspResponse { inner: self }
    }
}

struct TraceLspNotification<'a> {
    inner: &'a LspNotification,
}
struct TraceLspRequest<'a> {
    inner: &'a LspRequest,
}
struct TraceLspResponse<'a> {
    inner: &'a LspResponse,
}

impl std::fmt::Debug for TraceLspNotification<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.inner {
            LspNotification::DidOpenTextDocument(params) => {
                // Ignore the document itself in trace logs
                f.debug_tuple(self.inner.into())
                    .field(&params.text_document.uri)
                    .field(&params.text_document.version)
                    .field(&params.text_document.language_id)
                    .finish()
            }
            _ => std::fmt::Debug::fmt(self.inner, f),
        }
    }
}

impl std::fmt::Debug for TraceLspRequest<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self.inner, f)
    }
}

impl std::fmt::Debug for TraceLspResponse<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self.inner, f)
    }
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
        tracing::info!("Incoming: {request}");
        tracing::trace!("Incoming (debug):\n{request:#?}", request = request.trace());

        let (response_tx, mut response_rx) =
            tokio_unbounded_channel::<anyhow::Result<LspResponse>>();

        // Relay request to main loop
        self.events_tx
            .send(Event::Lsp(LspMessage::Request(request, response_tx)))
            .unwrap();

        // Wait for response from main loop
        let response = response_rx.recv().await.unwrap()?;

        tracing::info!("Outgoing: {response}");
        tracing::trace!(
            "Outgoing (debug):\n{response:#?}",
            response = response.trace()
        );
        Ok(response)
    }

    fn notify(&self, notif: LspNotification) {
        tracing::info!("Incoming: {notif}");
        tracing::trace!("Incoming (debug):\n{notif:#?}", notif = notif.trace());

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
            self.request(LspRequest::Shutdown).await,
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

    async fn range_formatting(
        &self,
        params: DocumentRangeFormattingParams,
    ) -> Result<Option<Vec<TextEdit>>> {
        cast_response!(
            self.request(LspRequest::DocumentRangeFormatting(params))
                .await,
            LspResponse::DocumentRangeFormatting
        )
    }
}

pub async fn start_lsp<I, O>(read: I, write: O)
where
    I: AsyncRead + Unpin,
    O: AsyncWrite,
{
    let (service, socket) = new_lsp();
    let server = tower_lsp::Server::new(read, write, socket);
    server.serve(service).await;
}

fn new_lsp() -> (LspService<Backend>, ClientSocket) {
    let init = |client: Client| {
        let (state, events_tx) = GlobalState::new(client);

        // Start main loop and hold onto the handle that keeps it alive
        let main_loop = state.start();

        Backend {
            events_tx,
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
