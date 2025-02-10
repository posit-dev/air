use air_formatter_test::spec::{SpecSnapshot, SpecTestFile};
use air_r_formatter::{context::RFormatOptions, RFormatLanguage};
use std::path::Path;

mod language {
    include!("language.rs");
}

pub fn run(spec_input_file: &str, _expected_file: &str, _test_directory: &str, _file_type: &str) {
    let root_path = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/specs/"));

    let spec_input_file = Path::new(spec_input_file);
    let test_file = SpecTestFile::try_from_file(spec_input_file, root_path);

    let options = format_options_from_code(test_file.input_code());
    let language = language::RTestFormatLanguage::default();

    let snapshot = SpecSnapshot::new(test_file, language, RFormatLanguage::new(options));

    snapshot.test()
}

/// Parse inlined format options provided in a snapshot test
///
/// At the very top of an R file, provide format options of the form (don't include
/// the backticks):
///
/// ```r
/// #' [format]
/// #' indent-width = 4
/// #' persistent-line-breaks = false
/// ```
fn format_options_from_code(code: &str) -> RFormatOptions {
    let lines = code.lines();

    // Skip blank lines, then collect all leading lines that start with `#'`
    let lines: Vec<&str> = lines
        .skip_while(|line| line.is_empty())
        .take_while(|line| line.starts_with("#'"))
        .collect();

    if lines.is_empty() {
        // No file specific configuration
        return RFormatOptions::default();
    }

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
