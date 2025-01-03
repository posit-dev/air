use crate::server::client::{Notifier, Requester};
use crate::server::Result;
use crate::session::Session;
use lsp_types as types;
use lsp_types::notification as notif;

pub(crate) struct DidChangeWatchedFiles;

impl super::NotificationHandler for DidChangeWatchedFiles {
    type NotificationType = notif::DidChangeWatchedFiles;
}

impl super::SyncNotificationHandler for DidChangeWatchedFiles {
    fn run(
        session: &mut Session,
        _notifier: Notifier,
        _requester: &mut Requester,
        params: types::DidChangeWatchedFilesParams,
    ) -> Result<()> {
        for change in &params.changes {
            session.reload_settings(&change.uri);
        }

        Ok(())
    }
}
