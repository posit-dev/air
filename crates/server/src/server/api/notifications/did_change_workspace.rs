use crate::error::ErrorVec;
use crate::server::api::LSPResult;
use crate::server::client::{Notifier, Requester};
use crate::server::Result;
use crate::session::Session;
use lsp_types as types;
use lsp_types::notification as notif;

pub(crate) struct DidChangeWorkspace;

impl super::NotificationHandler for DidChangeWorkspace {
    type NotificationType = notif::DidChangeWorkspaceFolders;
}

impl super::SyncNotificationHandler for DidChangeWorkspace {
    fn run(
        session: &mut Session,
        _notifier: Notifier,
        _requester: &mut Requester,
        params: types::DidChangeWorkspaceFoldersParams,
    ) -> Result<()> {
        // Collect all `errors` to ensure we don't drop any events if we encounter an error
        let mut errors = ErrorVec::new();

        for types::WorkspaceFolder { uri, .. } in params.event.added {
            errors.push_err(session.open_workspace_folder(&uri));
        }
        for types::WorkspaceFolder { uri, .. } in params.event.removed {
            errors.push_err(session.close_workspace_folder(&uri));
        }

        errors
            .into_result()
            .with_failure_code(lsp_server::ErrorCode::InvalidParams)
    }
}
