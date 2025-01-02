use air_r_parser::RParserOptions;
use air_r_syntax::RExpressionList;
use air_r_syntax::RSyntaxKind;
use air_r_syntax::RSyntaxNode;
use biome_formatter::LineEnding;
use biome_rowan::AstNode;
use biome_rowan::Language;
use biome_rowan::SyntaxElement;
use biome_rowan::WalkEvent;
use biome_text_size::{TextRange, TextSize};
use lsp_types::{self as types, request as req, Range};
use workspace::settings::FormatSettings;

use crate::document::TextEdit;
use crate::document::{PositionEncoding, TextDocument};
use crate::proto::TextRangeExt;
use crate::server::api::LSPResult;
use crate::server::{client::Notifier, Result};
use crate::session::{DocumentQuery, DocumentSnapshot};

type FormatRangeResponse = Option<Vec<lsp_types::TextEdit>>;

pub(crate) struct FormatRange;

impl super::RequestHandler for FormatRange {
    type RequestType = req::RangeFormatting;
}

impl super::BackgroundDocumentRequestHandler for FormatRange {
    fn document_url(
        params: &types::DocumentRangeFormattingParams,
    ) -> std::borrow::Cow<lsp_types::Url> {
        std::borrow::Cow::Borrowed(&params.text_document.uri)
    }

    fn run_with_snapshot(
        snapshot: DocumentSnapshot,
        _notifier: Notifier,
        params: types::DocumentRangeFormattingParams,
    ) -> Result<FormatRangeResponse> {
        format_document_range(&snapshot, params.range)
    }
}

/// Formats the specified [`Range`] in the [`DocumentSnapshot`].
#[tracing::instrument(level = "info", skip_all)]
fn format_document_range(snapshot: &DocumentSnapshot, range: Range) -> Result<FormatRangeResponse> {
    let text_document = snapshot.query().as_single_document();
    let query = snapshot.query();
    format_text_document_range(text_document, range, query, snapshot.encoding())
}

/// Formats the specified [`Range`] in the [`TextDocument`].
fn format_text_document_range(
    text_document: &TextDocument,
    range: Range,
    query: &DocumentQuery,
    encoding: PositionEncoding,
) -> Result<FormatRangeResponse> {
    let document_settings = query.settings();
    let formatter_settings = &document_settings.format;

    let ending = text_document.ending();
    let source = text_document.source_file();
    let text = source.contents();
    let range = TextRange::from_proto(range, source, encoding);

    let Some((new_text, new_range)) = format_source_range(text, formatter_settings, range)
        .with_failure_code(lsp_server::ErrorCode::InternalError)?
    else {
        return Ok(None);
    };

    let text_edit = TextEdit::replace(new_range, new_text);

    let edits = text_edit
        .into_proto(source, encoding, ending)
        .with_failure_code(lsp_server::ErrorCode::InternalError)?;

    Ok(Some(edits))
}

fn format_source_range(
    source: &str,
    formatter_settings: &FormatSettings,
    range: TextRange,
) -> anyhow::Result<Option<(String, TextRange)>> {
    let parse = air_r_parser::parse(source, RParserOptions::default());

    if parse.has_errors() {
        return Err(anyhow::anyhow!("Can't format when there are parse errors."));
    }

    // Always use `Lf` line endings on the way out from the formatter since we
    // internally store all LSP text documents with `Lf` endings
    let format_options = formatter_settings
        .to_format_options(source)
        .with_line_ending(LineEnding::Lf);

    let logical_lines = find_deepest_enclosing_logical_lines(parse.syntax(), range);
    if logical_lines.is_empty() {
        tracing::warn!("Can't find logical line");
        return Ok(None);
    };

    // Find the overall formatting range by concatenating the ranges of the logical lines.
    // We use the "non-whitespace-range" as that corresponds to what Biome will format.
    let new_range = logical_lines
        .iter()
        .map(text_non_whitespace_range)
        .reduce(|acc, new| acc.cover(new))
        .expect("`logical_lines` is non-empty");

    // We need to wrap in an `RRoot` otherwise the comments get attached too
    // deep in the tree. See `CommentsBuilderVisitor` in biome_formatter and the
    // `is_root` logic. Note that `node` needs to be wrapped in at least two
    // other nodes in order to fix this problem, and here we have an `RRoot` and
    // `RExpressionList` that do the job.
    //
    // Since we only format logical lines, it is fine to wrap in an expression list.
    let Some(exprs): Option<Vec<air_r_syntax::AnyRExpression>> = logical_lines
        .into_iter()
        .map(air_r_syntax::AnyRExpression::cast)
        .collect()
    else {
        tracing::warn!("Can't cast to `AnyRExpression`");
        return Ok(None);
    };

    let list = air_r_factory::r_expression_list(exprs);
    let eof = air_r_syntax::RSyntaxToken::new_detached(RSyntaxKind::EOF, "", vec![], vec![]);
    let root = air_r_factory::r_root(list, eof).build();

    let printed = biome_formatter::format_sub_tree(
        root.syntax(),
        air_r_formatter::RFormatLanguage::new(format_options),
    )?;

    if printed.range().is_none() {
        // Happens in edge cases when biome returns a `Printed::new_empty()`
        return Ok(None);
    };

    let mut new_text = printed.into_code();

    // Remove last hard break line from our artifical expression list
    new_text.pop();

    Ok(Some((new_text, new_range)))
}

// From biome_formatter
fn text_non_whitespace_range<E, L>(elem: &E) -> TextRange
where
    E: Into<SyntaxElement<L>> + Clone,
    L: Language,
{
    let elem: SyntaxElement<L> = elem.clone().into();

    let start = elem
        .leading_trivia()
        .into_iter()
        .flat_map(|trivia| trivia.pieces())
        .find_map(|piece| {
            if piece.is_whitespace() || piece.is_newline() {
                None
            } else {
                Some(piece.text_range().start())
            }
        })
        .unwrap_or_else(|| elem.text_trimmed_range().start());

    let end = elem
        .trailing_trivia()
        .into_iter()
        .flat_map(|trivia| trivia.pieces().rev())
        .find_map(|piece| {
            if piece.is_whitespace() || piece.is_newline() {
                None
            } else {
                Some(piece.text_range().end())
            }
        })
        .unwrap_or_else(|| elem.text_trimmed_range().end());

    TextRange::new(start, end)
}

/// Finds consecutive logical lines. Currently that's only expressions at
/// top-level or in a braced list.
fn find_deepest_enclosing_logical_lines(node: RSyntaxNode, range: TextRange) -> Vec<RSyntaxNode> {
    let start_lists = find_expression_lists(&node, range.start(), false);
    let end_lists = find_expression_lists(&node, range.end(), true);

    // Both vectors of lists should have a common prefix, starting from the
    // program's expression list. As soon as the lists diverge we stop.
    let Some(list) = start_lists
        .into_iter()
        .zip(end_lists)
        .take_while(|pair| pair.0 == pair.1)
        .map(|pair| pair.0)
        .last()
    else {
        // Should not happen as the range is always included in the program's expression list
        tracing::warn!("Can't find common list parent");
        return vec![];
    };

    let Some(list) = RExpressionList::cast(list) else {
        tracing::warn!("Can't cast to expression list");
        return vec![];
    };

    let iter = list.into_iter();

    // We've chosen to be liberal about user selections and always widen the
    // range to include the selection bounds. If we wanted to be conservative
    // instead, we could use this `filter()` instead of the `skip_while()` and
    // `take_while()`:
    //
    // ```rust
    // .filter(|node| range.contains_range(node.text_trimmed_range()))
    // ```
    let logical_lines: Vec<RSyntaxNode> = iter
        .map(|expr| expr.into_syntax())
        .skip_while(|node| !node.text_range().contains(range.start()))
        .take_while(|node| node.text_trimmed_range().start() <= range.end())
        .collect();

    logical_lines
}

fn find_expression_lists(node: &RSyntaxNode, offset: TextSize, end: bool) -> Vec<RSyntaxNode> {
    let mut preorder = node.preorder();
    let mut nodes: Vec<RSyntaxNode> = vec![];

    while let Some(event) = preorder.next() {
        match event {
            WalkEvent::Enter(node) => {
                let Some(parent) = node.parent() else {
                    continue;
                };

                let is_contained = if end {
                    let trimmed_node_range = node.text_trimmed_range();
                    trimmed_node_range.contains_inclusive(offset)
                } else {
                    let node_range = node.text_range();
                    node_range.contains(offset)
                };

                if !is_contained {
                    preorder.skip_subtree();
                    continue;
                }

                if parent.kind() == RSyntaxKind::R_EXPRESSION_LIST {
                    nodes.push(parent.clone());
                    continue;
                }
            }

            WalkEvent::Leave(_) => {}
        }
    }

    nodes
}

#[cfg(test)]
mod tests {
    use crate::document::TextDocument;
    use crate::{test::init_test_client, test::TestClientExt};

    #[test]
    fn test_format_range_none() {
        let mut client = init_test_client();

        #[rustfmt::skip]
        let (doc, range) = TextDocument::doodle_and_range(
"<<>>",
        );

        let output = client.format_document_range(&doc, range);
        insta::assert_snapshot!(output);

        #[rustfmt::skip]
        let (doc, range) = TextDocument::doodle_and_range(
"<<
>>",
        );

        let output = client.format_document_range(&doc, range);
        insta::assert_snapshot!(output);

        #[rustfmt::skip]
        let (doc, range) = TextDocument::doodle_and_range(
"<<1
>>",
        );

        let output = client.format_document_range(&doc, range);
        insta::assert_snapshot!(output);

        client.shutdown();
        client.exit();
    }

    #[test]
    fn test_format_range_logical_lines() {
        let mut client = init_test_client();

        // 2+2 is the logical line to format
        #[rustfmt::skip]
        let (doc, range) = TextDocument::doodle_and_range(
"1+1
<<2+2>>
",
        );
        let output = client.format_document_range(&doc, range);
        insta::assert_snapshot!(output);

        #[rustfmt::skip]
        let (doc, range) = TextDocument::doodle_and_range(
"1+1
#
<<2+2>>
",
        );

        let output = client.format_document_range(&doc, range);
        insta::assert_snapshot!(output);

        // The element in the braced expression is a logical line
        // FIXME: Should this be the whole `{2+2}` instead?
        #[rustfmt::skip]
        let (doc, range) = TextDocument::doodle_and_range(
"1+1
{<<2+2>>}
",
        );

        let output = client.format_document_range(&doc, range);
        insta::assert_snapshot!(output);

        #[rustfmt::skip]
        let (doc, range) = TextDocument::doodle_and_range(
"1+1
<<{2+2}>>
",
        );
        let output = client.format_document_range(&doc, range);
        insta::assert_snapshot!(output);

        // The deepest element in the braced expression is our target
        #[rustfmt::skip]
        let (doc, range) = TextDocument::doodle_and_range(
"1+1
{
  2+2
  {
    <<3+3>>
  }
}
",
        );

        let output = client.format_document_range(&doc, range);
        insta::assert_snapshot!(output);

        client.shutdown();
        client.exit();
    }

    #[test]
    fn test_format_range_mismatched_indent() {
        let mut client = init_test_client();

        #[rustfmt::skip]
        let (doc, range) = TextDocument::doodle_and_range(
"1
  <<2+2>>
",
        );

        // We don't change indentation when `2+2` is formatted
        let output = client.format_document_range(&doc, range);
        insta::assert_snapshot!(output);

        // Debatable: Should we make an effort to remove unneeded indentation
        // when it's part of the range?
        #[rustfmt::skip]
        let (doc, range) = TextDocument::doodle_and_range(
"1
<<  2+2>>
",
        );
        let output_wide = client.format_document_range(&doc, range);
        assert_eq!(output, output_wide);

        client.shutdown();
        client.exit();
    }

    #[test]
    fn test_format_range_multiple_lines() {
        let mut client = init_test_client();

        #[rustfmt::skip]
        let (doc, range) = TextDocument::doodle_and_range(
"1+1
<<#
2+2>>
",
        );

        let output1 = client.format_document_range(&doc, range);
        insta::assert_snapshot!(output1);

        #[rustfmt::skip]
        let (doc, range) = TextDocument::doodle_and_range(
"<<1+1
#
2+2>>
",
        );
        let output2 = client.format_document_range(&doc, range);
        insta::assert_snapshot!(output2);

        client.shutdown();
        client.exit();
    }

    #[test]
    fn test_format_range_unmatched_lists() {
        let mut client = init_test_client();

        #[rustfmt::skip]
        let (doc, range) = TextDocument::doodle_and_range(
"0+0
<<1+1
{
  2+2>>
}
3+3
",
        );

        let output1 = client.format_document_range(&doc, range);
        insta::assert_snapshot!(output1);

        #[rustfmt::skip]
        let (doc, range) = TextDocument::doodle_and_range(
"0+0
<<1+1
{
>>  2+2
}
3+3
",
        );
        let output2 = client.format_document_range(&doc, range);
        insta::assert_snapshot!(output2);

        #[rustfmt::skip]
        let (doc, range) = TextDocument::doodle_and_range(
"0+0
<<1+1
{
  2+2
}
>>3+3
",
        );
        let output3 = client.format_document_range(&doc, range);
        insta::assert_snapshot!(output3);

        #[rustfmt::skip]
        let (doc, range) = TextDocument::doodle_and_range(
"0+0
1+1
{
<<  2+2
}
>>3+3
",
        );
        let output4 = client.format_document_range(&doc, range);
        insta::assert_snapshot!(output4);

        #[rustfmt::skip]
        let (doc, range) = TextDocument::doodle_and_range(
"<<1+1>>
2+2
",
        );

        let output5 = client.format_document_range(&doc, range);
        insta::assert_snapshot!(output5);

        #[rustfmt::skip]
        let (doc, range) = TextDocument::doodle_and_range(
"1+1
<<2+2>>
",
        );

        let output6 = client.format_document_range(&doc, range);
        insta::assert_snapshot!(output6);

        client.shutdown();
        client.exit();
    }
}
