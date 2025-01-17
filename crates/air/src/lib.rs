use crate::args::Args;
use crate::args::Command;
use crate::status::ExitStatus;

pub mod args;
mod commands;
mod logging;
pub mod status;

pub fn run(args: Args) -> anyhow::Result<ExitStatus> {
    if !matches!(args.command, Command::LanguageServer(_)) {
        // The language server sets up its own logging
        logging::init_logging(
            args.global_options.log_level.unwrap_or_default(),
            args.global_options.no_color,
        );
    }

    match args.command {
        Command::Format(command) => commands::format::format(command),
        Command::LanguageServer(command) => commands::language_server::language_server(command),
    }
}
