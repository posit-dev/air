use air_r_formatter::context::RFormatOptions;
use air_r_formatter::format_node;
use air_r_parser::RParserOptions;
use air_r_parser::parse;
use settings::RoxygenExamples;

/// Roxygen example formatting is idempotent for already-formatted files, which is the 99%
/// case. But when you have under-indented code that a roxygen block is attached to, then
/// our "adjusted indent width" that we compute from the original source can be too large,
/// and a second indent pass after the underlying code has been reindented can cause the
/// roxygen code to reformat as well. We accept this limitation, especially since it is
/// known to converge in just one extra pass.
///
/// This lives outside the spec suite because that harness asserts idempotence on every
/// fixture via `check_reformat`.
#[test]
fn examples_under_indentation_converges_after_one_pass() {
    // `foo` is under-indented at column 0 in the original source, so our computed
    // adjusted line width is wrong
    let source = "\
outer(inner(deep(
#' @examples
#' some_call(argument_one, argument_two, argument_three, argument_four, arg5678)
foo <- function() {
}
)))
";

    fn format(source: &str) -> String {
        let parsed = parse(source, RParserOptions::default());
        assert!(!parsed.has_error(), "source must parse without errors");

        let options = RFormatOptions::default().with_roxygen_examples(RoxygenExamples::Enabled);

        format_node(options, &parsed.syntax())
            .unwrap()
            .print()
            .unwrap()
            .into_code()
    }

    let first = format(source);
    let second = format(&first);
    let third = format(&second);

    // The first pass wraps against the source indentation, the second against the
    // corrected (larger) one, so they differ
    assert_ne!(first, second);

    // But formatting is stable from the second pass onwards
    assert_eq!(second, third);

    insta::assert_snapshot!("examples_under_indentation_pass_1", first);
    insta::assert_snapshot!("examples_under_indentation_pass_2", second);
}
