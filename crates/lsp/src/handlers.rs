//
// handlers.rs
//
// Copyright (C) 2024 Posit Software, PBC. All rights reserved.
//
//

use struct_field_names_as_array::FieldNamesAsArray;
use tower_lsp::lsp_types;
use tower_lsp::lsp_types::DidChangeWatchedFilesRegistrationOptions;
use tower_lsp::lsp_types::FileSystemWatcher;
use tower_lsp::Client;
use tracing::Instrument;

use crate::config::VscDiagnosticsConfig;
use crate::config::VscDocumentConfig;
use crate::config::VscLogConfig;
use crate::main_loop::LspState;

// Handlers that do not mutate the world state. They take a sharing reference or
// a clone of the state.

pub(crate) async fn handle_initialized(
    client: &Client,
    lsp_state: &LspState,
) -> anyhow::Result<()> {
    let span = tracing::info_span!("handle_initialized").entered();

    // Register capabilities to the client
    let mut registrations: Vec<lsp_types::Registration> = vec![];

    if lsp_state
        .capabilities
        .dynamic_registration_for_did_change_configuration
    {
        // The `didChangeConfiguration` request instructs the client to send
        // a notification when the tracked settings have changed.
        //
        // Note that some settings, such as editor indentation properties, may be
        // changed by extensions or by the user without changing the actual
        // underlying setting. Unfortunately we don't receive updates in that case.
        let mut config_document_registrations = collect_regs(
            VscDocumentConfig::FIELD_NAMES_AS_ARRAY.to_vec(),
            VscDocumentConfig::section_from_key,
        );
        let mut config_diagnostics_registrations: Vec<lsp_types::Registration> = collect_regs(
            VscDiagnosticsConfig::FIELD_NAMES_AS_ARRAY.to_vec(),
            VscDiagnosticsConfig::section_from_key,
        );
        let mut config_log_registrations: Vec<lsp_types::Registration> = collect_regs(
            VscLogConfig::FIELD_NAMES_AS_ARRAY.to_vec(),
            VscLogConfig::section_from_key,
        );

        registrations.append(&mut config_document_registrations);
        registrations.append(&mut config_diagnostics_registrations);
        registrations.append(&mut config_log_registrations);
    }

    if lsp_state
        .capabilities
        .dynamic_registration_for_did_change_watched_files
    {
        // Watch for changes in configuration files so we can react dynamically
        let watch_air_toml_registration = lsp_types::Registration {
            id: String::from("air-toml-watcher"),
            method: "workspace/didChangeWatchedFiles".into(),
            register_options: Some(
                serde_json::to_value(DidChangeWatchedFilesRegistrationOptions {
                    watchers: vec![
                        FileSystemWatcher {
                            glob_pattern: lsp_types::GlobPattern::String("**/air.toml".into()),
                            kind: None,
                        },
                        FileSystemWatcher {
                            glob_pattern: lsp_types::GlobPattern::String("**/.air.toml".into()),
                            kind: None,
                        },
                    ],
                })
                .unwrap(),
            ),
        };

        registrations.push(watch_air_toml_registration);
    }

    if !registrations.is_empty() {
        client
            .register_capability(registrations)
            .instrument(span.exit())
            .await?;
    }

    Ok(())
}

fn collect_regs(
    fields: Vec<&str>,
    into_section: impl Fn(&str) -> &str,
) -> Vec<lsp_types::Registration> {
    fields
        .into_iter()
        .map(|field| lsp_types::Registration {
            id: uuid::Uuid::new_v4().to_string(),
            method: String::from("workspace/didChangeConfiguration"),
            register_options: Some(serde_json::json!({ "section": into_section(field) })),
        })
        .collect()
}
