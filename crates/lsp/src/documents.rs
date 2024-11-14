//
// documents.rs
//
// Copyright (C) 2022-2024 Posit Software, PBC. All rights reserved.
//
//

use tower_lsp::lsp_types;

use crate::config::DocumentConfig;
use crate::rust_analyzer::line_index::PositionEncoding;
use crate::rust_analyzer::utils::apply_document_changes;

#[derive(Clone)]
pub struct Document {
    pub contents: String,

    // FIXME: We'd ideally store the `GreenNode` but this type has
    // been made private in https://github.com/rome/tools/pull/1736.
    // Since `SyntaxNode` is not `Send`, we can't store them in the
    // world state.
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
    pub fn new(contents: String, version: Option<i32>) -> Self {
        Self {
            contents,
            syntax: (),
            version,
            config: Default::default(),
        }
    }

    pub fn on_did_change(&mut self, params: lsp_types::DidChangeTextDocumentParams) {
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

        let contents = apply_document_changes(
            PositionEncoding::Utf8, // TODO!
            &self.contents,
            params.content_changes,
        );

        self.contents = contents;
        self.version = Some(new_version);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_document_starts_at_0_0_with_leading_whitespace() {
        let _document = Document::new("\n\n# hi there".into(), None);
        // TODO!
        // let root = document.ast.root_node();
        // assert_eq!(root.start_position(), Point::new(0, 0));
    }
}
