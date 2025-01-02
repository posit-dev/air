use url::Url;

/// A unique document ID, derived from a URL passed as part of an LSP request.
/// This document ID currently always points to an R file, but eventually can also
/// point to a full notebook, or a cell within a notebook.
#[derive(Clone, Debug)]
pub(crate) enum DocumentKey {
    Text(Url),
    // If we ever want to support notebooks, start here:
    // Notebook(Url),
    // NotebookCell(Url),
}

impl std::fmt::Display for DocumentKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Text(url) => url.fmt(f),
        }
    }
}
