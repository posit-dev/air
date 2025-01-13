//
// handlers_format.rs
//
// Copyright (C) 2024 Posit Software, PBC. All rights reserved.
//
//

use air_r_formatter::format_node;
use air_r_syntax::{RExpressionList, RSyntaxKind, RSyntaxNode, WalkEvent};
use biome_rowan::{AstNode, Language, SyntaxElement};
use biome_text_size::{TextRange, TextSize};
use tower_lsp::lsp_types;

use crate::main_loop::LspState;
use crate::state::WorldState;
use crate::{from_proto, to_proto};

#[tracing::instrument(level = "info", skip_all)]
pub(crate) fn document_formatting(
    params: lsp_types::DocumentFormattingParams,
    lsp_state: &LspState,
    state: &WorldState,
) -> anyhow::Result<Option<Vec<lsp_types::TextEdit>>> {
    let doc = state.get_document(&params.text_document.uri)?;

    let settings = lsp_state.document_settings(&params.text_document.uri, &doc.config);
    let format_options = settings.format.to_format_options(&doc.contents);

    if doc.parse.has_errors() {
        // Refuse to format in the face of parse errors, but only log a warning
        // rather than returning an LSP error, as toast notifications here are distracting.
        tracing::warn!(
            "Failed to format {uri}. Can't format when there are parse errors.",
            uri = params.text_document.uri
        );
        return Ok(None);
    }

    let formatted = format_node(format_options, &doc.parse.syntax())?;
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
    lsp_state: &LspState,
    state: &WorldState,
) -> anyhow::Result<Option<Vec<lsp_types::TextEdit>>> {
    let doc = state.get_document(&params.text_document.uri)?;

    if doc.parse.has_errors() {
        // Refuse to format in the face of parse errors, but only log a warning
        // rather than returning an LSP error, as toast notifications here are distracting.
        tracing::warn!(
            "Failed to format {uri}. Can't format when there are parse errors.",
            uri = params.text_document.uri
        );
        return Ok(None);
    }

    let range =
        from_proto::text_range(&doc.line_index.index, params.range, doc.line_index.encoding)?;

    let settings = lsp_state.document_settings(&params.text_document.uri, &doc.config);
    let format_options = settings.format.to_format_options(&doc.contents);

    let logical_lines = find_deepest_enclosing_logical_lines(doc.parse.syntax(), range);
    if logical_lines.is_empty() {
        tracing::warn!("Can't find logical line");
        return Ok(None);
    };

    // Find the overall formatting range by concatenating the ranges of the logical lines.
    // We use the "non-whitespace-range" as that corresponds to what Biome will format.
    let format_range = logical_lines
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

    let format_info = biome_formatter::format_sub_tree(
        root.syntax(),
        air_r_formatter::RFormatLanguage::new(format_options),
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
    use crate::{documents::Document, test::with_client, test::TestClientExt};

    #[test]
    fn test_format() {
        with_client(|client| async {
            let mut client = client.lock().await;

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
        })
    }

    // https://github.com/posit-dev/air/issues/61
    #[test]
    fn test_format_minimal_diff() {
        with_client(|client| async {
            let mut client = client.lock().await;

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
        })
    }

    #[test]
    fn test_format_range_none() {
        with_client(|client| async {
            let mut client = client.lock().await;

            #[rustfmt::skip]
            let (doc, range) = Document::doodle_and_range(
"<<>>",
            );

            let output = client.format_document_range(&doc, range).await;
            insta::assert_snapshot!(output);

            #[rustfmt::skip]
            let (doc, range) = Document::doodle_and_range(
"<<
>>",
            );

            let output = client.format_document_range(&doc, range).await;
            insta::assert_snapshot!(output);

            #[rustfmt::skip]
            let (doc, range) = Document::doodle_and_range(
"<<1
>>",
            );

            let output = client.format_document_range(&doc, range).await;
            insta::assert_snapshot!(output);
        });
    }

    #[test]
    fn test_format_range_logical_lines() {
        with_client(|client| async {
            let mut client = client.lock().await;

            // 2+2 is the logical line to format
            #[rustfmt::skip]
            let (doc, range) = Document::doodle_and_range(
"1+1
<<2+2>>
",
            );
            let output = client.format_document_range(&doc, range).await;
            insta::assert_snapshot!(output);

            #[rustfmt::skip]
            let (doc, range) = Document::doodle_and_range(
"1+1
#
<<2+2>>
",
            );

            let output = client.format_document_range(&doc, range).await;
            insta::assert_snapshot!(output);

            // The element in the braced expression is a logical line
            // FIXME: Should this be the whole `{2+2}` instead?
            #[rustfmt::skip]
            let (doc, range) = Document::doodle_and_range(
"1+1
{<<2+2>>}
",
            );

            let output = client.format_document_range(&doc, range).await;
            insta::assert_snapshot!(output);

            #[rustfmt::skip]
            let (doc, range) = Document::doodle_and_range(
"1+1
<<{2+2}>>
",
            );
            let output = client.format_document_range(&doc, range).await;
            insta::assert_snapshot!(output);

            // The deepest element in the braced expression is our target
            #[rustfmt::skip]
            let (doc, range) = Document::doodle_and_range(
"1+1
{
  2+2
  {
    <<3+3>>
  }
}
",
            );

            let output = client.format_document_range(&doc, range).await;
            insta::assert_snapshot!(output);
        });
    }

    #[test]
    fn test_format_range_mismatched_indent() {
        with_client(|client| async {
            let mut client = client.lock().await;

            #[rustfmt::skip]
            let (doc, range) = Document::doodle_and_range(
"1
  <<2+2>>
",
            );

            // We don't change indentation when `2+2` is formatted
            let output = client.format_document_range(&doc, range).await;
            insta::assert_snapshot!(output);

            // Debatable: Should we make an effort to remove unneeded indentation
            // when it's part of the range?
            #[rustfmt::skip]
            let (doc, range) = Document::doodle_and_range(
"1
<<  2+2>>
",
            );
            let output_wide = client.format_document_range(&doc, range).await;
            assert_eq!(output, output_wide);
        });
    }

    #[test]
    fn test_format_range_multiple_lines() {
        with_client(|client| async {
            let mut client = client.lock().await;

            #[rustfmt::skip]
            let (doc, range) = Document::doodle_and_range(
"1+1
<<#
2+2>>
",
            );

            let output1 = client.format_document_range(&doc, range).await;
            insta::assert_snapshot!(output1);

            #[rustfmt::skip]
            let (doc, range) = Document::doodle_and_range(
"<<1+1
#
2+2>>
",
            );
            let output2 = client.format_document_range(&doc, range).await;
            insta::assert_snapshot!(output2);
        });
    }

    #[test]
    fn test_format_range_unmatched_lists() {
        with_client(|client| async {
            let mut client = client.lock().await;

            #[rustfmt::skip]
            let (doc, range) = Document::doodle_and_range(
"0+0
<<1+1
{
  2+2>>
}
3+3
",
            );

            let output1 = client.format_document_range(&doc, range).await;
            insta::assert_snapshot!(output1);

            #[rustfmt::skip]
            let (doc, range) = Document::doodle_and_range(
"0+0
<<1+1
{
>>  2+2
}
3+3
",
            );
            let output2 = client.format_document_range(&doc, range).await;
            insta::assert_snapshot!(output2);

            #[rustfmt::skip]
            let (doc, range) = Document::doodle_and_range(
"0+0
<<1+1
{
  2+2
}
>>3+3
",
            );
            let output3 = client.format_document_range(&doc, range).await;
            insta::assert_snapshot!(output3);

            #[rustfmt::skip]
            let (doc, range) = Document::doodle_and_range(
"0+0
1+1
{
<<  2+2
}
>>3+3
",
            );
            let output4 = client.format_document_range(&doc, range).await;
            insta::assert_snapshot!(output4);

            #[rustfmt::skip]
            let (doc, range) = Document::doodle_and_range(
"<<1+1>>
2+2
",
            );

            let output5 = client.format_document_range(&doc, range).await;
            insta::assert_snapshot!(output5);

            #[rustfmt::skip]
            let (doc, range) = Document::doodle_and_range(
"1+1
<<2+2>>
",
            );

            let output6 = client.format_document_range(&doc, range).await;
            insta::assert_snapshot!(output6);
        });
    }

    #[test]
    fn test_format_indent_options() {
        with_client(|client| async {
            let mut client = client.lock().await;

            #[rustfmt::skip]
            let mut doc = Document::doodle("{1}");

            doc.config.indent_width = Some(settings::IndentWidth(8.try_into().unwrap()));
            let output_8_spaces = client.format_document(&doc).await;
            insta::assert_snapshot!(output_8_spaces);

            doc.config.indent_style = Some(settings::IndentStyle::Tab);
            let output_tab = client.format_document(&doc).await;
            insta::assert_snapshot!(output_tab);
        });
    }

    #[test]
    fn test_format_range_indent_options() {
        with_client(|client| async {
            let mut client = client.lock().await;

            #[rustfmt::skip]
            let (mut doc, range) = Document::doodle_and_range("<<{1}>>");

            doc.config.indent_width = Some(settings::IndentWidth(8.try_into().unwrap()));
            let output_8_spaces = client.format_document_range(&doc, range).await;
            insta::assert_snapshot!(output_8_spaces);

            doc.config.indent_style = Some(settings::IndentStyle::Tab);
            let output_tab = client.format_document_range(&doc, range).await;
            insta::assert_snapshot!(output_tab);
        });
    }
}
