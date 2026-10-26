use clap::CommandFactory;

use crate::args::Args;
use crate::args::GenerateShellCompletionCommand;
use crate::status::ExitStatus;

pub(crate) fn generate_shell_completion(command: GenerateShellCompletionCommand) -> ExitStatus {
    clap_complete::generate(
        command.shell,
        &mut Args::command(),
        "air",
        &mut std::io::stdout(),
    );
    ExitStatus::Success
}
