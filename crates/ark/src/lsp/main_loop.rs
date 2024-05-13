//
// main_loop.rs
//
// Copyright (C) 2024 Posit Software, PBC. All rights reserved.
//
//

use std::future;

use anyhow::anyhow;
use tokio::sync::mpsc::unbounded_channel as tokio_unbounded_channel;
use tower_lsp::lsp_types;
use tower_lsp::lsp_types::Diagnostic;
use tower_lsp::lsp_types::MessageType;
use tower_lsp::Client;
use url::Url;

use crate::lsp::backend::LspMessage;
use crate::lsp::backend::LspNotification;
use crate::lsp::backend::LspRequest;
use crate::lsp::backend::LspResponse;
use crate::lsp::documents::Document;
use crate::lsp::handlers;
use crate::lsp::state::WorldState;
use crate::lsp::state_handlers;
use crate::lsp::state_handlers::ConsoleInputs;

pub(crate) type TokioUnboundedSender<T> = tokio::sync::mpsc::UnboundedSender<T>;
pub(crate) type TokioUnboundedReceiver<T> = tokio::sync::mpsc::UnboundedReceiver<T>;

// This is the syntax for trait aliases until an official one is stabilised
trait AnyhowFut<T>: std::future::Future<Output = anyhow::Result<T>> {}
impl<T, F> AnyhowFut<T> for F where F: std::future::Future<Output = anyhow::Result<T>> {}

#[derive(Debug)]
pub(crate) enum Event {
    Lsp(LspMessage),
    Task(LspTask),
    Kernel(KernelNotification),
}
#[derive(Debug)]
pub(crate) enum LspTask {
    Log(lsp_types::MessageType, String),
    RefreshDiagnostics(Url, Document, WorldState),
    RefreshAllDiagnostics(),
    PublishDiagnostics(Url, Vec<Diagnostic>, Option<i32>),
}

#[derive(Debug)]
pub(crate) enum KernelNotification {
    DidChangeConsoleInputs(ConsoleInputs),
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

    /// LSP client shared with tower-lsp
    client: Client,

    /// Event channel for the main loop. The tower-lsp methods forward
    /// notifications and requests here via `Event::Lsp`. We also receive
    /// messages from the kernel via `Event::Kernel`, and from ourselves via
    /// `Event::Task`.
    events_tx: TokioUnboundedSender<Event>,
    events_rx: TokioUnboundedReceiver<Event>,

    /// Handle to pending tasks, used to log errors and panics
    tasks: tokio::task::JoinSet<anyhow::Result<()>>,
}

impl GlobalState {
    /// Create a new global state
    ///
    /// # Arguments
    ///
    /// * `client`: The tower-lsp cient shared with the tower-lsp backend.
    pub(crate) fn new(client: Client) -> Self {
        // Transmission channel for the main loop events. Shared with the
        // tower-lsp backend and the Jupyter kernel.
        let (events_tx, events_rx) = tokio_unbounded_channel::<Event>();

        let mut tasks = tokio::task::JoinSet::new();

        // Prevent the task set from ever becoming empty, so that `join_next()`
        // never resolves to `None`.
        tasks.spawn(future::pending());

        Self {
            world: WorldState::default(),
            client,
            events_tx,
            events_rx,
            tasks,
        }
    }

    /// Get `Event` transmission channel
    pub(crate) fn events_tx(&self) -> TokioUnboundedSender<Event> {
        self.events_tx.clone()
    }

    /// Start the main loop
    ///
    /// This takes ownership of all global state and handles one by one LSP
    /// requests, notifications, and other internal events and tasks.
    pub(crate) fn main_loop(mut self) -> tokio::task::JoinHandle<()> {
        tokio::spawn(async move {
            loop {
                let event = self.next_event().await;
                if let Err(err) = self.handle_event(event).await {
                    self.log_error(format!("Failure while handling event: {err:?}"))
                }
            }
        })
    }

    async fn next_event(&mut self) -> Event {
        loop {
            tokio::select! {
                event = self.events_rx.recv() => {
                    return event.unwrap()
                },

                result = self.tasks.join_next() => {
                    match result {
                        None => unreachable!(),
                        Some(Err(err)) => self.log_error(format!("A task panicked: {err:?}")),
                        Some(Ok(Err(err))) => self.log_error(format!("A task failed: {err:?}")),
                        _ => (),
                    }
                }
            }
        }
    }

    #[rustfmt::skip]
    /// Handle event of main loop
    ///
    /// The events are attached to _exclusive_, _sharing_, or _snapshot_
    /// handlers.
    ///
    /// - Exclusive handlers are passed an `&mut` to the world state so they can
    ///   update it.
    /// - Sharing handlers are passed a simple reference. In principle we could
    ///   run these concurrently but we typically run a handler one at a time.
    /// - When concurrent handlers are needed for performance reason (one tick
    ///   of the main loop should be as fast as possible to increase throughput)
    ///   they are spawned on blocking threads and provided a snapshot (clone) of
    ///   the state.
    async fn handle_event(&mut self, event: Event) -> anyhow::Result<()> {
        match event {
            Event::Lsp(msg) => match msg {
                LspMessage::Notification(notif) => {
                    self.log_info(format!("{notif:#?}"));

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
                            state_handlers::did_open(params, self.events_tx(), &mut self.world)?;
                        },
                        LspNotification::DidChangeTextDocument(params) => {
                            state_handlers::did_change(params, self.events_tx(), &mut self.world)?;
                        },
                        LspNotification::DidSaveTextDocument(_params) => {
                            // Currently ignored
                        },
                        LspNotification::DidCloseTextDocument(params) => {
                            state_handlers::did_close(params, self.events_tx(), &mut self.world)?;
                        },
                    }
                },

                LspMessage::Request(request, tx) => {
                    self.log_info(format!("{request:#?}"));

                    match request {
                        LspRequest::Initialize(params) => {
                            Self::respond(tx, state_handlers::initialize(params, self.state_mut()), LspResponse::Initialize)?;
                        },
                        LspRequest::Shutdown() => {
                            // TODO
                            Self::respond(tx, Ok(()), LspResponse::Shutdown)?;
                        },
                        LspRequest::WorkspaceSymbol(params) => {
                            Self::respond(tx, handlers::symbol(params), LspResponse::WorkspaceSymbol)?;
                        },
                        LspRequest::DocumentSymbol(params) => {
                            Self::respond(tx, handlers::document_symbol(params, self.state_ref()), LspResponse::DocumentSymbol)?;
                        },
                        LspRequest::ExecuteCommand(_params) => {
                            Self::respond(tx, handlers::execute_command(&self.client).await, LspResponse::ExecuteCommand)?;
                        },
                        LspRequest::Completion(params) => {
                            Self::respond(tx, handlers::completion(params, self.state_ref()), LspResponse::Completion)?;
                        },
                        LspRequest::CompletionResolve(params) => {
                            Self::respond(tx, handlers::handle_completion_resolve(params), LspResponse::CompletionResolve)?;
                        },
                        LspRequest::Hover(params) => {
                            Self::respond(tx, handlers::handle_hover(params, self.state_ref()), LspResponse::Hover)?;
                        },
                        LspRequest::SignatureHelp(params) => {
                            Self::respond(tx, handlers::handle_signature_help(params, self.state_ref()), LspResponse::SignatureHelp)?;
                        },
                        LspRequest::GotoDefinition(params) => {
                            Self::respond(tx, handlers::handle_goto_definition(params, self.state_ref()), LspResponse::GotoDefinition)?;
                        },
                        LspRequest::GotoImplementation(_params) => {
                            // TODO
                            Self::respond(tx, Ok(None), LspResponse::GotoImplementation)?;
                        },
                        LspRequest::SelectionRange(params) => {
                            Self::respond(tx, handlers::handle_selection_range(params, self.state_ref()), LspResponse::SelectionRange)?;
                        },
                        LspRequest::References(params) => {
                            Self::respond(tx, handlers::handle_references(params, self.state_ref()), LspResponse::References)?;
                        },
                        LspRequest::StatementRange(params) => {
                            Self::respond(tx, handlers::handle_statement_range(params, self.state_ref()), LspResponse::StatementRange)?;
                        },
                        LspRequest::HelpTopic(params) => {
                            Self::respond(tx, handlers::handle_help_topic(params, self.state_ref()), LspResponse::HelpTopic)?;
                        },
                    };
                },
            },

            Event::Task(task) => match task {
                LspTask::Log(level, message) => {
                    self.log(level, message)
                },
                LspTask::RefreshDiagnostics(url, doc, state) => {
                    let events_tx = self.events_tx();
                    self.spawn_blocking(move || handlers::refresh_diagnostics(url, doc, events_tx, state));
                },
                LspTask::RefreshAllDiagnostics() => {
                    let state = self.state_clone();
                    let events_tx = self.events_tx();
                    handlers::refresh_all_diagnostics(&mut self.tasks, events_tx, state)?;
                },
                LspTask::PublishDiagnostics(uri, diagnostics, version) => {
                    handlers::publish_diagnostics(&self.client, uri, diagnostics, version).await?;
                },
            },

            Event::Kernel(notif) => match notif {
                KernelNotification::DidChangeConsoleInputs(inputs) => {
                    state_handlers::did_change_console_inputs(inputs, self.state_mut())?;
                },
            },
        }

        Ok(())
    }

    /// Respond to a request from the LSP
    ///
    /// We receive requests from the LSP client with a response channel.
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
    /// * - `response`: The response wrapped in a `anyhow::Result`.
    /// * - `into_lsp_response`: A constructor for the relevant `LspResponse` variant.
    fn respond<T>(
        response_tx: TokioUnboundedSender<anyhow::Result<LspResponse>>,
        response: anyhow::Result<T>,
        into_lsp_response: impl FnOnce(T) -> LspResponse,
    ) -> anyhow::Result<()> {
        let out = match response {
            Ok(_) => Ok(()),
            Err(ref err) => Err(anyhow!("Error while handling request: {err:?}")),
        };

        let response = response.map(into_lsp_response);

        // Ignore errors from a closed channel. This indicates the request has
        // been cancelled on the tower-lsp side.
        let _ = response_tx.send(response);

        out
    }

    /// Spawn a blocking task
    ///
    /// This runs handlers that do semantic analysis on a separate thread pool
    /// to avoid blocking the main loop.
    ///
    /// Use with care because this will cause out-of-order responses to LSP
    /// requests. This is allowed by the protocol as long as it doesn't affect
    /// the correctness of the responses. In particular, requests that respond
    /// with edits should be responded to in the order of arrival.
    ///
    /// If needed we could add a mechanism to mark handlers that must respond
    /// in order of arrival. Such a mechanism should probably not be the default
    /// because that would overly decrease the throughput of blocking tasks when
    /// a handler takes too much time.
    fn spawn_blocking<Handler>(&mut self, handler: Handler)
    where
        Handler: FnOnce() -> anyhow::Result<()>,
        Handler: Send + 'static,
    {
        self.tasks.spawn_blocking(|| handler());
    }

    fn state_ref(&self) -> &WorldState {
        &self.world
    }
    fn state_mut(&mut self) -> &mut WorldState {
        &mut self.world
    }
    fn state_clone(&self) -> WorldState {
        self.world.clone()
    }

    /// Log an LSP message via a new async task
    fn log(&self, level: MessageType, message: String) {
        // TODO: Make a single task that loops over a channel to preserve message order.
        // We should also avoid a `Log` task having to go through the `Event`
        // channel. It would also be interesting to integrate with the `tracing` crate.
        let client = self.client.clone();
        tokio::spawn(async move { client.log_message(level, message).await });
    }

    fn log_info(&self, message: String) {
        self.log(MessageType::INFO, message);
    }
    fn log_error(&self, message: String) {
        self.log(MessageType::ERROR, message);
    }
}
