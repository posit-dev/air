use air_r_formatter::format_node;
use air_r_parser::RParserOptions;
use lsp_types::request::Request;
use serde::Deserialize;
use serde::Serialize;

use crate::server::api::LSPResult;
use crate::server::client::Notifier;
use crate::server::Result;
use crate::session::DocumentSnapshot;

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub(crate) enum ViewFileKind {
    TreeSitter,
    SyntaxTree,
    FormatTree,
}

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ViewFileParams {
    /// From `lsp_types::TextDocumentPositionParams`
    pub(crate) text_document: lsp_types::TextDocumentIdentifier,
    pub(crate) position: lsp_types::Position,

    /// Viewer type
    pub(crate) kind: ViewFileKind,
}

#[derive(Debug)]
pub(crate) enum ViewFile {}

impl Request for ViewFile {
    type Params = ViewFileParams;
    type Result = String;
    const METHOD: &'static str = "air/viewFile";
}

impl super::RequestHandler for ViewFile {
    type RequestType = ViewFile;
}

impl super::BackgroundDocumentRequestHandler for ViewFile {
    super::define_document_url!(params: &ViewFileParams);
    fn run_with_snapshot(
        snapshot: DocumentSnapshot,
        _notifier: Notifier,
        params: ViewFileParams,
    ) -> Result<String> {
        view_file(&snapshot, &params)
    }
}

fn view_file(snapshot: &DocumentSnapshot, params: &ViewFileParams) -> Result<String> {
    let contents = snapshot.query().as_single_document().contents();
    let settings = snapshot.query().settings();

    match params.kind {
        ViewFileKind::TreeSitter => {
            let mut parser = tree_sitter::Parser::new();
            parser
                .set_language(&tree_sitter_r::LANGUAGE.into())
                .map_err(anyhow::Error::new)
                .with_failure_code(lsp_server::ErrorCode::InternalError)?;

            let Some(ast) = parser.parse(contents, None) else {
                return Err(anyhow::anyhow!("Internal error during document parsing."))
                    .with_failure_code(lsp_server::ErrorCode::InternalError);
            };

            if ast.root_node().has_error() {
                return Ok(String::from("*Parse error*"));
            }

            let mut output = String::new();
            let mut cursor = ast.root_node().walk();
            format_ts_node(&mut cursor, 0, &mut output);
            Ok(output)
        }

        ViewFileKind::SyntaxTree => {
            let parse = air_r_parser::parse(contents, RParserOptions::default());

            if parse.has_errors() {
                return Ok(String::from("*Parse error*"));
            }

            Ok(format!("{syntax:#?}", syntax = parse.syntax()))
        }

        ViewFileKind::FormatTree => {
            let parse = air_r_parser::parse(contents, RParserOptions::default());

            if parse.has_errors() {
                return Ok(String::from("*Parse error*"));
            }

            let format_options = settings.format.to_format_options(contents);

            let formatted = format_node(format_options, &parse.syntax())
                .map_err(anyhow::Error::new)
                .with_failure_code(lsp_server::ErrorCode::InternalError)?;

            Ok(format!("{document}", document = formatted.into_document()))
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
