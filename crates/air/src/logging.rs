use std::fmt::Display;
use std::str::FromStr;
use tracing_subscriber::Layer;
use tracing_subscriber::filter;
use tracing_subscriber::layer::SubscriberExt;

pub(crate) fn init_logging(log_level: LogLevel, color: bool) {
    let log_level = log_level.tracing_level();

    // Apply the log level to each air crate.
    // We don't report any logs from non-air crates in the CLI.
    let mut filter = filter::Targets::new();
    for target in crates::AIR_CRATE_NAMES {
        filter = filter.with_target(*target, log_level);
    }

    let layer = tracing_subscriber::fmt::layer()
        // Apply coloring decision
        .with_ansi(color)
        // i.e. Displaying `ERROR` or `WARN`
        .with_level(true)
        // Don't show the module name, not useful in a cli
        .with_target(false)
        // Don't show the timestamp, not useful in a cli
        .without_time()
        .with_writer(std::io::stderr)
        .with_filter(filter);

    let subscriber = tracing_subscriber::Registry::default().with(layer);

    tracing::subscriber::set_global_default(subscriber)
        .expect("Should be able to set the global subscriber exactly once.");

    // Emit message after subscriber is set up, so we actually see it
    tracing::trace!("Initialized logging");
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, clap::ValueEnum)]
pub(crate) enum LogLevel {
    Error,
    #[default]
    Warn,
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

impl FromStr for LogLevel {
    type Err = anyhow::Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "error" => Ok(LogLevel::Error),
            "warn" => Ok(LogLevel::Warn),
            "info" => Ok(LogLevel::Info),
            "debug" => Ok(LogLevel::Debug),
            "trace" => Ok(LogLevel::Trace),
            value => Err(anyhow::anyhow!("Can't parse log level from '{value}'.")),
        }
    }
}
