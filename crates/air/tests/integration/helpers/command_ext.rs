use std::fmt::Display;
use std::process::Command;
use std::process::ExitStatus;

pub trait CommandExt {
    /// Executes the command as a child process, waiting for it to finish and collecting all of its output.
    ///
    /// Like [Command::output], but also collects arguments
    ///
    /// The [Output] has a suitable [Display] method for capturing with insta
    fn run(&mut self) -> Output;
}

/// Like [std::process::Output], but augmented with `arguments`
pub struct Output {
    pub status: ExitStatus,
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
    pub arguments: Vec<String>,
}

impl CommandExt for Command {
    fn run(&mut self) -> Output {
        // Augment `std::process::Output` with the arguments
        let output = self.output().unwrap();

        let arguments: Vec<String> = self
            .get_args()
            .map(|x| x.to_string_lossy().into_owned())
            .collect();

        Output {
            status: output.status,
            stdout: output.stdout,
            stderr: output.stderr,
            arguments,
        }
    }
}

impl Display for Output {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let stdout = String::from_utf8_lossy(&self.stdout);
        let stderr = String::from_utf8_lossy(&self.stderr);
        let arguments = self.arguments.join(" ");

        // Normalize all `\` to `/` for snapshot stability
        let stdout = stdout.replace('\\', "/");
        let stderr = stderr.replace('\\', "/");
        let arguments = arguments.replace('\\', "/");

        // Replace Windows help documentation's `air.exe` with `air` for snapshot stability
        let stdout = stdout.replace("air.exe", "air");
        let stderr = stderr.replace("air.exe", "air");

        f.write_fmt(format_args!(
            "
success: {:?}
exit_code: {}
----- stdout -----
{}
----- stderr -----
{}
----- args -----
{}",
            self.status.success(),
            self.status.code().unwrap_or(1),
            stdout,
            stderr,
            arguments,
        ))
    }
}
