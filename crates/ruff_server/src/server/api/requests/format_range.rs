// +------------------------------------------------------------+
// | Code adopted from:                                         |
// | Repository: https://github.com/astral-sh/ruff.git          |
// | Commit: 5bc9d6d3aa694ab13f38dd5cf91b713fd3844380           |
// +------------------------------------------------------------+

use lsp_types::{self as types, request as req, Range};

use crate::edit::{PositionEncoding, TextDocument};
use crate::server::{client::Notifier, Result};
use crate::session::{DocumentQuery, DocumentSnapshot};

pub(crate) struct FormatRange;

impl super::RequestHandler for FormatRange {
    type RequestType = req::RangeFormatting;
}

impl super::BackgroundDocumentRequestHandler for FormatRange {
    super::define_document_url!(params: &types::DocumentRangeFormattingParams);
    fn run_with_snapshot(
        snapshot: DocumentSnapshot,
        _notifier: Notifier,
        params: types::DocumentRangeFormattingParams,
    ) -> Result<super::FormatResponse> {
        format_document_range(&snapshot, params.range)
    }
}

/// Formats the specified [`Range`] in the [`DocumentSnapshot`].
fn format_document_range(
    snapshot: &DocumentSnapshot,
    range: Range,
) -> Result<super::FormatResponse> {
    let text_document = snapshot.query().as_single_document();
    let query = snapshot.query();
    format_text_document_range(text_document, range, query, snapshot.encoding())
}

/// Formats the specified [`Range`] in the [`TextDocument`].
fn format_text_document_range(
    _text_document: &TextDocument,
    _range: Range,
    _query: &DocumentQuery,
    _encoding: PositionEncoding,
) -> Result<super::FormatResponse> {
    Ok(None)
    //     let document_settings = query.settings();
    //     let formatter_settings = &document_settings.format;
    //
    //     let text = text_document.contents();
    //     let index = text_document.index();
    //     let range = range.to_text_range(text, index, encoding);
    //
    //     let formatted_range = crate::format::format_range(
    //         text_document,
    //         query.source_type(),
    //         formatter_settings,
    //         range,
    //     )
    //     .with_failure_code(lsp_server::ErrorCode::InternalError)?;
    //
    //     Ok(formatted_range.map(|formatted_range| {
    //         vec![types::TextEdit {
    //             range: formatted_range
    //                 .source_range()
    //                 .to_range(text, index, encoding),
    //             new_text: formatted_range.into_code(),
    //         }]
    //     }))
}
