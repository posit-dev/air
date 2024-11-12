// TODO: Remove this
#![allow(dead_code)]

pub mod config;
pub mod documents;
pub mod encoding;
pub mod handlers;
pub mod main_loop;
pub mod state;
pub mod state_handlers;
pub mod tower_lsp;

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
