use air_r_formatter::{context::RFormatOptions, format_node};
use settings::{IndentStyle, LineWidth};
use tower_lsp::lsp_types;

use crate::state::WorldState;

#[derive(Debug, Eq, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ViewFileParams {
    /// From `lsp_types::TextDocumentPositionParams`
    pub(crate) text_document: lsp_types::TextDocumentIdentifier,
    pub(crate) position: lsp_types::Position,

    /// Viewer type
    pub(crate) kind: ViewFileKind,
}

#[derive(Debug, Eq, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) enum ViewFileKind {
    TreeSitter,
    SyntaxTree,
    FormatTree,
}

pub(crate) fn view_file(params: ViewFileParams, state: &WorldState) -> anyhow::Result<String> {
    let doc = state.get_document_or_error(&params.text_document.uri)?;

    match params.kind {
        ViewFileKind::TreeSitter => {
            let mut parser = tree_sitter::Parser::new();
            parser
                .set_language(&tree_sitter_r::LANGUAGE.into())
                .unwrap();

            let ast = parser.parse(&doc.contents, None).unwrap();

            if ast.root_node().has_error() {
                return Ok(String::from("*Parse error*"));
            }

            let mut output = String::new();
            let mut cursor = ast.root_node().walk();
            format_ts_node(&mut cursor, 0, &mut output);

            Ok(output)
        }

        ViewFileKind::SyntaxTree => {
            if doc.parse.has_error() {
                return Ok(String::from("*Parse error*"));
            }

            let syntax = doc.syntax();
            Ok(format!("{syntax:#?}"))
        }

        ViewFileKind::FormatTree => {
            if doc.parse.has_error() {
                return Ok(String::from("*Parse error*"));
            }

            let line_width = LineWidth::try_from(80).map_err(|err| anyhow::anyhow!("{err}"))?;

            let options = RFormatOptions::default()
                .with_indent_style(IndentStyle::Space)
                .with_line_width(line_width);

            let formatted = format_node(options.clone(), &doc.parse.syntax())?;
            Ok(format!("{}", formatted.into_document()))
        }
    }
}

fn format_ts_node(cursor: &mut tree_sitter::TreeCursor, depth: usize, output: &mut String) {
    let node = cursor.node();
    let field_name = match cursor.field_name() {
        Some(name) => format!("{name}: "),
        None => String::new(),
    };

    let start = node.start_position();
    let end = node.end_position();
    let node_type = node.kind();

    let indent = " ".repeat(depth * 4);
    let start = format!("{}, {}", start.row, start.column);
    let end = format!("{}, {}", end.row, end.column);

    output.push_str(&format!(
        "{indent}{field_name}{node_type} [{start}] - [{end}]\n",
    ));

    if cursor.goto_first_child() {
        loop {
            format_ts_node(cursor, depth + 1, output);
            if !cursor.goto_next_sibling() {
                break;
            }
        }
        cursor.goto_parent();
    }
}
