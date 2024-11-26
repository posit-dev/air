//
// documents.rs
//
// Copyright (C) 2022-2024 Posit Software, PBC. All rights reserved.
//
//

use anyhow::*;
use tower_lsp::lsp_types::DidChangeTextDocumentParams;
use tower_lsp::lsp_types::TextDocumentContentChangeEvent;

use crate::config::DocumentConfig;

#[derive(Clone)]
pub struct Document {
    pub syntax: (),

    // The version of the document we last synchronized with.
    // None if the document hasn't been synchronized yet.
    pub version: Option<i32>,

    // Configuration of the document, such as indentation settings.
    pub config: DocumentConfig,
}

impl std::fmt::Debug for Document {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Document")
            .field("syntax", &self.syntax)
            .finish()
    }
}

impl Document {
    pub fn new(_contents: &str, version: Option<i32>) -> Self {
        // TODO! Parse `contents`
        Self {
            syntax: (),
            version,
            config: Default::default(),
        }
    }

    pub fn on_did_change(&mut self, params: &DidChangeTextDocumentParams) {
        let new_version = params.text_document.version;

        // Check for out-of-order change notifications
        if let Some(old_version) = self.version {
            // According to the spec, versions might not be consecutive but they must be monotonically
            // increasing. If that's not the case this is a hard nope as we
            // can't maintain our state integrity. Currently panicking but in
            // principle we should shut down the LSP in an orderly fashion.
            if new_version < old_version {
                panic!(
                    "out-of-sync change notification: currently at {old_version}, got {new_version}"
                );
            }
        }

        for event in &params.content_changes {
            if let Err(err) = self.update(event) {
                panic!("Failed to update document: {err:?}");
            }
        }

        // Set new version
        self.version = Some(new_version);
    }

    fn update(&mut self, change: &TextDocumentContentChangeEvent) -> Result<()> {
        // Extract edit range. Nothing to do if there wasn't an edit.
        let _range = match change.range {
            Some(r) => r,
            None => return Ok(()),
        };

        // TODO!

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_document_starts_at_0_0_with_leading_whitespace() {
        let _document = Document::new("\n\n# hi there", None);
        // TODO!
        // let root = document.ast.root_node();
        // assert_eq!(root.start_position(), Point::new(0, 0));
    }
}
