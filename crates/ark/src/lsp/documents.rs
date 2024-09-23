//
// document.rs
//
// Copyright (C) 2022-2024 Posit Software, PBC. All rights reserved.
//
//

use anyhow::*;
use ropey::Rope;
use tower_lsp::lsp_types::DidChangeTextDocumentParams;
use tower_lsp::lsp_types::TextDocumentContentChangeEvent;
use tree_sitter::InputEdit;
use tree_sitter::Parser;
use tree_sitter::Point;
use tree_sitter::Tree;

use crate::lsp::config::DocumentConfig;
use crate::lsp::encoding::convert_position_to_point;
use crate::lsp::traits::rope::RopeExt;

fn compute_point(point: Point, text: &str) -> Point {
    // figure out where the newlines in this edit are
    let newline_indices: Vec<_> = text.match_indices('\n').collect();
    let num_newlines = newline_indices.len();
    let num_bytes = text.as_bytes().len();

    if newline_indices.len() == 0 {
        return Point::new(point.row, point.column + num_bytes);
    } else {
        let last_newline_index = newline_indices.last().unwrap();
        return Point::new(
            point.row + num_newlines,
            num_bytes - last_newline_index.0 - 1,
        );
    }
}

#[derive(Clone)]
pub struct Document {
    // The document's textual contents.
    pub contents: Rope,

    // The document's AST.
    pub ast: Tree,

    // The version of the document we last synchronized with.
    // None if the document hasn't been synchronized yet.
    pub version: Option<i32>,

    // Configuration of the document, such as indentation settings.
    pub config: DocumentConfig,
}

impl std::fmt::Debug for Document {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Document")
            .field("contents", &self.contents)
            .field("ast", &self.ast)
            .finish()
    }
}

impl Document {
    pub fn new(contents: &str, version: Option<i32>) -> Self {
        // A one-shot parser, assumes the `Document` won't be incrementally reparsed.
        // Useful for testing, `with_document()`, and `index_file()`.
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_r::LANGUAGE.into())
            .unwrap();

        Self::new_with_parser(contents, &mut parser, version)
    }

    pub fn new_with_parser(contents: &str, parser: &mut Parser, version: Option<i32>) -> Self {
        let document = Rope::from(contents);
        let ast = parser.parse(contents, None).unwrap();

        Self {
            contents: document,
            version,
            ast,
            config: Default::default(),
        }
    }

    pub fn on_did_change(&mut self, parser: &mut Parser, params: &DidChangeTextDocumentParams) {
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
            if let Err(err) = self.update(parser, event) {
                panic!("Failed to update document: {err:?}");
            }
        }

        // Set new version
        self.version = Some(new_version);
    }

    fn update(
        &mut self,
        parser: &mut Parser,
        change: &TextDocumentContentChangeEvent,
    ) -> Result<()> {
        // Extract edit range. Nothing to do if there wasn't an edit.
        let range = match change.range {
            Some(r) => r,
            None => return Ok(()),
        };

        // Update the AST. We do this before updating the underlying document
        // contents, because edit computations need to be done using the current
        // state of the document (prior to the edit being applied) so that byte
        // offsets can be computed correctly.
        let ast = &mut self.ast;

        let start_point = convert_position_to_point(&self.contents, range.start);
        let start_byte = self.contents.point_to_byte(start_point);

        let old_end_point = convert_position_to_point(&self.contents, range.end);
        let old_end_byte = self.contents.point_to_byte(old_end_point);

        let new_end_point = compute_point(start_point, &change.text);
        let new_end_byte = start_byte + change.text.as_bytes().len();

        // Confusing tree sitter names, the `start_position` is really a `Point`
        let edit = InputEdit {
            start_byte,
            old_end_byte,
            new_end_byte,
            start_position: start_point,
            old_end_position: old_end_point,
            new_end_position: new_end_point,
        };

        ast.edit(&edit);

        // Now, apply edits to the underlying document.
        // Convert from byte offsets to character offsets.
        let start_character = self.contents.byte_to_char(start_byte);
        let old_end_character = self.contents.byte_to_char(old_end_byte);

        // Remove the old slice of text, and insert the new slice of text.
        self.contents.remove(start_character..old_end_character);
        self.contents.insert(start_character, change.text.as_str());

        // We've edited the AST, and updated the document. We can now re-parse.
        let contents = &self.contents;
        let callback = &mut |byte, point| Self::parse_callback(contents, byte, point);

        let ast = parser.parse_with(callback, Some(&self.ast));
        self.ast = ast.unwrap();

        Ok(())
    }

    /// A tree-sitter `parse_with()` callback to efficiently return a slice of the
    /// document in the `Rope` that tree-sitter can reparse with.
    ///
    /// According to the tree-sitter docs:
    /// * `callback` A function that takes a byte offset and position and
    ///   returns a slice of UTF8-encoded text starting at that byte offset
    ///   and position. The slices can be of any length. If the given position
    ///   is at the end of the text, the callback should return an empty slice.
    ///
    /// We expect that tree-sitter will call the callback again with an updated `byte`
    /// if the chunk doesn't contain enough text to fully reparse.
    fn parse_callback(contents: &Rope, byte: usize, point: Point) -> &[u8] {
        // Get Rope "chunk" that lines up with this `byte`
        let Some((chunk, chunk_byte_idx, _chunk_char_idx, _chunk_line_idx)) =
            contents.get_chunk_at_byte(byte)
        else {
            let contents = contents.to_string();
            log::error!(
                "Failed to get Rope chunk at byte {byte}, point {point}. Text '{contents}'.",
            );
            return "\n".as_bytes();
        };

        // How far into this chunk are we?
        let byte = byte - chunk_byte_idx;

        // Now return the slice from that `byte` to the end of the chunk.
        // SAFETY: This should never panic, since `get_chunk_at_byte()` worked.
        let slice = &chunk[byte..];

        slice.as_bytes()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_computation() {
        // empty strings shouldn't do anything
        let point = compute_point(Point::new(0, 0), "");
        assert_eq!(point, Point::new(0, 0));

        let point = compute_point(Point::new(42, 42), "");
        assert_eq!(point, Point::new(42, 42));

        // text insertion without newlines should just extend the column position
        let point = compute_point(Point::new(0, 0), "abcdef");
        assert_eq!(point, Point::new(0, 6));

        // text insertion with newlines should change the row
        let point = compute_point(Point::new(0, 0), "abc\ndef\nghi");
        assert_eq!(point, Point::new(2, 3));

        let point = compute_point(Point::new(0, 0), "abcdefghi\n");
        assert_eq!(point, Point::new(1, 0));
    }

    #[test]
    fn test_document_starts_at_0_0_with_leading_whitespace() {
        let document = Document::new("\n\n# hi there", None);
        let root = document.ast.root_node();
        assert_eq!(root.start_position(), Point::new(0, 0));
    }
}
