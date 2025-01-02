//! Scheduling, I/O, and API endpoints.

use lsp_server as lsp;
use lsp_types as types;
use lsp_types::InitializeParams;
use std::num::NonZeroUsize;
// The new PanicInfoHook name requires MSRV >= 1.82
#[allow(deprecated)]
use std::panic::PanicInfo;
use types::DidChangeWatchedFilesRegistrationOptions;
use types::FileSystemWatcher;
use types::OneOf;
use types::TextDocumentSyncCapability;
use types::TextDocumentSyncKind;
use types::TextDocumentSyncOptions;
use types::WorkspaceFoldersServerCapabilities;

use self::connection::Connection;
use self::schedule::event_loop_thread;
use self::schedule::Scheduler;
use self::schedule::Task;
use crate::document::PositionEncoding;
use crate::message::try_show_message;
use crate::server::connection::ConnectionInitializer;
use crate::session::ResolvedClientCapabilities;
use crate::session::Session;

mod api;
mod client;
mod connection;
mod schedule;

pub(crate) use connection::ClientSender;

pub(crate) type Result<T> = std::result::Result<T, api::Error>;

pub struct Server {
    connection: Connection,
    client_capabilities: ResolvedClientCapabilities,
    worker_threads: NonZeroUsize,
    session: Session,
}

impl Server {
    pub fn new(
        worker_threads: NonZeroUsize,
        connection: lsp::Connection,
        connection_threads: Option<lsp::IoThreads>,
    ) -> anyhow::Result<Self> {
        let initializer = ConnectionInitializer::new(connection, connection_threads);

        let (id, initialize_params) = initializer.initialize_start()?;

        let client_capabilities = initialize_params.capabilities;
        let client_capabilities = ResolvedClientCapabilities::new(client_capabilities);
        let position_encoding = Self::find_best_position_encoding(&client_capabilities);
        let server_capabilities = Self::server_capabilities(position_encoding);

        let connection = initializer.initialize_finish(
            id,
            &server_capabilities,
            crate::SERVER_NAME,
            crate::SERVER_VERSION,
        )?;

        let InitializeParams {
            workspace_folders,
            client_info,
            ..
        } = initialize_params;

        let workspace_folders = workspace_folders.unwrap_or_default();

        // TODO: Get user specified options from `initialization_options`
        let log_level = None;
        let dependency_log_levels = None;

        crate::logging::init_logging(
            connection.make_sender(),
            log_level,
            dependency_log_levels,
            client_info,
        );

        crate::message::init_messenger(connection.make_sender());

        Ok(Self {
            connection,
            worker_threads,
            session: Session::new(
                client_capabilities.clone(),
                position_encoding,
                workspace_folders,
            )?,
            client_capabilities,
        })
    }

    pub fn run(self) -> anyhow::Result<()> {
        // The new PanicInfoHook name requires MSRV >= 1.82
        #[allow(deprecated)]
        type PanicHook = Box<dyn Fn(&PanicInfo<'_>) + 'static + Sync + Send>;
        struct RestorePanicHook {
            hook: Option<PanicHook>,
        }

        impl Drop for RestorePanicHook {
            fn drop(&mut self) {
                if let Some(hook) = self.hook.take() {
                    std::panic::set_hook(hook);
                }
            }
        }

        // unregister any previously registered panic hook
        // The hook will be restored when this function exits.
        let _ = RestorePanicHook {
            hook: Some(std::panic::take_hook()),
        };

        // When we panic, try to notify the client.
        std::panic::set_hook(Box::new(move |panic_info| {
            use std::io::Write;

            let backtrace = std::backtrace::Backtrace::force_capture();
            tracing::error!("{panic_info}\n{backtrace}");

            // we also need to print to stderr directly for when using `window/logMessage` because
            // the message won't be sent to the client.
            // But don't use `eprintln` because `eprintln` itself may panic if the pipe is broken.
            let mut stderr = std::io::stderr().lock();
            writeln!(stderr, "{panic_info}\n{backtrace}").ok();

            try_show_message(
                "The Air language server exited with a panic. See the logs for more details."
                    .to_string(),
                lsp_types::MessageType::ERROR,
            )
            .ok();
        }));

        event_loop_thread(move || {
            Self::event_loop(
                &self.connection,
                &self.client_capabilities,
                self.session,
                self.worker_threads,
            )?;
            self.connection.close()?;
            Ok(())
        })?
        .join()
    }

    #[allow(clippy::needless_pass_by_value)] // this is because we aren't using `next_request_id` yet.
    fn event_loop(
        connection: &Connection,
        resolved_client_capabilities: &ResolvedClientCapabilities,
        mut session: Session,
        worker_threads: NonZeroUsize,
    ) -> anyhow::Result<()> {
        let mut scheduler =
            schedule::Scheduler::new(&mut session, worker_threads, connection.make_sender());

        Self::try_register_capabilities(resolved_client_capabilities, &mut scheduler);
        for msg in connection.incoming() {
            if connection.handle_shutdown(&msg)? {
                break;
            }
            let task = match msg {
                lsp::Message::Request(req) => api::request(req),
                lsp::Message::Notification(notification) => api::notification(notification),
                lsp::Message::Response(response) => scheduler.response(response),
            };
            scheduler.dispatch(task);
        }

        Ok(())
    }

    fn try_register_capabilities(
        resolved_client_capabilities: &ResolvedClientCapabilities,
        scheduler: &mut Scheduler,
    ) {
        let _span = tracing::info_span!("try_register_capabilities").entered();

        // Register capabilities to the client
        let mut registrations: Vec<lsp_types::Registration> = vec![];

        if resolved_client_capabilities.dynamic_registration_for_did_change_watched_files {
            // Watch for changes in `air.toml` files so we can react dynamically
            let watch_air_toml_registration = lsp_types::Registration {
                id: String::from("air-toml-watcher"),
                method: "workspace/didChangeWatchedFiles".into(),
                register_options: Some(
                    serde_json::to_value(DidChangeWatchedFilesRegistrationOptions {
                        watchers: vec![FileSystemWatcher {
                            glob_pattern: lsp_types::GlobPattern::String("**/air.toml".into()),
                            kind: None,
                        }],
                    })
                    .unwrap(),
                ),
            };

            registrations.push(watch_air_toml_registration);
        } else {
            tracing::warn!("LSP client does not support watched files dynamic capability - automatic configuration reloading will not be available.");
        }

        if !registrations.is_empty() {
            let params = lsp_types::RegistrationParams { registrations };

            let response_handler = |()| {
                tracing::info!("Dynamic configuration successfully registered");
                Task::nothing()
            };

            if let Err(error) = scheduler
                .request::<lsp_types::request::RegisterCapability>(params, response_handler)
            {
                tracing::error!(
                    "An error occurred when trying to dynamically register capabilities: {error}"
                );
            }
        }
    }

    fn find_best_position_encoding(
        client_capabilities: &ResolvedClientCapabilities,
    ) -> PositionEncoding {
        // If the client supports UTF-8 we use that, even if it's not its
        // preferred encoding (at position 0). Otherwise we use the mandatory
        // UTF-16 encoding that all clients and servers must support, even if
        // the client would have preferred UTF-32. Note that VSCode and Positron
        // only support UTF-16.
        if client_capabilities
            .position_encodings
            .contains(&lsp_types::PositionEncodingKind::UTF8)
        {
            PositionEncoding::UTF8
        } else {
            PositionEncoding::UTF16
        }
    }

    fn server_capabilities(position_encoding: PositionEncoding) -> types::ServerCapabilities {
        types::ServerCapabilities {
            position_encoding: Some(position_encoding.into()),
            text_document_sync: Some(TextDocumentSyncCapability::Options(
                TextDocumentSyncOptions {
                    open_close: Some(true),
                    change: Some(TextDocumentSyncKind::INCREMENTAL),
                    will_save: Some(false),
                    will_save_wait_until: Some(false),
                    ..Default::default()
                },
            )),
            workspace: Some(types::WorkspaceServerCapabilities {
                workspace_folders: Some(WorkspaceFoldersServerCapabilities {
                    supported: Some(true),
                    change_notifications: Some(OneOf::Left(true)),
                }),
                file_operations: None,
            }),
            document_formatting_provider: Some(OneOf::Left(true)),
            document_range_formatting_provider: Some(OneOf::Left(true)),
            ..Default::default()
        }
    }
}
