use air_r_formatter::context::RFormatOptions;
use air_r_formatter::format_node;
use air_r_parser::RParserOptions;
use air_r_parser::parse;

#[test]
fn test_skipped_file_with_crlf_line_endings_matches_user_line_endings() {
    let text = "# fmt: skip file\r\n1+1\r\nx<-c(1,    2)\r\n";

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
    assert_eq!(result, "# fmt: skip file\n1+1\nx<-c(1,    2)\n");
}
