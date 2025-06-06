pub use tower_lsp::start_lsp;

pub mod capabilities;
pub mod documents;
pub mod file_patterns;
pub mod from_proto;
pub mod handlers;
pub mod handlers_ext;
pub mod handlers_format;
pub mod handlers_state;
pub mod logging;
pub mod main_loop;
pub mod notifications;
pub mod proto;
pub mod rust_analyzer;
pub mod settings;
pub mod settings_vsc;
pub mod state;
pub mod to_proto;
pub mod tower_lsp;
pub mod workspaces;

#[cfg(test)]
pub mod test;
