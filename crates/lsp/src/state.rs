use std::collections::HashMap;

use anyhow::anyhow;
use url::Url;

use crate::documents::Document;

#[derive(Clone, Default, Debug)]
/// The world state, i.e. all the inputs necessary for analysing or refactoring
/// code. This is a pure value. There is no interior mutability in this data
/// structure. It can be cloned and safely sent to other threads.
pub(crate) struct WorldState {
    /// Watched documents
    pub(crate) documents: HashMap<Url, Document>,

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
    #[allow(dead_code)]
    pub(crate) console_scopes: Vec<Vec<String>>,

    /// Currently installed packages
    #[allow(dead_code)]
    pub(crate) installed_packages: Vec<String>,
}

impl WorldState {
    pub(crate) fn get_document(&self, uri: &Url) -> Option<&Document> {
        self.documents.get(uri)
    }

    pub(crate) fn get_document_mut(&mut self, uri: &Url) -> Option<&mut Document> {
        self.documents.get_mut(uri)
    }

    pub(crate) fn get_document_or_error(&self, uri: &Url) -> anyhow::Result<&Document> {
        match self.get_document(uri) {
            Some(doc) => Ok(doc),
            None => Err(anyhow!("Can't find document for URI {uri}")),
        }
    }

    pub(crate) fn get_document_mut_or_error(&mut self, uri: &Url) -> anyhow::Result<&mut Document> {
        match self.get_document_mut(uri) {
            Some(doc) => Ok(doc),
            None => Err(anyhow!("Can't find document for URI {uri}")),
        }
    }

    pub(crate) fn workspace_uris(&self) -> Vec<Url> {
        self.documents.iter().map(|elt| elt.0.clone()).collect()
    }
}
