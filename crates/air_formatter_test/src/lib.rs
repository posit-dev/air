use biome_formatter::{CstFormatContext, FormatLanguage, FormatResult, Formatted, Printed};
use biome_parser::AnyParse;
use biome_rowan::Language;
use biome_rowan::{SyntaxNode, TextRange};

pub mod check_reformat;
pub mod snapshot_builder;
pub mod spec;

pub trait TestFormatLanguage {
    type SyntaxLanguage: Language + 'static;
    type FormatOptions: biome_formatter::FormatOptions + std::fmt::Display;
    type Context: CstFormatContext<Options = Self::FormatOptions>;
    type FormatLanguage: FormatLanguage<Context = Self::Context, SyntaxLanguage = Self::SyntaxLanguage>
        + 'static
        + Clone;

    fn parse(&self, text: &str) -> AnyParse;

    fn format_node(
        &self,
        language: Self::FormatLanguage,
        node: &SyntaxNode<Self::SyntaxLanguage>,
    ) -> FormatResult<Formatted<Self::Context>> {
        biome_formatter::format_node(node, language)
    }

    fn format_range(
        &self,
        language: Self::FormatLanguage,
        node: &SyntaxNode<Self::SyntaxLanguage>,
        range: TextRange,
    ) -> FormatResult<Printed> {
        biome_formatter::format_range(node, range, language)
    }

    fn to_format_language(&self) -> Self::FormatLanguage;
}
