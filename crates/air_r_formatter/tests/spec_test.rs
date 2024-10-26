use air_r_formatter::{context::RFormatOptions, RFormatLanguage};
use biome_formatter_test::spec::{SpecSnapshot, SpecTestFile};
use std::path::Path;

mod language {
    include!("language.rs");
}

pub fn run(spec_input_file: &str, _expected_file: &str, test_directory: &str, _file_type: &str) {
    let root_path = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/specs/"));

    let spec_input_file = Path::new(spec_input_file);
    let test_file = SpecTestFile::try_from_file(spec_input_file, root_path);

    let options = RFormatOptions::default();
    let language = language::RTestFormatLanguage::default();

    let snapshot = SpecSnapshot::new(
        test_file,
        test_directory,
        language,
        RFormatLanguage::new(options),
    );

    snapshot.test()
}
