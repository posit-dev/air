pub mod from_proto;
pub mod to_proto;

/// Our representation of [tower_lsp::PositionEncodingKind]
/// From `biome_lsp_converters::PositionEncoding`
#[derive(Clone, Copy, Debug)]
pub enum PositionEncoding {
    Utf8,
    Wide(biome_line_index::WideEncoding),
}
