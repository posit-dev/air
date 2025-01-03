//! Types and utilities for working with documents

mod encoding;
mod key;
mod text_diff;
mod text_document;
mod text_edit;

pub(crate) use encoding::PositionEncoding;
pub(crate) use key::DocumentKey;
pub(crate) use text_document::DocumentVersion;
pub(crate) use text_document::TextDocument;
pub(crate) use text_edit::{Indel, TextEdit};
