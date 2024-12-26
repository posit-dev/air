use std::sync::LazyLock;

use biome_text_size::TextSize;
use memchr::memchr2;
use memchr::memmem;

static CRLF_FINDER: LazyLock<memmem::Finder> = LazyLock::new(|| memmem::Finder::new(b"\r\n"));

/// Line ending styles
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum LineEnding {
    Lf,
    Cr,
    Crlf,
}

impl LineEnding {
    pub const fn as_str(&self) -> &'static str {
        match self {
            LineEnding::Lf => "\n",
            LineEnding::Crlf => "\r\n",
            LineEnding::Cr => "\r",
        }
    }

    #[allow(clippy::len_without_is_empty)]
    pub const fn len(&self) -> usize {
        match self {
            LineEnding::Lf | LineEnding::Cr => 1,
            LineEnding::Crlf => 2,
        }
    }

    pub fn text_len(&self) -> TextSize {
        match self {
            LineEnding::Lf | LineEnding::Cr => TextSize::from(1),
            LineEnding::Crlf => TextSize::from(2),
        }
    }
}

/// Finds the next newline character. Returns its position and the [`LineEnding`].
#[inline]
pub fn find_newline(text: &str) -> Option<(usize, LineEnding)> {
    let bytes = text.as_bytes();
    if let Some(position) = memchr2(b'\n', b'\r', bytes) {
        let line_ending = match bytes[position] {
            // Explicit branch for `\n` as this is the most likely path
            b'\n' => LineEnding::Lf,
            // '\r\n'
            b'\r' if bytes.get(position.saturating_add(1)) == Some(&b'\n') => LineEnding::Crlf,
            // '\r'
            _ => LineEnding::Cr,
        };

        Some((position, line_ending))
    } else {
        None
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
pub fn normalize_crlf_newlines(text: String) -> String {
    let mut buf = text.into_bytes();
    let mut gap_len = 0;
    let mut tail = buf.as_mut_slice();
    let mut crlf_seen = false;

    loop {
        let idx = match CRLF_FINDER.find(&tail[gap_len..]) {
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
        assert_eq!(find_newline(src), Some((1, LineEnding::Lf)));
        assert_eq!(normalize_crlf_newlines(src.to_string()), src);
    }

    #[test]
    fn dos() {
        let src = "\r\na\r\n\r\nb\r\nc\r\n\r\n\r\n\r\n";
        assert_eq!(find_newline(src), Some((0, LineEnding::Crlf)));
        assert_eq!(
            normalize_crlf_newlines(src.to_string()),
            "\na\n\nb\nc\n\n\n\n"
        );
    }

    #[test]
    fn mixed() {
        let src = "a\r\nb\r\nc\r\n\n\r\n\n";
        assert_eq!(find_newline(src), Some((1, LineEnding::Crlf)));
        assert_eq!(normalize_crlf_newlines(src.to_string()), "a\nb\nc\n\n\n\n");
    }

    #[test]
    fn none() {
        let src = "abc";
        assert_eq!(find_newline(src), None);
        assert_eq!(normalize_crlf_newlines(src.to_string()), src);
    }
}
