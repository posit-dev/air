use air_r_formatter::context::RFormatOptions;
use air_r_formatter::format_node;
use air_r_parser::RParserOptions;
use air_r_parser::parse;

#[test]
fn test_multiline_string_with_crlf_line_endings_matches_user_line_endings() {
    // Our string normalizer manually converts the inner `\r\n` captured by the string
    // content node to `\n`, which the printer then rewrites as the user's chosen
    // `LineEnding`
    let text = "\"multiline\r\nstring\"\r\n";

    let parse = parse(text, RParserOptions::default());

    let options = RFormatOptions::default().with_line_ending(settings::LineEnding::Crlf);
    let formatted = format_node(options, &parse.syntax()).unwrap();
    let result = formatted.print().unwrap();
    let result = result.as_code();
    assert_eq!(result, text);

    let options = RFormatOptions::default().with_line_ending(settings::LineEnding::Lf);
    let formatted = format_node(options, &parse.syntax()).unwrap();
    let result = formatted.print().unwrap();
    let result = result.as_code();
    assert_eq!(result, "\"multiline\nstring\"\n");
}
