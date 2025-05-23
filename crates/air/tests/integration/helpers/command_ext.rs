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

/// Like [std::process::Output], but augmented with `arguments` and a few extra methods
pub struct Output {
    pub status: ExitStatus,
    pub stdout: String,
    pub stderr: String,
    pub arguments: String,
}

impl Output {
    /// Normalize path separator for cross OS snapshot stability
    pub fn normalize_os_path_separator(self) -> Self {
        Self {
            status: self.status,
            stdout: self.stdout.replace('\\', "/"),
            stderr: self.stderr.replace('\\', "/"),
            arguments: self.arguments.replace('\\', "/"),
        }
    }

    /// Normalize executable name for cross OS snapshot stability
    pub fn normalize_os_executable_name(self) -> Self {
        Self {
            status: self.status,
            stdout: self.stdout.replace("air.exe", "air"),
            stderr: self.stderr.replace("air.exe", "air"),
            arguments: self.arguments.replace("air.exe", "air"),
        }
    }
}

impl CommandExt for Command {
    fn run(&mut self) -> Output {
        // Augment `std::process::Output` with the arguments
        let output = self.output().unwrap();

        // Go ahead and turn these into `String`
        let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
        let stderr = String::from_utf8_lossy(&output.stderr).into_owned();

        let arguments: Vec<String> = self
            .get_args()
            .map(|x| x.to_string_lossy().into_owned())
            .collect();

        let arguments = arguments.join(" ");

        Output {
            status: output.status,
            stdout,
            stderr,
            arguments,
        }
    }
}

impl Display for Output {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
            self.stdout,
            self.stderr,
            self.arguments,
        ))
    }
}
