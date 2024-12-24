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

pub(crate) struct Cancel;

impl super::NotificationHandler for Cancel {
    type NotificationType = notif::Cancel;
}

impl super::SyncNotificationHandler for Cancel {
    fn run(
        _session: &mut Session,
        _notifier: Notifier,
        _requester: &mut Requester,
        _params: types::CancelParams,
    ) -> Result<()> {
        // TODO(jane): Handle this once we have task cancellation in the scheduler.
        Ok(())
    }
}
