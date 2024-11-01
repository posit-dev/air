use air_r_formatter::context::RFormatOptions;
use air_r_parser::RParserOptions;

use crate::args::FormatCommand;
use crate::ExitStatus;

pub(crate) fn format(command: FormatCommand) -> anyhow::Result<ExitStatus> {
    let text = std::fs::read_to_string(&command.file)?;

    let parser_options = RParserOptions::default();
    let parsed = air_r_parser::parse(text.as_str(), parser_options);

    if parsed.has_errors() {
        return Ok(ExitStatus::Error);
    }

    let formatter_options = RFormatOptions::default();
    let formatted = air_r_formatter::format_node(formatter_options, &parsed.syntax())?;
    let result = formatted.print()?;
    let code = result.as_code();

    std::fs::write(command.file, code)?;

    Ok(ExitStatus::Success)
}
