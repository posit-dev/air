use crate::document::TextDocument;
use crate::server::client::{Notifier, Requester};
use crate::server::Result;
use crate::session::Session;
use lsp_types as types;
use lsp_types::notification as notif;

pub(crate) struct DidOpen;

impl super::NotificationHandler for DidOpen {
    type NotificationType = notif::DidOpenTextDocument;
}

impl super::SyncNotificationHandler for DidOpen {
    fn run(
        session: &mut Session,
        _notifier: Notifier,
        _requester: &mut Requester,
        types::DidOpenTextDocumentParams {
            text_document:
                types::TextDocumentItem {
                    uri, text, version, ..
                },
        }: types::DidOpenTextDocumentParams,
    ) -> Result<()> {
        let document = TextDocument::new(text, version);

        session.open_text_document(uri.clone(), document);

        Ok(())
    }
}
