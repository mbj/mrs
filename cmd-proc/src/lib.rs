#![doc = include_str!("../README.md")]

use std::borrow::Cow;
use std::ffi::OsStr;
use std::marker::PhantomData;

#[derive(Debug, thiserror::Error)]
#[error("Command execution failed: io_error={io_error:?}, exit_status={exit_status:?}")]
pub struct CommandError {
    pub io_error: Option<std::io::Error>,
    pub exit_status: Option<std::process::ExitStatus>,
}

async fn write_stdin(
    child: &mut tokio::process::Child,
    stdin_data: Option<Vec<u8>>,
) -> Result<(), CommandError> {
    use tokio::io::AsyncWriteExt;

    if let Some(data) = stdin_data {
        child
            .stdin
            .take()
            .unwrap()
            .write_all(&data)
            .await
            .map_err(|io_error| CommandError {
                io_error: Some(io_error),
                exit_status: None,
            })?;
    }

    Ok(())
}

async fn run_and_wait(
    mut child: tokio::process::Child,
    stdin_data: Option<Vec<u8>>,
    start: std::time::Instant,
) -> Result<std::process::Output, CommandError> {
    write_stdin(&mut child, stdin_data).await?;

    let output = child
        .wait_with_output()
        .await
        .map_err(|io_error| CommandError {
            io_error: Some(io_error),
            exit_status: None,
        })?;

    log::debug!(
        "exit_status={:?} runtime={:?}",
        output.status,
        start.elapsed()
    );

    Ok(output)
}

async fn run_and_wait_status(
    mut child: tokio::process::Child,
    stdin_data: Option<Vec<u8>>,
    start: std::time::Instant,
) -> Result<std::process::ExitStatus, CommandError> {
    write_stdin(&mut child, stdin_data).await?;

    let status = child.wait().await.map_err(|io_error| CommandError {
        io_error: Some(io_error),
        exit_status: None,
    })?;

    log::debug!("exit_status={:?} runtime={:?}", status, start.elapsed());

    Ok(status)
}

/// Validated environment variable name.
///
/// Ensures the name is non-empty and does not contain `=`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct EnvVariableName<'a>(Cow<'a, str>);

impl EnvVariableName<'_> {
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<OsStr> for EnvVariableName<'_> {
    fn as_ref(&self) -> &OsStr {
        self.0.as_ref().as_ref()
    }
}

impl std::fmt::Display for EnvVariableName<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl EnvVariableName<'static> {
    /// Validated environment variable name for `'static` inputs.
    ///
    /// # Panics
    ///
    /// Panics at compile time when used in a `const` context, or at runtime otherwise,
    /// if the name is empty or contains `=`.
    #[must_use]
    pub const fn from_static_or_panic(s: &'static str) -> Self {
        match validate_env_variable_name(s) {
            Ok(()) => {}
            Err(EnvVariableNameError::Empty) => {
                panic!("Environment variable name cannot be empty");
            }
            Err(EnvVariableNameError::ContainsEquals) => {
                panic!("Environment variable name cannot contain '='");
            }
        }
        Self(Cow::Borrowed(s))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum EnvVariableNameError {
    #[error("Environment variable name cannot be empty")]
    Empty,
    #[error("Environment variable name cannot contain '='")]
    ContainsEquals,
}

impl std::str::FromStr for EnvVariableName<'static> {
    type Err = EnvVariableNameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        validate_env_variable_name(s).map(|()| Self(Cow::Owned(s.to_string())))
    }
}

const fn validate_env_variable_name(s: &str) -> Result<(), EnvVariableNameError> {
    if s.is_empty() {
        return Err(EnvVariableNameError::Empty);
    }
    let bytes = s.as_bytes();
    let mut i = 0;
    // Iterator helpers are not const-stable yet; use a manual loop.
    while i < bytes.len() {
        if bytes[i] == b'=' {
            return Err(EnvVariableNameError::ContainsEquals);
        }
        i += 1;
    }
    Ok(())
}

mod sealed {
    pub trait Sealed {}
}

/// Marker trait for stream type parameters.
pub trait StreamMarker: sealed::Sealed {}

/// Marker type for stdout stream.
pub struct Stdout;

/// Marker type for stderr stream.
pub struct Stderr;

impl sealed::Sealed for Stdout {}
impl sealed::Sealed for Stderr {}
impl StreamMarker for Stdout {}
impl StreamMarker for Stderr {}

/// Result from capturing a single stream.
#[derive(Debug)]
pub struct CaptureSingleResult {
    pub bytes: Vec<u8>,
    pub status: std::process::ExitStatus,
}

/// Result from capturing both stdout and stderr.
#[derive(Debug)]
pub struct CaptureAllResult {
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
    pub status: std::process::ExitStatus,
}

async fn run_capture(
    mut command: Command,
    accept_nonzero_exit: bool,
) -> Result<std::process::Output, CommandError> {
    log::debug!("{:#?}", command.inner);

    if command.stdin_data.is_some() {
        command.inner.stdin(std::process::Stdio::piped());
    }

    let start = std::time::Instant::now();

    let child = command.inner.spawn().map_err(|io_error| CommandError {
        io_error: Some(io_error),
        exit_status: None,
    })?;

    let output = run_and_wait(child, command.stdin_data, start).await?;

    if accept_nonzero_exit || output.status.success() {
        Ok(output)
    } else {
        Err(CommandError {
            io_error: None,
            exit_status: Some(output.status),
        })
    }
}

/// Builder for capturing a single stream from a command.
///
/// The type parameter `S` indicates which stream is being captured:
/// `Stdout` or `Stderr`.
pub struct CaptureSingle<S: StreamMarker> {
    inner: tokio::process::Command,
    stdin_data: Option<Vec<u8>>,
    accept_nonzero_exit: bool,
    _marker: PhantomData<S>,
}

impl<S: StreamMarker> CaptureSingle<S> {
    /// Accept non-zero exit status instead of treating it as an error.
    #[must_use]
    pub fn accept_nonzero_exit(mut self) -> Self {
        self.accept_nonzero_exit = true;
        self
    }
}

impl CaptureSingle<Stdout> {
    /// Also capture stderr, transitioning to [`CaptureAll`].
    #[must_use]
    pub fn stderr_capture(mut self) -> CaptureAll {
        self.inner.stdout(std::process::Stdio::piped());
        self.inner.stderr(std::process::Stdio::piped());
        CaptureAll {
            inner: self.inner,
            stdin_data: self.stdin_data,
            accept_nonzero_exit: self.accept_nonzero_exit,
        }
    }

    /// Redirect stderr to /dev/null.
    #[must_use]
    pub fn stderr_null(mut self) -> Self {
        self.inner.stderr(std::process::Stdio::null());
        self
    }

    /// Inherit stderr from the parent process (default).
    #[must_use]
    pub fn stderr_inherit(mut self) -> Self {
        self.inner.stderr(std::process::Stdio::inherit());
        self
    }

    /// Stop capturing stdout (null), transitioning back to [`Command`].
    #[must_use]
    pub fn stdout_null(mut self) -> Command {
        self.inner.stdout(std::process::Stdio::null());
        Command {
            inner: self.inner,
            stdin_data: self.stdin_data,
        }
    }

    /// Stop capturing stdout (inherit), transitioning back to [`Command`].
    #[must_use]
    pub fn stdout_inherit(mut self) -> Command {
        self.inner.stdout(std::process::Stdio::inherit());
        Command {
            inner: self.inner,
            stdin_data: self.stdin_data,
        }
    }

    /// Execute the command and return captured stdout.
    pub async fn run(mut self) -> Result<CaptureSingleResult, CommandError> {
        self.inner.stdout(std::process::Stdio::piped());

        let command = Command {
            inner: self.inner,
            stdin_data: self.stdin_data,
        };

        let output = run_capture(command, self.accept_nonzero_exit).await?;

        Ok(CaptureSingleResult {
            bytes: output.stdout,
            status: output.status,
        })
    }

    /// Execute the command and return captured stdout as bytes.
    pub async fn bytes(self) -> Result<Vec<u8>, CommandError> {
        Ok(self.run().await?.bytes)
    }

    /// Execute the command and return captured stdout as a UTF-8 string.
    pub async fn string(self) -> Result<String, CommandError> {
        let bytes = self.bytes().await?;
        String::from_utf8(bytes).map_err(|utf8_error| CommandError {
            io_error: Some(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                utf8_error,
            )),
            exit_status: None,
        })
    }
}

impl CaptureSingle<Stderr> {
    /// Also capture stdout, transitioning to [`CaptureAll`].
    #[must_use]
    pub fn stdout_capture(mut self) -> CaptureAll {
        self.inner.stdout(std::process::Stdio::piped());
        self.inner.stderr(std::process::Stdio::piped());
        CaptureAll {
            inner: self.inner,
            stdin_data: self.stdin_data,
            accept_nonzero_exit: self.accept_nonzero_exit,
        }
    }

    /// Redirect stdout to /dev/null.
    #[must_use]
    pub fn stdout_null(mut self) -> Self {
        self.inner.stdout(std::process::Stdio::null());
        self
    }

    /// Inherit stdout from the parent process (default).
    #[must_use]
    pub fn stdout_inherit(mut self) -> Self {
        self.inner.stdout(std::process::Stdio::inherit());
        self
    }

    /// Stop capturing stderr (null), transitioning back to [`Command`].
    #[must_use]
    pub fn stderr_null(mut self) -> Command {
        self.inner.stderr(std::process::Stdio::null());
        Command {
            inner: self.inner,
            stdin_data: self.stdin_data,
        }
    }

    /// Stop capturing stderr (inherit), transitioning back to [`Command`].
    #[must_use]
    pub fn stderr_inherit(mut self) -> Command {
        self.inner.stderr(std::process::Stdio::inherit());
        Command {
            inner: self.inner,
            stdin_data: self.stdin_data,
        }
    }

    /// Execute the command and return captured stderr.
    pub async fn run(mut self) -> Result<CaptureSingleResult, CommandError> {
        self.inner.stderr(std::process::Stdio::piped());

        let command = Command {
            inner: self.inner,
            stdin_data: self.stdin_data,
        };

        let output = run_capture(command, self.accept_nonzero_exit).await?;

        Ok(CaptureSingleResult {
            bytes: output.stderr,
            status: output.status,
        })
    }

    /// Execute the command and return captured stderr as bytes.
    pub async fn bytes(self) -> Result<Vec<u8>, CommandError> {
        Ok(self.run().await?.bytes)
    }

    /// Execute the command and return captured stderr as a UTF-8 string.
    pub async fn string(self) -> Result<String, CommandError> {
        let bytes = self.bytes().await?;
        String::from_utf8(bytes).map_err(|utf8_error| CommandError {
            io_error: Some(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                utf8_error,
            )),
            exit_status: None,
        })
    }
}

/// Builder for capturing both stdout and stderr from a command.
pub struct CaptureAll {
    inner: tokio::process::Command,
    stdin_data: Option<Vec<u8>>,
    accept_nonzero_exit: bool,
}

impl CaptureAll {
    /// Accept non-zero exit status instead of treating it as an error.
    #[must_use]
    pub fn accept_nonzero_exit(mut self) -> Self {
        self.accept_nonzero_exit = true;
        self
    }

    /// Stop capturing stdout (null), transitioning to [`CaptureSingle<Stderr>`].
    #[must_use]
    pub fn stdout_null(mut self) -> CaptureSingle<Stderr> {
        self.inner.stdout(std::process::Stdio::null());
        CaptureSingle {
            inner: self.inner,
            stdin_data: self.stdin_data,
            accept_nonzero_exit: self.accept_nonzero_exit,
            _marker: PhantomData,
        }
    }

    /// Stop capturing stdout (inherit), transitioning to [`CaptureSingle<Stderr>`].
    #[must_use]
    pub fn stdout_inherit(mut self) -> CaptureSingle<Stderr> {
        self.inner.stdout(std::process::Stdio::inherit());
        CaptureSingle {
            inner: self.inner,
            stdin_data: self.stdin_data,
            accept_nonzero_exit: self.accept_nonzero_exit,
            _marker: PhantomData,
        }
    }

    /// Stop capturing stderr (null), transitioning to [`CaptureSingle<Stdout>`].
    #[must_use]
    pub fn stderr_null(mut self) -> CaptureSingle<Stdout> {
        self.inner.stderr(std::process::Stdio::null());
        CaptureSingle {
            inner: self.inner,
            stdin_data: self.stdin_data,
            accept_nonzero_exit: self.accept_nonzero_exit,
            _marker: PhantomData,
        }
    }

    /// Stop capturing stderr (inherit), transitioning to [`CaptureSingle<Stdout>`].
    #[must_use]
    pub fn stderr_inherit(mut self) -> CaptureSingle<Stdout> {
        self.inner.stderr(std::process::Stdio::inherit());
        CaptureSingle {
            inner: self.inner,
            stdin_data: self.stdin_data,
            accept_nonzero_exit: self.accept_nonzero_exit,
            _marker: PhantomData,
        }
    }

    /// Execute the command and return captured output from both streams.
    pub async fn run(mut self) -> Result<CaptureAllResult, CommandError> {
        self.inner.stdout(std::process::Stdio::piped());
        self.inner.stderr(std::process::Stdio::piped());

        let command = Command {
            inner: self.inner,
            stdin_data: self.stdin_data,
        };

        let output = run_capture(command, self.accept_nonzero_exit).await?;

        Ok(CaptureAllResult {
            stdout: output.stdout,
            stderr: output.stderr,
            status: output.status,
        })
    }
}

pub struct Command {
    inner: tokio::process::Command,
    stdin_data: Option<Vec<u8>>,
}

impl Command {
    pub fn new(value: impl AsRef<OsStr>) -> Self {
        Command {
            inner: tokio::process::Command::new(value),
            stdin_data: None,
        }
    }

    /// Asserts that two commands are equal by comparing their `Debug` representations.
    ///
    /// This is useful for testing that a command builder produces the expected command
    /// without actually executing it.
    ///
    /// # Panics
    ///
    /// Panics if the `Debug` output of the two commands' inner `tokio::process::Command` differ.
    #[cfg(feature = "test-utils")]
    pub fn test_eq(&self, other: &Self) {
        assert_eq!(format!("{:?}", self.inner), format!("{:?}", other.inner));
    }

    pub fn argument(mut self, value: impl AsRef<OsStr>) -> Self {
        self.inner.arg(value);
        self
    }

    pub fn optional_argument(mut self, optional: Option<impl AsRef<OsStr>>) -> Self {
        if let Some(value) = optional {
            self.inner.arg(value);
        }
        self
    }

    /// Adds a flag argument only if the condition is `true`.
    ///
    /// ```rust
    /// let verbose = true;
    /// cmd_proc::Command::new("echo")
    ///     .optional_flag(verbose, "--verbose");
    /// ```
    pub fn optional_flag(mut self, condition: bool, flag: impl AsRef<OsStr>) -> Self {
        if condition {
            self.inner.arg(flag);
        }
        self
    }

    /// Adds a CLI option (name + value).
    ///
    /// ```rust
    /// cmd_proc::Command::new("git")
    ///     .option("--message", "hello");
    /// // equivalent to: .argument("--message").argument("hello")
    /// ```
    pub fn option(mut self, name: impl AsRef<OsStr>, value: impl AsRef<OsStr>) -> Self {
        self.inner.arg(name);
        self.inner.arg(value);
        self
    }

    /// Adds a CLI option (name + value) only if the value is `Some`.
    ///
    /// ```rust
    /// cmd_proc::Command::new("git")
    ///     .optional_option("--author", Some("Alice"));
    /// // adds "--author Alice" only if Some
    /// ```
    pub fn optional_option(
        mut self,
        name: impl AsRef<OsStr>,
        value: Option<impl AsRef<OsStr>>,
    ) -> Self {
        if let Some(value) = value {
            self.inner.arg(name);
            self.inner.arg(value);
        }
        self
    }

    pub fn arguments<T: AsRef<OsStr>>(mut self, value: impl IntoIterator<Item = T>) -> Self {
        self.inner.args(value);
        self
    }

    pub fn working_directory(mut self, dir: impl AsRef<std::path::Path>) -> Self {
        self.inner.current_dir(dir);
        self
    }

    pub fn env(mut self, key: &EnvVariableName<'_>, val: impl AsRef<OsStr>) -> Self {
        self.inner.env(key, val);
        self
    }

    pub fn envs<'a, I, V>(mut self, vars: I) -> Self
    where
        I: IntoIterator<Item = (EnvVariableName<'a>, V)>,
        V: AsRef<OsStr>,
    {
        for (key, val) in vars {
            self.inner.env(key, val);
        }
        self
    }

    /// Remove an environment variable from the child process.
    #[must_use]
    pub fn env_remove(mut self, key: &EnvVariableName<'_>) -> Self {
        self.inner.env_remove(key);
        self
    }

    #[must_use]
    pub fn stdin_bytes(mut self, data: impl Into<Vec<u8>>) -> Self {
        self.stdin_data = Some(data.into());
        self
    }

    /// Capture stdout from this command.
    #[must_use]
    pub fn stdout_capture(self) -> CaptureSingle<Stdout> {
        CaptureSingle {
            inner: self.inner,
            stdin_data: self.stdin_data,
            accept_nonzero_exit: false,
            _marker: PhantomData,
        }
    }

    /// Capture stderr from this command.
    #[must_use]
    pub fn stderr_capture(self) -> CaptureSingle<Stderr> {
        CaptureSingle {
            inner: self.inner,
            stdin_data: self.stdin_data,
            accept_nonzero_exit: false,
            _marker: PhantomData,
        }
    }

    /// Redirect stdout to /dev/null.
    #[must_use]
    pub fn stdout_null(mut self) -> Self {
        self.inner.stdout(std::process::Stdio::null());
        self
    }

    /// Redirect stderr to /dev/null.
    #[must_use]
    pub fn stderr_null(mut self) -> Self {
        self.inner.stderr(std::process::Stdio::null());
        self
    }

    /// Inherit stdout from the parent process (default).
    #[must_use]
    pub fn stdout_inherit(mut self) -> Self {
        self.inner.stdout(std::process::Stdio::inherit());
        self
    }

    /// Inherit stderr from the parent process (default).
    #[must_use]
    pub fn stderr_inherit(mut self) -> Self {
        self.inner.stderr(std::process::Stdio::inherit());
        self
    }

    /// Consume the builder and return the underlying `tokio::process::Command`.
    ///
    /// Use this when you need full control over the child process (e.g.
    /// interactive stdin/stdout piping) beyond what the capture API provides.
    #[must_use]
    pub fn build(self) -> tokio::process::Command {
        self.inner
    }

    /// Execute the command and return success or an error.
    pub async fn status(mut self) -> Result<(), CommandError> {
        use std::process::Stdio;

        log::debug!("{:#?}", self.inner);

        if self.stdin_data.is_some() {
            self.inner.stdin(Stdio::piped());
        }

        let start = std::time::Instant::now();

        let child = self.inner.spawn().map_err(|io_error| CommandError {
            io_error: Some(io_error),
            exit_status: None,
        })?;

        let exit_status = run_and_wait_status(child, self.stdin_data, start).await?;

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

    #[tokio::test]
    async fn test_stdout_bytes_success() {
        assert_eq!(
            Command::new("echo")
                .argument("hello")
                .stdout_capture()
                .bytes()
                .await
                .unwrap(),
            b"hello\n"
        );
    }

    #[tokio::test]
    async fn test_stdout_bytes_nonzero_exit() {
        let error = Command::new("sh")
            .arguments(["-c", "exit 42"])
            .stdout_capture()
            .bytes()
            .await
            .unwrap_err();
        assert_eq!(
            error.exit_status.map(|status| status.code()),
            Some(Some(42))
        );
        assert!(error.io_error.is_none());
    }

    #[tokio::test]
    async fn test_stdout_bytes_io_error() {
        let error = Command::new("./nonexistent")
            .stdout_capture()
            .bytes()
            .await
            .unwrap_err();
        assert!(error.io_error.is_some());
        assert_eq!(error.io_error.unwrap().kind(), std::io::ErrorKind::NotFound);
        assert!(error.exit_status.is_none());
    }

    #[tokio::test]
    async fn test_stdout_string_success() {
        assert_eq!(
            Command::new("echo")
                .argument("hello")
                .stdout_capture()
                .string()
                .await
                .unwrap(),
            "hello\n"
        );
    }

    #[tokio::test]
    async fn test_stderr_bytes_success() {
        assert_eq!(
            Command::new("sh")
                .arguments(["-c", "echo error >&2"])
                .stderr_capture()
                .bytes()
                .await
                .unwrap(),
            b"error\n"
        );
    }

    #[tokio::test]
    async fn test_stderr_string_success() {
        assert_eq!(
            Command::new("sh")
                .arguments(["-c", "echo error >&2"])
                .stderr_capture()
                .string()
                .await
                .unwrap(),
            "error\n"
        );
    }

    #[tokio::test]
    async fn test_status_success() {
        assert!(Command::new("true").status().await.is_ok());
    }

    #[tokio::test]
    async fn test_status_nonzero_exit() {
        let error = Command::new("sh")
            .arguments(["-c", "exit 42"])
            .status()
            .await
            .unwrap_err();
        assert_eq!(
            error.exit_status.map(|status| status.code()),
            Some(Some(42))
        );
        assert!(error.io_error.is_none());
    }

    #[tokio::test]
    async fn test_status_io_error() {
        let error = Command::new("./nonexistent").status().await.unwrap_err();
        assert!(error.io_error.is_some());
        assert_eq!(error.io_error.unwrap().kind(), std::io::ErrorKind::NotFound);
        assert!(error.exit_status.is_none());
    }

    #[test]
    fn test_env_variable_name_from_static_or_panic() {
        const NAME: EnvVariableName<'static> = EnvVariableName::from_static_or_panic("PATH");
        assert_eq!(NAME.as_str(), "PATH");
    }

    #[test]
    fn test_env_variable_name_parse() {
        let name: EnvVariableName = "HOME".parse().unwrap();
        assert_eq!(name.as_str(), "HOME");
    }

    #[test]
    fn test_env_variable_name_empty() {
        let result: Result<EnvVariableName, _> = "".parse();
        assert!(matches!(result, Err(EnvVariableNameError::Empty)));
    }

    #[test]
    fn test_env_variable_name_contains_equals() {
        let result: Result<EnvVariableName, _> = "FOO=BAR".parse();
        assert!(matches!(result, Err(EnvVariableNameError::ContainsEquals)));
    }

    #[tokio::test]
    async fn test_env_with_variable() {
        let name: EnvVariableName = "MY_VAR".parse().unwrap();
        let output = Command::new("sh")
            .arguments(["-c", "echo $MY_VAR"])
            .env(&name, "hello")
            .stdout_capture()
            .string()
            .await
            .unwrap();
        assert_eq!(output, "hello\n");
    }

    #[tokio::test]
    async fn test_stdin_bytes() {
        let output = Command::new("cat")
            .stdin_bytes(b"hello world".as_slice())
            .stdout_capture()
            .string()
            .await
            .unwrap();
        assert_eq!(output, "hello world");
    }

    #[tokio::test]
    async fn test_stdin_bytes_vec() {
        let output = Command::new("cat")
            .stdin_bytes(vec![104, 105])
            .stdout_capture()
            .string()
            .await
            .unwrap();
        assert_eq!(output, "hi");
    }

    #[tokio::test]
    async fn test_capture_all_success() {
        let result = Command::new("echo")
            .argument("hello")
            .stdout_capture()
            .stderr_capture()
            .run()
            .await
            .unwrap();
        assert!(result.status.success());
        assert_eq!(result.stdout, b"hello\n");
        assert!(result.stderr.is_empty());
    }

    #[tokio::test]
    async fn test_capture_all_failure_with_stderr() {
        let result = Command::new("sh")
            .arguments(["-c", "echo error >&2; exit 1"])
            .stdout_capture()
            .stderr_capture()
            .accept_nonzero_exit()
            .run()
            .await
            .unwrap();
        assert!(!result.status.success());
        assert_eq!(String::from_utf8(result.stderr).unwrap(), "error\n");
    }

    #[tokio::test]
    async fn test_capture_all_io_error() {
        let error = Command::new("./nonexistent")
            .stdout_capture()
            .stderr_capture()
            .run()
            .await
            .unwrap_err();
        assert!(error.io_error.is_some());
        assert_eq!(error.io_error.unwrap().kind(), std::io::ErrorKind::NotFound);
    }

    #[tokio::test]
    async fn test_build() {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};

        let mut child = Command::new("cat")
            .build()
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .spawn()
            .unwrap();

        child
            .stdin
            .as_mut()
            .unwrap()
            .write_all(b"hello")
            .await
            .unwrap();
        drop(child.stdin.take());

        let mut output = String::new();
        child
            .stdout
            .as_mut()
            .unwrap()
            .read_to_string(&mut output)
            .await
            .unwrap();
        assert_eq!(output, "hello");

        let status = child.wait().await.unwrap();
        assert!(status.success());
    }

    #[tokio::test]
    async fn test_option() {
        let output = Command::new("echo")
            .option("-n", "hello")
            .stdout_capture()
            .string()
            .await
            .unwrap();
        assert_eq!(output, "hello");
    }

    #[tokio::test]
    async fn test_optional_option_some() {
        let output = Command::new("echo")
            .optional_option("-n", Some("hello"))
            .stdout_capture()
            .string()
            .await
            .unwrap();
        assert_eq!(output, "hello");
    }

    #[tokio::test]
    async fn test_optional_option_none() {
        let output = Command::new("echo")
            .optional_option("-n", None::<&str>)
            .argument("hello")
            .stdout_capture()
            .string()
            .await
            .unwrap();
        assert_eq!(output, "hello\n");
    }

    #[tokio::test]
    async fn test_optional_flag_true() {
        let output = Command::new("echo")
            .optional_flag(true, "-n")
            .argument("hello")
            .stdout_capture()
            .string()
            .await
            .unwrap();
        assert_eq!(output, "hello");
    }

    #[tokio::test]
    async fn test_optional_flag_false() {
        let output = Command::new("echo")
            .optional_flag(false, "-n")
            .argument("hello")
            .stdout_capture()
            .string()
            .await
            .unwrap();
        assert_eq!(output, "hello\n");
    }

    #[tokio::test]
    async fn test_stdout_null() {
        // stdout_null should discard output; command should still succeed
        Command::new("echo")
            .argument("hello")
            .stdout_null()
            .status()
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_stderr_null() {
        // stderr_null should discard stderr; command should still succeed
        Command::new("sh")
            .arguments(["-c", "echo error >&2"])
            .stderr_null()
            .status()
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_stdout_capture_stderr_null() {
        let output = Command::new("sh")
            .arguments(["-c", "echo out; echo err >&2"])
            .stdout_capture()
            .stderr_null()
            .string()
            .await
            .unwrap();
        assert_eq!(output, "out\n");
    }

    #[tokio::test]
    async fn test_accept_nonzero_exit_stdout() {
        let result = Command::new("sh")
            .arguments(["-c", "echo out; exit 42"])
            .stdout_capture()
            .accept_nonzero_exit()
            .run()
            .await
            .unwrap();
        assert!(!result.status.success());
        assert_eq!(result.bytes, b"out\n");
    }

    #[tokio::test]
    async fn test_accept_nonzero_exit_capture_all() {
        let result = Command::new("sh")
            .arguments(["-c", "echo out; echo err >&2; exit 42"])
            .stdout_capture()
            .stderr_capture()
            .accept_nonzero_exit()
            .run()
            .await
            .unwrap();
        assert!(!result.status.success());
        assert_eq!(result.stdout, b"out\n");
        assert_eq!(result.stderr, b"err\n");
    }
}
