use crate::args::LanguageServerCommand;
use crate::ExitStatus;

#[tokio::main]
pub(crate) async fn language_server(_command: LanguageServerCommand) -> anyhow::Result<ExitStatus> {
    // Returns after shutdown
    lsp::start_lsp(tokio::io::stdin(), tokio::io::stdout()).await;

    Ok(ExitStatus::Success)
}
