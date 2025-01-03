use biome_rowan::TextRange;
use lsp_types::TextDocumentContentChangeEvent;
use source_file::LineEnding;
use source_file::SourceFile;

use crate::document::PositionEncoding;
use crate::proto::TextRangeExt;

pub(crate) type DocumentVersion = i32;

/// The state of an individual document in the server. Stays up-to-date
/// with changes made by the user, including unsaved changes.
#[derive(Debug, Clone)]
pub struct TextDocument {
    /// The source file containing the contents and line index for the document.
    /// Line endings have been normalized to unix line endings here.
    source: SourceFile,
    /// The original line endings of the document. Used when sending changes back to the
    /// LSP client.
    ending: LineEnding,
    /// The latest version of the document, set by the LSP client. The server will panic in
    /// debug mode if we attempt to update the document with an 'older' version.
    version: DocumentVersion,
}

impl TextDocument {
    pub fn new(contents: String, version: DocumentVersion) -> Self {
        // Normalize to Unix line endings on the way in
        let (contents, ending) = source_file::normalize_newlines(contents);
        let source = SourceFile::new(contents);
        Self {
            source,
            ending,
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
        self.source.contents()
    }

    pub fn ending(&self) -> LineEnding {
        self.ending
    }

    pub fn source_file(&self) -> &SourceFile {
        &self.source
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

        if let [lsp_types::TextDocumentContentChangeEvent { range: None, .. }] = changes.as_slice()
        {
            tracing::trace!("Fast path - replacing entire document");
            // Unwrap: If-let ensures there is exactly 1 change event
            let change = changes.pop().unwrap();
            self.source = SourceFile::new(change.text);
            self.update_version(new_version);
            return;
        }

        for TextDocumentContentChangeEvent {
            range,
            text: change,
            ..
        } in changes
        {
            if let Some(range) = range {
                // Replace a range and rebuild the line index
                let range = TextRange::from_proto(range, &self.source, encoding);
                self.source.replace_range(
                    usize::from(range.start())..usize::from(range.end()),
                    &change,
                );
            } else {
                // Replace the whole file
                self.source = SourceFile::new(change);
            }
        }

        self.update_version(new_version);
    }

    fn update_version(&mut self, new_version: DocumentVersion) {
        let old_version = self.version;
        self.version = new_version;
        debug_assert!(self.version >= old_version);
    }
}

#[cfg(test)]
mod tests {
    use crate::document::{PositionEncoding, TextDocument};
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
            document.contents(),
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
        assert_eq!(document.contents(), "a𐐀bar");
    }
}
