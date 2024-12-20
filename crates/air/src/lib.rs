use crate::args::Args;
use crate::args::Command;
use crate::status::ExitStatus;

pub mod args;
mod commands;
pub mod status;

pub fn run(args: Args) -> anyhow::Result<ExitStatus> {
    match args.command {
        Command::Lsp(command) => commands::lsp::lsp(command),
        Command::Format(command) => commands::format::format(command),
    }
}
