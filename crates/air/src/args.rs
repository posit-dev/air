use clap::Parser;
use clap::Subcommand;
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    author,
    name = "air",
    about = "Air: An R language server and formatter",
    after_help = "For help with a specific command, see: `air help <command>`."
)]
#[command(version)]
pub struct Args {
    #[command(subcommand)]
    pub(crate) command: Command,
}

#[derive(Subcommand)]
pub(crate) enum Command {
    /// Start a language server
    Lsp(LspCommand),

    /// Format a set of files or directories
    Format(FormatCommand),
}

#[derive(Clone, Debug, Parser)]
pub(crate) struct LspCommand {}

#[derive(Clone, Debug, Parser)]
#[command(arg_required_else_help(true))]
pub(crate) struct FormatCommand {
    /// The files or directories to format
    pub paths: Vec<PathBuf>,

    /// If enabled, format results are not written back to the file. Instead,
    /// exit with a non-zero status code if any files would have been modified,
    /// and zero otherwise.
    #[arg(long)]
    pub check: bool,
}
