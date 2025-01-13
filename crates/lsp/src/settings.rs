use serde::Deserialize;
use serde_json::Value;

/// Settings of a document.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct DocumentSettings {
    pub indent_style: Option<settings::IndentStyle>,
    pub indent_width: Option<settings::IndentWidth>,
    pub line_width: Option<settings::LineWidth>,
}

/// This is the exact schema for initialization options sent in by the client
/// during initialization. Remember that initialization options are ones that are
/// strictly required at startup time, and most configuration options should really be
/// "pulled" dynamically by the server after startup and whenever we receive a
/// configuration change notification (#121).
#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub(crate) struct InitializationOptions {
    pub(crate) log_level: Option<crate::logging::LogLevel>,
    pub(crate) dependency_log_levels: Option<String>,
}

impl InitializationOptions {
    pub(crate) fn from_value(value: Value) -> Self {
        serde_json::from_value(value)
            .map_err(|err| {
                tracing::error!("Failed to deserialize initialization options: {err}. Falling back to default settings.");
            })
            .unwrap_or_default()
    }
}
