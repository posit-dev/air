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
//! - The LSP `InitializeParams.initializationOptions.logLevel` option is consulted. This
//!   can be set in VS Code / Positron using `air.logLevel`, or in Zed by supplying
//!   `initialization_options`.
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
//! - The LSP `InitializeParams.initializationOptions.dependencyLogLevels` option is
//!   consulted. This can be set in VS Code / Positron using `air.dependencyLogLevel`, or
//!   in Zed by supplying `initialization_options`.
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
use tower_lsp::lsp_types::ClientInfo;
use tower_lsp::lsp_types::MessageType;
use tower_lsp::Client;
use tracing_subscriber::filter;
use tracing_subscriber::fmt::time::LocalTime;
use tracing_subscriber::fmt::TestWriter;
use tracing_subscriber::{
    fmt::{writer::BoxMakeWriter, MakeWriter},
    layer::SubscriberExt,
    Layer,
};

// TODO:
// - Add `air.logLevel` and `air.dependencyLogLevels` as VS Code extension options that set
//   the log levels, and pass them through the arbitrary `initializationOptions` field of
//   `InitializeParams`.

const AIR_LOG_LEVEL: &str = "AIR_LOG_LEVEL";
const AIR_DEPENDENCY_LOG_LEVELS: &str = "AIR_DEPENDENCY_LOG_LEVELS";

pub(crate) struct LogMessage {
    contents: String,
}

pub(crate) type LogMessageSender = tokio::sync::mpsc::UnboundedSender<LogMessage>;
pub(crate) type LogMessageReceiver = tokio::sync::mpsc::UnboundedReceiver<LogMessage>;

pub(crate) struct LogState {
    client: Client,
    log_rx: LogMessageReceiver,
}

// Needed for spawning the loop
unsafe impl Sync for LogState {}

impl LogState {
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
            .map_err(|e| IoError::new(ErrorKind::Other, e))?;

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

pub(crate) fn init_logging(
    log_tx: LogMessageSender,
    log_level: Option<LogLevel>,
    dependency_log_levels: Option<String>,
    client_info: Option<&ClientInfo>,
) {
    let log_level = resolve_log_level(log_level);
    let dependency_log_levels = resolve_dependency_log_levels(dependency_log_levels);

    let writer = if client_info.is_some_and(|client_info| {
        client_info.name.starts_with("Zed") || client_info.name.starts_with("Visual Studio Code")
    }) {
        // These IDEs are known to support `window/logMessage` well
        BoxMakeWriter::new(LogWriterMaker::new(log_tx))
    } else if is_test_client(client_info) {
        // Ensures a standard `cargo test` captures output unless `-- --nocapture` is used
        BoxMakeWriter::new(TestWriter::default())
    } else {
        // Fallback for other editors / IDEs
        BoxMakeWriter::new(std::io::stderr)
    };

    let layer = tracing_subscriber::fmt::layer()
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
        .with_writer(writer)
        .with_filter(log_filter(log_level, dependency_log_levels));

    let subscriber = tracing_subscriber::Registry::default().with(layer);

    if is_test_client(client_info) {
        // During parallel testing, `set_global_default()` gets called multiple times
        // per process. That causes it to error, but we ignore this.
        tracing::subscriber::set_global_default(subscriber).ok();
    } else {
        tracing::subscriber::set_global_default(subscriber)
            .expect("Should be able to set the global subscriber exactly once.");
    }

    tracing::info!("Logging initialized with level: {log_level}");
}

/// We use a special `TestWriter` during tests to be compatible with `cargo test`'s
/// typical output capturing behavior.
///
/// Important notes:
/// - `cargo test` swallows all logs unless you use `-- --nocapture`.
/// - Tests run in parallel, so logs can be interleaved unless you run `--test-threads 1`.
///
/// We use `cargo test -- --nocapture --test-threads 1` on CI because of all of this.
fn is_test_client(client_info: Option<&ClientInfo>) -> bool {
    client_info.map_or(false, |client_info| client_info.name == "AirTestClient")
}

// TODO: Is there a way to generate this at compile time?
const TARGETS: &[&str] = &[
    "air_r_factory",
    "air_r_formatter",
    "air_r_parser",
    "air_r_syntax",
    "fs",
    "line_ending",
    "lsp",
];

fn log_filter(log_level: LogLevel, dependency_log_levels: Option<String>) -> filter::Targets {
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
    for target in TARGETS {
        filter = filter.with_target(*target, log_level);
    }

    filter
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
