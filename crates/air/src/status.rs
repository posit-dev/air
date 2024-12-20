use std::process::ExitCode;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum ExitStatus {
    /// Successful and there were no errors.
    Success,
    /// Successful but there were errors.
    Failure,
    /// Failed.
    Error,
}

impl From<ExitStatus> for ExitCode {
    fn from(status: ExitStatus) -> Self {
        match status {
            ExitStatus::Success => ExitCode::from(0),
            ExitStatus::Failure => ExitCode::from(1),
            ExitStatus::Error => ExitCode::from(u8::MAX),
        }
    }
}
