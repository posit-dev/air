// TODO: Remove this
#![allow(dead_code)]

pub use tower_lsp::start_lsp;

pub mod capabilities;
pub mod config;
pub mod crates;
pub mod documents;
pub mod encoding;
pub mod error;
pub mod from_proto;
pub mod handlers;
pub mod handlers_ext;
pub mod handlers_format;
pub mod handlers_state;
pub mod logging;
pub mod main_loop;
pub mod rust_analyzer;
pub mod state;
pub mod to_proto;
pub mod tower_lsp;
pub mod workspaces;

#[cfg(test)]
pub mod test_utils;
#[cfg(test)]
pub mod tower_lsp_test_client;
