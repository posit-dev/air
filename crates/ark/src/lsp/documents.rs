//
// document.rs
//
// Copyright (C) 2022 Posit Software, PBC. All rights reserved.
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

pub struct Document {
    // The document's textual contents.
    pub contents: Rope,

    // A set of pending changes for this document.
    pub pending: Vec<DidChangeTextDocumentParams>,

    // The version of the document we last synchronized with.
    // None if the document hasn't been synchronized yet.
    pub version: Option<i32>,

    // The parser used to generate the AST.
    pub parser: Parser,

    // The document's AST.
    pub ast: Tree,
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
        // create initial document from rope
        let document = Rope::from(contents);

        // create a parser for this document
        let mut parser = Parser::new();
        parser
            .set_language(tree_sitter_r::language())
            .expect("failed to create parser");
        let ast = parser.parse(contents, None).unwrap();

        let pending = Vec::new();

        // return generated document
        Self {
            contents: document,
            pending,
            version,
            parser,
            ast,
        }
    }

    pub fn on_did_change(&mut self, params: &DidChangeTextDocumentParams) -> Result<i32> {
        // Add pending changes.
        self.pending.push(params.clone());

        // Check the version of this update.
        //
        // If we receive version {n + 2} before {n + 1}, then we'll
        // bail here, and handle the {n + 2} change after we received
        // version {n + 1}.
        //
        // TODO: What if an intermediate document change is somehow dropped or lost?
        // Do we need a way to recover (e.g. reset the document state)?
        if let Some(old_version) = self.version {
            let new_version = params.text_document.version;
            if new_version > old_version + 1 {
                log::info!("on_did_change(): received out-of-order document changes; currently at {}; deferring {}", old_version, new_version);
                return Ok(old_version);
            }
        }

        // Get pending updates, sort by version, and then apply as many as we can.
        self.pending.sort_by(|lhs, rhs| {
            let lhs = lhs.text_document.version;
            let rhs = rhs.text_document.version;
            lhs.cmp(&rhs)
        });

        // Apply as many changes as we can, bailing if we hit a non consecutive change.
        let pending = std::mem::take(&mut self.pending);

        // We know there is at least 1 consecutive change to apply so that can serve
        // as the initial version since we don't always have a `self.version`.
        let mut loc = 0;
        let mut version = pending.first().unwrap().text_document.version - 1;

        for candidate in pending.iter() {
            let new_version = candidate.text_document.version;

            if new_version > version + 1 {
                // Not consecutive!
                log::info!(
                    "on_did_change(): applying changes [{}, {}]; deferring still out-of-order change {}.",
                    pending.first().unwrap().text_document.version,
                    version,
                    new_version
                );
                break;
            }

            loc += 1;
            version = new_version;
        }

        // Split into the actual changes we can apply and the remaining pending changes.
        let (changes, pending) = pending.split_at(loc);

        // We will still have to apply these later (if any).
        self.pending = pending.to_vec();

        // Apply the changes one-by-one.
        for change in changes {
            let content_changes = &change.content_changes;

            for event in content_changes {
                if let Err(error) = self.update(event) {
                    log::error!("error updating document: {}", error);
                }
            }
        }

        // Updates successfully applied; update cached document version.
        self.version = Some(version);

        Ok(version)
    }

    fn update(&mut self, change: &TextDocumentContentChangeEvent) -> Result<()> {
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

        let ast = self.parser.parse_with(callback, Some(&self.ast));
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
}
