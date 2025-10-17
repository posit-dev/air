use air_formatter_test::spec::{SpecSnapshot, SpecTestFile};
use air_r_formatter::{RFormatLanguage, context::RFormatOptions};
use std::path::Path;

mod language {
    include!("language.rs");
}

pub fn run(spec_input_file: &str, _expected_file: &str, _test_directory: &str, _file_type: &str) {
    let root_path = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/specs/"));

    let spec_input_file = Path::new(spec_input_file);
    let test_file = SpecTestFile::try_from_file(spec_input_file, root_path);

    let options = format_options_for_test(test_file.input_code());
    let language = language::RTestFormatLanguage::default();

    let snapshot = SpecSnapshot::new(test_file, language, RFormatLanguage::new(options));

    snapshot.test()
}

/// Generates an [RFormatOptions] for this test
///
/// At the very top of an R file, provide format options of the form (don't include
/// the backticks):
///
/// ```r
/// #' [format]
/// #' indent-width = 4
/// #' persistent-line-breaks = false
/// ```
///
/// Regardless of whether or not format options are provided in the test, we need to
/// generate our [RFormatOptions] via [workspace::toml_options::TomlOptions],
/// [workspace::settings::Settings], and in particular
/// [workspace::settings::FormatSettings] to ensure that all defaults are set correctly
/// (in particular, for `table`).
fn format_options_for_test(code: &str) -> RFormatOptions {
    let lines = code.lines();

    // Skip blank lines, then collect all leading lines that start with `#'`
    let lines: Vec<&str> = lines
        .skip_while(|line| line.is_empty())
        .take_while(|line| line.starts_with("#'"))
        .collect();

    // Strip off the `#'` and any leading whitespace, that leaves a TOML file left
    let lines: Vec<&str> = lines
        .into_iter()
        .map(|line| line.strip_prefix("#'").unwrap().trim_start())
        .collect();

    let contents = lines.join("\n");

    // Root directory isn't important here as long as we don't supply `exclude`,
    // which would not make sense anyways
    let root = Path::new("");

    let settings = workspace::toml::parse_air_inline_toml(&contents)
        .expect("Can parse inline TOML")
        .into_settings(root)
        .unwrap();

    settings.format.to_format_options(code)
}
