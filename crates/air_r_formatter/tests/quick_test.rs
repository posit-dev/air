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
    // let src = r#"
    //     lapply(
    //       my_long_list_my_long_list_my_long_list_my_long_list,
    //       function(my_long_argument) {
    //         my_long_body_my_long_body_my_long_body_my_long_body_my_long_body
    //       }
    //     )
    // "#;

    // let src = r#"
    //     lapply(my_long_list_my_long_list_my_long_list_my_long_list, {
    //         # foo
    //         my_long_body_my_long_body_my_long_body_my_long_body_my_long_body()
    //       }
    //     )
    // "#;

    let src = r#"
        map(x, {
            foo(a, b)
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
}
