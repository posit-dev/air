//
// handlers_format.rs
//
// Copyright (C) 2024 Posit Software, PBC. All rights reserved.
//
//

use air_r_syntax::{RExpressionList, RSyntaxKind, RSyntaxNode, WalkEvent};
use biome_rowan::{AstNode, Language, SyntaxElement};
use biome_text_size::{TextRange, TextSize};
use tower_lsp::lsp_types;
use workspace::format::FormattedSource;
use workspace::format::format_source_with_parse;

use crate::file_patterns::is_document_excluded_from_formatting;
use crate::main_loop::LspState;
use crate::proto::{from_proto, to_proto};
use crate::state::WorldState;

#[tracing::instrument(level = "info", skip_all)]
pub(crate) fn document_formatting(
    params: lsp_types::DocumentFormattingParams,
    lsp_state: &LspState,
    state: &WorldState,
) -> anyhow::Result<Option<Vec<lsp_types::TextEdit>>> {
    let uri = &params.text_document.uri;
    let doc = state.get_document_or_error(uri)?;

    let workspace_settings = lsp_state.workspace_document_settings(uri);

    match uri.to_file_path() {
        Ok(path) => {
            // TODO: `language_id` should be a property of the `Document` stored in `did_open()`
            let language_id = String::from("r");
            let settings = workspace_settings.settings();

            if is_document_excluded_from_formatting(&path, &settings.format, language_id) {
                return Ok(None);
            }
        }
        Err(_) => {
            // `untitled:Untitled-1` with an 'r' `language_id` comes through here, as an example
            tracing::trace!("Can't convert uri to file path, assuming we can format it: {uri}")
        }
    }

    if doc.parse.has_error() {
        // Refuse to format in the face of parse errors, but only log a warning
        // rather than returning an LSP error, as toast notifications here are distracting.
        tracing::warn!("Failed to format {uri}. Can't format when there are parse errors.");
        return Ok(None);
    }

    let format_options = workspace_settings.to_format_options(&doc.contents, &doc.settings);

    match format_source_with_parse(&doc.contents, &doc.parse, format_options)? {
        FormattedSource::Changed(mut formatted) => {
            // For notebook cells, remove the trailing newline that the formatter adds.
            // The formatter always adds a trailing newline for R files (which is correct
            // for standalone files), but notebook cells should not have this trailing newline
            // to avoid an empty line at the end of the chunk.
            if uri.scheme() == "vscode-notebook-cell" && formatted.ends_with('\n') {
                formatted.pop();
            }

            Ok(Some(to_proto::replace_all_edit(
                &doc.contents,
                &formatted,
                &doc.line_index,
                doc.position_encoding,
                doc.endings,
            )?))
        }
        FormattedSource::Unchanged => Ok(None),
    }
}

#[tracing::instrument(level = "info", skip_all)]
pub(crate) fn document_range_formatting(
    params: lsp_types::DocumentRangeFormattingParams,
    lsp_state: &LspState,
    state: &WorldState,
) -> anyhow::Result<Option<Vec<lsp_types::TextEdit>>> {
    let uri = &params.text_document.uri;
    let doc = state.get_document_or_error(uri)?;

    let workspace_settings = lsp_state.workspace_document_settings(uri);

    match uri.to_file_path() {
        Ok(path) => {
            // TODO: `language_id` should be a property of the `Document` stored in `did_open()`
            let language_id = String::from("r");
            let settings = workspace_settings.settings();

            if is_document_excluded_from_formatting(&path, &settings.format, language_id) {
                return Ok(None);
            }
        }
        Err(_) => {
            // `untitled:Untitled-1` with an 'r' `language_id` comes through here, as an example
            tracing::trace!("Can't convert uri to file path, assuming we can format it: {uri}")
        }
    }

    if doc.parse.has_error() {
        // Refuse to format in the face of parse errors, but only log a warning
        // rather than returning an LSP error, as toast notifications here are distracting.
        tracing::warn!("Failed to format {uri}. Can't format when there are parse errors.");
        return Ok(None);
    }

    let range = from_proto::text_range(params.range, &doc.line_index, doc.position_encoding)?;

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

    let format_options = workspace_settings.to_format_options(&doc.contents, &doc.settings);

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
    let edits = to_proto::replace_range_edit(
        format_range,
        format_text,
        &doc.line_index,
        doc.position_encoding,
        doc.endings,
    )?;

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
        .skip_while(|node| !node.text_range_with_trivia().contains(range.start()))
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
                    let node_range = node.text_range_with_trivia();
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
    use crate::documents::Document;
    use crate::proto::PositionEncoding;
    use crate::test::FileName;
    use crate::test::TestClientExt;
    use crate::test::new_test_client;
    use std::path::Path;
    use tower_lsp::lsp_types::DidChangeWorkspaceFoldersParams;
    use tower_lsp::lsp_types::WorkspaceFolder;
    use tower_lsp::lsp_types::WorkspaceFoldersChangeEvent;
    use url::Url;

    #[tokio::test]
    async fn test_format() {
        let mut client = new_test_client().await;

        #[rustfmt::skip]
        let doc = Document::doodle(
"
1
2+2
3 + 3 +
3",
        );

        let formatted = client.format_document(&doc, FileName::Random).await;
        insta::assert_snapshot!(formatted);
    }

    // https://github.com/posit-dev/air/issues/61
    #[tokio::test]
    async fn test_format_minimal_diff() {
        let mut client = new_test_client().await;

        #[rustfmt::skip]
        let doc = Document::doodle(
"1
2+2
3
",
        );

        let edits = client
            .format_document_edits(&doc, FileName::Random)
            .await
            .unwrap();
        assert!(edits.len() == 1);

        let edit = &edits[0];
        assert_eq!(edit.new_text, " + ");
    }

    #[tokio::test]
    async fn test_format_range_none() {
        let mut client = new_test_client().await;

        #[rustfmt::skip]
        let (doc, range) = Document::doodle_and_range(
"<<>>",
        );

        let output = client
            .format_document_range(&doc, FileName::Random, range)
            .await;
        insta::assert_snapshot!(output);

        #[rustfmt::skip]
        let (doc, range) = Document::doodle_and_range(
"<<
>>",
        );

        let output = client
            .format_document_range(&doc, FileName::Random, range)
            .await;
        insta::assert_snapshot!(output);

        #[rustfmt::skip]
        let (doc, range) = Document::doodle_and_range(
"<<1
>>",
        );

        let output = client
            .format_document_range(&doc, FileName::Random, range)
            .await;
        insta::assert_snapshot!(output);
    }

    #[tokio::test]
    async fn test_format_range_logical_lines() {
        let mut client = new_test_client().await;

        // 2+2 is the logical line to format
        #[rustfmt::skip]
        let (doc, range) = Document::doodle_and_range(
"1+1
<<2+2>>
",
        );
        let output = client
            .format_document_range(&doc, FileName::Random, range)
            .await;
        insta::assert_snapshot!(output);

        #[rustfmt::skip]
        let (doc, range) = Document::doodle_and_range(
"1+1
#
<<2+2>>
",
        );

        let output = client
            .format_document_range(&doc, FileName::Random, range)
            .await;
        insta::assert_snapshot!(output);

        // The element in the braced expression is a logical line
        // FIXME: Should this be the whole `{2+2}` instead?
        #[rustfmt::skip]
        let (doc, range) = Document::doodle_and_range(
"1+1
{<<2+2>>}
",
        );

        let output = client
            .format_document_range(&doc, FileName::Random, range)
            .await;
        insta::assert_snapshot!(output);

        #[rustfmt::skip]
        let (doc, range) = Document::doodle_and_range(
"1+1
<<{2+2}>>
",
        );
        let output = client
            .format_document_range(&doc, FileName::Random, range)
            .await;
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

        let output = client
            .format_document_range(&doc, FileName::Random, range)
            .await;
        insta::assert_snapshot!(output);
    }

    #[tokio::test]
    async fn test_format_range_mismatched_indent() {
        let mut client = new_test_client().await;

        #[rustfmt::skip]
        let (doc, range) = Document::doodle_and_range(
"1
  <<2+2>>
",
        );

        // We don't change indentation when `2+2` is formatted
        let output = client
            .format_document_range(&doc, FileName::Random, range)
            .await;
        insta::assert_snapshot!(output);

        // Debatable: Should we make an effort to remove unneeded indentation
        // when it's part of the range?
        #[rustfmt::skip]
        let (doc, range) = Document::doodle_and_range(
"1
<<  2+2>>
",
        );
        let output_wide = client
            .format_document_range(&doc, FileName::Random, range)
            .await;
        assert_eq!(output, output_wide);
    }

    #[tokio::test]
    async fn test_format_range_multiple_lines() {
        let mut client = new_test_client().await;

        #[rustfmt::skip]
        let (doc, range) = Document::doodle_and_range(
"1+1
<<#
2+2>>
",
        );

        let output1 = client
            .format_document_range(&doc, FileName::Random, range)
            .await;
        insta::assert_snapshot!(output1);

        #[rustfmt::skip]
        let (doc, range) = Document::doodle_and_range(
"<<1+1
#
2+2>>
",
        );
        let output2 = client
            .format_document_range(&doc, FileName::Random, range)
            .await;
        insta::assert_snapshot!(output2);
    }

    #[tokio::test]
    async fn test_format_range_unmatched_lists() {
        let mut client = new_test_client().await;

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

        let output1 = client
            .format_document_range(&doc, FileName::Random, range)
            .await;
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
        let output2 = client
            .format_document_range(&doc, FileName::Random, range)
            .await;
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
        let output3 = client
            .format_document_range(&doc, FileName::Random, range)
            .await;
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
        let output4 = client
            .format_document_range(&doc, FileName::Random, range)
            .await;
        insta::assert_snapshot!(output4);

        #[rustfmt::skip]
        let (doc, range) = Document::doodle_and_range(
"<<1+1>>
2+2
",
        );

        let output5 = client
            .format_document_range(&doc, FileName::Random, range)
            .await;
        insta::assert_snapshot!(output5);

        #[rustfmt::skip]
        let (doc, range) = Document::doodle_and_range(
"1+1
<<2+2>>
",
        );

        let output6 = client
            .format_document_range(&doc, FileName::Random, range)
            .await;
        insta::assert_snapshot!(output6);
    }

    #[tokio::test]
    async fn test_format_indent_options() {
        let mut client = new_test_client().await;

        #[rustfmt::skip]
        let mut doc = Document::doodle("{1}");

        doc.settings.indent_width = Some(settings::IndentWidth::try_from(8_u8).unwrap());
        let output_8_spaces = client.format_document(&doc, FileName::Random).await;
        insta::assert_snapshot!(output_8_spaces);

        doc.settings.indent_style = Some(settings::IndentStyle::Tab);
        let output_tab = client.format_document(&doc, FileName::Random).await;
        insta::assert_snapshot!(output_tab);
    }

    #[tokio::test]
    async fn test_format_range_indent_options() {
        let mut client = new_test_client().await;

        #[rustfmt::skip]
        let (mut doc, range) = Document::doodle_and_range("<<{1}>>");

        doc.settings.indent_width = Some(settings::IndentWidth::try_from(8_u8).unwrap());
        let output_8_spaces = client
            .format_document_range(&doc, FileName::Random, range)
            .await;
        insta::assert_snapshot!(output_8_spaces);

        doc.settings.indent_style = Some(settings::IndentStyle::Tab);
        let output_tab = client
            .format_document_range(&doc, FileName::Random, range)
            .await;
        insta::assert_snapshot!(output_tab);
    }

    #[tokio::test]
    async fn test_format_untitled_files() {
        let mut client = new_test_client().await;

        // `untitled:Untitled-1` is what VS Code gives us for an untitled file
        // that has its language id set to 'r'
        let filename = FileName::Url(String::from("untitled:Untitled-1"));

        let input = "1+1";
        let expect = "1 + 1\n";
        let doc = Document::doodle(input);
        let output = client.format_document(&doc, filename).await;

        assert_eq!(output, expect);
    }

    #[tokio::test]
    async fn test_format_notebook_cells() {
        let mut client = new_test_client().await;

        // Notebook cells use the `vscode-notebook-cell` scheme
        // They should NOT have a trailing newline, unlike regular files
        let filename = FileName::Url(String::from("vscode-notebook-cell:/path/to/notebook#cell1"));

        let input = "1+1";
        // Note: No trailing newline in the expected output for notebook cells
        let expect = "1 + 1";
        let doc = Document::doodle(input);
        let output = client.format_document(&doc, filename).await;

        assert_eq!(output, expect);

        // Test with multiple lines
        let filename = FileName::Url(String::from("vscode-notebook-cell:/path/to/notebook#cell2"));
        let input = "1+1\nx<-2";
        let expect = "1 + 1\nx <- 2";
        let doc = Document::doodle(input);
        let output = client.format_document(&doc, filename).await;

        assert_eq!(output, expect);
    }

    #[tokio::test]
    async fn test_format_default_excluded_files() {
        let as_file_url = |path: &str| {
            #[cfg(not(windows))]
            let prefix = "/";
            #[cfg(windows)]
            let prefix = "C:/";

            // file://<hostname>/<path>
            // - <hostname> is the empty string for us
            // - <path> must start with the right OS specific prefix
            format!("file:///{prefix}{path}")
        };

        let mut client = new_test_client().await;

        // `cpp11.R` is excluded from formatting by default
        let filename = FileName::Url(as_file_url("cpp11.R"));
        let input = "1+1";
        let doc = Document::doodle(input);
        let output = client.format_document(&doc, filename).await;
        assert_eq!(output, input);

        // `renv/` is excluded from formatting by default
        let filename = FileName::Url(as_file_url("renv/activate.R"));
        let input = "1+1";
        let doc = Document::doodle(input);
        let output = client.format_document(&doc, filename).await;
        assert_eq!(output, input);
    }

    #[tokio::test]
    async fn test_format_excluded_files() {
        let as_file_url = |path: &Path| format!("file:///{path}", path = path.display());

        let mut client = new_test_client().await;

        // Create a tempdir that will serve as our workspace
        let tempdir = tempfile::TempDir::new().unwrap();
        let tempdir = tempdir.path();

        // Note that `test.R` is formatted by default
        let input = "1+1";
        let output = "1 + 1\n";
        let url = as_file_url(tempdir.join("test.R").as_path());
        let filename = FileName::Url(url);
        let doc = Document::new(input.to_string(), Some(0), PositionEncoding::Utf8);
        let result = client.format_document(&doc, filename).await;
        assert_eq!(result, output);

        // Write the `air.toml` to disk inside the workspace so the server can discover it
        let air_path = tempdir.join("air.toml");
        let air_contents = r#"
[format]
exclude = ["test.R"]
default-exclude = false
"#;
        std::fs::write(&air_path, air_contents).unwrap();

        // Open the tempdir as the workspace, the server will load in the `air.toml`
        let workspace_folder = WorkspaceFolder {
            uri: Url::parse(&as_file_url(tempdir)).unwrap(),
            name: "workspace".to_string(),
        };
        client
            .did_change_workspace_folders(DidChangeWorkspaceFoldersParams {
                event: WorkspaceFoldersChangeEvent {
                    added: vec![workspace_folder],
                    removed: vec![],
                },
            })
            .await;

        // Now `test.R` should be excluded
        let input = "1+1";
        let url = as_file_url(tempdir.join("test.R").as_path());
        let filename = FileName::Url(url);
        let doc = Document::new(input.to_string(), Some(0), PositionEncoding::Utf8);
        let result = client.format_document(&doc, filename).await;
        assert_eq!(result, input);

        // And `cpp11.R` should now be formatted
        let input = "1+1";
        let output = "1 + 1\n";
        let url = as_file_url(tempdir.join("cpp11.R").as_path());
        let filename = FileName::Url(url);
        let doc = Document::new(input.to_string(), Some(0), PositionEncoding::Utf8);
        let result = client.format_document(&doc, filename).await;
        assert_eq!(result, output);
    }

    #[tokio::test]
    async fn test_format_skip_functions() {
        let as_file_url = |path: &Path| format!("file:///{path}", path = path.display());

        let mut client = new_test_client().await;

        // Create a tempdir that will serve as our workspace
        let tempdir = tempfile::TempDir::new().unwrap();
        let tempdir = tempdir.path();

        // Note that `graph_from_literal()` is formatted by default
        let input = r#"
igraph::graph_from_literal(Alice +--+ Jerry)
1+1
"#
        .trim_start();
        let output = r#"
igraph::graph_from_literal(Alice + --+Jerry)
1 + 1
"#
        .trim_start();
        let url = as_file_url(tempdir.join("test.R").as_path());
        let filename = FileName::Url(url);
        let doc = Document::new(input.to_string(), Some(0), PositionEncoding::Utf8);
        let result = client.format_document(&doc, filename).await;
        assert_eq!(result, output);

        // Write the `air.toml` to disk inside the workspace so the server can discover it
        let air_path = tempdir.join("air.toml");
        let air_contents = r#"
[format]
skip = ["graph_from_literal"]
"#;
        std::fs::write(&air_path, air_contents).unwrap();

        // Open the tempdir as the workspace, the server will load in the `air.toml`
        let workspace_folder = WorkspaceFolder {
            uri: Url::parse(&as_file_url(tempdir)).unwrap(),
            name: "workspace".to_string(),
        };
        client
            .did_change_workspace_folders(DidChangeWorkspaceFoldersParams {
                event: WorkspaceFoldersChangeEvent {
                    added: vec![workspace_folder],
                    removed: vec![],
                },
            })
            .await;

        // Now `graph_from_literal()` should be skipped, but `1+1` is still formatted
        let input = r#"
igraph::graph_from_literal(Alice +--+ Jerry)
1+1
"#
        .trim_start();
        let output = r#"
igraph::graph_from_literal(Alice +--+ Jerry)
1 + 1
"#
        .trim_start();
        let url = as_file_url(tempdir.join("test.R").as_path());
        let filename = FileName::Url(url);
        let doc = Document::new(input.to_string(), Some(0), PositionEncoding::Utf8);
        let result = client.format_document(&doc, filename).await;
        assert_eq!(result, output);
    }

    #[tokio::test]
    async fn test_files_outside_the_workspace_dont_get_workspace_settings() {
        // https://github.com/posit-dev/air/issues/294

        let as_file_url = |path: &Path| format!("file:///{path}", path = path.display());

        let mut client = new_test_client().await;

        // Create a tempdir that will serve as our base of operations
        let tempdir = tempfile::TempDir::new().unwrap();
        let tempdir = tempdir.path();

        // Note that `workspace < directory` lexicographically, but we still don't want
        // `workspace` settings to apply to files in `directory`. We want default settings
        // to apply.
        let workspace = tempdir.join("a");
        let directory = tempdir.join("b");

        std::fs::create_dir(&workspace).unwrap();

        // Write the `air.toml` to disk inside the workspace so the server can discover it
        let air_path = workspace.join("air.toml");
        let air_contents = r#"
[format]
indent-width = 8
"#;
        std::fs::write(&air_path, air_contents).unwrap();

        // Open `workspace` as the workspace, the server will load in the `air.toml`
        let workspace_folder = WorkspaceFolder {
            uri: Url::parse(&as_file_url(workspace.as_path())).unwrap(),
            name: "workspace".to_string(),
        };
        client
            .did_change_workspace_folders(DidChangeWorkspaceFoldersParams {
                event: WorkspaceFoldersChangeEvent {
                    added: vec![workspace_folder],
                    removed: vec![],
                },
            })
            .await;

        // `{directory}/test.R` should get default settings, as it is not under `{workspace}`
        let input = "list(\n  1\n)\n";
        let output = "list(\n  1\n)\n";
        let url = as_file_url(directory.join("test.R").as_path());
        let filename = FileName::Url(url);
        let doc = Document::new(input.to_string(), Some(0), PositionEncoding::Utf8);
        let result = client.format_document(&doc, filename).await;
        assert_eq!(result, output);

        // `{workspace}/test.R` should get workspace settings (and the increased indent)
        let input = "list(\n  1\n)\n";
        let output = "list(\n        1\n)\n";
        let url = as_file_url(workspace.join("test.R").as_path());
        let filename = FileName::Url(url);
        let doc = Document::new(input.to_string(), Some(0), PositionEncoding::Utf8);
        let result = client.format_document(&doc, filename).await;
        assert_eq!(result, output);
    }
}
