use crate::document::Block;
use crate::document::Cell;
use crate::document::Expectation;
use air_r_parser::RParse;
use air_r_parser::RParserOptions;
use air_r_parser::parse;
use air_r_syntax::RSyntaxNode;
use air_r_syntax::TextRange;
use biome_console::fmt::Formatter;
use biome_console::fmt::Termcolor;
use biome_console::markup;
use biome_diagnostics::Diagnostic;
use biome_diagnostics::DiagnosticExt;
use biome_diagnostics::PrintDiagnostic;
use biome_diagnostics::termcolor::NoColor;
use biome_parser::diagnostic::ParseDiagnostic;
use biome_rowan::SyntaxKind;
use biome_rowan::SyntaxNode;

#[derive(Debug)]
pub enum CheckError {
    RoundTrip { expected: String, got: String },
    ExpectedError,
    UnexpectedError(String),
    BogusNodes(String),
    UndiagnosedBogusNode { ranges: Vec<TextRange>, ast: String },
}

/// One code block to check
///
/// Emits CST and diagnostic cells to write in the snapshot
pub fn check(block: &Block, expectation: Expectation) -> Result<Vec<Cell>, CheckError> {
    let parsed = parse(&block.source, RParserOptions::default());
    let ast = format!("{:#?}", parsed.tree());
    let root = parsed.syntax();

    let round_tripped = root.to_string();
    if round_tripped != block.source {
        return Err(CheckError::RoundTrip {
            expected: block.source.clone(),
            got: round_tripped,
        });
    }

    let has_errors = parsed.has_errors();
    let has_bogus = has_bogus_nodes_or_missing_required_children(&ast, &root);

    match expectation {
        Expectation::Error => {
            if !has_errors {
                return Err(CheckError::ExpectedError);
            }
        }
        Expectation::Clean => {
            if has_errors {
                return Err(CheckError::UnexpectedError(render_diagnostics(
                    &parsed,
                    &block.source,
                )));
            }

            if has_bogus {
                return Err(CheckError::BogusNodes(ast));
            }
        }
    }

    let unexplained = undiagnosed_bogus_nodes(&root, parsed.diagnostics());
    if !unexplained.is_empty() {
        return Err(CheckError::UndiagnosedBogusNode {
            ranges: unexplained
                .iter()
                .map(SyntaxNode::text_trimmed_range)
                .collect(),
            ast,
        });
    }

    let mut cells = vec![Cell::Cst(ast)];
    if !parsed.diagnostics().is_empty() {
        cells.push(Cell::Diagnostics(render_diagnostics(
            &parsed,
            &block.source,
        )));
    }

    Ok(cells)
}

fn undiagnosed_bogus_nodes(
    root: &RSyntaxNode,
    diagnostics: &[ParseDiagnostic],
) -> Vec<RSyntaxNode> {
    let spans: Vec<TextRange> = diagnostics
        .iter()
        .filter_map(|diagnostic| diagnostic.location().span)
        .collect();

    root.descendants()
        .filter(|node| node.kind().is_bogus())
        .filter(|node| {
            let range = node.text_trimmed_range();
            !spans.iter().any(|span| span.intersect(range).is_some())
        })
        .collect()
}

fn has_bogus_nodes_or_missing_required_children(ast: &str, root: &RSyntaxNode) -> bool {
    ast.contains("missing (required)") || root.descendants().any(|node| node.kind().is_bogus())
}

fn render_diagnostics(parse: &RParse, source: &str) -> String {
    let mut out = Vec::new();

    for diagnostic in parse.diagnostics() {
        let diagnostic = diagnostic
            .clone()
            .with_file_path("test.R")
            .with_file_source_code(source);

        Formatter::new(&mut Termcolor(NoColor::new(&mut out)))
            .write_markup(markup! { {PrintDiagnostic::verbose(&diagnostic)} })
            .expect("writing a diagnostic to a string cannot fail");
    }

    let rendered = String::from_utf8(out).expect("rendered diagnostics are UTF-8");

    // Biome pads its frames out to the terminal width. Committed snapshots have
    // to survive an editor that strips trailing whitespace.
    let mut trimmed = String::with_capacity(rendered.len());
    for line in rendered.lines() {
        trimmed.push_str(line.trim_end());
        trimmed.push('\n');
    }

    trimmed.truncate(trimmed.trim_end().len());
    trimmed.push('\n');
    trimmed
}

#[cfg(test)]
mod tests {
    use super::*;

    const CLEAN: &str = "# a comment\n";

    /// Causes a lexer diagnostic
    const ERRORS: &str = "`a\n";

    fn block(source: &str) -> Block {
        Block {
            source: source.to_string(),
            generated: Vec::new(),
        }
    }

    #[test]
    fn a_clean_block_gets_a_cst_cell_and_no_diagnostics_cell() {
        let cells = check(&block(CLEAN), Expectation::Clean).expect("parses cleanly");

        assert!(matches!(cells.as_slice(), [Cell::Cst(_)]));
    }

    #[test]
    fn an_error_block_gets_both_cells() {
        let cells = check(&block(ERRORS), Expectation::Error).expect("errors as marked");

        let [Cell::Cst(_), Cell::Diagnostics(diagnostics)] = cells.as_slice() else {
            panic!("expected a cst cell and a diagnostics cell, got {cells:?}");
        };
        assert!(diagnostics.contains("Unterminated backtick-quoted name."));
    }

    #[test]
    fn an_unmarked_block_may_not_error() {
        let error = check(&block(ERRORS), Expectation::Clean).expect_err("errors");

        assert!(matches!(error, CheckError::UnexpectedError(_)));
    }

    #[test]
    fn an_error_block_must_error() {
        let error = check(&block(CLEAN), Expectation::Error).expect_err("parses cleanly");

        assert!(matches!(error, CheckError::ExpectedError));
    }
}
