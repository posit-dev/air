use std::path::PathBuf;

use air_r_formatter::context::RFormatOptions;
use air_r_parser::RParserOptions;

use crate::args::FormatCommand;
use crate::ExitStatus;

pub(crate) fn format(command: FormatCommand) -> anyhow::Result<ExitStatus> {
    format_file(&command.file)
}

// TODO: Should you exit after the first failure? Probably not, probably power through
// and elegantly report failures at the end? Since you've formatted stuff up to
// that point already anyways.
// TODO: Hook this up to a command
// TODO: Ignore anything but R files, of course
fn _format_dir(path: &PathBuf) -> anyhow::Result<ExitStatus> {
    let iter = std::fs::read_dir(path)?;

    for file in iter {
        let Ok(file) = file else {
            continue;
        };
        format_file(&file.path())?;
    }

    Ok(ExitStatus::Success)
}

fn format_file(path: &PathBuf) -> anyhow::Result<ExitStatus> {
    let text = std::fs::read_to_string(path)?;

    let parser_options = RParserOptions::default();
    let parsed = air_r_parser::parse(text.as_str(), parser_options);

    if parsed.has_errors() {
        return Ok(ExitStatus::Error);
    }

    let formatter_options = RFormatOptions::default();
    let formatted = air_r_formatter::format_node(formatter_options, &parsed.syntax())?;
    let result = formatted.print()?;
    let code = result.as_code();

    std::fs::write(path, code)?;

    Ok(ExitStatus::Success)
}
