//
// main_loop.rs
//
// Copyright (C) 2024 Posit Software, PBC. All rights reserved.
//
//

use std::future;
use std::pin::Pin;

use anyhow::anyhow;
use futures::StreamExt;
use tokio::sync::mpsc::unbounded_channel as tokio_unbounded_channel;
use tokio::task::JoinHandle;
use tower_lsp::lsp_types;
use tower_lsp::lsp_types::Diagnostic;
use tower_lsp::lsp_types::MessageType;
use tower_lsp::Client;
use url::Url;

use crate::lsp;
use crate::lsp::backend::LspMessage;
use crate::lsp::backend::LspNotification;
use crate::lsp::backend::LspRequest;
use crate::lsp::backend::LspResponse;
use crate::lsp::diagnostics;
use crate::lsp::documents::Document;
use crate::lsp::handlers;
use crate::lsp::state::ParserState;
use crate::lsp::state::WorldState;
use crate::lsp::state_handlers;
use crate::lsp::state_handlers::ConsoleInputs;

pub(crate) type TokioUnboundedSender<T> = tokio::sync::mpsc::UnboundedSender<T>;
pub(crate) type TokioUnboundedReceiver<T> = tokio::sync::mpsc::UnboundedReceiver<T>;

// The global instance of the auxiliary event channel, used for sending log
// messages or spawning threads from free functions. Since this is an unbounded
// channel, sending a log message is not async nor blocking. Tokio senders are
// Send and Sync so this global variable can be safely shared across threads.
static mut AUXILIARY_EVENT_TX: std::cell::OnceCell<TokioUnboundedSender<AuxiliaryEvent>> =
    std::cell::OnceCell::new();

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

    /// The state containing tree-sitter parsers for documents contained in the
    /// `WorldState`. Only used in exclusive ref handlers, and is not cloneable.
    parsers: ParserState,

    /// LSP client shared with tower-lsp and the log loop
    client: Client,

    /// Event channels for the main loop. The tower-lsp methods forward
    /// notifications and requests here via `Event::Lsp`. We also receive
    /// messages from the kernel via `Event::Kernel`, and from ourselves via
    /// `Event::Task`.
    events_tx: TokioUnboundedSender<Event>,
    events_rx: TokioUnboundedReceiver<Event>,
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
    /// * `client`: The tower-lsp cient shared with the tower-lsp backend
    ///   and auxiliary loop.
    pub(crate) fn new(client: Client) -> Self {
        // Transmission channel for the main loop events. Shared with the
        // tower-lsp backend and the Jupyter kernel.
        let (events_tx, events_rx) = tokio_unbounded_channel::<Event>();

        Self {
            world: WorldState::default(),
            parsers: ParserState::new(),
            client,
            events_tx,
            events_rx,
        }
    }

    /// Get `Event` transmission channel
    pub(crate) fn events_tx(&self) -> TokioUnboundedSender<Event> {
        self.events_tx.clone()
    }

    /// Start the main and auxiliary loops
    ///
    /// Returns a `JoinSet` that holds onto all tasks and state owned by the
    /// event loop. Drop it to cancel everything and shut down the service.
    pub(crate) fn start(self) -> tokio::task::JoinSet<()> {
        let mut set = tokio::task::JoinSet::<()>::new();

        // Spawn latency-sensitive auxiliary loop. Must be first to initialise
        // global transmission channel.
        let aux = AuxiliaryState::new(self.client.clone());
        set.spawn(async move { aux.start().await });

        // Spawn main loop
        set.spawn(async move { self.main_loop().await });

        set
    }

    /// Run main loop
    ///
    /// This takes ownership of all global state and handles one by one LSP
    /// requests, notifications, and other internal events.
    async fn main_loop(mut self) {
        loop {
            let event = self.next_event().await;
            if let Err(err) = self.handle_event(event).await {
                lsp::log_error!("Failure while handling event:\n{err:?}")
            }
        }
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
    async fn handle_event(&mut self, event: Event) -> anyhow::Result<()> {
        let loop_tick = std::time::Instant::now();

        match event {
            Event::Lsp(msg) => match msg {
                LspMessage::Notification(notif) => {
                    lsp::log_info!("{notif:#?}");

                    match notif {
                        LspNotification::Initialized(_params) => {
                        },
                        LspNotification::DidChangeWorkspaceFolders(_params) => {
                            // TODO: Restart indexer with new folders.
                        },
                        LspNotification::DidChangeConfiguration(_params) => {
                            // TODO: Get config updates.
                        },
                        LspNotification::DidChangeWatchedFiles(_params) => {
                            // TODO: Re-index the changed files.
                        },
                        LspNotification::DidOpenTextDocument(params) => {
                            state_handlers::did_open(params, &mut self.world, &mut self.parsers)?;
                        },
                        LspNotification::DidChangeTextDocument(params) => {
                            state_handlers::did_change(params, &mut self.world, &mut self.parsers)?;
                        },
                        LspNotification::DidSaveTextDocument(_params) => {
                            // Currently ignored
                        },
                        LspNotification::DidCloseTextDocument(params) => {
                            state_handlers::did_close(params, &mut self.world, &mut self.parsers)?;
                        },
                    }
                },

                LspMessage::Request(request, tx) => {
                    lsp::log_info!("{request:#?}");

                    match request {
                        LspRequest::Initialize(params) => {
                            respond(tx, state_handlers::initialize(params, self.state_mut()), LspResponse::Initialize)?;
                        },
                        LspRequest::Shutdown() => {
                            // TODO
                            respond(tx, Ok(()), LspResponse::Shutdown)?;
                        },
                        LspRequest::WorkspaceSymbol(params) => {
                            respond(tx, handlers::symbol(params), LspResponse::WorkspaceSymbol)?;
                        },
                        LspRequest::DocumentSymbol(params) => {
                            respond(tx, handlers::document_symbol(params, self.state_ref()), LspResponse::DocumentSymbol)?;
                        },
                        LspRequest::ExecuteCommand(_params) => {
                            respond(tx, handlers::execute_command(&self.client).await, LspResponse::ExecuteCommand)?;
                        },
                        LspRequest::Completion(params) => {
                            respond(tx, handlers::completion(params, self.state_ref()), LspResponse::Completion)?;
                        },
                        LspRequest::CompletionResolve(params) => {
                            respond(tx, handlers::handle_completion_resolve(params), LspResponse::CompletionResolve)?;
                        },
                        LspRequest::Hover(params) => {
                            respond(tx, handlers::handle_hover(params, self.state_ref()), LspResponse::Hover)?;
                        },
                        LspRequest::SignatureHelp(params) => {
                            respond(tx, handlers::handle_signature_help(params, self.state_ref()), LspResponse::SignatureHelp)?;
                        },
                        LspRequest::GotoDefinition(params) => {
                            respond(tx, handlers::handle_goto_definition(params, self.state_ref()), LspResponse::GotoDefinition)?;
                        },
                        LspRequest::GotoImplementation(_params) => {
                            // TODO
                            respond(tx, Ok(None), LspResponse::GotoImplementation)?;
                        },
                        LspRequest::SelectionRange(params) => {
                            respond(tx, handlers::handle_selection_range(params, self.state_ref()), LspResponse::SelectionRange)?;
                        },
                        LspRequest::References(params) => {
                            respond(tx, handlers::handle_references(params, self.state_ref()), LspResponse::References)?;
                        },
                        LspRequest::StatementRange(params) => {
                            respond(tx, handlers::handle_statement_range(params, self.state_ref()), LspResponse::StatementRange)?;
                        },
                        LspRequest::HelpTopic(params) => {
                            respond(tx, handlers::handle_help_topic(params, self.state_ref()), LspResponse::HelpTopic)?;
                        },
                    };
                },
            },

            Event::Kernel(notif) => match notif {
                KernelNotification::DidChangeConsoleInputs(inputs) => {
                    state_handlers::did_change_console_inputs(inputs, self.state_mut())?;
                },
            },
        }

        // TODO Make this threshold configurable by the client
        if loop_tick.elapsed() > std::time::Duration::from_millis(50) {
            lsp::log_info!("Handler took {}ms", loop_tick.elapsed().as_millis());
        }

        Ok(())
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
        response_tx: TokioUnboundedSender<anyhow::Result<LspResponse>>,
        handler: Handler,
        into_lsp_response: impl FnOnce(T) -> LspResponse + Send + 'static,
    ) where
        Handler: FnOnce() -> anyhow::Result<T>,
        Handler: Send + 'static,
    {
        lsp::spawn_blocking(move || {
            respond(response_tx, handler(), into_lsp_response).and(Ok(None))
        })
    }

    fn state_ref(&self) -> &WorldState {
        &self.world
    }
    fn state_mut(&mut self) -> &mut WorldState {
        &mut self.world
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
    fn new(client: Client) -> Self {
        // Channels for communication with the auxiliary loop
        let (auxiliary_event_tx, auxiliary_event_rx) = tokio_unbounded_channel::<AuxiliaryEvent>();

        // Set global instance of this channel. This is used for interacting
        // with the auxiliary loop (logging messages or spawning a task) from
        // free functions.
        unsafe {
            if let Some(val) = AUXILIARY_EVENT_TX.get_mut() {
                // Reset channel if already set. Happens e.g. on reconnection after a refresh.
                *val = auxiliary_event_tx.clone();
            } else {
                // Set channel for the first time
                AUXILIARY_EVENT_TX.set(auxiliary_event_tx.clone()).unwrap();
            }
        }

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

        Self {
            client,
            auxiliary_event_rx,
            tasks,
        }
    }

    /// Spawn the auxiliary loop
    ///
    /// Takes ownership of auxiliary state and spawns the low-latency auxiliary
    /// loop on the provided task set.
    async fn start(mut self) {
        loop {
            match self.next_event().await {
                AuxiliaryEvent::Log(level, message) => self.log(level, message).await,
                AuxiliaryEvent::SpawnedTask(handle) => self.tasks.push(Box::pin(handle)),
                AuxiliaryEvent::PublishDiagnostics(uri, diagnostics, version) => {
                    self.client
                        .publish_diagnostics(uri, diagnostics, version)
                        .await
                },
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

fn auxiliary_tx() -> &'static TokioUnboundedSender<AuxiliaryEvent> {
    // If we get here that means the LSP was initialised at least once. The
    // channel might be closed if the LSP was dropped, but it should exist.
    unsafe { AUXILIARY_EVENT_TX.get().unwrap() }
}

fn send_auxiliary(event: AuxiliaryEvent) {
    if let Err(err) = auxiliary_tx().send(event) {
        // The error includes the event
        log::warn!("LSP is shut down, can't send event:\n{err:?}");
    }
}

/// Send a message to the LSP client. This is non-blocking and treated on a
/// latency-sensitive task.
pub(crate) fn log(level: lsp_types::MessageType, message: String) {
    // Check that channel is still alive in case the LSP was closed.
    // If closed, fallthrough.
    if let Ok(_) = auxiliary_tx().send(AuxiliaryEvent::Log(level.clone(), message.clone())) {
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
pub(crate) fn spawn_blocking<Handler>(handler: Handler)
where
    Handler: FnOnce() -> anyhow::Result<Option<AuxiliaryEvent>>,
    Handler: Send + 'static,
{
    let handle = tokio::task::spawn_blocking(|| handler());

    // Send the join handle to the auxiliary loop so it can log any errors
    // or panics
    send_auxiliary(AuxiliaryEvent::SpawnedTask(handle));
}

pub(crate) fn spawn_diagnostics_refresh(uri: Url, document: Document, state: WorldState) {
    lsp::spawn_blocking(move || {
        let version = document.version;
        let diagnostics = diagnostics::generate_diagnostics(document, state);

        Ok(Some(AuxiliaryEvent::PublishDiagnostics(
            uri,
            diagnostics,
            version,
        )))
    })
}

pub(crate) fn spawn_diagnostics_refresh_all(state: WorldState) {
    for (url, document) in state.documents.iter() {
        spawn_diagnostics_refresh(url.clone(), document.clone(), state.clone())
    }
}

pub(crate) fn publish_diagnostics(uri: Url, diagnostics: Vec<Diagnostic>, version: Option<i32>) {
    send_auxiliary(AuxiliaryEvent::PublishDiagnostics(
        uri,
        diagnostics,
        version,
    ));
}
