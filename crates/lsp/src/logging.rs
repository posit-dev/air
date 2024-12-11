// --- source
// authors = ["Charlie Marsh"]
// license = "MIT"
// origin = "https://github.com/astral-sh/ruff/blob/03fb2e5ac1481e498f84474800b42a966e9843e1/crates/ruff_server/src/trace.rs"
// ---

//! The logging system for `air lsp`.
//!
//! Logs are controlled by the `air.logLevel` setting in VS Code,
//! passed through `InitializeParams` in the arbitrary `initializationOptions` field.
//!
//! Logs are also controlled by the `AIR_LOG` environment variable. This is preferred
//! over the extension setting, but it is unlikely to see them both be used together.
//! This can be used by other LSP clients if there is no easy way to set `initializationOptions`.
//!
//! By default, we fallback to `"info"`.
//!
//! For VS Code and Zed, which are known to support `window/logMessage` well, logging will
//! emit a `window/logMessage` message. Otherwise, logging will write to `stderr`,
//! which should appear in the logs for most LSP clients.
use core::str;
use serde::Deserialize;
use std::fmt::Display;
use std::io::{Error as IoError, ErrorKind, Write};
use tokio::sync::mpsc::unbounded_channel;
use tower_lsp::lsp_types::ClientInfo;
use tower_lsp::lsp_types::MessageType;
use tower_lsp::Client;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::fmt::time::LocalTime;
use tracing_subscriber::{
    fmt::{writer::BoxMakeWriter, MakeWriter},
    layer::SubscriberExt,
    Layer,
};

// TODO:
// - Add `air.logLevel` as a VS Code extension option that sets the log level,
//   and pass it through the arbitrary `initializationOptions` field of `InitializeParams`.

const LOG_ENV_KEY: &str = "AIR_LOG";

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
    client_info: Option<&ClientInfo>,
) {
    let log_level = resolve_log_level(log_level);

    let writer = if client_info.is_some_and(|client_info| {
        client_info.name.starts_with("Zed") || client_info.name.starts_with("Visual Studio Code")
    }) {
        BoxMakeWriter::new(LogWriterMaker::new(log_tx))
    } else {
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
        .with_filter(LogLevelFilter { filter: log_level });

    let subscriber = tracing_subscriber::Registry::default().with(layer);

    if !is_test_client(client_info) {
        tracing::subscriber::set_global_default(subscriber)
            .expect("Should be able to set the global subscriber.");
    }

    tracing::info!("Logging initialized with level: {log_level}");
}

/// We never log during tests as tests run in parallel within a single process,
/// but you can only have 1 global subscriber per process.
///
/// If you are debugging a single test, you can override this to emit messages to stderr.
///
/// Note that if you override this and run multiple tests in parallel, then the call
/// to `set_global_default()` will error causing a panic.
fn is_test_client(client_info: Option<&ClientInfo>) -> bool {
    client_info.map_or(false, |client_info| client_info.name == "AirTestClient")
}

fn resolve_log_level(log_level: Option<LogLevel>) -> LogLevel {
    // Check log environment variable, this overrides any Client options
    if let Some(log_level) = std::env::var(LOG_ENV_KEY)
        .ok()
        .and_then(|level| serde_json::from_value(serde_json::Value::String(level)).ok())
    {
        return log_level;
    }

    // Client specified log level through initialization parameters
    if let Some(log_level) = log_level {
        return log_level;
    }

    // Fallback
    LogLevel::default()
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

/// Filters out logs which have a log level lower than the level set by the client.
struct LogLevelFilter {
    filter: LogLevel,
}

impl<S> tracing_subscriber::layer::Filter<S> for LogLevelFilter {
    fn enabled(
        &self,
        meta: &tracing::Metadata<'_>,
        _: &tracing_subscriber::layer::Context<'_, S>,
    ) -> bool {
        // This is a potential reason to use `air_` as the crate prefix,
        // it would make it easy to set the `tracing_level()` for only air related crates
        let filter = if meta.target().starts_with("air") || meta.target().starts_with("lsp") {
            self.filter.tracing_level()
        } else {
            tracing::Level::INFO
        };

        meta.level() <= &filter
    }

    fn max_level_hint(&self) -> Option<tracing::level_filters::LevelFilter> {
        Some(LevelFilter::from_level(self.filter.tracing_level()))
    }
}
