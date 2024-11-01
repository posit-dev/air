use clap::Parser;
use clap::Subcommand;
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    author,
    name = "air",
    about = "Air: An R formatter",
    after_help = "For help with a specific command, see: `air help <command>`."
)]
#[command(version)]
pub struct Args {
    #[command(subcommand)]
    pub(crate) command: Command,
}

#[derive(Subcommand)]
pub(crate) enum Command {
    /// Format a file
    Format(FormatCommand),
}

#[derive(Clone, Debug, Parser)]
pub(crate) struct FormatCommand {
    /// The file to format
    pub file: PathBuf,
}
