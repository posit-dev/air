// --- source
// authors = ["Charlie Marsh"]
// license = "MIT"
// origin = "https://github.com/astral-sh/ruff/blob/03fb2e5ac1481e498f84474800b42a966e9843e1/crates/ruff_server/src/trace.rs"
// ---

//! The logging system for `air lsp`.
//!
//! Logs are controlled by the `air.server.log` setting in VS Code,
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
use lsp_types::ClientInfo;
use serde::Deserialize;
use std::io::{Error as IoError, ErrorKind, Write};
use tower_lsp::lsp_types;
use tower_lsp::lsp_types::MessageType;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::fmt::time::LocalTime;
use tracing_subscriber::{
    fmt::{writer::BoxMakeWriter, MakeWriter},
    layer::SubscriberExt,
    Layer,
};

use crate::main_loop::AuxiliaryEventSender;

// TODO:
// - Add `air.server.log` as a VS Code extension option that sets the log level,
//   and pass it through the arbitrary `initializationOptions` field of `InitializeParams`.
// - Add `AIR_LOG` environment variable that sets the log level as well, and prefer this
//   over the extension option as its the "harder" thing to set.

// A log writer that uses LSPs logMessage method.
struct LogWriter<'a> {
    auxiliary_event_tx: &'a AuxiliaryEventSender,
}

impl<'a> LogWriter<'a> {
    fn new(auxiliary_event_tx: &'a AuxiliaryEventSender) -> Self {
        Self { auxiliary_event_tx }
    }
}

impl Write for LogWriter<'_> {
    // We use `MessageType::LOG` to prevent the middleware from adding its own
    // timestamp and log level labels. We add that ourselves through tracing.
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let message = str::from_utf8(buf).map_err(|e| IoError::new(ErrorKind::InvalidData, e))?;

        self.auxiliary_event_tx
            .send(crate::main_loop::AuxiliaryEvent::Log(
                MessageType::LOG,
                message.to_string(),
            ))
            .map_err(|e| IoError::new(ErrorKind::Other, e))?;

        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

struct LogWriterMaker {
    auxiliary_event_tx: AuxiliaryEventSender,
}

impl LogWriterMaker {
    fn new(auxiliary_event_tx: AuxiliaryEventSender) -> Self {
        Self { auxiliary_event_tx }
    }
}

impl<'a> MakeWriter<'a> for LogWriterMaker {
    type Writer = LogWriter<'a>;

    fn make_writer(&'a self) -> Self::Writer {
        // We expect `make_writer_for()` to be called instead, but provide this just in case
        LogWriter::new(&self.auxiliary_event_tx)
    }
}

pub(crate) fn init_logging(
    auxiliary_event_tx: AuxiliaryEventSender,
    log_level: LogLevel,
    client_info: &Option<ClientInfo>,
) {
    let writer = if client_info.as_ref().is_some_and(|client_info| {
        client_info.name.starts_with("Zed") || client_info.name.starts_with("Visual Studio Code")
    }) {
        BoxMakeWriter::new(LogWriterMaker::new(auxiliary_event_tx))
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

    tracing::subscriber::set_global_default(subscriber)
        .expect("Should be able to set global default subscriber");
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
