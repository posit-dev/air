use air_cli::args::Args;
use air_cli::run;
use air_cli::status::ExitStatus;
use clap::Parser;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args = Args::parse();

    match run(args) {
        Ok(status) => status.into(),
        Err(err) => {
            use std::io::Write;

            // Use `writeln` instead of `eprintln` to avoid panicking when the stderr pipe is broken.
            let mut stderr = std::io::stderr().lock();

            // This communicates that this isn't a typical error but air itself hard-errored for
            // some reason (e.g. failed to resolve the configuration)
            writeln!(stderr, "air failed").ok();

            for cause in err.chain() {
                writeln!(stderr, "  Cause: {cause}").ok();
            }

            ExitStatus::Error.into()
        }
    }
}
