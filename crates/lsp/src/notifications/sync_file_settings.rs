use tower_lsp::lsp_types::notification::Notification;
use url::Url;

use crate::{main_loop::LspState, workspaces::WorkspaceSettings};

#[derive(serde::Serialize, serde::Deserialize)]
struct SyncFileSettingsParams {
    file_settings: Vec<FileSettings>,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct FileSettings {
    url: String,
    format: FileFormatSettings,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct FileFormatSettings {
    pub indent_style: settings::IndentStyle,
    pub indent_width: settings::IndentWidth,
    pub line_width: settings::LineWidth,
}

impl Notification for SyncFileSettingsParams {
    type Params = SyncFileSettingsParams;
    const METHOD: &'static str = "air/syncFileSettings";
}

impl From<workspace::settings::FormatSettings> for FileFormatSettings {
    fn from(value: workspace::settings::FormatSettings) -> Self {
        Self {
            indent_style: value.indent_style,
            indent_width: value.indent_width,
            line_width: value.line_width,
        }
    }
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
                        format: settings.format.clone().into(),
                    }),
                    // There is no TOML. Let the IDE use its own settings.
                    WorkspaceSettings::Fallback(_) => None,
                },
            )
            .collect();
        let params = SyncFileSettingsParams { file_settings };

        tracing::trace!("Sending notification with backpropagated settings");
        self.client
            .send_notification::<SyncFileSettingsParams>(params)
            .await;
    }
}
