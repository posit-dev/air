use air_formatter_test::TestFormatLanguage;
use air_r_formatter::RFormatLanguage;
use air_r_formatter::context::RFormatContext;
use air_r_formatter::context::RFormatOptions;
use air_r_parser::RParserOptions;
use air_r_syntax::RLanguage;
use biome_parser::AnyParse;

#[derive(Default)]
pub struct RTestFormatLanguage {}

impl TestFormatLanguage for RTestFormatLanguage {
    type SyntaxLanguage = RLanguage;
    type FormatOptions = RFormatOptions;
    type Context = RFormatContext;
    type FormatLanguage = RFormatLanguage;

    fn parse(&self, text: &str) -> AnyParse {
        air_r_parser::parse(text, RParserOptions::default()).into()
    }

    fn to_format_language(&self) -> Self::FormatLanguage {
        // TODO: Allow for configurable options through an `options.json` file
        RFormatLanguage::new(RFormatOptions::default())
    }
}
