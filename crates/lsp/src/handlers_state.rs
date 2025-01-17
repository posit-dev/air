//
// handlers_state.rs
//
// Copyright (C) 2024 Posit Software, PBC. All rights reserved.
//
//

use std::array::IntoIter;

use anyhow::anyhow;
use anyhow::Context;
use biome_lsp_converters::PositionEncoding;
use biome_lsp_converters::WideEncoding;
use serde_json::Value;
use struct_field_names_as_array::FieldNamesAsArray;
use tower_lsp::lsp_types;
use tower_lsp::lsp_types::ConfigurationItem;
use tower_lsp::lsp_types::DidChangeConfigurationParams;
use tower_lsp::lsp_types::DidChangeTextDocumentParams;
use tower_lsp::lsp_types::DidChangeWatchedFilesParams;
use tower_lsp::lsp_types::DidChangeWorkspaceFoldersParams;
use tower_lsp::lsp_types::DidCloseTextDocumentParams;
use tower_lsp::lsp_types::DidOpenTextDocumentParams;
use tower_lsp::lsp_types::FormattingOptions;
use tower_lsp::lsp_types::InitializeParams;
use tower_lsp::lsp_types::InitializeResult;
use tower_lsp::lsp_types::OneOf;
use tower_lsp::lsp_types::ServerCapabilities;
use tower_lsp::lsp_types::ServerInfo;
use tower_lsp::lsp_types::TextDocumentSyncCapability;
use tower_lsp::lsp_types::TextDocumentSyncKind;
use tower_lsp::lsp_types::WorkspaceFoldersServerCapabilities;
use tower_lsp::lsp_types::WorkspaceServerCapabilities;
use tracing::Instrument;
use url::Url;

use crate::capabilities::AirClientCapabilities;
use crate::documents::Document;
use crate::logging;
use crate::logging::LogMessageSender;
use crate::main_loop::LspState;
use crate::settings::DocumentSettings;
use crate::settings::InitializationOptions;
use crate::settings_vsc::indent_style_from_vsc;
use crate::settings_vsc::indent_width_from_usize;
use crate::settings_vsc::VscDiagnosticsSettings;
use crate::settings_vsc::VscDocumentSettings;
use crate::settings_vsc::VscGlobalSettings;
use crate::state::WorldState;
use crate::workspaces::WorkspaceSettingsResolver;

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
    log_tx: LogMessageSender,
) -> anyhow::Result<InitializeResult> {
    let InitializationOptions {
        log_level,
        dependency_log_levels,
    } = params.initialization_options.map_or_else(
        InitializationOptions::default,
        InitializationOptions::from_value,
    );

    lsp_state.log_state = Some(logging::init_logging(
        log_tx,
        log_level,
        dependency_log_levels,
        params.client_info.as_ref(),
    ));

    // Initialize the workspace settings resolver using the initial set of client provided `workspace_folders`
    lsp_state.workspace_settings_resolver = WorkspaceSettingsResolver::from_workspace_folders(
        params.workspace_folders.unwrap_or_default(),
    );

    lsp_state.capabilities = AirClientCapabilities::new(params.capabilities);

    // If the client supports UTF-8 we use that, even if it's not its
    // preferred encoding (at position 0). Otherwise we use the mandatory
    // UTF-16 encoding that all clients and servers must support, even if
    // the client would have preferred UTF-32. Note that VSCode and Positron
    // only support UTF-16.
    let position_encoding = if lsp_state
        .capabilities
        .position_encodings
        .contains(&lsp_types::PositionEncodingKind::UTF8)
    {
        lsp_state.position_encoding = PositionEncoding::Utf8;
        Some(lsp_types::PositionEncodingKind::UTF8)
    } else {
        lsp_state.position_encoding = PositionEncoding::Wide(WideEncoding::Utf16);
        Some(lsp_types::PositionEncodingKind::UTF16)
    };

    Ok(InitializeResult {
        server_info: Some(ServerInfo {
            name: "Air Language Server".to_string(),
            version: Some(env!("CARGO_PKG_VERSION").to_string()),
        }),
        capabilities: ServerCapabilities {
            position_encoding,
            text_document_sync: Some(TextDocumentSyncCapability::Kind(
                TextDocumentSyncKind::INCREMENTAL,
            )),
            workspace: Some(WorkspaceServerCapabilities {
                workspace_folders: Some(WorkspaceFoldersServerCapabilities {
                    supported: Some(true),
                    change_notifications: Some(OneOf::Left(true)),
                }),
                file_operations: None,
            }),
            document_formatting_provider: Some(OneOf::Left(true)),
            document_range_formatting_provider: Some(OneOf::Left(true)),
            ..ServerCapabilities::default()
        },
    })
}

#[tracing::instrument(level = "info", skip_all)]
pub(crate) async fn did_open(
    params: DidOpenTextDocumentParams,
    lsp_state: &mut LspState,
    state: &mut WorldState,
) -> anyhow::Result<()> {
    let contents = params.text_document.text;
    let uri = params.text_document.uri;
    let version = params.text_document.version;

    let document = Document::new(contents, Some(version), lsp_state.position_encoding);
    state.documents.insert(uri.clone(), document);

    // Propagate client settings to Air
    if lsp_state.capabilities.request_configuration {
        update_config(vec![uri.clone()], lsp_state, state)
            .instrument(tracing::info_span!("did_change_configuration"))
            .await?;
    }

    // Backpropagate Air settings to client
    lsp_state.sync_file_settings(vec![uri]).await;

    Ok(())
}

#[tracing::instrument(level = "info", skip_all)]
pub(crate) fn did_change(
    params: DidChangeTextDocumentParams,
    state: &mut WorldState,
) -> anyhow::Result<()> {
    let uri = &params.text_document.uri;
    let doc = state.get_document_mut(uri)?;
    doc.on_did_change(params);

    Ok(())
}

#[tracing::instrument(level = "info", skip_all)]
pub(crate) fn did_close(
    params: DidCloseTextDocumentParams,
    state: &mut WorldState,
) -> anyhow::Result<()> {
    let uri = params.text_document.uri;

    // Publish empty set of diagnostics to clear them
    // lsp::publish_diagnostics(uri.clone(), Vec::new(), None);

    state
        .documents
        .remove(&uri)
        .ok_or(anyhow!("Failed to remove document for URI: {uri}"))?;

    Ok(())
}

pub(crate) async fn did_change_configuration(
    _params: DidChangeConfigurationParams,
    lsp_state: &mut LspState,
    state: &mut WorldState,
) -> anyhow::Result<()> {
    // The LSP deprecated usage of `DidChangeConfigurationParams`, but still allows
    // servers to use the notification itself as a way to get notified that some
    // configuration that we watch has changed. When we detect any changes, we re-pull
    // everything we are interested in.

    update_config(state.workspace_uris(), lsp_state, state)
        .instrument(tracing::info_span!("did_change_configuration"))
        .await
}

pub(crate) fn did_change_workspace_folders(
    params: DidChangeWorkspaceFoldersParams,
    lsp_state: &mut LspState,
) -> anyhow::Result<()> {
    for lsp_types::WorkspaceFolder { uri, .. } in params.event.added {
        lsp_state.open_workspace_folder(&uri);
    }
    for lsp_types::WorkspaceFolder { uri, .. } in params.event.removed {
        lsp_state.close_workspace_folder(&uri);
    }
    Ok(())
}

pub(crate) async fn did_change_watched_files(
    params: DidChangeWatchedFilesParams,
    lsp_state: &mut LspState,
    state: &WorldState,
) -> anyhow::Result<()> {
    let mut changed_settings = false;

    for change in &params.changes {
        if lsp_state
            .workspace_settings_resolver
            .reload_workspaces_matched_by_url(&change.uri)
        {
            changed_settings = true;
        }
    }

    // Backpropagate settings for all opened files if an `air.toml` file changed
    if changed_settings {
        lsp_state.sync_file_settings(state.workspace_uris()).await;
    }

    Ok(())
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

    tracing::trace!(file = ?uri, "Got formatting settings: {:?}", &opts);

    doc.settings.indent_style = Some(indent_style_from_vsc(opts.insert_spaces));

    // Note that `tabSize` in the LSP protocol corresponds to `indentSize` in VS Code options.
    // And if Code's `indentSize` is aliased to Code's `tabSize`, we get the latter here.
    doc.settings.indent_width = Some(indent_width_from_usize(opts.tab_size as usize));

    // TODO:
    // `trim_trailing_whitespace`
    // `trim_final_newlines`
    // `insert_final_newline`
}

async fn update_config(
    uris: Vec<Url>,
    lsp_state: &mut LspState,
    state: &mut WorldState,
) -> anyhow::Result<()> {
    let mut items: Vec<ConfigurationItem> = vec![];

    let diagnostics_keys = VscDiagnosticsSettings::FIELD_NAMES_AS_ARRAY;
    let mut diagnostics_items: Vec<ConfigurationItem> = diagnostics_keys
        .iter()
        .map(|key| ConfigurationItem {
            scope_uri: None,
            section: Some(VscDiagnosticsSettings::section_from_key(key).into()),
        })
        .collect();
    items.append(&mut diagnostics_items);

    // For document configs we collect all pairs of URIs and config keys of
    // interest in a flat vector
    let document_keys = VscDocumentSettings::FIELD_NAMES_AS_ARRAY;
    let mut document_items: Vec<ConfigurationItem> =
        itertools::iproduct!(uris.iter(), document_keys.iter())
            .map(|(uri, key)| ConfigurationItem {
                scope_uri: Some(uri.clone()),
                section: Some(VscDocumentSettings::section_from_key(key).into()),
            })
            .collect();
    items.append(&mut document_items);

    let global_keys = VscGlobalSettings::FIELD_NAMES_AS_ARRAY;
    let mut global_items: Vec<ConfigurationItem> = global_keys
        .iter()
        .map(|key| ConfigurationItem {
            scope_uri: None,
            section: Some(VscGlobalSettings::section_from_key(key).into()),
        })
        .collect();
    items.append(&mut global_items);

    let configs = lsp_state.client.configuration(items).await?;

    // We got the config items in a flat vector that's guaranteed to be
    // ordered in the same way it was sent in. Be defensive and check that
    // we've got the expected number of items before we process them chunk
    // by chunk
    let n_diagnostics_items = diagnostics_keys.len();
    let n_document_items = document_keys.len() * uris.len();
    let n_global_items = global_keys.len();
    let n_items = n_diagnostics_items + n_document_items + n_global_items;

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
    let items = configs.by_ref().take(n_diagnostics_items);
    update_diagnostics_config(keys, items)?;

    // --- Documents
    let keys = document_keys.into_iter();
    let items = configs.by_ref().take(n_document_items);
    update_documents_config(keys, items, uris, state)?;

    // --- Global
    let keys = global_keys.into_iter();
    let items = configs.by_ref().take(n_global_items);
    update_global_config(keys, items, lsp_state, state).await?;

    Ok(())
}

fn update_diagnostics_config(
    keys: IntoIter<&str, 1>,
    items: impl Iterator<Item = Value>,
) -> anyhow::Result<()> {
    // Create a new `serde_json::Value::Object` manually to convert it
    // to a `VscDocumentConfig` with `from_value()`. This way serde_json
    // can type-check the dynamic JSON value we got from the client.
    let mut map = serde_json::Map::new();
    std::iter::zip(keys, items).for_each(|(key, item)| {
        map.insert(key.into(), item);
    });

    // TODO: Deserialise the VS Code configuration
    // let config: VscDiagnosticsConfig = serde_json::from_value(serde_json::Value::Object(map))?;
    // let config: DiagnosticsConfig = config.into();

    // let changed = state.config.diagnostics != config;
    // state.config.diagnostics = config;

    // if changed {
    //     lsp::spawn_diagnostics_refresh_all(state.clone());
    // }

    Ok(())
}

fn update_documents_config(
    keys: IntoIter<&str, 3>,
    mut items: impl Iterator<Item = Value>,
    uris: Vec<Url>,
    state: &mut WorldState,
) -> anyhow::Result<()> {
    // For each document, deserialise the vector of JSON values into a typed config
    for uri in uris {
        let uri_keys = keys.clone();
        let uri_items = items.by_ref().take(keys.len());

        let mut map = serde_json::Map::new();
        std::iter::zip(uri_keys, uri_items).for_each(|(key, item)| {
            map.insert(key.into(), item);
        });

        // Deserialise the VS Code configuration
        let config: VscDocumentSettings = serde_json::from_value(serde_json::Value::Object(map))?;
        tracing::trace!(file = ?uri, "Got VS Code settings: {:?}", &config);

        // Now convert the VS Code specific type into our own type
        let config: DocumentSettings = config.into();

        // Finally, update the document's config
        state.get_document_mut(&uri)?.settings = config;
    }

    Ok(())
}

async fn update_global_config(
    keys: IntoIter<&str, 3>,
    items: impl Iterator<Item = Value>,
    lsp_state: &mut LspState,
    state: &WorldState,
) -> anyhow::Result<()> {
    let log_state = lsp_state.log_state.as_mut().context("Missing log state")?;

    let old_sync_file_settings_with_client = lsp_state.settings.sync_file_settings_with_client;

    let mut map = serde_json::Map::new();
    std::iter::zip(keys, items).for_each(|(key, item)| {
        map.insert(key.into(), item);
    });

    // Deserialise the VS Code configuration
    let settings: VscGlobalSettings = serde_json::from_value(serde_json::Value::Object(map))?;

    // These log settings are not stored in the LSP state
    log_state.reload(settings.log_level, settings.dependency_log_levels.clone());

    // Convert and set the global LSP settings
    lsp_state.settings = settings.into();

    // If the client just enabled file settings synchronisation, sync them
    if !old_sync_file_settings_with_client && lsp_state.settings.sync_file_settings_with_client {
        lsp_state.sync_file_settings(state.workspace_uris()).await
    }

    Ok(())
}
