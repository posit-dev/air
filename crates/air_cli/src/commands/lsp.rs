use crate::args::LspCommand;
use crate::ExitStatus;

#[tokio::main]
pub(crate) async fn lsp(_command: LspCommand) -> anyhow::Result<ExitStatus> {
    // Returns after shutdown
    lsp::start_lsp(tokio::io::stdin(), tokio::io::stdout()).await;

    Ok(ExitStatus::Success)
}
