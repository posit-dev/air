//! Tools for managing a single source file
//!
//! In particular, [SourceFile] manages the conversions between UTF-8 byte offsets into a
//! [String], and line number + line offset (also known as row/column or row/character)
//! backed [SourceLocation]s, where the line offset is measured in UTF code units and is
//! dependent on the [LineOffsetEncoding] used. [SourceLocation]s are meant to easily map
//! to LSP `Position`s, and can handle the common `PositionEncodingKind`s of UTF-8,
//! UTF-16, and UTF-32.

pub use crate::source_file::SourceFile;
pub use crate::source_location::LineNumber;
pub use crate::source_location::LineOffset;
pub use crate::source_location::LineOffsetEncoding;
pub use crate::source_location::SourceLocation;

mod source_file;
mod source_location;
