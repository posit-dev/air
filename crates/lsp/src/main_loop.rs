//
// main_loop.rs
//
// Copyright (C) 2024 Posit Software, PBC. All rights reserved.
//
//

use std::collections::HashMap;
use std::future;
use std::pin::Pin;

use anyhow::anyhow;
use biome_lsp_converters::PositionEncoding;
use biome_lsp_converters::WideEncoding;
use futures::StreamExt;
use tokio::sync::mpsc::unbounded_channel as tokio_unbounded_channel;
use tokio::task::JoinHandle;
use tower_lsp::lsp_types;
use tower_lsp::lsp_types::Diagnostic;
use tower_lsp::lsp_types::MessageType;
use tower_lsp::Client;
use url::Url;

use crate::handlers;
use crate::handlers_ext;
use crate::handlers_format;
use crate::handlers_state;
use crate::handlers_state::ConsoleInputs;
use crate::state::WorldState;
use crate::tower_lsp::LspMessage;
use crate::tower_lsp::LspNotification;
use crate::tower_lsp::LspRequest;
use crate::tower_lsp::LspResponse;

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

#[derive(Debug)]
pub(crate) enum Event {
    Lsp(LspMessage),
    Kernel(KernelNotification),
}

#[derive(Debug)]
pub(crate) enum KernelNotification {
    DidChangeConsoleInputs(ConsoleInputs),
}

#[derive(Debug)]
pub(crate) enum AuxiliaryEvent {
    Log(lsp_types::MessageType, String),
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

    pub(crate) fn log_info(&self, message: String) {
        self.log(MessageType::INFO, message);
    }
    pub(crate) fn log_warn(&self, message: String) {
        self.log(MessageType::WARNING, message);
    }
    pub(crate) fn log_error(&self, message: String) {
        self.log(MessageType::ERROR, message);
    }

    /// Send an `AuxiliaryEvent::Log` message in a non-blocking way
    fn log(&self, level: lsp_types::MessageType, message: String) {
        // We're not connected to an LSP client when running unit tests
        if cfg!(test) {
            return;
        }

        // Check that channel is still alive in case the LSP was closed.
        // If closed, fallthrough.
        if self
            .send(AuxiliaryEvent::Log(level, message.clone()))
            .is_ok()
        {
            return;
        }

        // Log to the kernel as fallback
        log::warn!("LSP channel is closed, redirecting messages to Jupyter kernel");

        match level {
            MessageType::ERROR => log::error!("{message}"),
            MessageType::WARNING => log::warn!("{message}"),
            _ => log::info!("{message}"),
        };
    }

    /// Spawn a blocking task
    ///
    /// This runs tasks that do semantic analysis on a separate thread pool to avoid
    /// blocking the main loop.
    ///
    /// Can optionally return an event for the auxiliary loop (i.e. a log message or
    /// diagnostics publication).
    pub(crate) fn spawn_blocking_task<Handler>(&self, handler: Handler)
    where
        Handler: FnOnce() -> anyhow::Result<Option<AuxiliaryEvent>>,
        Handler: Send + 'static,
    {
        let handle = tokio::task::spawn_blocking(handler);

        // Send the join handle to the auxiliary loop so it can log any errors
        // or panics
        if let Err(err) = self.send(AuxiliaryEvent::SpawnedTask(handle)) {
            log::warn!("Failed to send task to auxiliary loop due to {err}");
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

    /// LSP client shared with tower-lsp and the log loop
    client: Client,

    /// Event channels for the main loop. The tower-lsp methods forward
    /// notifications and requests here via `Event::Lsp`. We also receive
    /// messages from the kernel via `Event::Kernel`, and from ourselves via
    /// `Event::Task`.
    events_tx: TokioUnboundedSender<Event>,
    events_rx: TokioUnboundedReceiver<Event>,

    /// State of the internal auxiliary loop.
    /// Fully managed by the `GlobalState`.
    /// Initialized as `Some()`, and converted to `None` on `start()`.
    auxiliary_state: Option<AuxiliaryState>,

    /// Event channel for sending to the auxiliary loop.
    /// Used for sending latency sensitive events like logs, tasks, and
    /// diagnostics.
    auxiliary_event_tx: AuxiliaryEventSender,
}

/// Unlike `WorldState`, `ParserState` cannot be cloned and is only accessed by
/// exclusive handlers.
pub(crate) struct LspState {
    /// The negociated encoding for document positions. Note that documents are
    /// always stored as UTF-8 in Rust Strings. This encoding is only used to
    /// translate UTF-16 positions sent by the client to UTF-8 ones.
    pub(crate) position_encoding: PositionEncoding,

    /// The set of tree-sitter document parsers managed by the `GlobalState`.
    pub(crate) parsers: HashMap<Url, tree_sitter::Parser>,

    /// List of capabilities for which we need to send a registration request
    /// when we get the `Initialized` notification.
    pub(crate) needs_registration: ClientCaps,
    // Add handle to aux loop here?
}

impl Default for LspState {
    fn default() -> Self {
        Self {
            // Default encoding specified in the LSP protocol
            position_encoding: PositionEncoding::Wide(WideEncoding::Utf16),
            parsers: Default::default(),
            needs_registration: Default::default(),
        }
    }
}

#[derive(Debug, Default)]
pub(crate) struct ClientCaps {
    pub(crate) did_change_configuration: bool,
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
    pub(crate) fn new(client: Client) -> Self {
        // Transmission channel for the main loop events. Shared with the
        // tower-lsp backend and the Jupyter kernel.
        let (events_tx, events_rx) = tokio_unbounded_channel::<Event>();

        let (auxiliary_state, auxiliary_event_tx) = AuxiliaryState::new(client.clone());

        Self {
            world: WorldState::default(),
            lsp_state: LspState::default(),
            client,
            auxiliary_state: Some(auxiliary_state),
            auxiliary_event_tx,
            events_tx,
            events_rx,
        }
    }

    /// Get `Event` transmission channel
    pub(crate) fn events_tx(&self) -> TokioUnboundedSender<Event> {
        self.events_tx.clone()
    }

    /// Get `AuxiliaryEvent` transmission channel
    pub(crate) fn auxiliary_event_tx(&self) -> AuxiliaryEventSender {
        self.auxiliary_event_tx.clone()
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
        // Spawn latency-sensitive auxiliary loop. Must be first to initialise
        // global transmission channel.
        let mut set = tokio::task::JoinSet::<()>::new();

        // Move the `auxiliary_state` owned by the global state to its own thread.
        // Unwrap: `start()` should only ever be called once.
        let auxiliary_state = self.auxiliary_state.take().unwrap();
        set.spawn(async move { auxiliary_state.start().await });

        loop {
            let event = self.next_event().await;
            match self.handle_event(event).await {
                Err(err) => self.log_error(format!("Failure while handling event:\n{err:?}")),
                Ok(LoopControl::Shutdown) => break,
                _ => {}
            }
        }

        log::trace!("Main loop closed. Shutting down auxiliary loop.");
        set.shutdown().await;

        log::trace!("Main loop exited.");
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
                            handlers::handle_initialized(&self.client, &self.lsp_state).await?;
                        },
                        LspNotification::DidChangeWorkspaceFolders(_params) => {
                            // TODO: Restart indexer with new folders.
                        },
                        LspNotification::DidChangeConfiguration(params) => {
                            handlers_state::did_change_configuration(params, &self.client, &mut self.world).await?;
                        },
                        LspNotification::DidChangeWatchedFiles(_params) => {
                            // TODO: Re-index the changed files.
                        },
                        LspNotification::DidOpenTextDocument(params) => {
                            handlers_state::did_open(params, &self.lsp_state, &mut self.world)?;
                        },
                        LspNotification::DidChangeTextDocument(params) => {
                            handlers_state::did_change(params, &mut self.world)?;
                        },
                        LspNotification::DidSaveTextDocument(_params) => {
                            // Currently ignored
                        },
                        LspNotification::DidCloseTextDocument(params) => {
                            handlers_state::did_close(params, &mut self.world, &self.auxiliary_event_tx)?;
                        },
                    }
                },

                LspMessage::Request(request, tx) => {
                    match request {
                        LspRequest::Initialize(params) => {
                            respond(tx, handlers_state::initialize(params, &mut self.lsp_state, &mut self.world), LspResponse::Initialize)?;
                        },
                        LspRequest::Shutdown() => {
                            out = LoopControl::Shutdown;
                            respond(tx, Ok(()), LspResponse::Shutdown)?;
                        },
                        LspRequest::DocumentFormatting(params) => {
                            respond(tx, handlers_format::document_formatting(params, &self.world), LspResponse::DocumentFormatting)?;
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
            self.log_info(format!("Handler took {}ms", loop_tick.elapsed().as_millis()));
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

    fn log_info(&self, message: String) {
        self.auxiliary_event_tx.log_info(message);
    }
    fn log_warn(&self, message: String) {
        self.auxiliary_event_tx.log_warn(message);
    }
    fn log_error(&self, message: String) {
        self.auxiliary_event_tx.log_error(message);
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
    async fn start(mut self) {
        loop {
            match self.next_event().await {
                AuxiliaryEvent::Log(level, message) => self.log(level, message).await,
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
                    Err(err) => self.log_error(format!("A task panicked:\n{err:?}")).await,
                    Ok(Err(err)) => self.log_error(format!("A task failed:\n{err:?}")).await,
                    _ => (),
                },
            }
        }
    }

    async fn log(&self, level: MessageType, message: String) {
        self.client.log_message(level, message).await
    }
    async fn log_error(&self, message: String) {
        self.client.log_message(MessageType::ERROR, message).await
    }
}
