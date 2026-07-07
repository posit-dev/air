use std::path::PathBuf;

use etcetera::BaseStrategy;

/// Air's user level configuration directory
///
/// - Linux/macOS: `{XDG_CONFIG_HOME}/air` falling back to `~/.config/air/` if unset
/// - Windows: `{APPDATA}\air\` falling back to `%APPDATA%\air\` if unset
///
/// Returns `None` if the base strategy can't be determined (e.g. no home directory).
pub fn user_config_directory() -> Option<PathBuf> {
    let strategy = match etcetera::choose_base_strategy() {
        Ok(strategy) => strategy,
        Err(error) => {
            tracing::warn!("Failed to determine user configuration directory:\n{error}");
            return None;
        }
    };

    Some(strategy.config_dir().join("air"))
}

/// The environment variable corresponding to the user configuration directory
///
/// Used in integration and unit tests to set custom config directory locations
pub fn user_config_directory_env_var() -> &'static str {
    if cfg!(windows) {
        "APPDATA"
    } else {
        "XDG_CONFIG_HOME"
    }
}

/// Set the environment variable corresponding to the user configuration directory
///
/// # Safety
///
/// For use in single-threaded nextest tests, where nothing else can concurrently
/// touch the environment
pub unsafe fn set_user_config_directory_env_var(directory: &std::path::Path) {
    unsafe { std::env::set_var(user_config_directory_env_var(), directory) }
}
