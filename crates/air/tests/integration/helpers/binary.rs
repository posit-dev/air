use std::path::PathBuf;

/// Path to the `air` binary
///
/// - Defined in integration tests only, as the `air` binary isn't built in unit tests
/// - Only accessible at compile time via the `env!()` macro
///
/// From https://doc.rust-lang.org/cargo/reference/cargo-targets.html#integration-tests:
///
/// "Binary targets are automatically built if there is an integration test. This allows
/// an integration test to execute the binary to exercise and test its behavior. The
/// `CARGO_BIN_EXE_<name>` environment variable is set when the integration test is built
/// so that it can use the env macro to locate the executable."
pub fn binary_path() -> PathBuf {
    env!("CARGO_BIN_EXE_air").into()
}
