use crate::args::{Args, CompletionCommand};
use crate::status::ExitStatus;
use clap::CommandFactory;

pub(crate) fn generate_completions(command: CompletionCommand) -> anyhow::Result<ExitStatus> {
    let mut app = Args::command();
    let name = app.get_name().to_string();
    clap_complete::generate(
        command.shell,
        &mut app,
        name,
        &mut std::io::stdout(),
    );

    Ok(ExitStatus::Success)
}
