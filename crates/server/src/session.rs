//! Data model, state management, and configuration resolution.

use std::sync::Arc;

use lsp_types::Url;
use lsp_types::WorkspaceFolder;

use crate::edit::{DocumentKey, DocumentVersion, PositionEncoding, TextDocument};

pub(crate) use self::capabilities::ResolvedClientCapabilities;
pub use self::index::DocumentQuery;

mod capabilities;
mod index;
mod workspaces;

/// The global state for the LSP
pub(crate) struct Session {
    /// Used to retrieve information about open documents and settings.
    index: index::Index,
    /// The global position encoding, negotiated during LSP initialization.
    position_encoding: PositionEncoding,
    /// Tracks what LSP features the client supports and doesn't support.
    resolved_client_capabilities: Arc<ResolvedClientCapabilities>,
}

/// An immutable snapshot of `Session` that references
/// a specific document.
pub(crate) struct DocumentSnapshot {
    #[allow(dead_code)]
    resolved_client_capabilities: Arc<ResolvedClientCapabilities>,
    document_ref: index::DocumentQuery,
    position_encoding: PositionEncoding,
}

impl Session {
    pub(crate) fn new(
        resolved_client_capabilities: ResolvedClientCapabilities,
        position_encoding: PositionEncoding,
        workspace_folders: Vec<WorkspaceFolder>,
    ) -> crate::Result<Self> {
        Ok(Self {
            position_encoding,
            index: index::Index::new(workspace_folders)?,
            resolved_client_capabilities: Arc::new(resolved_client_capabilities),
        })
    }

    pub(crate) fn key_from_url(&self, url: Url) -> DocumentKey {
        self.index.key_from_url(url)
    }

    /// Creates a document snapshot with the URL referencing the document to snapshot.
    pub(crate) fn take_snapshot(&self, url: Url) -> Option<DocumentSnapshot> {
        let key = self.key_from_url(url);
        Some(DocumentSnapshot {
            resolved_client_capabilities: self.resolved_client_capabilities.clone(),
            document_ref: self.index.make_document_ref(key)?,
            position_encoding: self.position_encoding,
        })
    }

    /// Iterates over the LSP URLs for all open text documents. These URLs are valid file paths.
    #[allow(dead_code)]
    pub(crate) fn text_document_urls(&self) -> impl Iterator<Item = &lsp_types::Url> + '_ {
        self.index.text_document_urls()
    }

    /// Updates a text document at the associated `key`.
    ///
    /// The document key must point to a text document, or this will throw an error.
    pub(crate) fn update_text_document(
        &mut self,
        key: &DocumentKey,
        content_changes: Vec<lsp_types::TextDocumentContentChangeEvent>,
        new_version: DocumentVersion,
    ) -> crate::Result<()> {
        let encoding = self.encoding();

        self.index
            .update_text_document(key, content_changes, new_version, encoding)
    }

    /// Registers a text document at the provided `url`.
    /// If a document is already open here, it will be overwritten.
    pub(crate) fn open_text_document(&mut self, url: Url, document: TextDocument) {
        self.index.open_text_document(url, document);
    }

    /// De-registers a document, specified by its key.
    /// Calling this multiple times for the same document is a logic error.
    pub(crate) fn close_document(&mut self, key: &DocumentKey) -> crate::Result<()> {
        self.index.close_document(key)?;
        Ok(())
    }

    /// Reloads the settings index
    pub(crate) fn reload_settings(&mut self, changed_url: &Url) {
        self.index.reload_settings(changed_url);
    }

    /// Open a workspace folder at the given `url`.
    pub(crate) fn open_workspace_folder(&mut self, url: &Url) -> crate::Result<()> {
        self.index.open_workspace_folder(url)
    }

    /// Close a workspace folder at the given `url`.
    pub(crate) fn close_workspace_folder(&mut self, url: &Url) -> crate::Result<()> {
        self.index.close_workspace_folder(url)?;
        Ok(())
    }

    #[allow(dead_code)]
    pub(crate) fn num_documents(&self) -> usize {
        self.index.num_documents()
    }

    #[allow(dead_code)]
    pub(crate) fn num_workspaces(&self) -> usize {
        self.index.num_workspaces()
    }

    #[allow(dead_code)]
    pub(crate) fn resolved_client_capabilities(&self) -> &ResolvedClientCapabilities {
        &self.resolved_client_capabilities
    }

    pub(crate) fn encoding(&self) -> PositionEncoding {
        self.position_encoding
    }
}

impl DocumentSnapshot {
    #[allow(dead_code)]
    pub(crate) fn resolved_client_capabilities(&self) -> &ResolvedClientCapabilities {
        &self.resolved_client_capabilities
    }

    pub fn query(&self) -> &index::DocumentQuery {
        &self.document_ref
    }

    pub(crate) fn encoding(&self) -> PositionEncoding {
        self.position_encoding
    }
}
