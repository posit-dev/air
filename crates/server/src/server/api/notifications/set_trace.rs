use crate::server::client::{Notifier, Requester};
use crate::server::Result;
use crate::session::Session;
use lsp_types as types;
use lsp_types::notification as notif;

pub(crate) struct SetTrace;

impl super::NotificationHandler for SetTrace {
    type NotificationType = notif::SetTrace;
}

impl super::SyncNotificationHandler for SetTrace {
    fn run(
        _session: &mut Session,
        _notifier: Notifier,
        _requester: &mut Requester,
        params: types::SetTraceParams,
    ) -> Result<()> {
        // Clients always send this request on initialization, but we don't use
        // log information from here.
        let value = params.value;
        tracing::trace!("Ignoring `$/setTrace` notification with value {value:?}");
        Ok(())
    }
}
