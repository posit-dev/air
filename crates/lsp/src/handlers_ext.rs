use std::path::PathBuf;

use air_r_formatter::{context::RFormatOptions, format_node};
use biome_lsp_converters::line_index;
use biome_lsp_converters::PositionEncoding;
use itertools::Either;
use itertools::Itertools;
use settings::{IndentStyle, LineWidth};
use tower_lsp::lsp_types;
use url::Url;
use workspace::discovery::discover_r_file_paths;
use workspace::format::format_file_with_normalized_line_endings;
use workspace::format::format_source_with_parse;
use workspace::format::FormattedFile;
use workspace::format::FormattedSource;
use workspace::settings::FormatSettings;

use crate::main_loop::LspState;
use crate::rust_analyzer::line_index::LineIndex;
use crate::state::WorldState;
use crate::to_proto::replace_all_edit;

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

#[derive(Debug, Eq, PartialEq, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WorkspaceFolderFormattingParams {
    /// The workspace folder to format
    ///
    /// It is guaranteed that the LSP server will have been notified about this workspace
    /// folder, either through [lsp_types::InitializeParams] or through
    /// [lsp_types::DidChangeWorkspaceFoldersParams].
    pub(crate) workspace_folder: lsp_types::WorkspaceFolder,
}

#[derive(Debug, Eq, PartialEq, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct WorkspaceFolderFormattingResult {
    /// The workspace edit containing the multi-file formatting edit
    ///
    /// If no formatting is required, [None] can be returned.
    pub(crate) workspace_edit: Option<lsp_types::WorkspaceEdit>,
}

pub(crate) fn workspace_folder_formatting(
    params: WorkspaceFolderFormattingParams,
    lsp_state: &LspState,
    state: &WorldState,
) -> anyhow::Result<WorkspaceFolderFormattingResult> {
    let Ok(workspace_folder_path) = params.workspace_folder.uri.to_file_path() else {
        return Err(anyhow::anyhow!(
            "Failed to convert workspace folder uri to file path: {uri}",
            uri = params.workspace_folder.uri
        ));
    };

    let Some(workspace_folder) = lsp_state
        .workspace_settings_resolver
        .workspace_folders()
        .iter()
        .find(|workspace_folder| workspace_folder.path() == workspace_folder_path)
    else {
        return Err(anyhow::anyhow!(
            "Workspace folder not recognized: {workspace_folder_path}",
            workspace_folder_path = workspace_folder_path.display()
        ));
    };

    let workspace_folder_path = workspace_folder.path();
    let workspace_folder_resolver = workspace_folder.value();

    tracing::trace!(
        "Formatting workspace folder: {workspace_folder_path}",
        workspace_folder_path = workspace_folder_path.display()
    );

    let paths = discover_r_file_paths(&[workspace_folder_path], workspace_folder_resolver, true);

    // For each path, format the underlying file. If no formatting changes are required,
    // the path is filtered out. Errors are passed through and relayed at the end.
    let (edits, errors): (Vec<lsp_types::TextDocumentEdit>, Vec<anyhow::Error>) = paths
        .into_iter()
        .filter_map(|path| match path {
            Ok(path) => {
                let settings = workspace_folder_resolver.resolve_or_fallback(&path);
                match format_workspace_file(
                    path,
                    &settings.format,
                    state,
                    lsp_state.position_encoding,
                ) {
                    Ok(edit) => edit.map(Ok),
                    Err(err) => Some(Err(err)),
                }
            }
            Err(err) => Some(Err(err.into())),
        })
        .partition_map(|edit| match edit {
            Ok(edit) => Either::Left(edit),
            Err(err) => Either::Right(err),
        });

    // Log individual file level errors
    for error in errors {
        tracing::error!("{error}");
    }

    // TODO: Add some tests around this, see notes
    // - In theory it won't matter if you have a sub folder open
    // (actually it will `wsf1/air.toml`, `wsf1/wsf2/test.R`, `wsf1/wsf2/subdir/air.toml`)
    // depends on where we stop looking for settings for test.R.

    let workspace_edit = if edits.is_empty() {
        None
    } else {
        Some(lsp_types::WorkspaceEdit {
            changes: None,
            document_changes: Some(lsp_types::DocumentChanges::Edits(edits)),
            change_annotations: None,
        })
    };

    Ok(WorkspaceFolderFormattingResult { workspace_edit })
}

fn format_workspace_file(
    path: PathBuf,
    settings: &FormatSettings,
    state: &WorldState,
    position_encoding: PositionEncoding,
) -> anyhow::Result<Option<lsp_types::TextDocumentEdit>> {
    let Ok(uri) = lsp_types::Url::from_file_path(&path) else {
        return Err(anyhow::anyhow!(
            "Failed to convert `path` to a valid URI: {path}",
            path = path.display()
        ));
    };

    match state.get_document(&uri) {
        Some(document) => {
            // If the server knows about the document, use those contents, as they will
            // contain up to date edits (even unsaved changes!).
            let old = &document.contents;
            let parse = &document.parse;

            if parse.has_error() {
                return Err(anyhow::anyhow!(
                    "Can't format file with parse errors: {path}",
                    path = path.display()
                ));
            }

            let options = settings.to_format_options(old);
            let new = format_source_with_parse(old, parse, options)?;

            match new {
                FormattedSource::Changed(ref new) => {
                    let line_index = &document.line_index;
                    let version = document.version;

                    Ok(Some(as_text_document_edit(
                        uri, old, new, line_index, version,
                    )?))
                }
                FormattedSource::Unchanged => Ok(None),
            }
        }
        None => {
            // We have to use normalized line endings when formatting in the LSP
            // - `Document` above also does this so it's good to be consistent
            // - `replace_all_edit()` will turn any `\n` into `\r\n` when `Crlf` is set as
            //   the ending, and we'll end up with `\r\r\n` if we don't normalize `\r\n`
            //   to `\n` here first.
            // It would be nice if we didn't have this restriction and could use the
            // original line endings everywhere (even in `Document`).
            let (file, endings) = format_file_with_normalized_line_endings(path, settings)?;

            match file {
                FormattedFile::Changed(file) => {
                    let old = file.old();
                    let new = file.new();

                    let line_index = LineIndex {
                        index: triomphe::Arc::new(line_index::LineIndex::new(old)),
                        endings,
                        encoding: position_encoding,
                    };

                    let version = None;

                    Ok(Some(as_text_document_edit(
                        uri,
                        old,
                        new,
                        &line_index,
                        version,
                    )?))
                }
                FormattedFile::Unchanged => Ok(None),
            }
        }
    }
}

fn as_text_document_edit(
    uri: Url,
    old: &str,
    new: &str,
    line_index: &LineIndex,
    version: Option<i32>,
) -> anyhow::Result<lsp_types::TextDocumentEdit> {
    let text_document = lsp_types::OptionalVersionedTextDocumentIdentifier { uri, version };

    let edits = replace_all_edit(line_index, old, new)?;

    let edits: Vec<lsp_types::OneOf<lsp_types::TextEdit, lsp_types::AnnotatedTextEdit>> =
        edits.into_iter().map(lsp_types::OneOf::Left).collect();

    Ok(lsp_types::TextDocumentEdit {
        text_document,
        edits,
    })
}
