use std::ffi::OsStr;

#[derive(Debug, thiserror::Error)]
#[error("Command execution failed: io_error={io_error:?}, exit_status={exit_status:?}")]
pub struct CaptureError {
    pub io_error: Option<std::io::Error>,
    pub exit_status: Option<std::process::ExitStatus>,
}

pub struct Command {
    inner: std::process::Command,
    stdin_data: Option<Vec<u8>>,
}

impl Command {
    pub fn new(value: impl AsRef<OsStr>) -> Self {
        Command {
            inner: std::process::Command::new(value),
            stdin_data: None,
        }
    }

    pub fn argument(mut self, value: impl AsRef<OsStr>) -> Self {
        self.inner.arg(value);

        self
    }

    pub fn optional_argument(mut self, optional: Option<impl AsRef<OsStr>>) -> Self {
        match optional {
            Some(value) => {
                self.inner.arg(value);
                self
            }
            None => self,
        }
    }

    pub fn arguments<T: AsRef<OsStr>>(mut self, value: impl IntoIterator<Item = T>) -> Self {
        self.inner.args(value);

        self
    }

    pub fn stdin_bytes(mut self, data: Vec<u8>) -> Self {
        self.stdin_data = Some(data);
        self
    }

    pub fn capture_only_stdout_result(mut self) -> Result<Vec<u8>, CaptureError> {
        log::debug!("{:#?}", self.inner);

        // Command::output sadly also captures stderr which we do not want in this case.
        self.inner.stdout(std::process::Stdio::piped());

        // Configure stdin if we have data to send
        if self.stdin_data.is_some() {
            self.inner.stdin(std::process::Stdio::piped());
        }

        match self.inner.spawn() {
            Ok(mut child) => {
                let mut io_error = None;
                let mut buf = vec![];

                // Write stdin data if present
                if let Some(data) = self.stdin_data
                    && let Some(mut stdin) = child.stdin.take()
                {
                    use std::io::Write;
                    if let Err(e) = stdin.write_all(&data) {
                        io_error = Some(e);
                    }
                    // stdin is dropped here, closing the pipe
                }

                // Read stdout if no previous IO error
                if io_error.is_none() {
                    let mut stdout = child.stdout.as_mut().unwrap();
                    if let Err(e) = std::io::Read::read_to_end(&mut stdout, &mut buf) {
                        io_error = Some(e);
                    }
                }

                let status = child.wait().unwrap();

                // Success case: exit 0 and no IO errors
                if status.success() && io_error.is_none() {
                    return Ok(buf);
                }

                // Error case: non-zero exit or IO error (or both)
                Err(CaptureError {
                    io_error,
                    exit_status: if status.success() { None } else { Some(status) },
                })
            }
            Err(error) => Err(CaptureError {
                io_error: Some(error),
                exit_status: None,
            }),
        }
    }

    pub fn capture_only_stdout(self) -> Vec<u8> {
        self.capture_only_stdout_result().unwrap()
    }

    pub fn capture_only_stdout_string(self) -> String {
        std::str::from_utf8(&self.capture_only_stdout())
            .unwrap()
            .to_string()
    }

    pub fn status(mut self) -> std::process::ExitStatus {
        log::debug!("{:#?}", self.inner);

        match self.inner.status() {
            Ok(status) => status,
            Err(error) => panic!("Failed to run container command: {error:#?}"),
        }
    }
}
