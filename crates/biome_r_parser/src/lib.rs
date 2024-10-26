mod options;
mod parse;

#[allow(unused)]
mod treesitter;

use biome_r_factory::RSyntaxFactory;
pub use options::RParserOptions;
pub use parse::parse;
pub use parse::parse_r_with_cache;

use biome_parser::tree_sink::LosslessTreeSink;
use biome_r_syntax::RLanguage;

pub(crate) type RLosslessTreeSink<'source> = LosslessTreeSink<'source, RLanguage, RSyntaxFactory>;
