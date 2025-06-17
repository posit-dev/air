use std::io::Write;

use crate::TestFormatLanguage;
use biome_rowan::SyntaxNode;

/// Perform a second pass of formatting on a file, printing a diff if the
/// output doesn't match the input
///
pub struct CheckReformat<'a, L>
where
    L: TestFormatLanguage,
{
    root: &'a SyntaxNode<L::SyntaxLanguage>,
    text: &'a str,

    language: &'a L,
    format_language: L::FormatLanguage,
}

impl<'a, L> CheckReformat<'a, L>
where
    L: TestFormatLanguage,
{
    pub fn new(
        root: &'a SyntaxNode<L::SyntaxLanguage>,
        text: &'a str,
        language: &'a L,
        format_language: L::FormatLanguage,
    ) -> Self {
        CheckReformat {
            root,
            text,
            language,
            format_language,
        }
    }

    pub fn check_reformat(&self) {
        let re_parse = self.language.parse(self.text);

        // Panic if the result from the formatter has syntax errors
        if re_parse.has_errors() {
            let mut buffer = Vec::new();

            for diagnostic in re_parse.diagnostics() {
                writeln!(&mut buffer, "{message}", message = diagnostic.message).unwrap();
            }

            panic!(
                "formatter output had syntax errors where input had none:\n{}",
                std::str::from_utf8(buffer.as_slice()).expect("non utf8 in error buffer")
            )
        }

        let formatted = match self
            .language
            .format_node(self.format_language.clone(), &re_parse.syntax())
        {
            Ok(formatted) => formatted,
            Err(err) => {
                panic!("failed to format: {}", err);
            }
        };

        let printed = formatted.print().unwrap();

        if self.text != printed.as_code() {
            let input_format_element = self
                .language
                .format_node(self.format_language.clone(), self.root)
                .unwrap();
            let pretty_input_ir = format!("{}", formatted.into_document());
            let pretty_reformat_ir = format!("{}", input_format_element.into_document());

            // Print a diff of the Formatter IR emitted for the input and the output
            let diff = similar_asserts::SimpleDiff::from_str(
                &pretty_input_ir,
                &pretty_reformat_ir,
                "input",
                "output",
            );

            println!("{diff}");

            similar_asserts::assert_eq!(self.text, printed.as_code());
        }
    }
}
