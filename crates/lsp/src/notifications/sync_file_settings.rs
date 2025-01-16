use tower_lsp::lsp_types::notification::Notification;
use url::Url;
use workspace::settings::Settings;

use crate::{main_loop::LspState, workspaces::WorkspaceSettings};

#[derive(serde::Serialize, serde::Deserialize)]
struct SyncFileSettings {
    file_settings: Vec<FileSettings>,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct FileSettings {
    url: String,
    settings: Settings,
}

impl Notification for SyncFileSettings {
    type Params = SyncFileSettings;
    const METHOD: &'static str = "air/syncFileSettings";
}

impl LspState {
    pub(crate) async fn notify_settings(&self, urls: Vec<Url>) {
        let file_settings: Vec<_> = urls
            .into_iter()
            .filter_map(
                |url| match self.workspace_settings_resolver.settings_for_url(&url) {
                    // There is a TOML to backpropagate
                    WorkspaceSettings::Toml(settings) => Some(FileSettings {
                        url: url.to_string(),
                        settings: settings.clone(),
                    }),
                    // There is no TOML. Let the IDE use its own settings.
                    WorkspaceSettings::Fallback(_) => None,
                },
            )
            .collect();
        let file_settings = SyncFileSettings { file_settings };

        tracing::trace!("Sending notification with backpropagated settings");
        self.client
            .send_notification::<SyncFileSettings>(file_settings)
            .await;
    }
}
