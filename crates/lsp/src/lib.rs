// TODO: Remove this
#![allow(dead_code)]

pub use tower_lsp::start_lsp;

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

#[cfg(test)]
pub mod tower_lsp_test_client;
