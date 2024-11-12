//
// encoding.rs
//
// Copyright (C) 2024 Posit Software, PBC. All rights reserved.
//
//

use tower_lsp::lsp_types;

/// `PositionEncodingKind` describes the encoding used for the `Position` `character`
/// column offset field. The `Position` `line` field is encoding agnostic, but the
/// `character` field specifies the number of characters offset from the beginning of
/// the line, and the "character" size is dependent on the encoding. The LSP specification
/// states:
///
/// - UTF8: Character offsets count UTF-8 code units (e.g. bytes).
/// - UTF16: Character offsets count UTF-16 code units (default).
/// - UTF32: Character offsets count UTF-32 code units (these are the same as Unicode
///   codepoints, so this `PositionEncodingKind` may also be used for an encoding-agnostic
///   representation of character offsets.)
///
/// The `vscode-languageclient` library that Positron uses on the frontend to create the
/// `Client` side of the LSP currently ONLY supports `UTF16`, and will error on anything
/// else. Their reasoning is that it is easier for the server (ark) to do the re-encoding,
/// since we are tracking the full document state. Track support for UTF-8 here:
/// https://github.com/microsoft/vscode-languageserver-node/issues/1224
///
/// The other interesting part of this is that `TextDocumentContentChangeEvent`s that
/// come through the `did_change()` event and the `TextDocumentItem` that comes through
/// the `did_open()` event encode the `text` of the change/document in UTF-8, even though
/// the `Range` (in the case of `did_change()`) that tells you where to apply the change
/// uses UTF-16, so that's cool. UTF-8 `text` is forced to come through due to how the
/// LSP specification uses jsonrpc, where the content fields must be 'utf-8' encoded:
/// https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#contentPart
/// This at least means we have a guarantee that the document itself and any updates to
/// it will be encoded in UTF-8, even if the Positions are UTF-16.
///
/// So we need a way to convert the UTF-16 `Position`s to UTF-8 `tree_sitter::Point`s and
/// back. This requires the document itself, and is what the helpers in this file implement.
pub fn get_position_encoding_kind() -> lsp_types::PositionEncodingKind {
    lsp_types::PositionEncodingKind::UTF16
}

/// Converts a character offset into a particular line from UTF-16 to UTF-8
fn convert_character_from_utf16_to_utf8(x: &str, character: usize) -> usize {
    if x.is_ascii() {
        // Fast pass
        return character;
    }

    // Initial check, since loop would skip this case
    if character == 0 {
        return character;
    }

    let mut n = 0;

    // For each `u32` sized `char`, figure out the equivalent size in UTF-16
    // world of that `char`. Once we hit the requested number of `character`s,
    // that means we have indexed into `x` to the correct position, at which
    // point we can take the current bytes based `pos` that marks the start of
    // this `char`, and add on its UTF-8 based size to return an adjusted column
    // offset. We use `==` because I'm fairly certain they should always align
    // exactly, and it would be good to log if that isn't the case.
    for (pos, char) in x.char_indices() {
        n += char.len_utf16();

        if n == character {
            return pos + char.len_utf8();
        }
    }

    log::error!("Failed to locate UTF-16 offset of {character}. Line: '{x}'.");
    return 0;
}

/// Converts a character offset into a particular line from UTF-8 to UTF-16
fn convert_character_from_utf8_to_utf16(x: &str, character: usize) -> usize {
    if x.is_ascii() {
        // Fast pass
        return character;
    }

    // The UTF-8 -> UTF-16 case is slightly simpler. We just slice into `x`
    // using our existing UTF-8 offset, reencode the slice as a UTF-16 based
    // iterator, and count up the pieces.
    match x.get(..character) {
        Some(x) => x.encode_utf16().count(),
        None => {
            let n = x.len();
            log::error!(
                "Tried to take UTF-8 character {character}, but only {n} characters exist. Line: '{x}'."
            );
            0
        }
    }
}
