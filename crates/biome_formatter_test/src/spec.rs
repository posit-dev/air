use crate::check_reformat::CheckReformat;
use crate::snapshot_builder::{SnapshotBuilder, SnapshotOutput};
use crate::TestFormatLanguage;
use biome_formatter::{FormatLanguage, FormatOptions, Printed};
use biome_parser::AnyParse;
use biome_rowan::{TextRange, TextSize};
use std::ops::Range;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct SpecTestFile<'a> {
    input_file: &'a Path,
    root_path: &'a Path,

    input_code: String,

    range_start_index: Option<usize>,
    range_end_index: Option<usize>,
}

impl<'a> SpecTestFile<'a> {
    pub fn try_from_file(input_file: &'a Path, root_path: &'a Path) -> SpecTestFile<'a> {
        assert!(
            input_file.is_file(),
            "The input '{}' must exist and be a file.",
            input_file.display()
        );

        let input_code = std::fs::read_to_string(input_file).unwrap();

        // For the whole file, not a specific range right now
        let range_start_index = None;
        let range_end_index = None;

        SpecTestFile {
            input_file,
            root_path,

            input_code,

            range_start_index,
            range_end_index,
        }
    }

    pub fn input_code(&self) -> &str {
        &self.input_code
    }

    pub fn file_name(&self) -> &str {
        self.input_file.to_str().unwrap()
    }

    pub fn input_file(&self) -> &Path {
        self.input_file
    }

    pub fn relative_file_name(&self) -> &str {
        self.input_file
            .strip_prefix(self.root_path)
            .unwrap_or_else(|_| {
                panic!(
                    "failed to strip prefix {:?} from {:?}",
                    self.root_path, self.input_file
                )
            })
            .to_str()
            .expect("failed to get relative file name")
    }

    fn range(&self) -> (Option<usize>, Option<usize>) {
        (self.range_start_index, self.range_end_index)
    }
}

pub struct SpecSnapshot<'a, L>
where
    L: TestFormatLanguage,
{
    test_file: SpecTestFile<'a>,
    test_directory: PathBuf,
    language: L,
    format_language: L::FormatLanguage,
}

impl<'a, L> SpecSnapshot<'a, L>
where
    L: TestFormatLanguage,
{
    pub fn new(
        test_file: SpecTestFile<'a>,
        test_directory: &str,
        language: L,
        format_language: L::FormatLanguage,
    ) -> Self {
        let test_directory = PathBuf::from(test_directory);

        SpecSnapshot {
            test_file,
            test_directory,
            language,
            format_language,
        }
    }

    fn formatted(
        &self,
        parsed: &AnyParse,
        format_language: L::FormatLanguage,
    ) -> (String, Printed) {
        let has_errors = parsed.has_errors();
        let syntax = parsed.syntax();

        let range = self.test_file.range();

        let result = match range {
            (Some(start), Some(end)) => self.language.format_range(
                format_language.clone(),
                &syntax,
                TextRange::new(
                    TextSize::try_from(start).unwrap(),
                    TextSize::try_from(end).unwrap(),
                ),
            ),
            _ => self
                .language
                .format_node(format_language.clone(), &syntax)
                .map(|formatted| formatted.print().unwrap()),
        };
        let formatted = result.expect("formatting failed");

        let output_code = match range {
            (Some(_), Some(_)) => {
                let range = formatted
                    .range()
                    .expect("the result of format_range should have a range");

                let mut output_code = self.test_file.input_code.clone();
                output_code.replace_range(Range::<usize>::from(range), formatted.as_code());

                // Check if output code is a valid syntax
                let parsed = self.language.parse(&output_code);

                if parsed.has_errors() {
                    panic!(
                        "{:?} format range produced an invalid syntax tree: {:?}",
                        self.test_file.input_file, output_code
                    )
                }

                output_code
            }
            _ => {
                let output_code = formatted.as_code();

                if !has_errors {
                    let check_reformat = CheckReformat::new(
                        &syntax,
                        output_code,
                        self.test_file.file_name(),
                        &self.language,
                        format_language,
                    );
                    check_reformat.check_reformat();
                }

                output_code.to_string()
            }
        };

        (output_code, formatted)
    }

    pub fn test(self) {
        let input_file = self.test_file().input_file();

        let mut snapshot_builder = SnapshotBuilder::new(input_file)
            .with_input(self.test_file.input_code())
            .with_separator()
            .with_multiple_outputs();

        let parsed = self.language.parse(self.test_file.input_code());

        let (output_code, printed) = self.formatted(&parsed, self.format_language.clone());

        let max_width = self.format_language.options().line_width().value() as usize;

        snapshot_builder = snapshot_builder
            .with_output_and_options(
                SnapshotOutput::new(&output_code).with_index(1),
                self.format_language.options(),
            )
            .with_unimplemented(&printed)
            .with_lines_exceeding_max_width(&output_code, max_width);

        let options_path = self.test_directory.join("options.json");
        if options_path.exists() {
            // TODO:! It would be very cool if we could support this `options.json` file!
            // It's going to require a way to merge deserialized partial options
            // with `RFormatOptions::default()`, and fleshing out `to_format_language()`.
            panic!("`options.json` is not currently supported.");

            //             let mut options_path = BiomePath::new(&options_path);
            //
            //             let mut settings = Settings::default();
            //             // SAFETY: we checked its existence already, we assume we have rights to read it
            //             let (test_options, diagnostics) = deserialize_from_str::<PartialConfiguration>(
            //                 options_path.get_buffer_from_file().as_str(),
            //             )
            //             .consume();
            //             settings
            //                 .merge_with_configuration(test_options.unwrap_or_default(), None, None, &[])
            //                 .unwrap();
            //
            //             if !diagnostics.is_empty() {
            //                 for diagnostic in diagnostics {
            //                     println!("{:?}", print_diagnostic_to_string(&diagnostic));
            //                 }
            //
            //                 panic!("Configuration is invalid");
            //             }
            //
            //             let format_language = self
            //                 .language
            //                 .to_format_language(&settings, &DocumentFileSource::from_path(input_file));
            //
            //             let (mut output_code, printed) = self.formatted(&parsed, format_language.clone());
            //
            //             let max_width = format_language.options().line_width().value() as usize;
            //
            //             // There are some logs that print different line endings, and we can't snapshot those
            //             // otherwise we risk automatically having them replaced with LF by git.
            //             //
            //             // This is a workaround, and it might not work for all cases.
            //             const CRLF_PATTERN: &str = "\r\n";
            //             const CR_PATTERN: &str = "\r";
            //             output_code = output_code
            //                 .replace(CRLF_PATTERN, "<CRLF>\n")
            //                 .replace(CR_PATTERN, "<CR>\n");
            //
            //             snapshot_builder = snapshot_builder
            //                 .with_output_and_options(
            //                     SnapshotOutput::new(&output_code).with_index(1),
            //                     format_language.options(),
            //                 )
            //                 .with_unimplemented(&printed)
            //                 .with_lines_exceeding_max_width(&output_code, max_width);
        }

        snapshot_builder.finish(self.test_file.relative_file_name());
    }

    fn test_file(&self) -> &SpecTestFile {
        &self.test_file
    }
}
