//! ## The Ruff Language Server

pub use server::Server;

#[macro_use]
mod message;

mod crates;
mod edit;
mod error;
mod logging;
mod proto;
mod server;

mod session;
#[cfg(test)]
mod test;

pub(crate) const SERVER_NAME: &str = "Air Language Server";
pub(crate) const SERVER_VERSION: &str = env!("CARGO_PKG_VERSION");

/// A common result type used in most cases where a
/// result type is needed.
pub(crate) type Result<T> = anyhow::Result<T>;