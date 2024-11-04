use air_formatter_test::check_reformat::CheckReformat;
use air_r_formatter::context::RFormatOptions;
use air_r_formatter::format_node;
use air_r_formatter::RFormatLanguage;
use air_r_parser::parse;
use air_r_parser::RParserOptions;
use air_r_syntax::RRoot;
use biome_formatter::IndentStyle;
use biome_formatter::LineWidth;
use biome_rowan::AstNode;

mod language {
    include!("language.rs");
}

// Use this test check if your snippet prints as you wish, without using a snapshot
#[ignore]
#[test]
fn quick_test() {
    let src = r#"
test_that("description", {
  1 + 1
})
    "#;

    let parse = parse(src, RParserOptions::default());

    let options = RFormatOptions::default()
        .with_indent_style(IndentStyle::Space)
        .with_line_width(LineWidth::try_from(80).unwrap());

    let formatted = format_node(options.clone(), &parse.syntax()).unwrap();
    let result = formatted.print().unwrap();

    println!("---- Parser Representation ----");
    println!("{:#?}", RRoot::unwrap_cast(parse.syntax()));
    println!("---- IR Representation ----");
    println!("{}", formatted.into_document());
    println!();
    println!("---- Formatted Code ----");
    println!("start\n{}\nend", result.as_code());

    let root = &parse.syntax();
    let language = language::RTestFormatLanguage::default();

    // Does a second pass of formatting to ensure nothing changes (i.e. stable)
    let check_reformat = CheckReformat::new(
        root,
        result.as_code(),
        "quick_test",
        &language,
        RFormatLanguage::new(options),
    );
    check_reformat.check_reformat();
}
