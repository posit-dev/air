use std::process::ExitCode;

use air::args::Args;
use air::run;
use clap::Parser;

fn main() -> ExitCode {
    let args = Args::parse();
    run(args).into()
}
