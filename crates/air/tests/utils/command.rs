use std::path::PathBuf;
use std::process::Command;

/// Run a command to completion, capturing info like its stdout and stderr
///
/// Inspired by insta_cmd, but simpler
pub fn run(command: &mut Command) -> String {
    let output = command.output().unwrap();

    let args: Vec<String> = command
        .get_args()
        .map(|x| x.to_string_lossy().into_owned())
        .collect();

    format!(
        "
success: {:?}
exit_code: {}
----- stdout -----
{}
----- stderr -----
{}
----- args -----
{}",
        output.status.success(),
        output.status.code().unwrap_or(1),
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
        args.join("\n"),
    )
}

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
pub fn get_air_bin() -> PathBuf {
    env!("CARGO_BIN_EXE_air").into()
}
