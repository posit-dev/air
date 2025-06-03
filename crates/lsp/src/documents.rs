//
// documents.rs
//
// Copyright (C) 2022-2024 Posit Software, PBC. All rights reserved.
//
//

use settings::LineEnding;
use source_file::LineOffsetEncoding;
use source_file::SourceFile;
use tower_lsp::lsp_types;

use crate::settings::DocumentSettings;

#[derive(Clone)]
pub struct Document {
    /// The normalized current contents of the document. UTF-8 Rust string with
    /// Unix line endings.
    pub source_file: SourceFile,

    /// The encoding negotiated with the client for this document used when converting
    /// line offsets to and from the client i.e. for ([lsp_types::Position] <->
    /// [biome_text_size::TextSize])
    pub encoding: LineOffsetEncoding,

    /// The line endings used in the [SourceFile]. Always [LineEnding::Lf] due to
    /// up front normalization, but we pull from this to pass to other helpers, so
    /// we keep it around.
    pub endings: LineEnding,

    /// We store the syntax tree in the document for now.
    /// We will think about laziness and incrementality in the future.
    pub parse: air_r_parser::Parse,

    /// The version of the document we last synchronized with.
    /// None if the document hasn't been synchronized yet.
    pub version: Option<i32>,

    /// Settings of the document, such as indentation settings.
    pub settings: DocumentSettings,
}

impl std::fmt::Debug for Document {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Document")
            .field("syntax", &self.parse)
            .finish()
    }
}

impl Document {
    pub fn new(contents: String, version: Option<i32>, encoding: LineOffsetEncoding) -> Self {
        // Detect existing endings
        let endings = line_ending::infer(&contents);

        // Normalize to Unix line endings
        let contents = match endings {
            LineEnding::Lf => contents,
            LineEnding::Crlf => line_ending::normalize(contents),
        };

        // Always Unix line endings
        let endings = LineEnding::Lf;

        let source_file = SourceFile::new(contents);

        // Parse document immediately for now
        let parse = air_r_parser::parse(source_file.contents(), Default::default());

        Self {
            source_file,
            encoding,
            endings,
            parse,
            version,
            settings: Default::default(),
        }
    }

    /// For unit tests
    pub fn doodle(contents: &str) -> Self {
        Self::new(contents.into(), None, LineOffsetEncoding::UTF8)
    }

    #[cfg(test)]
    pub fn doodle_and_range(contents: &str) -> (Self, biome_text_size::TextRange) {
        let (contents, range) = crate::test::extract_marked_range(contents);
        let doc = Self::new(contents, None, LineOffsetEncoding::UTF8);
        (doc, range)
    }

    pub fn apply_changes(
        &mut self,
        mut changes: Vec<lsp_types::TextDocumentContentChangeEvent>,
        new_version: i32,
    ) {
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
        for event in &mut changes {
            let text = std::mem::take(&mut event.text);
            event.text = line_ending::normalize(text);
        }

        if let [lsp_types::TextDocumentContentChangeEvent { range: None, .. }] = changes.as_slice()
        {
            tracing::trace!("Fast path - replacing entire document");
            // Unwrap: If-let ensures there is exactly 1 change event
            let change = changes.pop().unwrap();
            self.source_file = SourceFile::new(change.text);
        } else {
            // Handle all changes individually
            for lsp_types::TextDocumentContentChangeEvent { range, text, .. } in changes {
                if let Some(range) = range {
                    // Replace a range and reanalyze the line starts
                    let range =
                        crate::from_proto::text_range(range, &self.source_file, self.encoding);
                    self.source_file
                        .replace_range(usize::from(range.start())..usize::from(range.end()), &text);
                } else {
                    // Replace the whole file
                    self.source_file = SourceFile::new(text);
                }
            }
        }

        // Update other fields
        self.parse = air_r_parser::parse(self.source_file.contents(), Default::default());
        self.version = Some(new_version);
    }

    /// Convenient accessor that returns an annotated `SyntaxNode` type
    pub fn syntax(&self) -> air_r_syntax::RSyntaxNode {
        self.parse.syntax()
    }
}

#[cfg(test)]
mod tests {
    use air_r_syntax::RSyntaxNode;
    use biome_text_size::{TextRange, TextSize};

    use crate::text_edit::TextEdit;
    use crate::to_proto;

    use super::*;

    #[test]
    fn test_document_starts_at_0_with_leading_whitespace() {
        let document = Document::doodle("\n\n# hi there");
        let root = document.syntax();
        assert_eq!(
            root.text_range(),
            TextRange::new(TextSize::from(0), TextSize::from(12))
        );
    }

    #[test]
    fn test_document_syntax() {
        let mut doc = Document::doodle("foo(bar)");

        let original_syntax: RSyntaxNode = doc.parse.syntax();
        insta::assert_debug_snapshot!(original_syntax);

        let edit = TextEdit::replace(
            TextRange::new(TextSize::from(4_u32), TextSize::from(7)),
            String::from("1 + 2"),
        );
        let edits = to_proto::doc_change_vec(edit, &doc.source_file, doc.encoding, doc.endings);
        doc.apply_changes(edits, 1);

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

        let utf8_content_changes = vec![lsp_types::TextDocumentContentChangeEvent {
            range: Some(utf8_range),
            range_length: None,
            text: String::from("bar"),
        }];
        let utf16_content_changes = vec![lsp_types::TextDocumentContentChangeEvent {
            range: Some(utf16_range),
            range_length: None,
            text: String::from("bar"),
        }];

        let mut document = Document::new("a𐐀b".into(), None, LineOffsetEncoding::UTF8);
        document.apply_changes(utf8_content_changes, 1);
        assert_eq!(document.source_file.contents(), "a𐐀bar");

        let mut document = Document::new("a𐐀b".into(), None, LineOffsetEncoding::UTF16);
        document.apply_changes(utf16_content_changes, 1);
        assert_eq!(document.source_file.contents(), "a𐐀bar");
    }
}
