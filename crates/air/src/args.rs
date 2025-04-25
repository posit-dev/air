use clap::Parser;
use clap::Subcommand;
use std::path::PathBuf;

use crate::commands::format;
use crate::logging;

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

    #[clap(flatten)]
    pub(crate) global_options: GlobalOptions,
}

#[derive(Subcommand)]
pub(crate) enum Command {
    /// Format a set of files or directories
    Format(FormatCommand),

    /// Start a language server
    LanguageServer(LanguageServerCommand),
}

#[derive(Clone, Debug, Parser)]
#[command(arg_required_else_help(true))]
pub(crate) struct FormatCommand {
    /// The files or directories to format
    pub paths: Vec<PathBuf>,

    /// If enabled, format results are not written back to the file. Instead, exit with a
    /// non-zero status code if any files would have been modified, and zero otherwise. An
    /// optional `OUTPUT_FORMAT` type can be provided, one of: `interactive` or `github`.
    /// Defaults to `interactive`.
    #[arg(
        long,
        num_args = 0..=1,
        require_equals = true,
        default_missing_value = "interactive",
        value_name = "OUTPUT_FORMAT"
    )]
    pub check: Option<format::OutputFormat>,
}

#[derive(Clone, Debug, Parser)]
pub(crate) struct LanguageServerCommand {}

/// All configuration options that can be passed "globally"
#[derive(Debug, Default, clap::Args)]
#[command(next_help_heading = "Global options")]
pub(crate) struct GlobalOptions {
    /// The log level. One of: `error`, `warn`, `info`, `debug`, or `trace`. Defaults
    /// to `warn`.
    #[arg(long, global = true, require_equals = true, value_name = "LEVEL")]
    pub(crate) log_level: Option<logging::LogLevel>,

    /// Disable colored output. To turn colored output off, either set this option or set
    /// the environment variable `NO_COLOR` to any non-zero value.
    #[arg(long, global = true)]
    pub(crate) no_color: bool,
}
