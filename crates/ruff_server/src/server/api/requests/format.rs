// +------------------------------------------------------------+
// | Code adopted from:                                         |
// | Repository: https://github.com/astral-sh/ruff.git          |
// | Commit: 5bc9d6d3aa694ab13f38dd5cf91b713fd3844380           |
// +------------------------------------------------------------+

use lsp_types::{self as types, request as req};
use types::TextEdit;

use ruff_source_file::LineIndex;

use crate::edit::{PositionEncoding, Replacement, TextDocument, ToRangeExt};
use crate::server::api::LSPResult;
use crate::server::{client::Notifier, Result};
use crate::session::{DocumentQuery, DocumentSnapshot};

pub(crate) struct Format;

impl super::RequestHandler for Format {
    type RequestType = req::Formatting;
}

impl super::BackgroundDocumentRequestHandler for Format {
    super::define_document_url!(params: &types::DocumentFormattingParams);
    fn run_with_snapshot(
        snapshot: DocumentSnapshot,
        _notifier: Notifier,
        _params: types::DocumentFormattingParams,
    ) -> Result<super::FormatResponse> {
        format_document(&snapshot)
    }
}

/// Formats either a full text document or an specific notebook cell. If the query within the snapshot is a notebook document
/// with no selected cell, this will throw an error.
pub(super) fn format_document(snapshot: &DocumentSnapshot) -> Result<super::FormatResponse> {
    let text_document = snapshot.query().as_single_document();
    let query = snapshot.query();
    format_text_document(text_document, query, snapshot.encoding())
}

fn format_text_document(
    text_document: &TextDocument,
    query: &DocumentQuery,
    encoding: PositionEncoding,
) -> Result<super::FormatResponse> {
    let document_settings = query.settings();
    let formatter_settings = &document_settings.format;

    let source = text_document.contents();

    let formatted = crate::format::format(source, formatter_settings)
        .with_failure_code(lsp_server::ErrorCode::InternalError)?;
    let Some(formatted) = formatted else {
        return Ok(None);
    };

    let unformatted_index = text_document.index();
    let formatted_index: LineIndex = LineIndex::from_source_text(&formatted);

    let Replacement {
        source_range,
        modified_range: formatted_range,
    } = Replacement::between(
        source,
        unformatted_index.line_starts(),
        &formatted,
        formatted_index.line_starts(),
    );

    Ok(Some(vec![TextEdit {
        range: source_range.to_range(source, unformatted_index, encoding),
        new_text: formatted[formatted_range].to_owned(),
    }]))
}
