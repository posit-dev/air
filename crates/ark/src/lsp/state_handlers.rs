//
// state_handlers.rs
//
// Copyright (C) 2024 Posit Software, PBC. All rights reserved.
//
//

use std::path::Path;

use anyhow::anyhow;
use tower_lsp::lsp_types::CompletionOptions;
use tower_lsp::lsp_types::DidChangeTextDocumentParams;
use tower_lsp::lsp_types::DidCloseTextDocumentParams;
use tower_lsp::lsp_types::DidOpenTextDocumentParams;
use tower_lsp::lsp_types::ExecuteCommandOptions;
use tower_lsp::lsp_types::HoverProviderCapability;
use tower_lsp::lsp_types::ImplementationProviderCapability;
use tower_lsp::lsp_types::InitializeParams;
use tower_lsp::lsp_types::InitializeResult;
use tower_lsp::lsp_types::OneOf;
use tower_lsp::lsp_types::SelectionRangeProviderCapability;
use tower_lsp::lsp_types::ServerCapabilities;
use tower_lsp::lsp_types::ServerInfo;
use tower_lsp::lsp_types::SignatureHelpOptions;
use tower_lsp::lsp_types::TextDocumentSyncCapability;
use tower_lsp::lsp_types::TextDocumentSyncKind;
use tower_lsp::lsp_types::WorkDoneProgressOptions;
use tower_lsp::lsp_types::WorkspaceFoldersServerCapabilities;
use tower_lsp::lsp_types::WorkspaceServerCapabilities;

use crate::lsp;
use crate::lsp::documents::Document;
use crate::lsp::encoding::get_position_encoding_kind;
use crate::lsp::indexer;
use crate::lsp::state::WorldState;

// Handlers that mutate the world state

/// Information sent from the kernel to the LSP after each top-level evaluation.
#[derive(Debug)]
pub struct ConsoleInputs {
    /// List of console scopes, from innermost (global or debug) to outermost
    /// scope. Currently the scopes are vectors of symbol names. TODO: In the
    /// future, we should send structural information like search path, and let
    /// the LSP query us for the contents so that the LSP can cache the
    /// information.
    pub console_scopes: Vec<Vec<String>>,

    /// Packages currently installed in the library path. TODO: Should send
    /// library paths instead and inspect and cache package information in the LSP.
    pub installed_packages: Vec<String>,
}

// Handlers taking exclusive references to global state

pub(crate) fn initialize(
    params: InitializeParams,
    state: &mut WorldState,
) -> anyhow::Result<InitializeResult> {
    // Initialize the workspace folders
    let mut folders: Vec<String> = Vec::new();
    if let Some(workspace_folders) = params.workspace_folders {
        for folder in workspace_folders.iter() {
            state.workspace.folders.push(folder.uri.clone());
            if let Ok(path) = folder.uri.to_file_path() {
                if let Some(path) = path.to_str() {
                    folders.push(path.to_string());
                }
            }
        }
    }

    // Start indexing
    lsp::spawn_blocking(|| {
        indexer::start(folders);
        Ok(None)
    });

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
            selection_range_provider: Some(SelectionRangeProviderCapability::Simple(true)),
            hover_provider: Some(HoverProviderCapability::from(true)),
            completion_provider: Some(CompletionOptions {
                resolve_provider: Some(true),
                trigger_characters: Some(vec!["$".to_string(), "@".to_string(), ":".to_string()]),
                work_done_progress_options: Default::default(),
                all_commit_characters: None,
                ..Default::default()
            }),
            signature_help_provider: Some(SignatureHelpOptions {
                trigger_characters: Some(vec!["(".to_string(), ",".to_string(), "=".to_string()]),
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

pub(crate) fn did_open(
    params: DidOpenTextDocumentParams,
    state: &mut WorldState,
) -> anyhow::Result<()> {
    let contents = params.text_document.text.as_str();
    let uri = params.text_document.uri;
    let version = params.text_document.version;

    let document = Document::new(contents, Some(version));
    state.documents.insert(uri.clone(), document.clone());

    lsp::spawn_diagnostics_refresh(uri, document, state.clone());

    Ok(())
}

pub(crate) fn did_change(
    params: DidChangeTextDocumentParams,
    state: &mut WorldState,
) -> anyhow::Result<()> {
    let uri = &params.text_document.uri;
    let doc = state.get_document_mut(uri)?;

    // Respond to document updates
    doc.on_did_change(&params);

    // Update index
    if let Ok(path) = uri.to_file_path() {
        let path = Path::new(&path);
        if let Err(err) = indexer::update(&doc, &path) {
            lsp::log_error!("{err:?}");
        }
    }

    // Refresh diagnostics
    lsp::spawn_diagnostics_refresh(uri.clone(), doc.clone(), state.clone());

    Ok(())
}

pub(crate) fn did_close(
    params: DidCloseTextDocumentParams,
    state: &mut WorldState,
) -> anyhow::Result<()> {
    let uri = params.text_document.uri;

    // Publish empty set of diagnostics to clear them
    lsp::publish_diagnostics(uri.clone(), Vec::new(), None);

    state
        .documents
        .remove(&uri)
        .ok_or(anyhow!("Failed to remove document for URI: {uri}"))?;

    lsp::log_info!("did_close(): closed document with URI: '{uri}'.");

    Ok(())
}

pub(crate) fn did_change_console_inputs(
    inputs: ConsoleInputs,
    state: &mut WorldState,
) -> anyhow::Result<()> {
    state.console_scopes = inputs.console_scopes;
    state.installed_packages = inputs.installed_packages;

    // We currently rely on global console scopes for diagnostics, in particular
    // during package development in conjunction with `devtools::load_all()`.
    // Ideally diagnostics would not rely on these though, and we wouldn't need
    // to refresh from here.
    lsp::spawn_diagnostics_refresh_all(state.clone());

    Ok(())
}
