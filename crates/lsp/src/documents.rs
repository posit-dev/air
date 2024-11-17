//
// documents.rs
//
// Copyright (C) 2022-2024 Posit Software, PBC. All rights reserved.
//
//

use tower_lsp::lsp_types;

use crate::config::DocumentConfig;
use crate::rust_analyzer::line_index::{LineEndings, LineIndex, PositionEncoding};
use crate::rust_analyzer::utils::apply_document_changes;

#[derive(Clone)]
pub struct Document {
    /// The normalized current contents of the document. UTF-8 Rust string with
    /// Unix line endings.
    pub contents: String,

    /// Map of new lines in `contents`. Also contains line endings type in the
    /// original document (we only store Unix lines) and the position encoding
    /// type of the session. This provides all that is needed to send data back
    /// to the client with positions in the correct coordinate space and
    /// correctly formatted text.
    pub line_index: LineIndex,

    /// We store the syntax tree in the document for now.
    /// We will think about laziness and incrementality in the future.
    pub parse: biome_parser::AnyParse,

    /// The version of the document we last synchronized with.
    /// None if the document hasn't been synchronized yet.
    pub version: Option<i32>,

    /// Configuration of the document, such as indentation settings.
    pub config: DocumentConfig,
}

impl std::fmt::Debug for Document {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Document")
            .field("syntax", &self.parse)
            .finish()
    }
}

impl Document {
    pub fn new(
        contents: String,
        version: Option<i32>,
        position_encoding: PositionEncoding,
    ) -> Self {
        // Normalize to Unix line endings
        let (contents, endings) = LineEndings::normalize(contents);

        // Create line index to keep track of newline offsets
        let line_index = LineIndex {
            index: triomphe::Arc::new(line_index::LineIndex::new(&contents)),
            endings,
            encoding: position_encoding,
        };

        // Parse document immediately for now
        let parse = air_r_parser::parse(&contents, Default::default());

        Self {
            contents,
            line_index,
            parse,
            version,
            config: Default::default(),
        }
    }

    /// For unit tests
    pub fn doodle(contents: &str) -> Self {
        Self::new(contents.into(), None, PositionEncoding::Utf8)
    }

    pub fn on_did_change(&mut self, mut params: lsp_types::DidChangeTextDocumentParams) {
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

        // Normalize line endings. Changing the line length of inserted or
        // replaced text can't invalidate the text change events, even those
        // applied subsequently, since those changes are specified with [line,
        // col] coordinates.
        for event in &mut params.content_changes {
            let text = std::mem::take(&mut event.text);
            event.text = LineEndings::normalize(text).0;
        }

        let contents = apply_document_changes(
            self.line_index.encoding,
            &self.contents,
            params.content_changes,
        );

        // No incrementality for now
        let parse = air_r_parser::parse(&contents, Default::default());

        self.parse = parse;
        self.contents = contents;
        self.version = Some(new_version);
    }
}

#[cfg(test)]
mod tests {
    use air_r_syntax::RSyntaxNode;
    use text_size::{TextRange, TextSize};

    use crate::rust_analyzer::text_edit::TextEdit;
    use crate::to_proto;

    use super::*;

    fn dummy_versioned_doc() -> lsp_types::VersionedTextDocumentIdentifier {
        lsp_types::VersionedTextDocumentIdentifier {
            uri: url::Url::parse("file:///foo").unwrap(),
            version: 1,
        }
    }

    #[test]
    fn test_document_starts_at_0_0_with_leading_whitespace() {
        let _document = Document::doodle("\n\n# hi there");
        // TODO!
        // let root = document.ast.root_node();
        // assert_eq!(root.start_position(), Point::new(0, 0));
    }

    #[test]
    fn test_document_syntax() {
        let mut doc = Document::doodle("foo(bar)");

        let original_syntax: RSyntaxNode = doc.parse.syntax();
        insta::assert_debug_snapshot!(original_syntax);

        let edit = TextEdit::replace(
            TextRange::new(TextSize::new(4), TextSize::new(7)),
            String::from("1 + 2"),
        );
        let edits = to_proto::doc_edit_vec(&doc.line_index, edit);

        let params = lsp_types::DidChangeTextDocumentParams {
            text_document: dummy_versioned_doc(),
            content_changes: edits,
        };
        doc.on_did_change(params);

        let updated_syntax: RSyntaxNode = doc.parse.syntax();
        insta::assert_debug_snapshot!(updated_syntax);
    }

    #[test]
    fn test_document_position_encoding() {
        // Replace `b` after `𐐀` which is at position 5 in UTF-8
        let utf8_range = lsp_types::Range {
            start: lsp_types::Position {
                line: 0,
                character: 5,
            },
            end: lsp_types::Position {
                line: 0,
                character: 6,
            },
        };

        // `b` is at position 3 in UTF-16
        let utf16_range = lsp_types::Range {
            start: lsp_types::Position {
                line: 0,
                character: 3,
            },
            end: lsp_types::Position {
                line: 0,
                character: 4,
            },
        };

        let mut utf8_replace_params = lsp_types::DidChangeTextDocumentParams {
            text_document: dummy_versioned_doc(),
            content_changes: vec![],
        };
        let mut utf16_replace_params = utf8_replace_params.clone();

        utf8_replace_params.content_changes = vec![lsp_types::TextDocumentContentChangeEvent {
            range: Some(utf8_range),
            range_length: None,
            text: String::from("bar"),
        }];
        utf16_replace_params.content_changes = vec![lsp_types::TextDocumentContentChangeEvent {
            range: Some(utf16_range),
            range_length: None,
            text: String::from("bar"),
        }];

        let mut document = Document::new("a𐐀b".into(), None, PositionEncoding::Utf8);
        document.on_did_change(utf8_replace_params);
        assert_eq!(document.contents, "a𐐀bar");

        let mut document = Document::new(
            "a𐐀b".into(),
            None,
            PositionEncoding::Wide(line_index::WideEncoding::Utf16),
        );
        document.on_did_change(utf16_replace_params);
        assert_eq!(document.contents, "a𐐀bar");
    }
}
