mod options;
mod parse;

#[allow(unused)]
mod treesitter;

use air_r_factory::RSyntaxFactory;
pub use options::RParserOptions;
pub use parse::RParse;
pub use parse::parse;

use air_r_syntax::RLanguage;
use biome_parser::tree_sink::LosslessTreeSink;

pub(crate) type RLosslessTreeSink<'source> = LosslessTreeSink<'source, RLanguage, RSyntaxFactory>;
