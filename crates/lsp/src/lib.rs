pub use tower_lsp::start_lsp;

pub mod capabilities;
pub mod diff;
pub mod documents;
pub mod file_patterns;
pub mod handlers;
pub mod handlers_ext;
pub mod handlers_format;
pub mod handlers_state;
pub mod logging;
pub mod main_loop;
pub mod notifications;
pub mod proto;
pub mod settings;
pub mod settings_vsc;
pub mod state;
pub mod text_edit;
pub mod tower_lsp;
pub mod workspaces;

#[cfg(test)]
pub mod test;
