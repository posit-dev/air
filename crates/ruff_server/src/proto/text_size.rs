use crate::edit::PositionEncoding;
use biome_rowan::TextRange;
use biome_text_size::TextSize;
use lsp_types as types;
use ruff_source_file::OneIndexed;
use ruff_source_file::{LineIndex, SourceLocation};

// We don't own this type so we need a helper trait
pub(crate) trait TextSizeExt {
    fn into_proto(
        self,
        text: &str,
        index: &LineIndex,
        encoding: PositionEncoding,
    ) -> types::Position;

    fn from_proto(
        position: types::Position,
        text: &str,
        index: &LineIndex,
        encoding: PositionEncoding,
    ) -> Self;
}

impl TextSizeExt for TextSize {
    fn into_proto(
        self,
        text: &str,
        index: &LineIndex,
        encoding: PositionEncoding,
    ) -> types::Position {
        source_location_to_position(&offset_to_source_location(self, text, index, encoding))
    }

    fn from_proto(
        position: types::Position,
        text: &str,
        index: &LineIndex,
        encoding: PositionEncoding,
    ) -> Self {
        let line = index.line_range(
            OneIndexed::from_zero_indexed(u32_index_to_usize(position.line)),
            text,
        );

        let column_offset = match encoding {
            PositionEncoding::UTF8 => TextSize::from(position.character),

            PositionEncoding::UTF16 => {
                // Fast path for ASCII only documents
                if index.is_ascii() {
                    TextSize::from(position.character)
                } else {
                    // UTF-16 encodes characters either as one or two 16 bit words.
                    // The `position` is the 16-bit word offset from the start of the line (and not the character offset)
                    utf8_column_offset(position.character, &text[line])
                }
            }

            PositionEncoding::UTF32 => {
                // UTF-32 uses 4 bytes for each character. Meaning, the position is a character offset.
                return index.offset(
                    OneIndexed::from_zero_indexed(u32_index_to_usize(position.line)),
                    OneIndexed::from_zero_indexed(u32_index_to_usize(position.character)),
                    text,
                );
            }
        };

        line.start() + column_offset.clamp(TextSize::from(0), line.end())
    }
}

fn u32_index_to_usize(index: u32) -> usize {
    usize::try_from(index).expect("u32 fits in usize")
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
