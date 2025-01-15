//
// notifications.rs
//
// Copyright (C) 2024 Posit Software, PBC. All rights reserved.
//
//

use tower_lsp::lsp_types::notification::Notification;
use url::Url;
use workspace::settings::Settings;

use crate::{main_loop::LspState, workspaces::WorkspaceSettings};

struct SettingsNotifications {}

#[derive(serde::Serialize, serde::Deserialize)]
struct FileSettings {
    path: String,
    settings: Settings,
}

impl Notification for SettingsNotifications {
    type Params = Vec<FileSettings>;
    const METHOD: &'static str = "air/tomlSettings";
}

impl LspState {
    pub(crate) async fn notify_settings(&self, urls: Vec<Url>) {
        let settings: Vec<_> = urls
            .into_iter()
            .filter_map(
                |url| match self.workspace_settings_resolver.settings_for_url(&url) {
                    // There is a TOML to backpropagate
                    WorkspaceSettings::Toml(settings) => Some(FileSettings {
                        path: url.to_string(),
                        settings: settings.clone(),
                    }),
                    // There is no TOML. Let the IDE use its own settings.
                    WorkspaceSettings::Fallback(_) => None,
                },
            )
            .collect();

        tracing::trace!("Sending notification with backpropagated settings");
        self.client
            .send_notification::<SettingsNotifications>(settings)
            .await;
    }
}
