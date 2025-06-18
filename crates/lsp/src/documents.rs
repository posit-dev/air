//
// documents.rs
//
// Copyright (C) 2022-2024 Posit Software, PBC. All rights reserved.
//
//

use std::ops::Range;

use settings::LineEnding;
use tower_lsp::lsp_types;

use crate::proto::from_proto;
use crate::proto::PositionEncoding;
use crate::settings::DocumentSettings;

#[derive(Clone)]
pub struct Document {
    /// The normalized current contents of the document. UTF-8 Rust string with
    /// Unix line endings.
    pub contents: String,

    /// Index of new lines and non-UTF-8 characters in `contents`. Used for converting
    /// between line/col [tower_lsp::Position]s with a specified [PositionEncoding] to
    /// [biome_text_size::TextSize] offsets.
    pub line_index: biome_line_index::LineIndex,

    /// Original line endings, before normalization to Unix line endings
    pub endings: LineEnding,

    /// Encoding used by [tower_lsp::Position] `character` offsets
    pub position_encoding: PositionEncoding,

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
    pub fn new(
        contents: String,
        version: Option<i32>,
        position_encoding: PositionEncoding,
    ) -> Self {
        // Detect existing endings
        let endings = line_ending::infer(&contents);

        // Normalize to Unix line endings
        let contents = match endings {
            LineEnding::Lf => contents,
            LineEnding::Crlf => line_ending::normalize(contents),
        };

        // TODO: Handle user requested line ending preference here
        // by potentially overwriting `endings` if the user didn't
        // select `LineEndings::Auto`, and then pass that to `LineIndex`.

        // Create line index to keep track of newline offsets
        let line_index = biome_line_index::LineIndex::new(&contents);

        // Parse document immediately for now
        let parse = air_r_parser::parse(&contents, Default::default());

        Self {
            contents,
            line_index,
            endings,
            position_encoding,
            parse,
            version,
            settings: Default::default(),
        }
    }

    /// For unit tests
    pub fn doodle(contents: &str) -> Self {
        Self::new(contents.into(), None, PositionEncoding::Utf8)
    }

    #[cfg(test)]
    pub fn doodle_and_range(contents: &str) -> (Self, biome_text_size::TextRange) {
        let (contents, range) = crate::test::extract_marked_range(contents);
        let doc = Self::new(contents, None, PositionEncoding::Utf8);
        (doc, range)
    }

    // --- source
    // authors = ["rust-analyzer team"]
    // license = "MIT OR Apache-2.0"
    // origin = "https://github.com/rust-lang/rust-analyzer/blob/master/crates/rust-analyzer/src/lsp/utils.rs"
    // ---
    pub fn on_did_change(
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

        // If at least one of the changes is a full document change, use the last of them
        // as the starting point and ignore all previous changes. We then know that all
        // changes after this (if any!) are incremental changes.
        //
        // If we do have a full document change, that implies the `last_start_line`
        // corresponding to that change is line 0, which will correctly force a rebuild
        // of the line index before applying any incremental changes. We don't go ahead
        // and rebuild the line index here, because it is guaranteed to be rebuilt for
        // us on the way out.
        let (changes, mut last_start_line) =
            match changes.iter().rposition(|change| change.range.is_none()) {
                Some(idx) => {
                    let incremental = changes.split_off(idx + 1);
                    // Unwrap: `rposition()` confirmed this index contains a full document change
                    let change = changes.pop().unwrap();
                    self.contents = line_ending::normalize(change.text);
                    (incremental, 0)
                }
                None => (changes, u32::MAX),
            };

        // Handle all incremental changes after the last full document change. We don't
        // typically get >1 incremental change as the user types, but we do get them in a
        // batch after a find-and-replace, or after a format-on-save request.
        //
        // Some editors like VS Code send the edits in reverse order (from the bottom of
        // file -> top of file). We can take advantage of this, because applying an edit
        // on, say, line 10, doesn't invalidate the `line_index` if we then need to apply
        // an additional edit on line 5. That said, we may still have edits that cross
        // lines, so rebuilding the `line_index` is not always unavoidable.
        //
        // We also normalize line endings. Changing the line length of inserted or
        // replaced text can't invalidate the text change events since the location of the
        // change itself is specified with [line, col] coordinates, separate from the
        // actual contents of the change.
        for change in changes {
            let range = change
                .range
                .expect("`None` case already handled by finding the last full document change.");

            // If the end of this change is at or past the start of the last change, then
            // the `line_index` needed to apply this change is now invalid, so we have to
            // rebuild it.
            if range.end.line >= last_start_line {
                self.line_index = biome_line_index::LineIndex::new(&self.contents);
            }
            last_start_line = range.start.line;

            // This is a panic if we can't convert. It means we can't keep the document up
            // to date and something is very wrong.
            let range: Range<usize> =
                from_proto::text_range(range, &self.line_index, self.position_encoding)
                    .expect("Can convert `range` from `Position` to `TextRange`.")
                    .into();

            self.contents
                .replace_range(range, &line_ending::normalize(change.text));
        }

        // Rebuild the `line_index` after applying the final edit, and sync other fields
        self.line_index = biome_line_index::LineIndex::new(&self.contents);
        self.parse = air_r_parser::parse(&self.contents, Default::default());
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

    use crate::proto::to_proto;
    use crate::text_edit::TextEdit;

    use super::*;

    #[test]
    fn test_document_starts_at_0_with_leading_whitespace() {
        let document = Document::doodle("\n\n# hi there");
        let root = document.syntax();
        assert_eq!(
            root.text_range_with_trivia(),
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
        let edits =
            to_proto::doc_edit_vec(edit, &doc.line_index, doc.position_encoding, doc.endings)
                .unwrap();
        doc.on_did_change(edits, 1);

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

        let mut document = Document::new("a𐐀b".into(), None, PositionEncoding::Utf8);
        document.on_did_change(utf8_content_changes, 1);
        assert_eq!(document.contents, "a𐐀bar");

        let mut document = Document::new(
            "a𐐀b".into(),
            None,
            PositionEncoding::Wide(biome_line_index::WideEncoding::Utf16),
        );
        document.on_did_change(utf16_content_changes, 1);
        assert_eq!(document.contents, "a𐐀bar");
    }
}
