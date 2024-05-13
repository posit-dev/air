//
// backend.rs
//
// Copyright (C) 2022-2024 Posit Software, PBC. All rights reserved.
//
//

#![allow(deprecated)]

use std::sync::Arc;

use crossbeam::channel::Sender;
use serde_json::Value;
use stdext::result::ResultOrLog;
use tokio::net::TcpListener;
use tokio::runtime::Runtime;
use tokio::sync::mpsc::unbounded_channel as tokio_unbounded_channel;
use tower_lsp::jsonrpc;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::request::GotoImplementationParams;
use tower_lsp::lsp_types::request::GotoImplementationResponse;
use tower_lsp::lsp_types::SelectionRange;
use tower_lsp::lsp_types::*;
use tower_lsp::Client;
use tower_lsp::LanguageServer;
use tower_lsp::LspService;
use tower_lsp::Server;

use crate::interface::RMain;
use crate::lsp::help_topic;
use crate::lsp::help_topic::HelpTopicParams;
use crate::lsp::help_topic::HelpTopicResponse;
use crate::lsp::main_loop::Event;
use crate::lsp::main_loop::GlobalState;
use crate::lsp::main_loop::TokioUnboundedSender;
use crate::lsp::statement_range;
use crate::lsp::statement_range::StatementRangeParams;
use crate::lsp::statement_range::StatementRangeResponse;
use crate::r_task;

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
    WorkspaceSymbol(WorkspaceSymbolParams),
    DocumentSymbol(DocumentSymbolParams),
    ExecuteCommand(ExecuteCommandParams),
    Completion(CompletionParams),
    CompletionResolve(CompletionItem),
    Hover(HoverParams),
    SignatureHelp(SignatureHelpParams),
    GotoDefinition(GotoDefinitionParams),
    GotoImplementation(GotoImplementationParams),
    SelectionRange(SelectionRangeParams),
    References(ReferenceParams),
    StatementRange(StatementRangeParams),
    HelpTopic(HelpTopicParams),
}

#[derive(Debug)]
pub(crate) enum LspResponse {
    Initialize(InitializeResult),
    Shutdown(()),
    WorkspaceSymbol(Option<Vec<SymbolInformation>>),
    DocumentSymbol(Option<DocumentSymbolResponse>),
    ExecuteCommand(Option<Value>),
    Completion(Option<CompletionResponse>),
    CompletionResolve(CompletionItem),
    Hover(Option<Hover>),
    SignatureHelp(Option<SignatureHelp>),
    GotoDefinition(Option<GotoDefinitionResponse>),
    GotoImplementation(Option<GotoImplementationResponse>),
    SelectionRange(Option<Vec<SelectionRange>>),
    References(Option<Vec<Location>>),
    StatementRange(Option<StatementRangeResponse>),
    HelpTopic(Option<HelpTopicResponse>),
}

#[derive(Debug)]
pub struct Backend {
    /// LSP client, use this for direct interaction with the client.
    pub client: Client,

    /// Channel for communication with the main loop.
    events_tx: TokioUnboundedSender<Event>,
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

    async fn symbol(
        &self,
        params: WorkspaceSymbolParams,
    ) -> Result<Option<Vec<SymbolInformation>>> {
        cast_response!(
            self.request(LspRequest::WorkspaceSymbol(params)).await,
            LspResponse::WorkspaceSymbol
        )
    }

    async fn document_symbol(
        &self,
        params: DocumentSymbolParams,
    ) -> Result<Option<DocumentSymbolResponse>> {
        cast_response!(
            self.request(LspRequest::DocumentSymbol(params)).await,
            LspResponse::DocumentSymbol
        )
    }

    async fn execute_command(&self, params: ExecuteCommandParams) -> Result<Option<Value>> {
        cast_response!(
            self.request(LspRequest::ExecuteCommand(params)).await,
            LspResponse::ExecuteCommand
        )
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

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        cast_response!(
            self.request(LspRequest::Completion(params)).await,
            LspResponse::Completion
        )
    }

    async fn completion_resolve(&self, item: CompletionItem) -> Result<CompletionItem> {
        cast_response!(
            self.request(LspRequest::CompletionResolve(item)).await,
            LspResponse::CompletionResolve
        )
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        cast_response!(
            self.request(LspRequest::Hover(params)).await,
            LspResponse::Hover
        )
    }

    async fn signature_help(&self, params: SignatureHelpParams) -> Result<Option<SignatureHelp>> {
        cast_response!(
            self.request(LspRequest::SignatureHelp(params)).await,
            LspResponse::SignatureHelp
        )
    }

    async fn goto_definition(
        &self,
        params: GotoDefinitionParams,
    ) -> Result<Option<GotoDefinitionResponse>> {
        cast_response!(
            self.request(LspRequest::GotoDefinition(params)).await,
            LspResponse::GotoDefinition
        )
    }

    async fn goto_implementation(
        &self,
        params: GotoImplementationParams,
    ) -> Result<Option<GotoImplementationResponse>> {
        cast_response!(
            self.request(LspRequest::GotoImplementation(params)).await,
            LspResponse::GotoImplementation
        )
    }

    async fn selection_range(
        &self,
        params: SelectionRangeParams,
    ) -> Result<Option<Vec<SelectionRange>>> {
        cast_response!(
            self.request(LspRequest::SelectionRange(params)).await,
            LspResponse::SelectionRange
        )
    }

    async fn references(&self, params: ReferenceParams) -> Result<Option<Vec<Location>>> {
        cast_response!(
            self.request(LspRequest::References(params)).await,
            LspResponse::References
        )
    }
}

// Custom methods for the backend.
//
// NOTE: Request / notification methods _must_ accept a params object,
// even for notifications that don't include any auxiliary data.
//
// I'm not positive, but I think this is related to the way VSCode
// serializes parameters for notifications / requests when no data
// is supplied. Instead of supplying "nothing", it supplies something
// like `[null]` which tower_lsp seems to quietly reject when attempting
// to invoke the registered method.
//
// See also:
//
// https://github.com/Microsoft/vscode-languageserver-node/blob/18fad46b0e8085bb72e1b76f9ea23a379569231a/client/src/common/client.ts#L802-L838
// https://github.com/Microsoft/vscode-languageserver-node/blob/18fad46b0e8085bb72e1b76f9ea23a379569231a/client/src/common/client.ts#L701-L752
impl Backend {
    async fn statement_range(
        &self,
        params: StatementRangeParams,
    ) -> jsonrpc::Result<Option<StatementRangeResponse>> {
        cast_response!(
            self.request(LspRequest::StatementRange(params)).await,
            LspResponse::StatementRange
        )
    }

    async fn help_topic(
        &self,
        params: HelpTopicParams,
    ) -> jsonrpc::Result<Option<HelpTopicResponse>> {
        cast_response!(
            self.request(LspRequest::HelpTopic(params)).await,
            LspResponse::HelpTopic
        )
    }

    async fn notification(&self, params: Option<Value>) {
        log::info!("Received Positron notification: {:?}", params);
    }
}

pub fn start_lsp(runtime: Arc<Runtime>, address: String, conn_init_tx: Sender<bool>) {
    runtime.block_on(async {
        #[cfg(feature = "runtime-agnostic")]
        use tokio_util::compat::TokioAsyncReadCompatExt;
        #[cfg(feature = "runtime-agnostic")]
        use tokio_util::compat::TokioAsyncWriteCompatExt;

        log::trace!("Connecting to LSP at '{}'", &address);
        let listener = TcpListener::bind(&address).await.unwrap();

        // Notify frontend that we are ready to accept connections
        conn_init_tx
            .send(true)
            .or_log_warning("Couldn't send LSP server init notification");

        let (stream, _) = listener.accept().await.unwrap();
        log::trace!("Connected to LSP at '{}'", address);
        let (read, write) = tokio::io::split(stream);

        #[cfg(feature = "runtime-agnostic")]
        let (read, write) = (read.compat(), write.compat_write());

        let init = |client: Client| {
            let state = GlobalState::new(client.clone());
            let events_tx = state.events_tx();

            state.main_loop();

            // Forward event channel along to `RMain`.
            // This also updates an outdated channel after a reconnect.
            // `RMain` should be initialized by now, since the caller of this
            // function waits to receive the init notification sent on
            // `kernel_init_rx`. Even if it isn't, this should be okay because
            // `r_task()` defensively blocks until its sender is initialized.
            r_task({
                let events_tx = events_tx.clone();
                move || {
                    let main = RMain::get_mut();
                    main.set_lsp_channel(events_tx);
                }
            });

            Backend { client, events_tx }
        };

        let (service, socket) = LspService::build(init)
            .custom_method(
                statement_range::POSITRON_STATEMENT_RANGE_REQUEST,
                Backend::statement_range,
            )
            .custom_method(help_topic::POSITRON_HELP_TOPIC_REQUEST, Backend::help_topic)
            .custom_method("positron/notification", Backend::notification)
            .finish();

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
        message,
        data: None,
    }
}
