use crate::check::CheckError;
use crate::check::check;
use crate::corpus;
use crate::document::Document;
use std::fmt::Write as _;
use std::fs;
use std::path::Path;

pub fn run_file(input_path: &Path) {
    let display = corpus::relative(input_path);
    let snapshot_path = corpus::snapshot_of(input_path);

    let input =
        fs::read_to_string(input_path).unwrap_or_else(|err| panic!("cannot read {display}: {err}"));
    let input = Document::parse(&input).unwrap_or_else(|err| panic!("{display}: {err}"));

    let committed = fs::read_to_string(&snapshot_path)
        .unwrap_or_else(|_| panic!("{display} has no snapshot. Run `just accept`."));

    // Marker violations fail before any diffing, so a parser bug is not
    // reported as a confusing snapshot mismatch
    let mut generated = input.clone();
    let mut failures = Vec::new();

    for section in &mut generated.sections {
        let expectation = section.expectation;
        let name = section.name.clone();

        // A heading may hold several blocks, so the name alone would not say
        // which one failed
        for (index, block) in section.blocks_mut().enumerate() {
            match check(block, expectation) {
                Ok(cells) => block.generated = cells,
                Err(error) => failures.push((name.clone(), index, error)),
            }
        }
    }

    if !failures.is_empty() {
        panic!("{}", render_failures(&display, &failures));
    }

    // Check that the input and the snapshot are structurally the same
    let snapshot = Document::parse(&committed)
        .unwrap_or_else(|err| panic!("{}: {err}", corpus::relative(&snapshot_path)));

    assert_eq!(
        input.shape(),
        snapshot.shape(),
        "{display} and its snapshot have drifted apart — run `just accept`",
    );

    let rendered = generated.render_snapshot();
    if rendered != committed {
        let diff = similar::TextDiff::from_lines(&committed, &rendered);
        panic!(
            "{}\n{} is out of date. Run `just accept`.",
            diff.unified_diff().header("committed", "generated"),
            corpus::relative(&snapshot_path),
        );
    }
}

fn render_failures(display: &str, failures: &[(String, usize, CheckError)]) -> String {
    let mut out = format!("{} failing block(s) in {display}\n", failures.len());

    for (name, index, error) in failures {
        write!(out, "\n## {name} (block {})\n\n", index + 1).unwrap();
        out.push_str(&render_failure(error));
    }

    out
}

fn render_failure(error: &CheckError) -> String {
    match error {
        CheckError::RoundTrip { expected, got } => format!(
            "The parse did not round-trip, so a token was dropped or invented.\n\
             input:  {expected:?}\n\
             output: {got:?}\n"
        ),
        CheckError::ExpectedError => {
            "Marked `[error]` but produced no error diagnostic.\n".to_string()
        }
        CheckError::UnexpectedError(diagnostics) => {
            format!("Expected a clean parse, but:\n\n{diagnostics}")
        }
        CheckError::BogusNodes(ast) => {
            format!("Bogus nodes in a clean test:\n\n{ast}\n")
        }
        CheckError::UndiagnosedBogusNode { ranges, ast } => format!(
            "Bogus nodes that no diagnostic points at, at {ranges:?}.\n\
             {ast}\n"
        ),
    }
}
