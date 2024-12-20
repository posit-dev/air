mod error;
mod options;
mod parse;

#[allow(unused)]
mod treesitter;

use air_r_factory::RSyntaxFactory;
pub use error::ParseError;
pub use options::RParserOptions;
pub use parse::parse;
pub use parse::parse_r_with_cache;
pub use parse::Parse;

use air_r_syntax::RLanguage;
use biome_parser::tree_sink::LosslessTreeSink;

pub(crate) type RLosslessTreeSink<'source> = LosslessTreeSink<'source, RLanguage, RSyntaxFactory>;
