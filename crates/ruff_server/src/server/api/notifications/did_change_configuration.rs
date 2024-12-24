// +------------------------------------------------------------+
// | Code adopted from:                                         |
// | Repository: https://github.com/astral-sh/ruff.git          |
// | Commit: 5bc9d6d3aa694ab13f38dd5cf91b713fd3844380           |
// +------------------------------------------------------------+

use crate::server::client::{Notifier, Requester};
use crate::server::Result;
use crate::session::Session;
use lsp_types as types;
use lsp_types::notification as notif;

pub(crate) struct DidChangeConfiguration;

impl super::NotificationHandler for DidChangeConfiguration {
    type NotificationType = notif::DidChangeConfiguration;
}

impl super::SyncNotificationHandler for DidChangeConfiguration {
    fn run(
        _session: &mut Session,
        _notifier: Notifier,
        _requester: &mut Requester,
        _params: types::DidChangeConfigurationParams,
    ) -> Result<()> {
        // TODO(jane): get this wired up after the pre-release
        Ok(())
    }
}
