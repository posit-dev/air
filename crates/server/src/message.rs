use anyhow::Context;
use lsp_types::notification::Notification;
use std::sync::OnceLock;

use crate::server::ClientSender;

static MESSENGER: OnceLock<ClientSender> = OnceLock::new();

pub(crate) fn init_messenger(client_sender: ClientSender, is_test_client: bool) {
    let result = MESSENGER.set(client_sender);

    // During testing, `init_messenger()` will be called multiple times
    // within the same process, potentially at the same time across threads.
    // This probably isn't great, because if we call `show_err_msg!()` from a
    // test thread where the `ClientSender` has been shutdown, then we will panic.
    if !is_test_client {
        result.expect("Messenger should only be initialized once");
    }
}

pub(crate) fn show_message(message: String, message_type: lsp_types::MessageType) {
    try_show_message(message, message_type).unwrap();
}

pub(super) fn try_show_message(
    message: String,
    message_type: lsp_types::MessageType,
) -> anyhow::Result<()> {
    MESSENGER
        .get()
        .ok_or_else(|| anyhow::anyhow!("Messenger not initialized"))?
        .send(lsp_server::Message::Notification(
            lsp_server::Notification {
                method: lsp_types::notification::ShowMessage::METHOD.into(),
                params: serde_json::to_value(lsp_types::ShowMessageParams {
                    typ: message_type,
                    message,
                })?,
            },
        ))
        .context("Failed to send message")?;

    Ok(())
}

/// Sends a request to display an error to the client with a formatted message. The error is sent
/// in a `window/showMessage` notification.
macro_rules! show_err_msg {
    ($msg:expr$(, $($arg:tt),*)?) => {
        crate::message::show_message(::core::format_args!($msg, $($($arg),*)?).to_string(), lsp_types::MessageType::ERROR)
    };
}

/// Sends a request to display a warning to the client with a formatted message. The warning is
/// sent in a `window/showMessage` notification.
#[allow(unused_macros)]
macro_rules! show_warn_msg {
    ($msg:expr$(, $($arg:tt),*)?) => {
        crate::message::show_message(::core::format_args!($msg, $($($arg),*)?).to_string(), lsp_types::MessageType::WARNING)
    };
}
