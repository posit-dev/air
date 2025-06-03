use biome_text_size::TextRange;
use biome_text_size::TextSize;
use source_file::LineNumber;
use source_file::LineOffset;
use source_file::LineOffsetEncoding;
use source_file::SourceFile;
use source_file::SourceLocation;
use tower_lsp::lsp_types;

pub fn offset(
    position: lsp_types::Position,
    source: &SourceFile,
    encoding: LineOffsetEncoding,
) -> TextSize {
    let source_location = SourceLocation::new(
        LineNumber::from(position.line),
        LineOffset::new(position.character, encoding),
    );
    source.offset(source_location)
}

pub fn text_range(
    range: lsp_types::Range,
    source: &SourceFile,
    encoding: LineOffsetEncoding,
) -> TextRange {
    TextRange::new(
        self::offset(range.start, source, encoding),
        self::offset(range.end, source, encoding),
    )
}
