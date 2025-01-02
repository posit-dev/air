use biome_rowan::TextRange;
use lsp_types::TextDocumentContentChangeEvent;
use source_file::LineEnding;
use source_file::LineIndex;

use crate::edit::PositionEncoding;
use crate::proto::TextRangeExt;

pub(crate) type DocumentVersion = i32;

/// The state of an individual document in the server. Stays up-to-date
/// with changes made by the user, including unsaved changes.
#[derive(Debug, Clone)]
pub struct TextDocument {
    /// The string contents of the document, normalized to unix line endings.
    contents: String,
    /// The original line endings of the document.
    ending: LineEnding,
    /// A computed line index for the document. This should always reflect
    /// the current version of `contents`. Using a function like [`Self::modify`]
    /// will re-calculate the line index automatically when the `contents` value is updated.
    index: LineIndex,
    /// The latest version of the document, set by the LSP client. The server will panic in
    /// debug mode if we attempt to update the document with an 'older' version.
    version: DocumentVersion,
}

impl TextDocument {
    pub fn new(contents: String, version: DocumentVersion) -> Self {
        // Normalize to Unix line endings
        let (contents, ending) = source_file::normalize_newlines(contents);
        let index = LineIndex::from_source_text(&contents);
        Self {
            contents,
            ending,
            index,
            version,
        }
    }

    #[cfg(test)]
    pub fn doodle(contents: &str) -> Self {
        Self::new(contents.into(), 0)
    }

    #[cfg(test)]
    pub fn doodle_and_range(contents: &str) -> (Self, biome_text_size::TextRange) {
        let (contents, range) = crate::test::extract_marked_range(contents);
        let doc = Self::new(contents, 0);
        (doc, range)
    }

    pub fn contents(&self) -> &str {
        &self.contents
    }

    pub fn ending(&self) -> LineEnding {
        self.ending
    }

    pub fn index(&self) -> &LineIndex {
        &self.index
    }

    pub fn version(&self) -> DocumentVersion {
        self.version
    }

    pub fn apply_changes(
        &mut self,
        mut changes: Vec<lsp_types::TextDocumentContentChangeEvent>,
        new_version: DocumentVersion,
        encoding: PositionEncoding,
    ) {
        // Normalize line endings. Changing the line length of inserted or
        // replaced text can't invalidate the text change events, even those
        // applied subsequently, since those changes are specified with [line,
        // col] coordinates.
        for change in &mut changes.iter_mut() {
            let text = std::mem::take(&mut change.text);
            (change.text, _) = source_file::normalize_newlines(text);
        }

        if let [lsp_types::TextDocumentContentChangeEvent {
            range: None, text, ..
        }] = changes.as_slice()
        {
            tracing::trace!("Fast path - replacing entire document");
            self.modify(|contents, version| {
                contents.clone_from(text);
                *version = new_version;
            });
            return;
        }

        let mut new_contents = self.contents().to_string();
        let mut active_index = self.index().clone();

        for TextDocumentContentChangeEvent {
            range,
            text: change,
            ..
        } in changes
        {
            if let Some(range) = range {
                let range = TextRange::from_proto(range, &new_contents, &active_index, encoding);

                new_contents.replace_range(
                    usize::from(range.start())..usize::from(range.end()),
                    &change,
                );
            } else {
                new_contents = change;
            }

            active_index = LineIndex::from_source_text(&new_contents);
        }

        self.modify_with_manual_index(|contents, version, index| {
            *index = active_index;
            *contents = new_contents;
            *version = new_version;
        });
    }

    pub fn update_version(&mut self, new_version: DocumentVersion) {
        self.modify_with_manual_index(|_, version, _| {
            *version = new_version;
        });
    }

    // A private function for modifying the document's internal state
    fn modify(&mut self, func: impl FnOnce(&mut String, &mut DocumentVersion)) {
        self.modify_with_manual_index(|c, v, i| {
            func(c, v);
            *i = LineIndex::from_source_text(c);
        });
    }

    // A private function for overriding how we update the line index by default.
    fn modify_with_manual_index(
        &mut self,
        func: impl FnOnce(&mut String, &mut DocumentVersion, &mut LineIndex),
    ) {
        let old_version = self.version;
        func(&mut self.contents, &mut self.version, &mut self.index);
        debug_assert!(self.version >= old_version);
    }
}

#[cfg(test)]
mod tests {
    use crate::edit::{PositionEncoding, TextDocument};
    use lsp_types::{Position, TextDocumentContentChangeEvent};

    #[test]
    fn redo_edit() {
        let mut document = TextDocument::new(
            r#""""
测试comment
一些测试内容
"""
import click


@click.group()
def interface():
    pas
"#
            .to_string(),
            0,
        );

        // Add an `s`, remove it again (back to the original code), and then re-add the `s`
        document.apply_changes(
            vec![
                TextDocumentContentChangeEvent {
                    range: Some(lsp_types::Range::new(
                        Position::new(9, 7),
                        Position::new(9, 7),
                    )),
                    range_length: Some(0),
                    text: "s".to_string(),
                },
                TextDocumentContentChangeEvent {
                    range: Some(lsp_types::Range::new(
                        Position::new(9, 7),
                        Position::new(9, 8),
                    )),
                    range_length: Some(1),
                    text: String::new(),
                },
                TextDocumentContentChangeEvent {
                    range: Some(lsp_types::Range::new(
                        Position::new(9, 7),
                        Position::new(9, 7),
                    )),
                    range_length: Some(0),
                    text: "s".to_string(),
                },
            ],
            1,
            PositionEncoding::UTF16,
        );

        assert_eq!(
            &document.contents,
            r#""""
测试comment
一些测试内容
"""
import click


@click.group()
def interface():
    pass
"#
        );
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

        let utf8_replace_params = vec![lsp_types::TextDocumentContentChangeEvent {
            range: Some(utf8_range),
            range_length: None,
            text: String::from("bar"),
        }];
        let utf16_replace_params = vec![lsp_types::TextDocumentContentChangeEvent {
            range: Some(utf16_range),
            range_length: None,
            text: String::from("bar"),
        }];

        let mut document = TextDocument::new("a𐐀b".into(), 1);
        document.apply_changes(
            utf8_replace_params,
            document.version + 1,
            PositionEncoding::UTF8,
        );
        assert_eq!(document.contents(), "a𐐀bar");

        let mut document = TextDocument::new("a𐐀b".into(), 1);
        document.apply_changes(
            utf16_replace_params,
            document.version + 1,
            PositionEncoding::UTF16,
        );
        assert_eq!(document.contents, "a𐐀bar");
    }
}
