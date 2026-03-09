use workspace::discovery;

use crate::ExitStatus;
use crate::args::FormatCommand;

mod paths;
mod stdin;

#[derive(Copy, Clone, Debug)]
enum FormatMode {
    Write,
    Check,
}

pub(crate) fn format(command: FormatCommand) -> anyhow::Result<ExitStatus> {
    if let Some(status) = check_argument_consistency(&command) {
        return Ok(status);
    }

    let mode = FormatMode::from_command(&command);

    let (exclude, include) = if command.force {
        (discovery::Exclude::Nothing, discovery::Include::Everything)
    } else {
        (discovery::Exclude::Matched, discovery::Include::Matched)
    };

    match command.stdin_file_path {
        Some(path) => stdin::format(path, mode, exclude, include),
        None => paths::format(command.paths, mode, exclude, include),
    }
}

fn check_argument_consistency(command: &FormatCommand) -> Option<ExitStatus> {
    if command.stdin_file_path.is_some() && !command.paths.is_empty() {
        tracing::error!(
            "Can't supply paths when reading from stdin: {paths}",
            paths = command
                .paths
                .iter()
                .map(|path| format!("'{path}'", path = path.display()))
                .collect::<Vec<String>>()
                .join(",")
        );
        return Some(ExitStatus::Error);
    }

    None
}

impl FormatMode {
    fn from_command(command: &FormatCommand) -> Self {
        if command.check {
            FormatMode::Check
        } else {
            FormatMode::Write
        }
    }
}
