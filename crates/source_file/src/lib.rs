pub use crate::newlines::{find_newline, infer_line_ending, normalize_newlines, LineEnding};
pub use crate::source_file::SourceFile;
pub use crate::source_location::LineNumber;
pub use crate::source_location::LineOffset;
pub use crate::source_location::LineOffsetEncoding;
pub use crate::source_location::SourceLocation;

mod line_index;
mod newlines;
mod source_file;
mod source_location;
