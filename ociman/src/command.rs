use std::ffi::OsStr;

#[derive(Debug, thiserror::Error)]
#[error("Command execution failed: io_error={io_error:?}, exit_status={exit_status:?}")]
pub struct CommandError {
    pub io_error: Option<std::io::Error>,
    pub exit_status: Option<std::process::ExitStatus>,
}

/// Which stream to capture from a command.
#[derive(Clone, Copy)]
enum CaptureStream {
    Stdout,
    Stderr,
}

/// Builder for capturing command output.
pub struct Capture {
    command: Command,
    stream: CaptureStream,
}

impl Capture {
    fn new(command: Command, stream: CaptureStream) -> Self {
        Self { command, stream }
    }

    /// Execute the command and return captured output as bytes.
    pub fn bytes(mut self) -> Result<Vec<u8>, CommandError> {
        use std::io::Write;
        use std::process::Stdio;

        log::debug!("{:#?}", self.command.inner);

        self.command.inner.stdout(Stdio::piped());
        self.command.inner.stderr(Stdio::piped());

        if self.command.stdin_data.is_some() {
            self.command.inner.stdin(Stdio::piped());
        }

        let mut child = self
            .command
            .inner
            .spawn()
            .map_err(|io_error| CommandError {
                io_error: Some(io_error),
                exit_status: None,
            })?;

        if let Some(stdin_data) = self.command.stdin_data {
            child
                .stdin
                .take()
                .unwrap()
                .write_all(&stdin_data)
                .map_err(|io_error| CommandError {
                    io_error: Some(io_error),
                    exit_status: None,
                })?;
        }

        let output = child.wait_with_output().map_err(|io_error| CommandError {
            io_error: Some(io_error),
            exit_status: None,
        })?;

        if output.status.success() {
            Ok(match self.stream {
                CaptureStream::Stdout => output.stdout,
                CaptureStream::Stderr => output.stderr,
            })
        } else {
            Err(CommandError {
                io_error: None,
                exit_status: Some(output.status),
            })
        }
    }

    /// Execute the command and return captured output as a UTF-8 string.
    pub fn string(self) -> Result<String, CommandError> {
        let bytes = self.bytes()?;
        String::from_utf8(bytes).map_err(|utf8_error| CommandError {
            io_error: Some(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                utf8_error,
            )),
            exit_status: None,
        })
    }
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

    pub fn working_directory(mut self, dir: impl AsRef<std::path::Path>) -> Self {
        self.inner.current_dir(dir);
        self
    }

    pub fn env(mut self, key: impl AsRef<OsStr>, val: impl AsRef<OsStr>) -> Self {
        self.inner.env(key, val);
        self
    }

    pub fn stdin_bytes(mut self, data: Vec<u8>) -> Self {
        self.stdin_data = Some(data);
        self
    }

    /// Capture stdout from this command.
    pub fn stdout(self) -> Capture {
        Capture::new(self, CaptureStream::Stdout)
    }

    /// Capture stderr from this command.
    pub fn stderr(self) -> Capture {
        Capture::new(self, CaptureStream::Stderr)
    }

    /// Execute the command and return success or an error.
    pub fn status(mut self) -> Result<(), CommandError> {
        use std::io::Write;
        use std::process::Stdio;

        log::debug!("{:#?}", self.inner);

        if self.stdin_data.is_some() {
            self.inner.stdin(Stdio::piped());
        }

        let mut child = self.inner.spawn().map_err(|io_error| CommandError {
            io_error: Some(io_error),
            exit_status: None,
        })?;

        if let Some(stdin_data) = self.stdin_data {
            child
                .stdin
                .take()
                .unwrap()
                .write_all(&stdin_data)
                .map_err(|io_error| CommandError {
                    io_error: Some(io_error),
                    exit_status: None,
                })?;
        }

        let exit_status = child.wait().map_err(|io_error| CommandError {
            io_error: Some(io_error),
            exit_status: None,
        })?;

        if exit_status.success() {
            Ok(())
        } else {
            Err(CommandError {
                io_error: None,
                exit_status: Some(exit_status),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stdout_bytes_success() {
        assert_eq!(
            Command::new("echo")
                .argument("hello")
                .stdout()
                .bytes()
                .unwrap(),
            b"hello\n"
        );
    }

    #[test]
    fn test_stdout_bytes_nonzero_exit() {
        let error = Command::new("sh")
            .arguments(["-c", "exit 42"])
            .stdout()
            .bytes()
            .unwrap_err();
        assert_eq!(
            error.exit_status.map(|status| status.code()),
            Some(Some(42))
        );
        assert!(error.io_error.is_none());
    }

    #[test]
    fn test_stdout_bytes_io_error() {
        let error = Command::new("./nonexistent").stdout().bytes().unwrap_err();
        assert!(error.io_error.is_some());
        assert_eq!(error.io_error.unwrap().kind(), std::io::ErrorKind::NotFound);
        assert!(error.exit_status.is_none());
    }

    #[test]
    fn test_stdout_string_success() {
        assert_eq!(
            Command::new("echo")
                .argument("hello")
                .stdout()
                .string()
                .unwrap(),
            "hello\n"
        );
    }

    #[test]
    fn test_stderr_bytes_success() {
        assert_eq!(
            Command::new("sh")
                .arguments(["-c", "echo error >&2"])
                .stderr()
                .bytes()
                .unwrap(),
            b"error\n"
        );
    }

    #[test]
    fn test_stderr_string_success() {
        assert_eq!(
            Command::new("sh")
                .arguments(["-c", "echo error >&2"])
                .stderr()
                .string()
                .unwrap(),
            "error\n"
        );
    }

    #[test]
    fn test_status_success() {
        assert!(matches!(Command::new("true").status(), Ok(())));
    }

    #[test]
    fn test_status_nonzero_exit() {
        let error = Command::new("sh")
            .arguments(["-c", "exit 42"])
            .status()
            .unwrap_err();
        assert_eq!(
            error.exit_status.map(|status| status.code()),
            Some(Some(42))
        );
        assert!(error.io_error.is_none());
    }

    #[test]
    fn test_status_io_error() {
        let error = Command::new("./nonexistent").status().unwrap_err();
        assert!(error.io_error.is_some());
        assert_eq!(error.io_error.unwrap().kind(), std::io::ErrorKind::NotFound);
        assert!(error.exit_status.is_none());
    }
}
