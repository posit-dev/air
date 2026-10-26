use std::fmt::Display;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::process::ExitStatus;

pub trait CommandExt {
    /// Executes the command as a child process, waiting for it to finish and collecting all of its output.
    ///
    /// Like [Command::output], but also collects arguments
    ///
    /// The [Output] has a suitable [Display] method for capturing with insta
    fn run(&mut self) -> Output;

    /// Like [CommandExt::run], but with stdin support
    fn run_with_stdin(&mut self, stdin: String) -> Output;
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

    /// Remove `arguments`, useful when absolute paths are provided,
    /// which would capture the caller's machine specific paths
    pub fn remove_arguments(self) -> Self {
        Self {
            status: self.status,
            stdout: self.stdout,
            stderr: self.stderr,
            arguments: String::from("<removed>"),
        }
    }

    /// Replace an `air.toml` path in stderr with `[AIR_TOML]`, and redraw the `━` rule
    /// of its diagnostic header, whose length depends on the length of the path
    pub fn replace_air_toml_path(self, path: &Path) -> Self {
        // Biome fills the header with `━` up to this width, with at least `MIN_RULE`
        const HEADER_WIDTH: usize = 100;
        const MIN_RULE: usize = 10;

        let stderr = self
            .stderr
            .replace(&path.display().to_string(), "[AIR_TOML]")
            .split_inclusive('\n')
            .map(|line| {
                let header = line.trim_end_matches('\n');
                if !header.starts_with("[AIR_TOML]") || !header.ends_with('━') {
                    return line.to_string();
                }
                let text = header.trim_end_matches('━');
                let rule = HEADER_WIDTH
                    .saturating_sub(text.chars().count())
                    .max(MIN_RULE);
                format!("{text}{}\n", "━".repeat(rule))
            })
            .collect();

        Self {
            status: self.status,
            stdout: self.stdout,
            stderr,
            arguments: self.arguments,
        }
    }
}

impl CommandExt for Command {
    fn run(&mut self) -> Output {
        // Augment `std::process::Output` with the arguments
        let output = self.output().unwrap();

        // Augment `std::process::Output` with the arguments
        Output::from_command_output_and_arguments(output, self.get_args())
    }

    fn run_with_stdin(&mut self, stdin: String) -> Output {
        let mut child = self.spawn().unwrap();

        let mut handle = child.stdin.take().unwrap();
        std::thread::spawn(move || {
            handle.write_all(stdin.as_bytes()).unwrap();
        });

        let output = child.wait_with_output().unwrap();

        // Augment `std::process::Output` with the arguments
        Output::from_command_output_and_arguments(output, self.get_args())
    }
}

impl Output {
    fn from_command_output_and_arguments(
        output: std::process::Output,
        arguments: std::process::CommandArgs,
    ) -> Self {
        let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
        let stderr = String::from_utf8_lossy(&output.stderr).into_owned();

        let arguments: Vec<String> = arguments
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
