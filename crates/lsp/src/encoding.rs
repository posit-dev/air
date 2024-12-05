//
// encoding.rs
//
// Copyright (C) 2024 Posit Software, PBC. All rights reserved.
//
//

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
    0
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
