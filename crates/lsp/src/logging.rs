// --- source
// authors = ["Charlie Marsh"]
// license = "MIT"
// origin = "https://github.com/astral-sh/ruff/blob/03fb2e5ac1481e498f84474800b42a966e9843e1/crates/ruff_server/src/trace.rs"
// ---

//! The logging system for `air lsp`.
//!
//! ## Air crate logs
//!
//! For air crates, a single log level is supplied as one of: error, warn, info, debug,
//! or trace, which is applied to all air crates that log.
//!
//! Resolution strategy:
//!
//! - The environment variable `AIR_LOG_LEVEL` is consulted.
//!
//! - The configuration variable `air.logLevel` is consulted. This can be set in two ways:
//!
//!   - As an initialization option, i.e. `InitializeParams.initializationOptions.logLevel`,
//!     which is passed in on startup. In VS Code / Positron our extension preemptively
//!     looks for the global `air.logLevel` configuration option and passes it through.
//!     In Zed you'd set this in `initialization_options`.
//!
//!   - As a dynamic global configuration option. We watch for `air.logLevel` configuration
//!     changes and update the log level accordingly.
//!
//! - If neither are supplied, we fallback to `"info"`.
//!
//! ## Dependency crate logs
//!
//! For dependency crates, either a single log level can be supplied, or comma separated
//! `target=level` pairs can be supplied, like `tower_lsp=debug,tokio=info`.
//!
//! Resolution strategy:
//!
//! - The environment variable `AIR_DEPENDENCY_LOG_LEVELS` is consulted.
//!
//! - The configuration variable `air.dependencyLogLevels` is consulted. It can be set in
//!   the same ways as `air.logLevel` above.
//!
//! - If neither are supplied, we fallback to no logging for dependency crates.
//!
//! ## IDE support
//!
//! For VS Code and Zed, which are known to support `window/logMessage` well, logging will
//! emit a `window/logMessage` message. Otherwise, logging will write to `stderr`,
//! which should appear in the logs for most LSP clients.
use core::str;
use serde::Deserialize;
use std::fmt::Display;
use std::io::{Error as IoError, ErrorKind, Write};
use std::str::FromStr;
use tokio::sync::mpsc::unbounded_channel;
use tower_lsp::Client;
use tower_lsp::lsp_types::ClientInfo;
use tower_lsp::lsp_types::MessageType;
use tracing_subscriber::filter;
use tracing_subscriber::fmt::TestWriter;
use tracing_subscriber::fmt::time::LocalTime;
use tracing_subscriber::reload;
use tracing_subscriber::{
    fmt::{MakeWriter, writer::BoxMakeWriter},
    layer::SubscriberExt,
};

const AIR_LOG_LEVEL: &str = "AIR_LOG_LEVEL";
const AIR_DEPENDENCY_LOG_LEVELS: &str = "AIR_DEPENDENCY_LOG_LEVELS";

pub(crate) struct LogMessage {
    contents: String,
}

pub(crate) type LogMessageSender = tokio::sync::mpsc::UnboundedSender<LogMessage>;
pub(crate) type LogMessageReceiver = tokio::sync::mpsc::UnboundedReceiver<LogMessage>;

pub(crate) struct LogThreadState {
    client: Client,
    log_rx: LogMessageReceiver,
}

// Needed for spawning the loop
unsafe impl Sync for LogThreadState {}

impl LogThreadState {
    pub(crate) fn new(client: Client) -> (Self, LogMessageSender) {
        let (log_tx, log_rx) = unbounded_channel::<LogMessage>();
        let state = Self { client, log_rx };
        (state, log_tx)
    }

    /// Start the log loop
    ///
    /// Takes ownership of log state and start the low-latency log loop.
    ///
    /// We use `MessageType::LOG` to prevent the middleware from adding its own
    /// timestamp and log level labels. We add that ourselves through tracing.
    pub(crate) async fn start(mut self) {
        while let Some(message) = self.log_rx.recv().await {
            self.client
                .log_message(MessageType::LOG, message.contents)
                .await
        }

        // Channel has been closed.
        // All senders have been dropped or `close()` was called.
    }
}

// A log writer that uses LSPs logMessage method.
struct LogWriter<'a> {
    log_tx: &'a LogMessageSender,
}

impl<'a> LogWriter<'a> {
    fn new(log_tx: &'a LogMessageSender) -> Self {
        Self { log_tx }
    }
}

impl Write for LogWriter<'_> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let contents = str::from_utf8(buf).map_err(|e| IoError::new(ErrorKind::InvalidData, e))?;
        let contents = contents.to_string();

        // Forward the log message to the latency sensitive log thread,
        // which is in charge of forwarding to the client in an async manner.
        self.log_tx
            .send(LogMessage { contents })
            .map_err(IoError::other)?;

        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

struct LogWriterMaker {
    log_tx: LogMessageSender,
}

impl LogWriterMaker {
    fn new(log_tx: LogMessageSender) -> Self {
        Self { log_tx }
    }
}

impl<'a> MakeWriter<'a> for LogWriterMaker {
    type Writer = LogWriter<'a>;

    fn make_writer(&'a self) -> Self::Writer {
        LogWriter::new(&self.log_tx)
    }
}

pub(crate) type LogReloadHandle = tracing_subscriber::reload::Handle<
    tracing_subscriber::filter::Targets,
    tracing_subscriber::Registry,
>;

/// Log state managed by the LSP
///
/// This state allows for dynamically updating the log level at runtime using [`reload`].
///
/// It works using an `RwLock` that's only ever locked by us inside `self.handle.reload()`,
/// this seems to be fast enough even though atomics are checked at every log call site.
/// Notably we only lock if the new log levels are actually different, which is important
/// since we call [`reload`] when ANY configuration changes.
pub(crate) struct LogState {
    handle: LogReloadHandle,

    /// The log level as provided by the client, before any extra processing is done.
    /// Used to check if an update is required.
    log_level: Option<LogLevel>,

    /// The dependency log levels as provided by the client, before any extra processing is done
    /// Used to check if an update is required.
    dependency_log_levels: Option<String>,
}

impl LogState {
    pub(crate) fn reload(
        &mut self,
        log_level: Option<LogLevel>,
        dependency_log_levels: Option<String>,
    ) {
        if (self.log_level == log_level) && (self.dependency_log_levels == dependency_log_levels) {
            // Nothing changed
            return;
        }

        let (filter, message) = log_filter(log_level, dependency_log_levels.clone());

        match self.handle.reload(filter) {
            Ok(()) => {
                // Update to match the new filter
                tracing::info!("{message}");
                self.log_level = log_level;
                self.dependency_log_levels = dependency_log_levels;
            }
            Err(error) => {
                // Log and return without updating our internal log level
                tracing::error!("Failed to update log level: {error}");
            }
        }
    }
}

pub(crate) fn init_logging(
    log_tx: LogMessageSender,
    log_level: Option<LogLevel>,
    dependency_log_levels: Option<String>,
    client_info: Option<&ClientInfo>,
) -> LogState {
    let (filter, message) = log_filter(log_level, dependency_log_levels.clone());
    let (filter_layer, handle) = reload::Layer::new(filter);

    let writer = if client_info.is_some_and(|client_info| {
        client_info.name.starts_with("Zed") || client_info.name.starts_with("Visual Studio Code")
    }) {
        // These IDEs are known to support `window/logMessage` well
        BoxMakeWriter::new(LogWriterMaker::new(log_tx))
    } else if is_test_client(client_info) {
        // Ensures a standard `cargo nextest run` captures output unless `--no-capture` is used
        BoxMakeWriter::new(TestWriter::default())
    } else {
        // Fallback for other editors / IDEs
        BoxMakeWriter::new(std::io::stderr)
    };

    let writer_layer = tracing_subscriber::fmt::layer()
        // Spend the effort cleaning up the logs before writing them.
        // Particularly useful for instrumented logs with spans.
        .pretty()
        // Disable ANSI escapes, those are not supported in Code
        .with_ansi(false)
        // Display source code file paths
        .with_file(true)
        // Display source code line numbers
        .with_line_number(true)
        // Don't display the thread ID or thread name
        .with_thread_ids(false)
        .with_thread_names(false)
        // Don't display the event's target (module path).
        // Mostly redundant with file paths.
        .with_target(false)
        // Display local time rather than UTC
        .with_timer(LocalTime::rfc_3339())
        // Display the log level
        .with_level(true)
        .with_writer(writer);

    let subscriber = tracing_subscriber::Registry::default()
        .with(filter_layer)
        .with(writer_layer);

    tracing::subscriber::set_global_default(subscriber)
        .expect("Should be able to set the global subscriber exactly once.");

    // Emit message after subscriber is set up, so we actually see it
    tracing::info!("{message}");

    LogState {
        handle,
        log_level,
        dependency_log_levels,
    }
}

/// We use a special `TestWriter` during tests to be compatible with `cargo nextest run`'s
/// typical output capturing behavior (even during integration tests!).
///
/// Importantly, note that `cargo nextest run` swallows all logs for passing tests unless
/// you use `--no-capture`, which is the correct expected behavior.
fn is_test_client(client_info: Option<&ClientInfo>) -> bool {
    client_info.is_some_and(|client_info| client_info.name == "AirTestClient")
}

fn log_filter(
    log_level: Option<LogLevel>,
    dependency_log_levels: Option<String>,
) -> (filter::Targets, String) {
    let log_level = resolve_log_level(log_level);
    let dependency_log_levels = resolve_dependency_log_levels(dependency_log_levels);

    // Create the update message with resolved levels, it will get logged at the
    // appropriate time by the caller
    let message = format!(
        "Updating log level:
Log level: {log_level}
Dependency log levels: {dependency_log_levels:?}"
    );

    // Initialize `filter` from dependency log levels.
    // If nothing is supplied, dependency logs are completely off.
    let mut filter = match dependency_log_levels {
        Some(dependency_log_levels) => match filter::Targets::from_str(&dependency_log_levels) {
            Ok(level) => level,
            Err(_) => filter::Targets::new(),
        },
        None => filter::Targets::new(),
    };

    let log_level = log_level.tracing_level();

    // Apply the air log level to each air crate that logs
    for target in crates::AIR_CRATE_NAMES {
        filter = filter.with_target(*target, log_level);
    }

    (filter, message)
}

fn resolve_log_level(log_level: Option<LogLevel>) -> LogLevel {
    // Check log environment variable, this overrides any Client options
    if let Some(log_level) = std::env::var(AIR_LOG_LEVEL)
        .ok()
        .and_then(|level| serde_json::from_value(serde_json::Value::String(level)).ok())
    {
        return log_level;
    }

    // Client specified log level through initialization parameters
    if let Some(log_level) = log_level {
        return log_level;
    }

    // Default to info logs for air crates
    LogLevel::Info
}

fn resolve_dependency_log_levels(dependency_log_levels: Option<String>) -> Option<String> {
    // Check dependency log environment variable, this overrides any Client options
    if let Ok(dependency_log_levels) = std::env::var(AIR_DEPENDENCY_LOG_LEVELS) {
        return Some(dependency_log_levels);
    }

    // Client specified log level through initialization parameters
    if dependency_log_levels.is_some() {
        return dependency_log_levels;
    }

    // Default to no logs for dependency crates
    None
}

#[derive(Clone, Copy, Debug, Deserialize, Default, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "lowercase")]
pub(crate) enum LogLevel {
    Error,
    Warn,
    #[default]
    Info,
    Debug,
    Trace,
}

impl LogLevel {
    fn tracing_level(self) -> tracing::Level {
        match self {
            Self::Error => tracing::Level::ERROR,
            Self::Warn => tracing::Level::WARN,
            Self::Info => tracing::Level::INFO,
            Self::Debug => tracing::Level::DEBUG,
            Self::Trace => tracing::Level::TRACE,
        }
    }
}

impl Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Error => f.write_str("Error"),
            Self::Warn => f.write_str("Warn"),
            Self::Info => f.write_str("Info"),
            Self::Debug => f.write_str("Debug"),
            Self::Trace => f.write_str("Trace"),
        }
    }
}
