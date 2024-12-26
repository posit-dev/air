#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub use crate::line_index::LineIndex;
pub use crate::newlines::{find_newline, normalize_crlf_newlines, LineEnding};
pub use crate::one_indexed::OneIndexed;
pub use crate::source_location::SourceLocation;

mod line_index;
mod newlines;
mod one_indexed;
mod source_location;
