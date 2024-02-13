//
// backend.rs
//
// Copyright (C) 2022-2024 Posit Software, PBC. All rights reserved.
//
//

#![allow(deprecated)]

use std::path::Path;
use std::sync::Arc;

use crossbeam::channel::Sender;
use dashmap::DashMap;
use parking_lot::Mutex;
use serde_json::Value;
use stdext::result::ResultOrLog;
use stdext::*;
use tokio::net::TcpListener;
use tokio::runtime::Runtime;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::request::GotoImplementationParams;
use tower_lsp::lsp_types::request::GotoImplementationResponse;
use tower_lsp::lsp_types::*;
use tower_lsp::Client;
use tower_lsp::LanguageServer;
use tower_lsp::LspService;
use tower_lsp::Server;

use crate::interface::RMain;
use crate::lsp::completions::provide_completions;
use crate::lsp::completions::resolve_completion;
use crate::lsp::definitions::goto_definition;
use crate::lsp::diagnostics;
use crate::lsp::document_context::DocumentContext;
use crate::lsp::documents::Document;
use crate::lsp::encoding::convert_position_to_point;
use crate::lsp::encoding::get_position_encoding_kind;
use crate::lsp::help_topic;
use crate::lsp::hover::hover;
use crate::lsp::indexer;
use crate::lsp::indexer::IndexerStateManager;
use crate::lsp::signature_help::signature_help;
use crate::lsp::statement_range;
use crate::lsp::symbols;
use crate::r_task;

#[macro_export]
macro_rules! backend_trace {

    ($self: expr, $($rest: expr),*) => {{
        let message = format!($($rest, )*);
        $self.client.log_message(tower_lsp::lsp_types::MessageType::INFO, message).await
    }};

}

#[derive(Debug)]
pub struct Workspace {
    pub folders: Vec<Url>,
}

impl Default for Workspace {
    fn default() -> Self {
        Self {
            folders: Default::default(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Backend {
    pub client: Client,
    pub documents: Arc<DashMap<Url, Document>>,
    pub workspace: Arc<Mutex<Workspace>>,
    pub indexer_state_manager: IndexerStateManager,
}

impl Backend {
    pub fn with_document<T, F>(&self, path: &Path, mut callback: F) -> anyhow::Result<T>
    where
        F: FnMut(&Document) -> anyhow::Result<T>,
    {
        let mut fallback = || {
            let contents = std::fs::read_to_string(path)?;
            let document = Document::new(contents.as_str(), None);
            return callback(&document);
        };

        // If we have a cached copy of the document (because we're monitoring it)
        // then use that; otherwise, try to read the document from the provided
        // path and use that instead.
        let uri = unwrap!(Url::from_file_path(path), Err(_) => {
            log::info!("couldn't construct uri from {}; reading from disk instead", path.display());
            return fallback();
        });

        let document = unwrap!(self.documents.get(&uri), None => {
            log::info!("no document for uri {}; reading from disk instead", uri);
            return fallback();
        });

        return callback(document.value());
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, params: InitializeParams) -> Result<InitializeResult> {
        backend_trace!(self, "initialize({:#?})", params);

        // initialize the set of known workspaces
        let mut workspace = self.workspace.lock();

        // initialize the workspace folders
        let mut folders: Vec<String> = Vec::new();
        if let Some(workspace_folders) = params.workspace_folders {
            for folder in workspace_folders.iter() {
                workspace.folders.push(folder.uri.clone());
                if let Ok(path) = folder.uri.to_file_path() {
                    if let Some(path) = path.to_str() {
                        folders.push(path.to_string());
                    }
                }
            }
        }

        // start indexing
        indexer::start(folders, self.indexer_state_manager.clone());

        Ok(InitializeResult {
            server_info: Some(ServerInfo {
                name: "Amalthea R Kernel (ARK)".to_string(),
                version: Some(env!("CARGO_PKG_VERSION").to_string()),
            }),
            capabilities: ServerCapabilities {
                position_encoding: Some(get_position_encoding_kind()),
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::INCREMENTAL,
                )),
                selection_range_provider: None,
                hover_provider: Some(HoverProviderCapability::from(true)),
                completion_provider: Some(CompletionOptions {
                    resolve_provider: Some(true),
                    trigger_characters: Some(vec![
                        "$".to_string(),
                        "@".to_string(),
                        ":".to_string(),
                    ]),
                    work_done_progress_options: Default::default(),
                    all_commit_characters: None,
                    ..Default::default()
                }),
                signature_help_provider: Some(SignatureHelpOptions {
                    trigger_characters: Some(vec![
                        "(".to_string(),
                        ",".to_string(),
                        "=".to_string(),
                    ]),
                    retrigger_characters: None,
                    work_done_progress_options: WorkDoneProgressOptions {
                        work_done_progress: None,
                    },
                }),
                definition_provider: Some(OneOf::Left(true)),
                type_definition_provider: None,
                implementation_provider: Some(ImplementationProviderCapability::Simple(true)),
                references_provider: Some(OneOf::Left(true)),
                document_symbol_provider: Some(OneOf::Left(true)),
                workspace_symbol_provider: Some(OneOf::Left(true)),
                execute_command_provider: Some(ExecuteCommandOptions {
                    commands: vec![],
                    work_done_progress_options: Default::default(),
                }),
                workspace: Some(WorkspaceServerCapabilities {
                    workspace_folders: Some(WorkspaceFoldersServerCapabilities {
                        supported: Some(true),
                        change_notifications: Some(OneOf::Left(true)),
                    }),
                    file_operations: None,
                }),
                ..ServerCapabilities::default()
            },
        })
    }

    async fn initialized(&self, params: InitializedParams) {
        backend_trace!(self, "initialized({:?})", params);
    }

    async fn shutdown(&self) -> Result<()> {
        backend_trace!(self, "shutdown()");
        Ok(())
    }

    async fn did_change_workspace_folders(&self, params: DidChangeWorkspaceFoldersParams) {
        backend_trace!(self, "did_change_workspace_folders({:?})", params);

        // TODO: Re-start indexer with new folders.
    }

    async fn did_change_configuration(&self, params: DidChangeConfigurationParams) {
        backend_trace!(self, "did_change_configuration({:?})", params);
    }

    async fn did_change_watched_files(&self, params: DidChangeWatchedFilesParams) {
        backend_trace!(self, "did_change_watched_files({:?})", params);

        // TODO: Re-index the changed files.
    }

    async fn symbol(
        &self,
        params: WorkspaceSymbolParams,
    ) -> Result<Option<Vec<SymbolInformation>>> {
        backend_trace!(self, "symbol({:?})", params);

        let response = unwrap!(symbols::symbols(self, &params), Err(error) => {
            log::error!("{:?}", error);
            return Ok(None);
        });

        Ok(Some(response))
    }

    async fn document_symbol(
        &self,
        params: DocumentSymbolParams,
    ) -> Result<Option<DocumentSymbolResponse>> {
        backend_trace!(self, "document_symbols({})", params.text_document.uri);

        let response = unwrap!(symbols::document_symbols(self, &params), Err(error) => {
            log::error!("{:?}", error);
            return Ok(None);
        });

        Ok(Some(DocumentSymbolResponse::Nested(response)))
    }

    async fn execute_command(&self, params: ExecuteCommandParams) -> Result<Option<Value>> {
        backend_trace!(self, "execute_command({:?})", params);

        match self.client.apply_edit(WorkspaceEdit::default()).await {
            Ok(res) if res.applied => self.client.log_message(MessageType::INFO, "applied").await,
            Ok(_) => self.client.log_message(MessageType::INFO, "rejected").await,
            Err(err) => self.client.log_message(MessageType::ERROR, err).await,
        }

        Ok(None)
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        backend_trace!(self, "did_open({}", params.text_document.uri);

        let contents = params.text_document.text.as_str();
        let uri = params.text_document.uri;
        let version = params.text_document.version;

        self.documents
            .insert(uri.clone(), Document::new(contents, Some(version)));

        diagnostics::refresh_diagnostics(self.clone(), uri.clone(), Some(version));
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        backend_trace!(self, "did_change({:?})", params);

        // get reference to document
        let uri = &params.text_document.uri;
        let mut doc = unwrap!(self.documents.get_mut(uri), None => {
            backend_trace!(self, "did_change(): unexpected document uri '{}'", uri);
            return;
        });

        // respond to document updates
        let version = unwrap!(doc.on_did_change(&params), Err(error) => {
            backend_trace!(
                self,
                "did_change(): unexpected error applying updates {}",
                error
            );
            return;
        });

        // update index
        if let Ok(path) = uri.to_file_path() {
            let path = Path::new(&path);
            if let Err(error) = indexer::update(&doc, &path) {
                log::error!("{:?}", error);
            }
        }

        // publish diagnostics - but only publish them if the version of
        // the document now matches the version of the change after applying
        // it in `on_did_change()` (i.e. no changes left in the out of order queue)
        if params.text_document.version == version {
            diagnostics::refresh_diagnostics(self.clone(), uri.clone(), Some(version));
        }
    }

    async fn did_save(&self, params: DidSaveTextDocumentParams) {
        backend_trace!(self, "did_save({:?}", params);
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        backend_trace!(self, "did_close({:?}", params);

        let uri = params.text_document.uri;

        diagnostics::clear_diagnostics(self.clone(), uri.clone(), None);

        match self.documents.remove(&uri) {
            Some(_) => {
                backend_trace!(self, "did_close(): closed document with URI: '{uri}'.");
            },
            None => {
                backend_trace!(
                    self,
                    "did_close(): failed to remove document with unknown URI: '{uri}'."
                );
            },
        };
    }

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        backend_trace!(self, "completion({:?})", params);

        // Get reference to document.
        let uri = &params.text_document_position.text_document.uri;
        let document = unwrap!(self.documents.get_mut(uri), None => {
            backend_trace!(self, "completion(): No document associated with URI {}", uri);
            return Ok(None);
        });

        let position = params.text_document_position.position;
        let point = convert_position_to_point(&document.contents, position);

        let trigger = params.context.and_then(|ctxt| ctxt.trigger_character);

        // Build the document context.
        let context = DocumentContext::new(&document, point, trigger);
        log::info!("Completion context: {:#?}", context);

        let completions = r_task(|| provide_completions(&self, &context));

        let completions = unwrap!(completions, Err(err) => {
            backend_trace!(self, "completion(): Failed to provide completions: {err:?}.");
            return Ok(None)
        });

        if !completions.is_empty() {
            Ok(Some(CompletionResponse::Array(completions)))
        } else {
            Ok(None)
        }
    }

    async fn completion_resolve(&self, mut item: CompletionItem) -> Result<CompletionItem> {
        backend_trace!(self, "completion_resolve({:?})", item);

        // Try resolving the completion item
        let result = r_task(|| unsafe { resolve_completion(&mut item) });

        // Handle error case
        if let Err(err) = result {
            log::error!("Failed to resolve completion item due to: {err:?}.");
        }

        Ok(item)
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        backend_trace!(self, "hover({:?})", params);

        // get document reference
        let uri = &params.text_document_position_params.text_document.uri;
        let document = unwrap!(self.documents.get_mut(uri), None => {
            backend_trace!(self, "hover(): No document associated with URI {}", uri);
            return Ok(None);
        });

        let position = params.text_document_position_params.position;
        let point = convert_position_to_point(&document.contents, position);

        // build document context
        let context = DocumentContext::new(&document, point, None);

        // request hover information
        let result = r_task(|| unsafe { hover(&context) });

        // unwrap errors
        let result = unwrap!(result, Err(error) => {
            log::error!("{:?}", error);
            return Ok(None);
        });

        // unwrap empty options
        let result = unwrap!(result, None => {
            return Ok(None);
        });

        // we got a result; use it
        Ok(Some(Hover {
            contents: HoverContents::Markup(result),
            range: None,
        }))
    }

    async fn signature_help(&self, params: SignatureHelpParams) -> Result<Option<SignatureHelp>> {
        // get document reference
        let uri = &params.text_document_position_params.text_document.uri;
        let document = unwrap!(self.documents.get_mut(uri), None => {
            backend_trace!(self, "signature_help(): No document associated with URI {}", uri);
            return Ok(None);
        });

        let position = params.text_document_position_params.position;
        let point = convert_position_to_point(&document.contents, position);

        let context = DocumentContext::new(&document, point, None);

        // request signature help
        let result = r_task(|| unsafe { signature_help(&context) });

        // unwrap errors
        let result = unwrap!(result, Err(error) => {
            log::error!("{:?}", error);
            return Ok(None);
        });

        // unwrap empty options
        let result = unwrap!(result, None => {
            return Ok(None);
        });

        Ok(Some(result))
    }

    async fn goto_definition(
        &self,
        params: GotoDefinitionParams,
    ) -> Result<Option<GotoDefinitionResponse>> {
        backend_trace!(self, "goto_definition({:?})", params);

        // get reference to document
        let uri = &params.text_document_position_params.text_document.uri;
        let document = unwrap!(self.documents.get(uri), None => {
            backend_trace!(self, "completion(): No document associated with URI {}", uri);
            return Ok(None);
        });

        // build goto definition context
        let result = unwrap!(unsafe { goto_definition(&document, params) }, Err(error) => {
            log::error!("{}", error);
            return Ok(None);
        });

        Ok(result)
    }

    async fn goto_implementation(
        &self,
        params: GotoImplementationParams,
    ) -> Result<Option<GotoImplementationResponse>> {
        backend_trace!(self, "goto_implementation({:?})", params);
        let _ = params;
        log::error!("Got a textDocument/implementation request, but it is not implemented");
        return Ok(None);
    }

    async fn references(&self, params: ReferenceParams) -> Result<Option<Vec<Location>>> {
        backend_trace!(self, "references({:?})", params);

        let locations = match self.find_references(params) {
            Ok(locations) => locations,
            Err(_error) => {
                return Ok(None);
            },
        };

        if locations.is_empty() {
            Ok(None)
        } else {
            Ok(Some(locations))
        }
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
            // Create backend.
            // Note that DashMap uses synchronization primitives internally, so we
            // don't guard access to the map via a mutex.
            let backend = Backend {
                client,
                documents: Arc::new(DashMap::new()),
                workspace: Arc::new(Mutex::new(Workspace::default())),
                indexer_state_manager: IndexerStateManager::new(),
            };

            // Forward `backend` along to `RMain`.
            // This also updates an outdated `backend` after a reconnect.
            // `RMain` should be initialized by now, since the caller of this
            // function waits to receive the init notification sent on
            // `kernel_init_rx`. Even if it isn't, this should be okay because
            // `r_task()` defensively blocks until its sender is initialized.
            r_task({
                let backend = backend.clone();
                move || {
                    let main = RMain::get_mut();
                    main.set_lsp_backend(backend);
                }
            });

            backend
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
