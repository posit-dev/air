use std::num::NonZeroUsize;

use ruff_server::Server;

use crate::args::LanguageServerCommand;
use crate::ExitStatus;

pub(crate) fn language_server(_command: LanguageServerCommand) -> anyhow::Result<ExitStatus> {
    let four = NonZeroUsize::new(4).unwrap();

    // by default, we set the number of worker threads to `num_cpus`, with a maximum of 4.
    let worker_threads = std::thread::available_parallelism()
        .unwrap_or(four)
        .max(four);

    let (connection, connection_threads) = lsp_server::Connection::stdio();

    let server = Server::new(worker_threads, connection, Some(connection_threads))?;
    server.run().map(|()| ExitStatus::Success)
}
