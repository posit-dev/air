//! We maintain the invariant that all internal strings use `\n` as line separator.
//! This module does line ending conversion and detection (so that we can
//! convert back to `\r\n` on the way out as needed).

use std::sync::LazyLock;

use memchr::memmem;
use settings::LineEnding;

static FINDER: LazyLock<memmem::Finder> = LazyLock::new(|| memmem::Finder::new(b"\r\n"));

pub fn infer(x: &str) -> LineEnding {
    match FINDER.find(x.as_bytes()) {
        // Saw `\r\n`
        Some(_) => LineEnding::Crlf,
        // No `\r\n`, or empty file
        None => LineEnding::Lf,
    }
}

/// Normalize line endings within a string
///
/// We replace `\r\n` with `\n` in-place, which doesn't break utf-8 encoding.
/// While we *can* call `as_mut_vec` and do surgery on the live string
/// directly, let's rather steal the contents of `x`. This makes the code
/// safe even if a panic occurs.
///
/// # Source
///
/// ---
/// authors = ["rust-analyzer team"]
/// license = "MIT OR Apache-2.0"
/// origin = "https://github.com/rust-lang/rust-analyzer/blob/master/crates/rust-analyzer/src/line_index.rs"
/// ---
pub fn normalize(x: String) -> String {
    let mut buf = x.into_bytes();
    let mut gap_len = 0;
    let mut tail = buf.as_mut_slice();
    let mut crlf_seen = false;

    loop {
        let idx = match FINDER.find(&tail[gap_len..]) {
            None if crlf_seen => tail.len(),
            // SAFETY: buf is unchanged and therefore still contains utf8 data
            None => return unsafe { String::from_utf8_unchecked(buf) },
            Some(idx) => {
                crlf_seen = true;
                idx + gap_len
            }
        };
        tail.copy_within(gap_len..idx, 0);
        tail = &mut tail[idx - gap_len..];
        if tail.len() == gap_len {
            break;
        }
        gap_len += 1;
    }

    // Account for removed `\r`.
    // After `set_len`, `buf` is guaranteed to contain utf-8 again.
    unsafe {
        let new_len = buf.len() - gap_len;
        buf.set_len(new_len);
        String::from_utf8_unchecked(buf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unix() {
        let src = "a\nb\nc\n\n\n\n";
        assert_eq!(infer(src), LineEnding::Lf);
        assert_eq!(normalize(src.to_string()), src);
    }

    #[test]
    fn dos() {
        let src = "\r\na\r\n\r\nb\r\nc\r\n\r\n\r\n\r\n";
        assert_eq!(infer(src), LineEnding::Crlf);
        assert_eq!(normalize(src.to_string()), "\na\n\nb\nc\n\n\n\n");
    }

    #[test]
    fn mixed() {
        let src = "a\r\nb\r\nc\r\n\n\r\n\n";
        assert_eq!(infer(src), LineEnding::Crlf);
        assert_eq!(normalize(src.to_string()), "a\nb\nc\n\n\n\n");
    }

    #[test]
    fn none() {
        let src = "abc";
        assert_eq!(infer(src), LineEnding::Lf);
        assert_eq!(normalize(src.to_string()), src);
    }
}
