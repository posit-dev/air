//
// handlers_format.rs
//
// Copyright (C) 2024 Posit Software, PBC. All rights reserved.
//
//

use air_r_formatter::{context::RFormatOptions, format_node};
use air_r_syntax::{RExpressionList, RSyntaxKind, RSyntaxNode, WalkEvent};
use biome_formatter::{IndentStyle, LineWidth};
use biome_rowan::{AstNode, Language, SyntaxElement};
use biome_text_size::{TextRange, TextSize};
use tower_lsp::lsp_types;

use crate::state::WorldState;
use crate::{from_proto, to_proto};

#[tracing::instrument(level = "info", skip_all)]
pub(crate) fn document_formatting(
    params: lsp_types::DocumentFormattingParams,
    state: &WorldState,
) -> anyhow::Result<Option<Vec<lsp_types::TextEdit>>> {
    let doc = state.get_document(&params.text_document.uri)?;

    let line_width = LineWidth::try_from(80).map_err(|err| anyhow::anyhow!("{err}"))?;

    // TODO: Handle FormattingOptions
    let options = RFormatOptions::default()
        .with_indent_style(IndentStyle::Space)
        .with_line_width(line_width);

    if doc.parse.has_errors() {
        return Err(anyhow::anyhow!("Can't format when there are parse errors."));
    }

    let formatted = format_node(options.clone(), &doc.parse.syntax())?;
    let output = formatted.print()?.into_code();

    // Do we need to check that `doc` is indeed an R file? What about special
    // files that don't have extensions like `NAMESPACE`, do we hard-code a
    // list? What about unnamed temporary files?

    let edits = to_proto::replace_all_edit(&doc.line_index, &doc.contents, &output)?;
    Ok(Some(edits))
}

#[tracing::instrument(level = "info", skip_all)]
pub(crate) fn document_range_formatting(
    params: lsp_types::DocumentRangeFormattingParams,
    state: &WorldState,
) -> anyhow::Result<Option<Vec<lsp_types::TextEdit>>> {
    let doc = state.get_document(&params.text_document.uri)?;

    let line_width = LineWidth::try_from(80).map_err(|err| anyhow::anyhow!("{err}"))?;
    let range =
        from_proto::text_range(&doc.line_index.index, params.range, doc.line_index.encoding)?;

    // TODO: Handle FormattingOptions
    let options = RFormatOptions::default()
        .with_indent_style(IndentStyle::Space)
        .with_line_width(line_width);

    let logical_lines = find_deepest_enclosing_logical_lines(doc.parse.syntax(), range);
    if logical_lines.is_empty() {
        tracing::warn!("Can't find logical line");
        return Ok(None);
    };

    // Find the overall formatting range by concatenating the ranges of the logical lines.
    // We use the "non-whitespace-range" as that corresponds to what Biome will format.
    let format_range = logical_lines
        .iter()
        .map(|line| text_non_whitespace_range(line))
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
        .map(|node| air_r_syntax::AnyRExpression::cast(node))
        .collect()
    else {
        tracing::warn!("Can't cast to `AnyRExpression`");
        return Ok(None);
    };

    let list = air_r_factory::r_expression_list(exprs);
    let eof = air_r_syntax::RSyntaxToken::new_detached(RSyntaxKind::EOF, "", vec![], vec![]);
    let root = air_r_factory::r_root(list, eof).build();

    let format_info = biome_formatter::format_sub_tree(
        root.syntax(),
        air_r_formatter::RFormatLanguage::new(options),
    )?;

    if format_info.range().is_none() {
        // Happens in edge cases when biome returns a `Printed::new_empty()`
        return Ok(None);
    };

    let mut format_text = format_info.into_code();

    // Remove last hard break line from our artifical expression list
    format_text.pop();
    let edits = to_proto::replace_range_edit(&doc.line_index, format_range, format_text)?;

    Ok(Some(edits))
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
        .zip(end_lists.into_iter())
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

    let logical_lines: Vec<RSyntaxNode> = iter
        .map(|expr| expr.into_syntax())
        .skip_while(|node| !node.text_range().contains(range.start()))
        .take_while(|node| node.text_range().start() <= range.end())
        .collect();

    logical_lines
}

fn find_expression_lists(
    node: &RSyntaxNode,
    offset: TextSize,
    inclusive: bool,
) -> Vec<RSyntaxNode> {
    let mut preorder = node.preorder();
    let mut nodes: Vec<RSyntaxNode> = vec![];

    while let Some(event) = preorder.next() {
        match event {
            WalkEvent::Enter(node) => {
                let Some(parent) = node.parent() else {
                    continue;
                };

                let node_range = node.text_range();

                let is_contained = if inclusive {
                    node_range.contains_inclusive(offset)
                } else {
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
    use biome_text_size::{TextRange, TextSize};

    use crate::{
        documents::Document, tower_lsp::init_test_client, tower_lsp_test_client::TestClientExt,
    };

    #[tests_macros::lsp_test]
    async fn test_format() {
        let mut client = init_test_client().await;

        #[rustfmt::skip]
        let doc = Document::doodle(
"
1
2+2
3 + 3 +
3",
        );

        let formatted = client.format_document(&doc).await;
        insta::assert_snapshot!(formatted);

        client
    }

    // https://github.com/posit-dev/air/issues/61
    #[tests_macros::lsp_test]
    async fn test_format_minimal_diff() {
        let mut client = init_test_client().await;

        #[rustfmt::skip]
        let doc = Document::doodle(
"1
2+2
3
",
        );

        let edits = client.format_document_edits(&doc).await.unwrap();
        assert!(edits.len() == 1);

        let edit = &edits[0];
        assert_eq!(edit.new_text, " + ");

        client
    }

    #[tests_macros::lsp_test]
    async fn test_format_range_none() {
        let mut client = init_test_client().await;

        #[rustfmt::skip]
        let doc = Document::doodle(
"",
        );
        let range = TextRange::new(TextSize::from(0), TextSize::from(0));

        let output = client.format_document_range(&doc, range).await;
        insta::assert_snapshot!(output);

        #[rustfmt::skip]
        let doc = Document::doodle(
"
",
        );
        let range = TextRange::new(TextSize::from(0), TextSize::from(1));

        let output = client.format_document_range(&doc, range).await;
        insta::assert_snapshot!(output);

        #[rustfmt::skip]
        let doc = Document::doodle(
"1
",
        );
        let range = TextRange::new(TextSize::from(0), TextSize::from(1));

        let output = client.format_document_range(&doc, range).await;
        insta::assert_snapshot!(output);

        client
    }

    #[tests_macros::lsp_test]
    async fn test_format_range_logical_lines() {
        let mut client = init_test_client().await;

        // 2+2 is the logical line to format
        #[rustfmt::skip]
        let doc = Document::doodle(
"1+1
2+2
",
        );
        let range = TextRange::new(TextSize::from(4), TextSize::from(7));
        let output = client.format_document_range(&doc, range).await;
        insta::assert_snapshot!(output);

        #[rustfmt::skip]
        let doc = Document::doodle(
"1+1
#
2+2
",
        );
        let range = TextRange::new(TextSize::from(6), TextSize::from(9));

        let output = client.format_document_range(&doc, range).await;
        insta::assert_snapshot!(output);

        // The element in the braced expression is a logical line
        // FIXME: Should this be the whole `{2+2}` instead?
        #[rustfmt::skip]
        let doc = Document::doodle(
"1+1
{2+2}
",
        );

        let range = TextRange::new(TextSize::from(5), TextSize::from(8));
        let output = client.format_document_range(&doc, range).await;
        insta::assert_snapshot!(output);

        // Including braces
        let range = TextRange::new(TextSize::from(4), TextSize::from(9));
        let output = client.format_document_range(&doc, range).await;
        insta::assert_snapshot!(output);

        // The deepest element in the braced expression is our target
        #[rustfmt::skip]
        let doc = Document::doodle(
"1+1
{
  2+2
  {
    3+3
  }
}
",
        );

        let range = TextRange::new(TextSize::from(20), TextSize::from(23));
        let output = client.format_document_range(&doc, range).await;
        insta::assert_snapshot!(output);
        client
    }

    #[tests_macros::lsp_test]
    async fn test_format_range_mismatched_indent() {
        let mut client = init_test_client().await;

        #[rustfmt::skip]
        let doc = Document::doodle(
"1
  2+2
",
        );

        // We don't change indentation when `2+2` is formatted
        let range = TextRange::new(TextSize::from(4), TextSize::from(7));
        let output = client.format_document_range(&doc, range).await;
        insta::assert_snapshot!(output);

        // Debatable: Should we make an effort to remove unneeded indentation
        // when it's part of the range?
        let range_wide = TextRange::new(TextSize::from(2), TextSize::from(7));
        let output_wide = client.format_document_range(&doc, range_wide).await;
        assert_eq!(output, output_wide);

        client
    }

    #[tests_macros::lsp_test]
    async fn test_format_range_multiple_lines() {
        let mut client = init_test_client().await;

        #[rustfmt::skip]
        let doc = Document::doodle(
"1+1
#
2+2
",
        );

        // Selecting the last two lines
        let range = TextRange::new(TextSize::from(4), TextSize::from(9));
        let output1 = client.format_document_range(&doc, range).await;
        insta::assert_snapshot!(output1);

        // Selecting all three lines
        let range = TextRange::new(TextSize::from(0), TextSize::from(9));
        let output2 = client.format_document_range(&doc, range).await;
        insta::assert_snapshot!(output2);

        client
    }

    #[tests_macros::lsp_test]
    async fn test_format_range_unmatched_lists() {
        let mut client = init_test_client().await;

        #[rustfmt::skip]
        let doc = Document::doodle(
"0+0
1+1
{
  2+2
}
3+3
",
        );

        // Selecting lines 2-4
        let range = TextRange::new(TextSize::from(4), TextSize::from(15));
        let output1 = client.format_document_range(&doc, range).await;
        insta::assert_snapshot!(output1);

        // Selecting lines 2-4 with partial selection on line 4
        let range = TextRange::new(TextSize::from(4), TextSize::from(10));
        let output2 = client.format_document_range(&doc, range).await;
        insta::assert_snapshot!(output2);

        // Selecting lines 2-6
        let range = TextRange::new(TextSize::from(4), TextSize::from(18));
        let output3 = client.format_document_range(&doc, range).await;
        insta::assert_snapshot!(output3);

        // Selecting lines 4-6
        let range = TextRange::new(TextSize::from(10), TextSize::from(18));
        let output4 = client.format_document_range(&doc, range).await;
        insta::assert_snapshot!(output4);

        client
    }
}
