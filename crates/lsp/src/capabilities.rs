//
// capabilities.rs
//
// Copyright (C) 2025 Posit Software, PBC. All rights reserved.
//
//

use tower_lsp::lsp_types::ClientCapabilities;
use tower_lsp::lsp_types::PositionEncodingKind;

/// The subset of [ClientCapabilities] the Client sends over that we actually do
/// something with
#[derive(Debug, Default)]
pub(crate) struct AirClientCapabilities {
    pub(crate) position_encodings: Vec<PositionEncodingKind>,
    pub(crate) dynamic_registration_for_did_change_configuration: bool,
    pub(crate) dynamic_registration_for_did_change_watched_files: bool,
    pub(crate) request_configuration: bool,
}

impl AirClientCapabilities {
    pub(crate) fn new(capabilities: ClientCapabilities) -> Self {
        let position_encodings = capabilities
            .general
            .and_then(|general_client_capabilities| general_client_capabilities.position_encodings)
            .unwrap_or(vec![PositionEncodingKind::UTF16]);

        let dynamic_registration_for_did_change_configuration = capabilities
            .workspace
            .as_ref()
            .and_then(|workspace| workspace.did_change_configuration)
            .and_then(|did_change_configuration| did_change_configuration.dynamic_registration)
            .unwrap_or(false);

        let dynamic_registration_for_did_change_watched_files = capabilities
            .workspace
            .as_ref()
            .and_then(|workspace| workspace.did_change_watched_files)
            .and_then(|watched_files| watched_files.dynamic_registration)
            .unwrap_or_default();

        let configuration = capabilities
            .workspace
            .as_ref()
            .and_then(|workspace| workspace.configuration)
            .unwrap_or_default();

        Self {
            position_encodings,
            dynamic_registration_for_did_change_configuration,
            dynamic_registration_for_did_change_watched_files,
            request_configuration: configuration,
        }
    }
}
