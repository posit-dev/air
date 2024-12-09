// TODO: Remove this
#![allow(dead_code)]

use std::sync::OnceLock;

pub use tower_lsp::start_server;

pub mod config;
pub mod documents;
pub mod encoding;
pub mod from_proto;
pub mod handlers;
pub mod handlers_ext;
pub mod handlers_format;
pub mod handlers_state;
pub mod main_loop;
pub mod rust_analyzer;
pub mod state;
pub mod to_proto;
pub mod tower_lsp;

/// Are we in a test LSP server?
///
/// `start_test_server()` sets this to `true`, which disables logging to the LSP client
/// during tests, making it easier to track sent/received requests.
pub static TESTING: OnceLock<bool> = OnceLock::new();

// These send LSP messages in a non-async and non-blocking way.
// The LOG level is not timestamped so we're not using it.
macro_rules! log_info {
    ($($arg:tt)+) => ($crate::_log!(tower_lsp::lsp_types::MessageType::INFO, $($arg)+))
}
macro_rules! log_warn {
    ($($arg:tt)+) => ($crate::_log!(tower_lsp::lsp_types::MessageType::WARNING, $($arg)+))
}
macro_rules! log_error {
    ($($arg:tt)+) => ($crate::_log!(tower_lsp::lsp_types::MessageType::ERROR, $($arg)+))
}
macro_rules! _log {
    ($lvl:expr, $($arg:tt)+) => ({
        $crate::main_loop::log($lvl, format!($($arg)+));
    });
}

pub(crate) use _log;
pub(crate) use log_error;
pub(crate) use log_info;
pub(crate) use log_warn;
