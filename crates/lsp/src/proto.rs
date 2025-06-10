#[derive(Clone, Copy, Debug)]
pub enum PositionEncoding {
    Utf8,
    Wide(biome_line_index::WideEncoding),
}
