use crate::args::{Args, GenerateShellCompletionCommand};
use crate::status::ExitStatus;
use clap::CommandFactory;

pub(crate) fn generate_shell_completion(
    command: GenerateShellCompletionCommand,
) -> anyhow::Result<ExitStatus> {
    clap_complete::generate(
        command.shell,
        &mut Args::command(),
        "air",
        &mut std::io::stdout(),
    );
    Ok(ExitStatus::Success)
}
