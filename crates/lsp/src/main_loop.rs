//
// main_loop.rs
//
// Copyright (C) 2024 Posit Software, PBC. All rights reserved.
//
//

use std::future;
use std::pin::Pin;

use anyhow::anyhow;
use biome_line_index::WideEncoding;
use futures::StreamExt;
use tokio::sync::mpsc::unbounded_channel as tokio_unbounded_channel;
use tokio::task::JoinHandle;
use tower_lsp::Client;
use tower_lsp::lsp_types::Diagnostic;
use url::Url;

use crate::capabilities::AirClientCapabilities;
use crate::handlers;
use crate::handlers_ext;
use crate::handlers_format;
use crate::handlers_state;
use crate::handlers_state::ConsoleInputs;
use crate::logging;
use crate::logging::LogMessageSender;
use crate::logging::LogThreadState;
use crate::proto::PositionEncoding;
use crate::settings::GlobalSettings;
use crate::state::WorldState;
use crate::tower_lsp::LspMessage;
use crate::tower_lsp::LspNotification;
use crate::tower_lsp::LspRequest;
use crate::tower_lsp::LspResponse;
use crate::workspaces::WorkspaceSettings;
use crate::workspaces::WorkspaceSettingsResolver;

pub(crate) type TokioUnboundedSender<T> = tokio::sync::mpsc::UnboundedSender<T>;
pub(crate) type TokioUnboundedReceiver<T> = tokio::sync::mpsc::UnboundedReceiver<T>;

// This is the syntax for trait aliases until an official one is stabilised.
// This alias is for the future of a `JoinHandle<anyhow::Result<T>>`
trait AnyhowJoinHandleFut<T>:
    future::Future<Output = std::result::Result<anyhow::Result<T>, tokio::task::JoinError>>
{
}
impl<T, F> AnyhowJoinHandleFut<T> for F where
    F: future::Future<Output = std::result::Result<anyhow::Result<T>, tokio::task::JoinError>>
{
}

// Alias for a list of join handle futures
type TaskList<T> = futures::stream::FuturesUnordered<Pin<Box<dyn AnyhowJoinHandleFut<T> + Send>>>;

#[allow(clippy::large_enum_variant)]
#[derive(Debug)]
pub(crate) enum Event {
    Lsp(LspMessage),
    #[allow(dead_code)]
    Kernel(KernelNotification),
}

#[derive(Debug)]
pub(crate) enum KernelNotification {
    #[allow(dead_code)]
    DidChangeConsoleInputs(ConsoleInputs),
}

#[derive(Debug)]
pub(crate) enum AuxiliaryEvent {
    #[allow(dead_code)]
    PublishDiagnostics(Url, Vec<Diagnostic>, Option<i32>),
    SpawnedTask(JoinHandle<anyhow::Result<Option<AuxiliaryEvent>>>),
}

#[derive(Debug, Clone)]
pub(crate) struct AuxiliaryEventSender {
    inner: TokioUnboundedSender<AuxiliaryEvent>,
}

impl AuxiliaryEventSender {
    pub(crate) fn new(tx: TokioUnboundedSender<AuxiliaryEvent>) -> Self {
        Self { inner: tx }
    }

    /// Passthrough `send()` method to the underlying sender
    pub(crate) fn send(
        &self,
        message: AuxiliaryEvent,
    ) -> Result<(), tokio::sync::mpsc::error::SendError<AuxiliaryEvent>> {
        self.inner.send(message)
    }

    /// Spawn a blocking task
    ///
    /// This runs tasks that do semantic analysis on a separate thread pool to avoid
    /// blocking the main loop.
    ///
    /// Can optionally return an event for the auxiliary loop (i.e. diagnostics publication).
    pub(crate) fn spawn_blocking_task<Handler>(&self, handler: Handler)
    where
        Handler: FnOnce() -> anyhow::Result<Option<AuxiliaryEvent>>,
        Handler: Send + 'static,
    {
        let handle = tokio::task::spawn_blocking(handler);

        // Send the join handle to the auxiliary loop so it can log any errors
        // or panics
        if let Err(err) = self.send(AuxiliaryEvent::SpawnedTask(handle)) {
            tracing::warn!("Failed to send task to auxiliary loop due to {err}");
        }
    }
}

/// Global state for the main loop
///
/// This is a singleton that fully owns the source of truth for `WorldState`
/// which contains the inputs of all LSP methods. The `main_loop()` method is
/// the heart of the LSP. The tower-lsp backend and the Jupyter kernel
/// communicate with the main loop through the `Event` channel that is passed on
/// construction.
pub(crate) struct GlobalState {
    /// The global world state containing all inputs for LSP analysis lives
    /// here. The dispatcher provides refs, exclusive refs, or snapshots
    /// (clones) to handlers.
    world: WorldState,

    /// The state containing LSP configuration and tree-sitter parsers for
    /// documents contained in the `WorldState`. Only used in exclusive ref
    /// handlers, and is not cloneable.
    lsp_state: LspState,

    /// Event receiver channel for the main loop. The tower-lsp methods forward
    /// notifications and requests here via `Event::Lsp`. We also receive
    /// messages from the kernel via `Event::Kernel`, and from ourselves via
    /// `Event::Task`.
    events_rx: TokioUnboundedReceiver<Event>,

    /// Auxiliary state that gets moved to the auxiliary thread,
    /// and our channel for communicating with that thread.
    /// Used for sending latency sensitive events like tasks and diagnostics.
    auxiliary_state: Option<AuxiliaryState>,
    auxiliary_event_tx: AuxiliaryEventSender,

    /// Log state that gets moved to the log thread,
    /// and a channel for communicating with that thread which we
    /// pass on to `init_logging()` during `initialize()`.
    log_thread_state: Option<LogThreadState>,
    log_tx: Option<LogMessageSender>,
}

/// Unlike `WorldState`, `LspState` cannot be cloned and is only accessed by
/// exclusive handlers.
pub(crate) struct LspState {
    /// LSP client shared with tower-lsp and the log loop
    pub(crate) client: Client,

    /// Resolver to look up [`Settings`] given a document [`Url`]
    pub(crate) workspace_settings_resolver: WorkspaceSettingsResolver,

    /// The negociated encoding for document positions. Note that documents are
    /// always stored as UTF-8 in Rust Strings. This encoding is only used to
    /// translate UTF-16 positions sent by the client to UTF-8 ones.
    pub(crate) position_encoding: PositionEncoding,

    /// List of client capabilities that we care about
    pub(crate) capabilities: AirClientCapabilities,

    /// State used to dynamically update the log level
    pub(crate) log_state: Option<logging::LogState>,

    /// Global settings communicated by the client
    pub(crate) settings: GlobalSettings,
}

impl LspState {
    pub(crate) fn new(client: Client) -> Self {
        Self {
            client,
            workspace_settings_resolver: Default::default(),
            // All servers and clients have to support UTF-16 so that's the default
            position_encoding: PositionEncoding::Wide(WideEncoding::Utf16),
            capabilities: Default::default(),
            log_state: Default::default(),
            settings: Default::default(),
        }
    }
}

impl LspState {
    pub(crate) fn workspace_document_settings(&self, url: &Url) -> WorkspaceSettings<'_> {
        self.workspace_settings_resolver.settings_for_url(url)
    }

    pub(crate) fn open_workspace_folder(&mut self, url: &Url) {
        self.workspace_settings_resolver.open_workspace_folder(url)
    }

    pub(crate) fn close_workspace_folder(&mut self, url: &Url) {
        self.workspace_settings_resolver.close_workspace_folder(url)
    }
}

enum LoopControl {
    Shutdown,
    None,
}

/// State for the auxiliary loop
///
/// The auxiliary loop handles latency-sensitive events such as log messages. A
/// main loop tick might takes many milliseconds and might have a lot of events
/// in queue, so it's not appropriate for events that need immediate handling.
///
/// The auxiliary loop currently handles:
/// - Log messages.
/// - Joining of spawned blocking tasks to relay any errors or panics to the LSP log.
struct AuxiliaryState {
    client: Client,
    auxiliary_event_rx: TokioUnboundedReceiver<AuxiliaryEvent>,
    tasks: TaskList<Option<AuxiliaryEvent>>,
}

impl GlobalState {
    /// Create a new global state
    ///
    /// # Arguments
    ///
    /// * `client`: The tower-lsp client shared with the tower-lsp backend
    ///   and auxiliary loop.
    pub(crate) fn new(client: Client) -> (Self, TokioUnboundedSender<Event>) {
        // Transmission channel for the main loop events. Shared with the
        // tower-lsp backend and the Jupyter kernel.
        let (events_tx, events_rx) = tokio_unbounded_channel::<Event>();

        let (log_thread_state, log_tx) = LogThreadState::new(client.clone());
        let (auxiliary_state, auxiliary_event_tx) = AuxiliaryState::new(client.clone());

        let state = Self {
            world: WorldState::default(),
            lsp_state: LspState::new(client),
            events_rx,
            auxiliary_state: Some(auxiliary_state),
            auxiliary_event_tx,
            log_thread_state: Some(log_thread_state),
            log_tx: Some(log_tx),
        };

        (state, events_tx)
    }

    /// Start the main and auxiliary loops
    ///
    /// Returns a `JoinSet` that holds onto all tasks and state owned by the
    /// event loop. Drop it to cancel everything and shut down the service.
    pub(crate) fn start(self) -> tokio::task::JoinSet<()> {
        let mut set = tokio::task::JoinSet::<()>::new();

        // Spawn main loop
        set.spawn(async move { self.main_loop().await });

        set
    }

    /// Run main loop
    ///
    /// This takes ownership of all global state and handles one by one LSP
    /// requests, notifications, and other internal events.
    async fn main_loop(mut self) {
        // Spawn latency-sensitive auxiliary and log threads.
        let mut set = tokio::task::JoinSet::<()>::new();

        // Take ownership over `log_thread_state` and start the log thread.
        // Unwrap: `start()` should only be called once.
        let log_thread_state = self.log_thread_state.take().unwrap();
        set.spawn(async move { log_thread_state.start().await });

        // Take ownership over `auxiliary_state` and start the auxiliary thread.
        // Unwrap: `start()` should only be called once.
        let auxiliary_state = self.auxiliary_state.take().unwrap();
        set.spawn(async move { auxiliary_state.start().await });

        loop {
            let event = self.next_event().await;
            match self.handle_event(event).await {
                Err(err) => tracing::error!("Failure while handling event:\n{err:?}"),
                Ok(LoopControl::Shutdown) => break,
                _ => {}
            }
        }

        tracing::trace!("Main loop closed. Shutting down auxiliary and log loop.");
        set.shutdown().await;
    }

    async fn next_event(&mut self) -> Event {
        self.events_rx.recv().await.unwrap()
    }

    #[rustfmt::skip]
    /// Handle event of main loop
    ///
    /// The events are attached to _exclusive_, _sharing_, or _concurrent_
    /// handlers.
    ///
    /// - Exclusive handlers are passed an `&mut` to the world state so they can
    ///   update it.
    /// - Sharing handlers are passed a simple reference. In principle we could
    ///   run these concurrently but we run these one handler at a time for simplicity.
    /// - When concurrent handlers are needed for performance reason (one tick
    ///   of the main loop should be as fast as possible to increase throughput)
    ///   they are spawned on blocking threads and provided a snapshot (clone) of
    ///   the state.
    async fn handle_event(&mut self, event: Event) -> anyhow::Result<LoopControl> {
        let loop_tick = std::time::Instant::now();
        let mut out = LoopControl::None;

        match event {
            Event::Lsp(msg) => match msg {
                LspMessage::Notification(notif) => {
                    match notif {
                        LspNotification::Initialized(_params) => {
                            handlers::handle_initialized(&self.lsp_state).await?;
                        },
                        LspNotification::DidChangeWorkspaceFolders(params) => {
                            handlers_state::did_change_workspace_folders(params, &mut self.lsp_state)?;
                        },
                        LspNotification::DidChangeConfiguration(params) => {
                            handlers_state::did_change_configuration(params, &mut self.lsp_state, &mut self.world).await?;
                        },
                        LspNotification::DidChangeWatchedFiles(params) => {
                            handlers_state::did_change_watched_files(params, &mut self.lsp_state, &self.world).await?;
                        },
                        LspNotification::DidOpenTextDocument(params) => {
                            handlers_state::did_open(params, &mut self.lsp_state, &mut self.world).await?;
                        },
                        LspNotification::DidChangeTextDocument(params) => {
                            handlers_state::did_change(params, &mut self.world)?;
                        },
                        LspNotification::DidSaveTextDocument(_params) => {
                            // Currently ignored
                        },
                        LspNotification::DidCloseTextDocument(params) => {
                            handlers_state::did_close(params, &mut self.world)?;
                        },
                    }
                },

                LspMessage::Request(request, tx) => {
                    match request {
                        LspRequest::Initialize(params) => {
                            // Unwrap: `Initialize` method should only be called once.
                            let log_tx = self.log_tx.take().unwrap();
                            respond(tx, handlers_state::initialize(params, &mut self.lsp_state, log_tx), LspResponse::Initialize)?;
                        },
                        LspRequest::Shutdown => {
                            out = LoopControl::Shutdown;
                            respond(tx, Ok(()), LspResponse::Shutdown)?;
                        },
                        LspRequest::DocumentFormatting(params) => {
                            handlers_state::did_change_formatting_options(&params.text_document.uri, &params.options, &mut self.world);
                            respond(tx, handlers_format::document_formatting(params, &self.lsp_state, &self.world), LspResponse::DocumentFormatting)?;
                        },
                        LspRequest::DocumentRangeFormatting(params) => {
                            handlers_state::did_change_formatting_options(&params.text_document.uri, &params.options, &mut self.world);
                            respond(tx, handlers_format::document_range_formatting(params, &self.lsp_state, &self.world), LspResponse::DocumentRangeFormatting)?;
                        },
                        LspRequest::AirViewFile(params) => {
                            respond(tx, handlers_ext::view_file(params, &self.world), LspResponse::AirViewFile)?;
                        },
                    };
                },
            },

            Event::Kernel(notif) => match notif {
                KernelNotification::DidChangeConsoleInputs(_inputs) => {
                    // TODO
                },
            },
        }

        // TODO Make this threshold configurable by the client
        if loop_tick.elapsed() > std::time::Duration::from_millis(50) {
            tracing::trace!("Handler took {}ms", loop_tick.elapsed().as_millis());
        }

        Ok(out)
    }

    #[allow(dead_code)] // Currently unused
    /// Spawn blocking thread for LSP request handler
    ///
    /// Use this for handlers that might take too long to handle on the main
    /// loop and negatively affect throughput.
    ///
    /// The LSP protocol allows concurrent handling as long as it doesn't affect
    /// correctness of responses. For instance handlers that only inspect the
    /// world state could be run concurrently. On the other hand, handlers that
    /// manipulate documents (e.g. formatting or refactoring) should not.
    fn spawn_handler<T, Handler>(
        &self,
        response_tx: TokioUnboundedSender<anyhow::Result<LspResponse>>,
        handler: Handler,
        into_lsp_response: impl FnOnce(T) -> LspResponse + Send + 'static,
    ) where
        Handler: FnOnce() -> anyhow::Result<T>,
        Handler: Send + 'static,
    {
        self.auxiliary_event_tx.spawn_blocking_task(move || {
            respond(response_tx, handler(), into_lsp_response).and(Ok(None))
        });
    }
}

/// Respond to a request from the LSP
///
/// We receive requests from the LSP client with a response channel. Once we
/// have a response, we send it to tower-lsp which will forward it to the
/// client.
///
/// The response channel will be closed if the request has been cancelled on
/// the tower-lsp side. In that case the future of the async request method
/// has been dropped, along with the receiving side of this channel. It's
/// unclear whether we want to support this sort of client-side cancellation
/// better. We should probably focus on cancellation of expensive tasks
/// running on side threads when the world state has changed.
///
/// # Arguments
///
/// * - `response_tx`: A response channel for the tower-lsp request handler.
/// * - `response`: The response wrapped in a `anyhow::Result`. Errors are logged.
/// * - `into_lsp_response`: A constructor for the relevant `LspResponse` variant.
fn respond<T>(
    response_tx: TokioUnboundedSender<anyhow::Result<LspResponse>>,
    response: anyhow::Result<T>,
    into_lsp_response: impl FnOnce(T) -> LspResponse,
) -> anyhow::Result<()> {
    let out = match response {
        Ok(_) => Ok(()),
        Err(ref err) => Err(anyhow!("Error while handling request:\n{err:?}")),
    };

    let response = response.map(into_lsp_response);

    // Ignore errors from a closed channel. This indicates the request has
    // been cancelled on the tower-lsp side.
    let _ = response_tx.send(response);

    out
}

// Needed for spawning the loop
unsafe impl Sync for AuxiliaryState {}

impl AuxiliaryState {
    fn new(client: Client) -> (Self, AuxiliaryEventSender) {
        // Channels for communication with the auxiliary loop
        let (auxiliary_event_tx, auxiliary_event_rx) = tokio_unbounded_channel::<AuxiliaryEvent>();
        let auxiliary_event_tx = AuxiliaryEventSender::new(auxiliary_event_tx);

        // List of pending tasks for which we manage the lifecycle (mainly relay
        // errors and panics)
        let tasks = futures::stream::FuturesUnordered::new();

        // Prevent the stream from ever being empty so that `tasks.next()` never
        // resolves to `None`
        let pending =
            tokio::task::spawn(future::pending::<anyhow::Result<Option<AuxiliaryEvent>>>());
        let pending =
            Box::pin(pending) as Pin<Box<dyn AnyhowJoinHandleFut<Option<AuxiliaryEvent>> + Send>>;
        tasks.push(pending);

        let state = Self {
            client,
            auxiliary_event_rx,
            tasks,
        };

        (state, auxiliary_event_tx)
    }

    /// Start the auxiliary loop
    ///
    /// Takes ownership of auxiliary state and start the low-latency auxiliary
    /// loop.
    async fn start(mut self) -> ! {
        loop {
            match self.next_event().await {
                AuxiliaryEvent::SpawnedTask(handle) => self.tasks.push(Box::pin(handle)),
                AuxiliaryEvent::PublishDiagnostics(uri, diagnostics, version) => {
                    self.client
                        .publish_diagnostics(uri, diagnostics, version)
                        .await
                }
            }
        }
    }

    async fn next_event(&mut self) -> AuxiliaryEvent {
        loop {
            tokio::select! {
                event = self.auxiliary_event_rx.recv() => return event.unwrap(),

                handle = self.tasks.next() => match handle.unwrap() {
                    // A joined task returned an event for us, handle it
                    Ok(Ok(Some(event))) => return event,

                    // Otherwise relay any errors and loop back into select
                    Err(err) => tracing::error!("A task panicked:\n{err:?}"),
                    Ok(Err(err)) => tracing::error!("A task failed:\n{err:?}"),
                    _ => (),
                },
            }
        }
    }
}
