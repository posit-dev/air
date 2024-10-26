// use biome_formatter_test::TestFormatLanguage;
// use biome_fs::BiomePath;
// use biome_parser::AnyParse;
// use biome_r_formatter::context::RFormatContext;
// use biome_r_formatter::RFormatLanguage;
// use biome_r_parser::RParserOptions;
// use biome_r_syntax::RLanguage;
// use biome_service::settings::ServiceLanguage;
// use biome_service::settings::Settings;
// use biome_service::workspace::DocumentFileSource;
//
// #[derive(Default)]
// pub struct RTestFormatLanguage {}
//
// impl TestFormatLanguage for RTestFormatLanguage {
//     type ServiceLanguage = RLanguage;
//     type Context = RFormatContext;
//     type FormatLanguage = RFormatLanguage;
//
//     fn parse(&self, text: &str) -> AnyParse {
//         biome_r_parser::parse(text, RParserOptions::default())
//     }
//
//     fn to_format_language(
//         &self,
//         settings: &Settings,
//         file_source: &DocumentFileSource,
//     ) -> Self::FormatLanguage {
//         let language_settings = &settings.languages.r.formatter;
//         let options = Self::ServiceLanguage::resolve_format_options(
//             Some(&settings.formatter),
//             Some(&settings.override_settings),
//             Some(language_settings),
//             &BiomePath::new(""),
//             file_source,
//         );
//         RFormatLanguage::new(options)
//     }
// }
