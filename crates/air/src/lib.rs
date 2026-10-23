use biome_console::ColorMode;
use biome_console::EnvConsole;

use crate::args::Args;
use crate::args::Command;
use crate::status::ExitStatus;

pub mod args;
mod color;
mod commands;
mod diagnostics;
mod logging;
pub mod status;

pub fn run(args: Args) -> anyhow::Result<ExitStatus> {
    // The language server sets up its own logging
    if let Command::LanguageServer(command) = args.command {
        return commands::language_server::language_server(command);
    }

    let color = color::init_color(args.global_options.no_color);

    logging::init_logging(args.global_options.log_level.unwrap_or_default(), color);

    let color_mode = if color {
        ColorMode::Enabled
    } else {
        ColorMode::Disabled
    };

    let mut console = EnvConsole::new(color_mode);

    match args.command {
        Command::GenerateShellCompletion(command) => {
            commands::generate_shell_completion::generate_shell_completion(command)
        }
        Command::Format(command) => commands::format::format(command, &mut console),
        Command::LanguageServer(_) => unreachable!("Handled above"),
    }
}
