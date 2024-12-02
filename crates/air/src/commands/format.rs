use std::path::PathBuf;

use air_r_formatter::context::RFormatOptions;
use air_r_parser::RParserOptions;
use line_ending::LineEnding;

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
    let contents = std::fs::read_to_string(path)?;

    let line_ending = line_ending::infer(&contents);

    // Normalize to Unix line endings
    let contents = match line_ending {
        LineEnding::Lf => contents,
        LineEnding::Crlf => line_ending::normalize(contents),
    };

    let parser_options = RParserOptions::default();
    let parsed = air_r_parser::parse(contents.as_str(), parser_options);

    if parsed.has_errors() {
        return Ok(ExitStatus::Error);
    }

    // TODO: Respect user specified `LineEnding` option too, not just inferred line endings
    let line_ending = match line_ending {
        LineEnding::Lf => biome_formatter::LineEnding::Lf,
        LineEnding::Crlf => biome_formatter::LineEnding::Crlf,
    };

    let formatter_options = RFormatOptions::default().with_line_ending(line_ending);
    let formatted = air_r_formatter::format_node(formatter_options, &parsed.syntax())?;
    let result = formatted.print()?;
    let code = result.as_code();

    std::fs::write(path, code)?;

    Ok(ExitStatus::Success)
}
