use std::sync::OnceLock;

use tower_lsp::lsp_types;

use crate::main_loop::send_auxiliary;
use crate::main_loop::AuxiliaryEvent;

/// Should logs be emitted at all?
///
/// `start_test_server()` sets this to `false`, which disables logging to the LSP client
/// during tests, making it easier to track sent/received requests.
static EMIT_LOGS: OnceLock<bool> = OnceLock::new();

// These send LSP messages in a non-async and non-blocking way.
// The LOG level is not timestamped so we're not using it.
#[macro_export]
macro_rules! log_info {
    ($($arg:tt)+) => ($crate::_log!(tower_lsp::lsp_types::MessageType::INFO, $($arg)+))
}
#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)+) => ($crate::_log!(tower_lsp::lsp_types::MessageType::WARNING, $($arg)+))
}
#[macro_export]
macro_rules! log_error {
    ($($arg:tt)+) => ($crate::_log!(tower_lsp::lsp_types::MessageType::ERROR, $($arg)+))
}
#[macro_export]
macro_rules! _log {
    ($lvl:expr, $($arg:tt)+) => ({
        $crate::logging::log($lvl, format!($($arg)+));
    });
}

pub(crate) fn init_logging(emit_logs: bool) {
    EMIT_LOGS
        .set(emit_logs)
        .expect("`EMIT_LOGS` can't be set more than once.");
}

/// Send a message to the LSP client. This is non-blocking and treated on a
/// latency-sensitive task.
pub(crate) fn log(level: lsp_types::MessageType, message: String) {
    // We don't want to send logs to the client when running integration tests,
    // as they interfere with our ability to track sent/received requests.
    if !EMIT_LOGS.get().unwrap_or(&false) {
        return;
    }

    // Check that channel is still alive in case the LSP was closed.
    // If closed, fallthrough.
    if send_auxiliary(AuxiliaryEvent::Log(level, message.clone())).is_ok() {
        return;
    }

    // TODO: Remove this when `log::` is our main way to log, and log to stderr instead
    // as the fallback
    // Log to `log::` as fallback
    log::warn!("LSP channel is closed, redirecting messages to `log::`");

    match level {
        lsp_types::MessageType::ERROR => log::error!("{message}"),
        lsp_types::MessageType::WARNING => log::warn!("{message}"),
        _ => log::info!("{message}"),
    };
}
