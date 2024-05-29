use std::sync::Arc;

use dashmap::DashMap;
use parking_lot::Mutex;
use url::Url;

use crate::lsp::documents::Document;

#[derive(Clone, Default, Debug)]
pub struct WorldState {
    pub documents: Arc<DashMap<Url, Document>>,
    pub workspace: Arc<Mutex<Workspace>>,
}

#[derive(Default, Debug)]
pub struct Workspace {
    pub folders: Vec<Url>,
}
