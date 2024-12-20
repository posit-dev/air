use serde::Deserialize;
use serde_json::Value;
use url::Url;

// These settings are only needed once, typically for initialization.
// They are read at the global scope on the client side and are never refreshed.
#[derive(Debug, Deserialize, Default, Clone)]
#[cfg_attr(test, derive(PartialEq, Eq))]
#[serde(rename_all = "camelCase")]
pub(crate) struct ClientGlobalSettings {
    pub(crate) log_level: Option<crate::logging::LogLevel>,
    pub(crate) dependency_log_levels: Option<String>,
}

/// This is a direct representation of the user level settings schema sent
/// by the client. It is refreshed after configuration changes.
#[derive(Debug, Deserialize, Default, Clone)]
#[cfg_attr(test, derive(PartialEq, Eq))]
#[serde(rename_all = "camelCase")]
pub(crate) struct ClientSettings {}

/// This is a direct representation of the workspace level settings schema sent by the
/// client. It is the same as the user level settings with the addition of the workspace
/// path.
#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq, Eq))]
#[serde(rename_all = "camelCase")]
pub(crate) struct ClientWorkspaceSettings {
    pub(crate) url: Url,
    #[serde(flatten)]
    pub(crate) settings: ClientSettings,
}

/// This is the exact schema for initialization options sent in by the client
/// during initialization.
#[derive(Debug, Deserialize, Default)]
#[cfg_attr(test, derive(PartialEq, Eq))]
#[serde(rename_all = "camelCase")]
pub(crate) struct InitializationOptions {
    pub(crate) global_settings: ClientGlobalSettings,
    pub(crate) user_settings: ClientSettings,
    pub(crate) workspace_settings: Vec<ClientWorkspaceSettings>,
}

impl InitializationOptions {
    pub(crate) fn from_value(value: Value) -> Self {
        serde_json::from_value(value)
            .map_err(|err| {
                tracing::error!("Failed to deserialize initialization options: {err}. Falling back to default client settings.");
            })
            .unwrap_or_default()
    }
}
