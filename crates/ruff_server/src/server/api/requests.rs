mod format;
mod format_range;

use super::{
    define_document_url,
    traits::{BackgroundDocumentRequestHandler, RequestHandler},
};
pub(super) use format::Format;
pub(super) use format_range::FormatRange;

type FormatResponse = Option<Vec<lsp_types::TextEdit>>;
