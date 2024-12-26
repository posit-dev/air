// +------------------------------------------------------------+
// | Code adopted from:                                         |
// | Repository: https://github.com/astral-sh/ruff.git          |
// | Commit: 5bc9d6d3aa694ab13f38dd5cf91b713fd3844380           |
// +------------------------------------------------------------+

use super::PositionEncoding;
use biome_text_size::{TextRange, TextSize};
use lsp_types as types;
use ruff_source_file::OneIndexed;
use ruff_source_file::{LineIndex, SourceLocation};

pub(crate) trait RangeExt {
    fn to_text_range(&self, text: &str, index: &LineIndex, encoding: PositionEncoding)
        -> TextRange;
}

pub(crate) trait ToRangeExt {
    fn to_range(&self, text: &str, index: &LineIndex, encoding: PositionEncoding) -> types::Range;
}

fn u32_index_to_usize(index: u32) -> usize {
    usize::try_from(index).expect("u32 fits in usize")
}

impl RangeExt for lsp_types::Range {
    fn to_text_range(
        &self,
        text: &str,
        index: &LineIndex,
        encoding: PositionEncoding,
    ) -> TextRange {
        let start_line = index.line_range(
            OneIndexed::from_zero_indexed(u32_index_to_usize(self.start.line)),
            text,
        );
        let end_line = index.line_range(
            OneIndexed::from_zero_indexed(u32_index_to_usize(self.end.line)),
            text,
        );

        let (start_column_offset, end_column_offset) = match encoding {
            PositionEncoding::UTF8 => (
                TextSize::from(self.start.character),
                TextSize::from(self.end.character),
            ),

            PositionEncoding::UTF16 => {
                // Fast path for ASCII only documents
                if index.is_ascii() {
                    (
                        TextSize::from(self.start.character),
                        TextSize::from(self.end.character),
                    )
                } else {
                    // UTF16 encodes characters either as one or two 16 bit words.
                    // The position in `range` is the 16-bit word offset from the start of the line (and not the character offset)
                    // UTF-16 with a text that may use variable-length characters.
                    (
                        utf8_column_offset(self.start.character, &text[start_line]),
                        utf8_column_offset(self.end.character, &text[end_line]),
                    )
                }
            }
            PositionEncoding::UTF32 => {
                // UTF-32 uses 4 bytes for each character. Meaning, the position in range is a character offset.
                return TextRange::new(
                    index.offset(
                        OneIndexed::from_zero_indexed(u32_index_to_usize(self.start.line)),
                        OneIndexed::from_zero_indexed(u32_index_to_usize(self.start.character)),
                        text,
                    ),
                    index.offset(
                        OneIndexed::from_zero_indexed(u32_index_to_usize(self.end.line)),
                        OneIndexed::from_zero_indexed(u32_index_to_usize(self.end.character)),
                        text,
                    ),
                );
            }
        };

        TextRange::new(
            start_line.start() + start_column_offset.clamp(TextSize::from(0), start_line.end()),
            end_line.start() + end_column_offset.clamp(TextSize::from(0), end_line.end()),
        )
    }
}

impl ToRangeExt for TextRange {
    fn to_range(&self, text: &str, index: &LineIndex, encoding: PositionEncoding) -> types::Range {
        types::Range {
            start: source_location_to_position(&offset_to_source_location(
                self.start(),
                text,
                index,
                encoding,
            )),
            end: source_location_to_position(&offset_to_source_location(
                self.end(),
                text,
                index,
                encoding,
            )),
        }
    }
}

/// Converts a UTF-16 code unit offset for a given line into a UTF-8 column number.
fn utf8_column_offset(utf16_code_unit_offset: u32, line: &str) -> TextSize {
    let mut utf8_code_unit_offset = TextSize::from(0);

    let mut i = 0u32;

    for c in line.chars() {
        if i >= utf16_code_unit_offset {
            break;
        }

        // Count characters encoded as two 16 bit words as 2 characters.
        {
            utf8_code_unit_offset +=
                TextSize::from(u32::try_from(c.len_utf8()).expect("utf8 len always <=4"));
            i += u32::try_from(c.len_utf16()).expect("utf16 len always <=2");
        }
    }

    utf8_code_unit_offset
}

fn offset_to_source_location(
    offset: TextSize,
    text: &str,
    index: &LineIndex,
    encoding: PositionEncoding,
) -> SourceLocation {
    match encoding {
        PositionEncoding::UTF8 => {
            let row = index.line_index(offset);
            let column = offset - index.line_start(row, text);

            SourceLocation {
                column: OneIndexed::from_zero_indexed(column.into()),
                row,
            }
        }
        PositionEncoding::UTF16 => {
            let row = index.line_index(offset);

            let column = if index.is_ascii() {
                (offset - index.line_start(row, text)).into()
            } else {
                let up_to_line = &text[TextRange::new(index.line_start(row, text), offset)];
                up_to_line.encode_utf16().count()
            };

            SourceLocation {
                column: OneIndexed::from_zero_indexed(column),
                row,
            }
        }
        PositionEncoding::UTF32 => index.source_location(offset, text),
    }
}

fn source_location_to_position(location: &SourceLocation) -> types::Position {
    types::Position {
        line: u32::try_from(location.row.to_zero_indexed()).expect("row usize fits in u32"),
        character: u32::try_from(location.column.to_zero_indexed())
            .expect("character usize fits in u32"),
    }
}
