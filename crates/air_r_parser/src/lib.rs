mod diagnostics;
mod lexer;
mod options;
mod parse;
mod parser;
mod syntax;
mod token_source;

pub use crate::options::RParserOptions;
pub use crate::parse::RParse;
pub use crate::parse::parse;

use air_r_factory::RSyntaxFactory;
use air_r_syntax::RLanguage;
use biome_parser::tree_sink::LosslessTreeSink;

pub(crate) type RLosslessTreeSink<'source> = LosslessTreeSink<'source, RLanguage, RSyntaxFactory>;
