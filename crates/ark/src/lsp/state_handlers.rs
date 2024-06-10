//
// state_handlers.rs
//
// Copyright (C) 2024 Posit Software, PBC. All rights reserved.
//
//

use std::path::Path;

use anyhow::anyhow;
use serde_json::Value;
use struct_field_names_as_array::FieldNamesAsArray;
use tower_lsp::lsp_types::CompletionOptions;
use tower_lsp::lsp_types::ConfigurationItem;
use tower_lsp::lsp_types::DidChangeConfigurationParams;
use tower_lsp::lsp_types::DidChangeTextDocumentParams;
use tower_lsp::lsp_types::DidCloseTextDocumentParams;
use tower_lsp::lsp_types::DidOpenTextDocumentParams;
use tower_lsp::lsp_types::DocumentOnTypeFormattingOptions;
use tower_lsp::lsp_types::ExecuteCommandOptions;
use tower_lsp::lsp_types::FormattingOptions;
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
use tracing::Instrument;
use tree_sitter::Parser;
use url::Url;

use crate::lsp;
use crate::lsp::config::indent_style_from_lsp;
use crate::lsp::config::DocumentConfig;
use crate::lsp::config::VscDiagnosticsConfig;
use crate::lsp::config::VscDocumentConfig;
use crate::lsp::diagnostics::DiagnosticsConfig;
use crate::lsp::documents::Document;
use crate::lsp::encoding::get_position_encoding_kind;
use crate::lsp::indexer;
use crate::lsp::main_loop::LspState;
use crate::lsp::state::workspace_uris;
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

#[tracing::instrument(level = "info", skip_all)]
pub(crate) fn initialize(
    params: InitializeParams,
    lsp_state: &mut LspState,
    state: &mut WorldState,
) -> anyhow::Result<InitializeResult> {
    // Take note of supported capabilities so we can register them in the
    // `Initialized` handler
    if let Some(ws_caps) = params.capabilities.workspace {
        if matches!(ws_caps.did_change_configuration, Some(caps) if matches!(caps.dynamic_registration, Some(true)))
        {
            lsp_state.needs_registration.did_change_configuration = true;
        }
    }

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

    // Start first round of indexing
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
            document_on_type_formatting_provider: Some(DocumentOnTypeFormattingOptions {
                first_trigger_character: String::from("\n"),
                more_trigger_character: None,
            }),
            ..ServerCapabilities::default()
        },
    })
}

#[tracing::instrument(level = "info", skip_all)]
pub(crate) fn did_open(
    params: DidOpenTextDocumentParams,
    lsp_state: &mut LspState,
    state: &mut WorldState,
) -> anyhow::Result<()> {
    let contents = params.text_document.text.as_str();
    let uri = params.text_document.uri;
    let version = params.text_document.version;

    let language = tree_sitter_r::language();
    let mut parser = Parser::new();
    parser.set_language(&language).unwrap();

    let document = Document::new_with_parser(contents, &mut parser, Some(version));

    lsp_state.parsers.insert(uri.clone(), parser);
    state.documents.insert(uri.clone(), document.clone());

    // NOTE: Do we need to call `update_config()` here?
    // update_config(vec![uri]).await;

    update_index(&uri, &document);
    lsp::spawn_diagnostics_refresh(uri, document, state.clone());

    Ok(())
}

#[tracing::instrument(level = "info", skip_all)]
pub(crate) fn did_change(
    params: DidChangeTextDocumentParams,
    lsp_state: &mut LspState,
    state: &mut WorldState,
) -> anyhow::Result<()> {
    let uri = &params.text_document.uri;
    let doc = state.get_document_mut(uri)?;

    let mut parser = lsp_state
        .parsers
        .get_mut(uri)
        .ok_or(anyhow!("No parser for {uri}"))?;

    doc.on_did_change(&mut parser, &params);

    update_index(uri, doc);
    lsp::spawn_diagnostics_refresh(uri.clone(), doc.clone(), state.clone());

    Ok(())
}

#[tracing::instrument(level = "info", skip_all)]
pub(crate) fn did_close(
    params: DidCloseTextDocumentParams,
    lsp_state: &mut LspState,
    state: &mut WorldState,
) -> anyhow::Result<()> {
    let uri = params.text_document.uri;

    // Publish empty set of diagnostics to clear them
    lsp::publish_diagnostics(uri.clone(), Vec::new(), None);

    state
        .documents
        .remove(&uri)
        .ok_or(anyhow!("Failed to remove document for URI: {uri}"))?;

    lsp_state
        .parsers
        .remove(&uri)
        .ok_or(anyhow!("Failed to remove parser for URI: {uri}"))?;

    lsp::log_info!("did_close(): closed document with URI: '{uri}'.");

    Ok(())
}

pub(crate) async fn did_change_configuration(
    _params: DidChangeConfigurationParams,
    client: &tower_lsp::Client,
    state: &mut WorldState,
) -> anyhow::Result<()> {
    // The notification params sometimes contain data but it seems in practice
    // we should just ignore it. Instead we need to pull the settings again for
    // all URI of interest.

    update_config(workspace_uris(state), client, state)
        .instrument(tracing::info_span!("did_change_configuration"))
        .await
}

#[tracing::instrument(level = "info", skip_all)]
pub(crate) fn did_change_formatting_options(
    uri: &Url,
    opts: &FormattingOptions,
    state: &mut WorldState,
) {
    let Ok(doc) = state.get_document_mut(uri) else {
        return;
    };

    // The information provided in formatting requests is more up-to-date
    // than the user settings because it also includes changes made to the
    // configuration of particular editors. However the former is less rich
    // than the latter: it does not allow the tab size to differ from the
    // indent size, as in the R core sources. So we just ignore the less
    // rich updates in this case.
    if doc.config.indent.indent_size != doc.config.indent.tab_width {
        return;
    }

    doc.config.indent.indent_size = opts.tab_size as usize;
    doc.config.indent.tab_width = opts.tab_size as usize;
    doc.config.indent.indent_style = indent_style_from_lsp(opts.insert_spaces);

    // TODO:
    // `trim_trailing_whitespace`
    // `trim_final_newlines`
    // `insert_final_newline`
}

async fn update_config(
    uris: Vec<Url>,
    client: &tower_lsp::Client,
    state: &mut WorldState,
) -> anyhow::Result<()> {
    let mut items: Vec<ConfigurationItem> = vec![];

    let diagnostics_keys = VscDiagnosticsConfig::FIELD_NAMES_AS_ARRAY;
    let mut diagnostics_items: Vec<ConfigurationItem> = diagnostics_keys
        .iter()
        .map(|key| ConfigurationItem {
            scope_uri: None,
            section: Some(VscDiagnosticsConfig::section_from_key(key).into()),
        })
        .collect();
    items.append(&mut diagnostics_items);

    // For document configs we collect all pairs of URIs and config keys of
    // interest in a flat vector
    let document_keys = VscDocumentConfig::FIELD_NAMES_AS_ARRAY;
    let mut document_items: Vec<ConfigurationItem> =
        itertools::iproduct!(uris.iter(), document_keys.iter())
            .map(|(uri, key)| ConfigurationItem {
                scope_uri: Some(uri.clone()),
                section: Some(VscDocumentConfig::section_from_key(key).into()),
            })
            .collect();
    items.append(&mut document_items);

    let configs = client.configuration(items).await?;

    // We got the config items in a flat vector that's guaranteed to be
    // ordered in the same way it was sent in. Be defensive and check that
    // we've got the expected number of items before we process them chunk
    // by chunk
    let n_document_items = document_keys.len();
    let n_diagnostics_items = diagnostics_keys.len();
    let n_items = n_diagnostics_items + (n_document_items * uris.len());

    if configs.len() != n_items {
        return Err(anyhow!(
            "Unexpected number of retrieved configurations: {}/{}",
            configs.len(),
            n_items
        ));
    }

    let mut configs = configs.into_iter();

    // --- Diagnostics
    let keys = diagnostics_keys.into_iter();
    let items: Vec<Value> = configs.by_ref().take(n_diagnostics_items).collect();

    // Create a new `serde_json::Value::Object` manually to convert it
    // to a `VscDocumentConfig` with `from_value()`. This way serde_json
    // can type-check the dynamic JSON value we got from the client.
    let mut map = serde_json::Map::new();
    std::iter::zip(keys, items).for_each(|(key, item)| {
        map.insert(key.into(), item);
    });

    // Deserialise the VS Code configuration
    let config: VscDiagnosticsConfig = serde_json::from_value(serde_json::Value::Object(map))?;
    let config: DiagnosticsConfig = config.into();

    let changed = state.config.diagnostics != config;
    state.config.diagnostics = config;

    if changed {
        lsp::spawn_diagnostics_refresh_all(state.clone());
    }

    // --- Documents
    // For each document, deserialise the vector of JSON values into a typed config
    for uri in uris.into_iter() {
        let keys = document_keys.into_iter();
        let items: Vec<Value> = configs.by_ref().take(n_document_items).collect();

        let mut map = serde_json::Map::new();
        std::iter::zip(keys, items).for_each(|(key, item)| {
            map.insert(key.into(), item);
        });

        // Deserialise the VS Code configuration
        let config: VscDocumentConfig = serde_json::from_value(serde_json::Value::Object(map))?;

        // Now convert the VS Code specific type into our own type
        let config: DocumentConfig = config.into();

        // Finally, update the document's config
        state.get_document_mut(&uri)?.config = config;
    }

    Ok(())
}

#[tracing::instrument(level = "info", skip_all)]
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

// FIXME: The initial indexer is currently racing against our state notification
// handlers. The indexer is synchronised through a mutex but we might end up in
// a weird state. Eventually the index should be moved to WorldState and created
// on demand with Salsa instrumenting and cancellation.
fn update_index(uri: &url::Url, doc: &Document) {
    if let Ok(path) = uri.to_file_path() {
        let path = Path::new(&path);
        if let Err(err) = indexer::update(&doc, &path) {
            lsp::log_error!("{err:?}");
        }
    }
}
