/// A tool for collecting multiple anyhow errors into a single [`anyhow::Result`]
///
/// Only applicable if the intended `Ok()` value at the end is `()`.
#[derive(Debug, Default)]
pub(crate) struct ErrorVec {
    errors: Vec<anyhow::Error>,
}

impl ErrorVec {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    /// Conditionally push to the error vector if the `result` is an `Err` case
    pub(crate) fn push_err<T>(&mut self, result: anyhow::Result<T>) {
        match result {
            Ok(_) => (),
            Err(error) => self.push(error),
        }
    }

    /// Push a new error to the error vector
    pub(crate) fn push(&mut self, error: anyhow::Error) {
        self.errors.push(error);
    }

    /// Convert a error vector into a single [`anyhow::Result`] that knows how to print
    /// each of the individual errors
    pub(crate) fn into_result(self) -> anyhow::Result<()> {
        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(anyhow::anyhow!(self))
        }
    }
}

impl std::error::Error for ErrorVec {}

impl std::fmt::Display for ErrorVec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.errors.len() > 1 {
            f.write_str("Multiple errors:\n")?;
        }

        for error in &self.errors {
            std::fmt::Display::fmt(error, f)?;
        }

        Ok(())
    }
}
