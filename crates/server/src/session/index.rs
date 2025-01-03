use std::borrow::Cow;
use std::path::PathBuf;
use std::{path::Path, sync::Arc};

use lsp_types::Url;
use lsp_types::WorkspaceFolder;
use rustc_hash::FxHashMap;

use workspace::settings::Settings;

use crate::document::{DocumentKey, DocumentVersion, PositionEncoding, TextDocument};
use crate::session::workspaces::WorkspaceSettingsResolver;

/// Stores and tracks all open documents in a session, along with their associated settings.
#[derive(Default)]
pub(crate) struct Index {
    /// Maps all document file URLs to the associated document controller
    documents: FxHashMap<Url, DocumentController>,

    /// Maps a workspace folder root to its settings.
    settings: WorkspaceSettingsResolver,
}

/// A mutable handler to an underlying document.
#[derive(Debug)]
enum DocumentController {
    Text(Arc<TextDocument>),
}

/// A read-only query to an open document.
/// This query can 'select' a text document, but eventually could gain support for
/// selecting notebooks or individual notebook cells.
/// It also includes document settings.
#[derive(Clone)]
pub enum DocumentQuery {
    Text {
        file_url: Url,
        document: Arc<TextDocument>,
        settings: Arc<Settings>,
    },
}

impl Index {
    pub(super) fn new(workspace_folders: Vec<WorkspaceFolder>) -> anyhow::Result<Self> {
        Ok(Self {
            documents: FxHashMap::default(),
            settings: WorkspaceSettingsResolver::from_workspace_folders(workspace_folders),
        })
    }

    pub(super) fn text_document_urls(&self) -> impl Iterator<Item = &Url> + '_ {
        self.documents
            .iter()
            .filter(|(_, doc)| doc.as_text().is_some())
            .map(|(url, _)| url)
    }

    pub(super) fn update_text_document(
        &mut self,
        key: &DocumentKey,
        content_changes: Vec<lsp_types::TextDocumentContentChangeEvent>,
        new_version: DocumentVersion,
        encoding: PositionEncoding,
    ) -> anyhow::Result<()> {
        let controller = self.document_controller_for_key(key)?;
        let Some(document) = controller.as_text_mut() else {
            anyhow::bail!("Text document URI does not point to a text document");
        };
        document.apply_changes(content_changes, new_version, encoding);
        Ok(())
    }

    pub(super) fn key_from_url(&self, url: Url) -> DocumentKey {
        DocumentKey::Text(url)
    }

    pub(super) fn open_workspace_folder(&mut self, url: &Url) {
        self.settings.open_workspace_folder(url)
    }

    pub(super) fn close_workspace_folder(&mut self, url: &Url) {
        self.settings.close_workspace_folder(url)
    }

    pub(super) fn num_documents(&self) -> usize {
        self.documents.len()
    }

    pub(super) fn num_workspaces(&self) -> usize {
        self.settings.len()
    }

    pub(super) fn make_document_ref(&self, key: DocumentKey) -> Option<DocumentQuery> {
        let url = self.url_for_key(&key)?.clone();

        let settings = self.settings_for_url(&url);

        let controller = self.documents.get(&url)?;
        Some(controller.make_ref(url, settings))
    }

    /// Reloads relevant existing settings files based on a changed settings file path.
    pub(super) fn reload_settings(&mut self, changed_url: &Url) {
        self.settings.reload_workspaces_matched_by_url(changed_url);
    }

    pub(super) fn open_text_document(&mut self, url: Url, document: TextDocument) {
        self.documents
            .insert(url, DocumentController::new_text(document));
    }

    pub(super) fn close_document(&mut self, key: &DocumentKey) -> anyhow::Result<()> {
        let Some(url) = self.url_for_key(key).cloned() else {
            anyhow::bail!("Tried to close unavailable document `{key}`");
        };

        let Some(_) = self.documents.remove(&url) else {
            anyhow::bail!("tried to close document that didn't exist at {}", url)
        };
        Ok(())
    }

    // TODO: Index should manage per workspace client settings at some point once we have some
    // pub(super) fn client_settings(
    //     &self,
    //     key: &DocumentKey,
    //     global_settings: &ClientSettings,
    // ) -> ResolvedClientSettings {
    //     let Some(url) = self.url_for_key(key) else {
    //         return ResolvedClientSettings::global(global_settings);
    //     };
    //     let Some(WorkspaceSettings {
    //         client_settings, ..
    //     }) = self.settings_for_url(url)
    //     else {
    //         return ResolvedClientSettings::global(global_settings);
    //     };
    //     client_settings.clone()
    // }

    fn document_controller_for_key(
        &mut self,
        key: &DocumentKey,
    ) -> anyhow::Result<&mut DocumentController> {
        let Some(url) = self.url_for_key(key).cloned() else {
            anyhow::bail!("Tried to open unavailable document `{key}`");
        };
        let Some(controller) = self.documents.get_mut(&url) else {
            anyhow::bail!("Document controller not available at `{}`", url);
        };
        Ok(controller)
    }

    fn url_for_key<'a>(&'a self, key: &'a DocumentKey) -> Option<&'a Url> {
        match key {
            DocumentKey::Text(path) => Some(path),
        }
    }

    fn settings_for_url(&self, url: &Url) -> Arc<Settings> {
        self.settings.settings_for_url(url)
    }
}

impl DocumentController {
    fn new_text(document: TextDocument) -> Self {
        Self::Text(Arc::new(document))
    }

    fn make_ref(&self, file_url: Url, settings: Arc<Settings>) -> DocumentQuery {
        match &self {
            Self::Text(document) => DocumentQuery::Text {
                file_url,
                document: document.clone(),
                settings,
            },
        }
    }

    pub(crate) fn as_text(&self) -> Option<&TextDocument> {
        match self {
            Self::Text(document) => Some(document),
        }
    }

    pub(crate) fn as_text_mut(&mut self) -> Option<&mut TextDocument> {
        Some(match self {
            Self::Text(document) => Arc::make_mut(document),
        })
    }
}

impl DocumentQuery {
    /// Get the document settings associated with this query.
    pub(crate) fn settings(&self) -> &Settings {
        // Note that `&Arc<Settings>` nicely derefs to `&Settings` here automatically
        match self {
            Self::Text { settings, .. } => settings,
        }
    }

    /// Get the version of document selected by this query.
    #[allow(dead_code)]
    pub(crate) fn version(&self) -> DocumentVersion {
        match self {
            Self::Text { document, .. } => document.version(),
        }
    }

    /// Get the URL for the document selected by this query.
    pub(crate) fn file_url(&self) -> &Url {
        match self {
            Self::Text { file_url, .. } => file_url,
        }
    }

    /// Get the path for the document selected by this query.
    ///
    /// Returns `None` if this is an unsaved (untitled) document.
    ///
    /// The path isn't guaranteed to point to a real path on the filesystem. This is the case
    /// for unsaved (untitled) documents.
    #[allow(dead_code)]
    pub(crate) fn file_path(&self) -> Option<PathBuf> {
        self.file_url().to_file_path().ok()
    }

    /// Get the path for the document selected by this query, ignoring whether the file exists on disk.
    ///
    /// Returns the URL's path if this is an unsaved (untitled) document.
    #[allow(dead_code)]
    pub(crate) fn virtual_file_path(&self) -> Cow<Path> {
        self.file_path().map_or_else(
            || Cow::Borrowed(Path::new(self.file_url().path())),
            Cow::Owned,
        )
    }

    /// Attempt to access the single inner text document selected by the query.
    pub(crate) fn as_single_document(&self) -> &TextDocument {
        match self {
            Self::Text { document, .. } => document,
        }
    }
}
