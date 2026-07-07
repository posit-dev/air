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
use tower_lsp::lsp_types::GlobPattern;
use tower_lsp::lsp_types::OneOf;
use tower_lsp::lsp_types::RelativePattern;
use tower_lsp::lsp_types::Url;
use tracing::Instrument;

use crate::main_loop::LspState;
use crate::settings_vsc::VscDiagnosticsSettings;
use crate::settings_vsc::VscDocumentSettings;
use crate::settings_vsc::VscGlobalSettings;

// Handlers that do not mutate the world state. They take a sharing reference or
// a clone of the state.

pub(crate) async fn handle_initialized(lsp_state: &LspState) -> anyhow::Result<()> {
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
            VscDocumentSettings::FIELD_NAMES_AS_ARRAY.to_vec(),
            VscDocumentSettings::section_from_key,
        );
        let mut config_diagnostics_registrations = collect_regs(
            VscDiagnosticsSettings::FIELD_NAMES_AS_ARRAY.to_vec(),
            VscDiagnosticsSettings::section_from_key,
        );
        let mut config_global_registrations = collect_regs(
            VscGlobalSettings::FIELD_NAMES_AS_ARRAY.to_vec(),
            VscGlobalSettings::section_from_key,
        );

        registrations.append(&mut config_document_registrations);
        registrations.append(&mut config_diagnostics_registrations);
        registrations.append(&mut config_global_registrations);
    }

    if lsp_state
        .capabilities
        .dynamic_registration_for_did_change_watched_files
    {
        registrations.push(air_toml_watched_file_registration(
            lsp_state
                .capabilities
                .relative_pattern_support_for_did_change_watched_files,
        ));
    }

    if !registrations.is_empty() {
        lsp_state
            .client
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

// Watch for changes in configuration files so we can react dynamically
fn air_toml_watched_file_registration(relative_pattern_support: bool) -> lsp_types::Registration {
    let mut watchers = Vec::new();

    // Project level `air.toml`s. `GlobPattern::String` is relative to the workspace.
    watchers.push(FileSystemWatcher {
        glob_pattern: GlobPattern::String("**/air.toml".into()),
        kind: None,
    });
    watchers.push(FileSystemWatcher {
        glob_pattern: GlobPattern::String("**/.air.toml".into()),
        kind: None,
    });

    // User configuration level `air.toml` lives outside any workspace folder, so we
    // need `GlobPattern::Relative` if the IDE supports it
    if relative_pattern_support
        && let Some(directory) = workspace::config::user_config_directory()
        && let Ok(directory) = Url::from_directory_path(&directory)
    {
        watchers.push(FileSystemWatcher {
            glob_pattern: GlobPattern::Relative(RelativePattern {
                base_uri: OneOf::Right(directory.clone()),
                pattern: String::from("air.toml"),
            }),
            kind: None,
        });
        watchers.push(FileSystemWatcher {
            glob_pattern: GlobPattern::Relative(RelativePattern {
                base_uri: OneOf::Right(directory.clone()),
                pattern: String::from(".air.toml"),
            }),
            kind: None,
        });
    }

    let register_options =
        Some(serde_json::to_value(DidChangeWatchedFilesRegistrationOptions { watchers }).unwrap());

    lsp_types::Registration {
        id: String::from("air-toml-watcher"),
        method: String::from("workspace/didChangeWatchedFiles"),
        register_options,
    }
}
