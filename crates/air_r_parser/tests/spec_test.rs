use air_r_parser::ParseError;
use air_r_parser::{parse, RParserOptions};
use air_r_syntax::{RRoot, RSyntaxNode};
use biome_console::fmt::{Formatter, Termcolor};
use biome_console::markup;
use biome_diagnostics::display::PrintDiagnostic;
use biome_diagnostics::termcolor;
use biome_diagnostics::DiagnosticExt;
use biome_parser::prelude::ParseDiagnostic;
use biome_rowan::SyntaxNode;
use biome_rowan::SyntaxSlot;
use biome_rowan::{AstNode, SyntaxKind};
use std::fmt::Write;
use std::fs;
use std::path::Path;

#[derive(Copy, Clone)]
pub enum ExpectedOutcome {
    Pass,
    Fail,
    Undefined,
}

#[ignore]
#[test]
fn quick_test() {
    let code = "
    1$foo
    1$2";
    let options = RParserOptions::default();
    let parsed = parse(code, options);
    let root = RRoot::unwrap_cast(parsed.syntax());
    println!("{root:#?}");
}

pub fn run(test_case: &str, _snapshot_name: &str, test_directory: &str, outcome_str: &str) {
    let outcome = match outcome_str {
        "ok" => ExpectedOutcome::Pass,
        "error" => ExpectedOutcome::Fail,
        "undefined" => ExpectedOutcome::Undefined,
        _ => panic!("Invalid expected outcome {outcome_str}"),
    };

    let test_case_path = Path::new(test_case);

    let file_name = test_case_path
        .file_name()
        .expect("Expected test to have a file name")
        .to_str()
        .expect("File name to be valid UTF8");

    let content = fs::read_to_string(test_case_path)
        .expect("Expected test path to be a readable file in UTF8 encoding");

    // Normalize to Unix line endings
    let content = line_ending::normalize(content);

    let options = RParserOptions::default();
    let parsed = parse(&content, options);
    let root = RRoot::unwrap_cast(parsed.syntax());
    let formatted_ast = format!("{:#?}", root);

    let mut snapshot = String::new();
    writeln!(snapshot, "\n## Input\n\n```R\n{content}\n```\n\n").unwrap();

    writeln!(
        snapshot,
        r#"## AST

```
{formatted_ast}
```

## CST

```
{:#?}
```
"#,
        parsed.syntax()
    )
    .unwrap();

    if parsed.has_errors() {
        let diagnostics: Vec<ParseDiagnostic> = parsed
            .errors()
            .iter()
            .map(ParseError::diagnostic)
            .map(ParseDiagnostic::clone)
            .collect();

        let mut diagnostics_buffer = termcolor::Buffer::no_color();

        let termcolor = &mut Termcolor(&mut diagnostics_buffer);
        let mut formatter = Formatter::new(termcolor);

        for diagnostic in diagnostics {
            let error = diagnostic
                .clone()
                .with_file_path(file_name)
                .with_file_source_code(&content);

            formatter
                .write_markup(markup! {
                    {PrintDiagnostic::verbose(&error)}
                })
                .expect("failed to emit diagnostic");
        }

        let formatted_diagnostics =
            std::str::from_utf8(diagnostics_buffer.as_slice()).expect("non utf8 in error buffer");

        if matches!(outcome, ExpectedOutcome::Pass) {
            panic!("Expected no errors to be present in a test case that is expected to pass but the following diagnostics are present:\n{formatted_diagnostics}")
        }

        writeln!(snapshot, "## Diagnostics\n\n```").unwrap();
        snapshot.write_str(formatted_diagnostics).unwrap();

        writeln!(snapshot, "```\n").unwrap();
    }

    match outcome {
        ExpectedOutcome::Pass => {
            let missing_required = formatted_ast.contains("missing (required)");
            if missing_required
                || parsed
                    .syntax()
                    .descendants()
                    .any(|node: RSyntaxNode| node.kind().is_bogus())
            {
                panic!("Parsed tree of a 'OK' test case should not contain any missing required children or bogus nodes: \n {formatted_ast:#?} \n\n {formatted_ast}");
            }

            let syntax = parsed.syntax();
            if has_bogus_nodes_or_empty_slots(&syntax) {
                panic!("modified tree has bogus nodes or empty slots:\n{syntax:#?} \n\n {syntax}")
            }
        }
        ExpectedOutcome::Fail => {
            if !parsed.has_errors() {
                panic!("Failing test must have diagnostics");
            }
        }
        _ => {}
    }

    insta::with_settings!({
        prepend_module_to_snapshot => false,
        snapshot_path => &test_directory,
    }, {
        insta::assert_snapshot!(file_name, snapshot);
    });
}

/// This check is used in the parser test to ensure it doesn't emit
/// bogus nodes without diagnostics.
pub fn has_bogus_nodes_or_empty_slots<L: biome_rowan::Language>(node: &SyntaxNode<L>) -> bool {
    node.descendants().any(|descendant| {
        let kind = descendant.kind();
        if kind.is_bogus() {
            return true;
        }

        if kind.is_list() {
            return descendant
                .slots()
                .any(|slot| matches!(slot, SyntaxSlot::Empty { .. }));
        }

        false
    })
}
