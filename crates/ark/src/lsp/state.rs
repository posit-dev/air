use std::hash::RandomState;
use std::path::Path;
use std::sync::Arc;

use anyhow::anyhow;
use dashmap::DashMap;
use parking_lot::Mutex;
use url::Url;

use crate::lsp::documents::Document;

// This is a work in progress. Currently contains synchronised objects but this
// shouldn't be necessary after we have implemented synchronisation of LSP handlers.
// The handlers will either get a shared ref or an exclusive ref to the worldstate.

#[derive(Clone, Default, Debug)]
/// The world state, i.e. all the inputs necessary for analysing or refactoring code.
pub(crate) struct WorldState {
    /// Watched documents
    pub documents: Arc<DashMap<Url, Document>>,

    /// Watched folders
    pub workspace: Arc<Mutex<Workspace>>,

    /// The scopes for the console. This currently contains a list (outer `Vec`)
    /// of names (inner `Vec`) within the environments on the search path, starting
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
pub(crate) struct Workspace {
    pub folders: Vec<Url>,
}

impl WorldState {
    pub(crate) fn get_document(
        &self,
        uri: &Url,
    ) -> anyhow::Result<dashmap::mapref::one::Ref<Url, Document, RandomState>> {
        if let Some(doc) = self.documents.get(uri) {
            Ok(doc)
        } else {
            Err(anyhow!("Can't find document for URI {uri}"))
        }
    }

    pub(crate) fn get_document_mut(
        &self,
        uri: &Url,
    ) -> anyhow::Result<dashmap::mapref::one::RefMut<Url, Document, RandomState>> {
        if let Some(doc) = self.documents.get_mut(uri) {
            Ok(doc)
        } else {
            Err(anyhow!("Can't find document for URI {uri}"))
        }
    }
}

pub(crate) fn with_document<T, F>(
    path: &Path,
    state: &WorldState,
    mut callback: F,
) -> anyhow::Result<T>
where
    F: FnMut(&Document) -> anyhow::Result<T>,
{
    let mut fallback = || {
        let contents = std::fs::read_to_string(path)?;
        let document = Document::new(contents.as_str(), None);
        return callback(&document);
    };

    // If we have a cached copy of the document (because we're monitoring it)
    // then use that; otherwise, try to read the document from the provided
    // path and use that instead.
    let Ok(uri) = Url::from_file_path(path) else {
        log::info!(
            "couldn't construct uri from {}; reading from disk instead",
            path.display()
        );
        return fallback();
    };

    let Ok(document) = state.get_document(&uri) else {
        log::info!("no document for uri {uri}; reading from disk instead");
        return fallback();
    };

    return callback(document.value());
}
