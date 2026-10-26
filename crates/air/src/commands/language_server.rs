use crate::ExitStatus;
use crate::args::LanguageServerCommand;

#[tokio::main]
pub(crate) async fn language_server(_command: LanguageServerCommand) -> ExitStatus {
    // Returns after shutdown
    lsp::start_lsp(tokio::io::stdin(), tokio::io::stdout()).await;

    ExitStatus::Success
}
