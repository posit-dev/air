use std::sync::Arc;

use dashmap::DashMap;
use parking_lot::Mutex;
use url::Url;

use crate::lsp::documents::Document;

// This is a work in progress. Currently contains synchronised objects but this
// shouldn't be necessary after we have implemented synchronisation of LSP handlers.
// The handlers will either get a shared ref or an exclusive ref to the worldstate.

#[derive(Clone, Default, Debug)]
/// The world state, i.e. all the inputs necessary for analysing or refactoring code.
pub struct WorldState {
    /// Watched documents
    pub documents: Arc<DashMap<Url, Document>>,

    /// Watched folders
    pub workspace: Arc<Mutex<Workspace>>,

    /// The scopes for the console. This currently contains a list (outer `Vec`)
    /// of names (inner `Vec`) of the environments on the search path, starting
    /// from the global environment and ending with the base package. Eventually
    /// this might also be populated with the scope for the current environment
    /// in debug sessions (not implemented yet).
    ///
    /// This is currently one of the main sources of known symbols for
    /// diagnostics. In the future we should better delineate interactive
    /// contexts (e.g. the console, but scripts might also be treated as
    /// interactive, which could be a user setting) and non-interactive ones
    /// (e.g. a package). In non-interactive contexts, the lexical scopes
    /// examined for diagnostics should be fully determined by variable bindings
    /// and imports (code-first diagnostics).
    ///
    /// In the future this should probably become more complex with a list of
    /// either symbol names (as is now the case) or named environments, such as
    /// `pkg:ggplot2`. Storing named environments here will allow the LSP to
    /// retrieve the symbols in a pull fashion (the whole console scopes are
    /// currently pushed to the LSP), and cache the symbols with Salsa. The
    /// performance is not currently an issue but this could change once we do
    /// more analysis of symbols in the search path.
    pub console_scopes: Arc<Mutex<Vec<Vec<String>>>>,

    /// Currently installed packages
    pub installed_packages: Arc<Mutex<Vec<String>>>,
}

#[derive(Default, Debug)]
pub struct Workspace {
    pub folders: Vec<Url>,
}
