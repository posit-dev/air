use crate::args::LspCommand;
use crate::ExitStatus;

pub(crate) fn lsp(_command: LspCommand) -> anyhow::Result<ExitStatus> {
    // Returns after shutdown
    lsp::start_lsp();

    Ok(ExitStatus::Success)
}
