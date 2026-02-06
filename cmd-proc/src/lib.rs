use std::borrow::Cow;
use std::ffi::OsStr;

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

/// Output from a command execution, including both streams and exit status.
///
/// Unlike `Capture`, this does not treat non-zero exit as an error.
/// Use this when you need to inspect stderr on failure.
#[derive(Debug)]
pub struct Output {
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
    pub status: std::process::ExitStatus,
}

impl Output {
    /// Returns true if the command exited successfully.
    #[must_use]
    pub fn success(&self) -> bool {
        self.status.success()
    }

    /// Converts stdout to a UTF-8 string.
    ///
    /// # Errors
    ///
    /// Returns an error if stdout is not valid UTF-8.
    pub fn into_stdout_string(self) -> Result<String, std::string::FromUtf8Error> {
        String::from_utf8(self.stdout)
    }

    /// Converts stderr to a UTF-8 string.
    ///
    /// # Errors
    ///
    /// Returns an error if stderr is not valid UTF-8.
    pub fn into_stderr_string(self) -> Result<String, std::string::FromUtf8Error> {
        String::from_utf8(self.stderr)
    }
}

/// Which stream to capture from a command.
#[derive(Clone, Copy)]
enum CaptureSingleStream {
    Stdout,
    Stderr,
}

/// Stdio configuration for spawned processes.
#[derive(Clone, Copy, Default)]
pub enum Stdio {
    /// Pipe the stream, allowing reading/writing from the parent process.
    Piped,
    /// Inherit the stream from the parent process.
    #[default]
    Inherit,
    /// Redirect to /dev/null.
    Null,
}

impl From<Stdio> for std::process::Stdio {
    fn from(stdio: Stdio) -> Self {
        match stdio {
            Stdio::Piped => std::process::Stdio::piped(),
            Stdio::Inherit => std::process::Stdio::inherit(),
            Stdio::Null => std::process::Stdio::null(),
        }
    }
}

/// Builder for spawning a child process.
///
/// Created by [`Command::spawn`]. Configure stdin/stdout/stderr, then call [`Spawn::run`].
pub struct Spawn {
    command: Command,
    stdin: Stdio,
    stdout: Stdio,
    stderr: Stdio,
}

impl Spawn {
    fn new(command: Command) -> Self {
        Self {
            command,
            stdin: Stdio::Inherit,
            stdout: Stdio::Inherit,
            stderr: Stdio::Inherit,
        }
    }

    /// Configure stdin for the child process.
    #[must_use]
    pub fn stdin(mut self, stdio: Stdio) -> Self {
        self.stdin = stdio;
        self
    }

    /// Configure stdout for the child process.
    #[must_use]
    pub fn stdout(mut self, stdio: Stdio) -> Self {
        self.stdout = stdio;
        self
    }

    /// Configure stderr for the child process.
    #[must_use]
    pub fn stderr(mut self, stdio: Stdio) -> Self {
        self.stderr = stdio;
        self
    }

    /// Spawn the child process.
    pub fn run(mut self) -> Result<Child, CommandError> {
        log::debug!("{:#?}", self.command.inner);

        self.command.inner.stdin(self.stdin);
        self.command.inner.stdout(self.stdout);
        self.command.inner.stderr(self.stderr);

        let inner = self
            .command
            .inner
            .spawn()
            .map_err(|io_error| CommandError {
                io_error: Some(io_error),
                exit_status: None,
            })?;

        Ok(Child { inner })
    }
}

/// A running child process.
///
/// Created by [`Spawn::run`].
#[derive(Debug)]
pub struct Child {
    inner: tokio::process::Child,
}

impl Child {
    /// Returns a mutable reference to the child's stdin handle.
    pub fn stdin(&mut self) -> Option<&mut tokio::process::ChildStdin> {
        self.inner.stdin.as_mut()
    }

    /// Returns a mutable reference to the child's stdout handle.
    pub fn stdout(&mut self) -> Option<&mut tokio::process::ChildStdout> {
        self.inner.stdout.as_mut()
    }

    /// Returns a mutable reference to the child's stderr handle.
    pub fn stderr(&mut self) -> Option<&mut tokio::process::ChildStderr> {
        self.inner.stderr.as_mut()
    }

    /// Takes ownership of the child's stdin handle.
    pub fn take_stdin(&mut self) -> Option<tokio::process::ChildStdin> {
        self.inner.stdin.take()
    }

    /// Takes ownership of the child's stdout handle.
    pub fn take_stdout(&mut self) -> Option<tokio::process::ChildStdout> {
        self.inner.stdout.take()
    }

    /// Takes ownership of the child's stderr handle.
    pub fn take_stderr(&mut self) -> Option<tokio::process::ChildStderr> {
        self.inner.stderr.take()
    }

    /// Waits for the child to exit and returns its exit status.
    pub async fn wait(mut self) -> Result<std::process::ExitStatus, CommandError> {
        self.inner.wait().await.map_err(|io_error| CommandError {
            io_error: Some(io_error),
            exit_status: None,
        })
    }

    /// Simultaneously waits for the child to exit and collects all output.
    pub async fn wait_with_output(self) -> Result<Output, CommandError> {
        let output = self
            .inner
            .wait_with_output()
            .await
            .map_err(|io_error| CommandError {
                io_error: Some(io_error),
                exit_status: None,
            })?;

        Ok(Output {
            stdout: output.stdout,
            stderr: output.stderr,
            status: output.status,
        })
    }
}

/// Builder for capturing command output.
pub struct Capture {
    command: Command,
    stream: CaptureSingleStream,
    accept_nonzero_exit: bool,
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

impl Capture {
    fn new(command: Command, stream: CaptureSingleStream) -> Self {
        Self {
            command,
            stream,
            accept_nonzero_exit: false,
        }
    }

    /// Accept non-zero exit status instead of treating it as an error.
    #[must_use]
    pub fn accept_nonzero_exit(mut self) -> Self {
        self.accept_nonzero_exit = true;
        self
    }

    /// Execute the command and return captured output as bytes.
    pub async fn bytes(mut self) -> Result<Vec<u8>, CommandError> {
        use std::process::Stdio;

        let (stdout, stderr) = match self.stream {
            CaptureSingleStream::Stdout => (Stdio::piped(), Stdio::inherit()),
            CaptureSingleStream::Stderr => (Stdio::inherit(), Stdio::piped()),
        };

        self.command.inner.stdout(stdout);
        self.command.inner.stderr(stderr);

        let output = run_capture(self.command, self.accept_nonzero_exit).await?;

        Ok(match self.stream {
            CaptureSingleStream::Stdout => output.stdout,
            CaptureSingleStream::Stderr => output.stderr,
        })
    }

    /// Execute the command and return captured output as a UTF-8 string.
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
pub struct CaptureAllStreams {
    command: Command,
    accept_nonzero_exit: bool,
}

impl CaptureAllStreams {
    fn new(command: Command) -> Self {
        Self {
            command,
            accept_nonzero_exit: false,
        }
    }

    /// Accept non-zero exit status instead of treating it as an error.
    #[must_use]
    pub fn accept_nonzero_exit(mut self) -> Self {
        self.accept_nonzero_exit = true;
        self
    }

    /// Execute the command and return captured output.
    pub async fn output(mut self) -> Result<Output, CommandError> {
        use std::process::Stdio;

        self.command.inner.stdout(Stdio::piped());
        self.command.inner.stderr(Stdio::piped());

        let output = run_capture(self.command, self.accept_nonzero_exit).await?;

        Ok(Output {
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
    /// ```ignore
    /// command.optional_flag(verbose, "--verbose")
    /// // adds "--verbose" only if verbose is true
    /// ```
    pub fn optional_flag(mut self, condition: bool, flag: impl AsRef<OsStr>) -> Self {
        if condition {
            self.inner.arg(flag);
        }
        self
    }

    /// Adds a CLI option (name + value).
    ///
    /// ```ignore
    /// command.option("--message", "hello")
    /// // equivalent to: command.argument("--message").argument("hello")
    /// ```
    pub fn option(mut self, name: impl AsRef<OsStr>, value: impl AsRef<OsStr>) -> Self {
        self.inner.arg(name);
        self.inner.arg(value);
        self
    }

    /// Adds a CLI option (name + value) only if the value is `Some`.
    ///
    /// ```ignore
    /// command.optional_option("--name", optional)
    /// // adds "--name <value>" only if optional is Some
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
    pub fn capture_stdout(self) -> Capture {
        Capture::new(self, CaptureSingleStream::Stdout)
    }

    /// Capture stderr from this command.
    #[must_use]
    pub fn capture_stderr(self) -> Capture {
        Capture::new(self, CaptureSingleStream::Stderr)
    }

    /// Capture both stdout and stderr from this command.
    #[must_use]
    pub fn capture_stderr_stdout(self) -> CaptureAllStreams {
        CaptureAllStreams::new(self)
    }

    /// Spawn the command as a child process.
    ///
    /// Returns a [`Spawn`] builder to configure stdin/stdout/stderr before running.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use cmd_proc::{Command, Stdio};
    /// use tokio::io::AsyncBufReadExt;
    ///
    /// # async fn example() -> Result<(), cmd_proc::CommandError> {
    /// let mut child = Command::new("server")
    ///     .argument("--port=8080")
    ///     .spawn()
    ///     .stdin(Stdio::Piped)
    ///     .stdout(Stdio::Piped)
    ///     .stderr(Stdio::Inherit)
    ///     .run()
    ///     .unwrap();
    ///
    /// // Read a line from stdout
    /// let mut line = String::new();
    /// tokio::io::BufReader::new(child.stdout().unwrap())
    ///     .read_line(&mut line)
    ///     .await
    ///     .unwrap();
    ///
    /// // Close stdin to signal the child to exit
    /// drop(child.take_stdin());
    /// child.wait().await.unwrap();
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn spawn(self) -> Spawn {
        Spawn::new(self)
    }

    /// Execute the command and return full output regardless of exit status.
    ///
    /// Unlike `stdout()` and `stderr()`, this does not treat non-zero exit as an error.
    /// Use this when you need to inspect both streams or handle failure cases with stderr.
    pub async fn output(mut self) -> Result<Output, CommandError> {
        use std::process::Stdio;

        log::debug!("{:#?}", self.inner);

        self.inner.stdout(Stdio::piped());
        self.inner.stderr(Stdio::piped());

        if self.stdin_data.is_some() {
            self.inner.stdin(Stdio::piped());
        }

        let start = std::time::Instant::now();

        let child = self.inner.spawn().map_err(|io_error| CommandError {
            io_error: Some(io_error),
            exit_status: None,
        })?;

        let output = run_and_wait(child, self.stdin_data, start).await?;

        Ok(Output {
            stdout: output.stdout,
            stderr: output.stderr,
            status: output.status,
        })
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
                .capture_stdout()
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
            .capture_stdout()
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
            .capture_stdout()
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
                .capture_stdout()
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
                .capture_stderr()
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
                .capture_stderr()
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
            .capture_stdout()
            .string()
            .await
            .unwrap();
        assert_eq!(output, "hello\n");
    }

    #[tokio::test]
    async fn test_stdin_bytes() {
        let output = Command::new("cat")
            .stdin_bytes(b"hello world".as_slice())
            .capture_stdout()
            .string()
            .await
            .unwrap();
        assert_eq!(output, "hello world");
    }

    #[tokio::test]
    async fn test_stdin_bytes_vec() {
        let output = Command::new("cat")
            .stdin_bytes(vec![104, 105])
            .capture_stdout()
            .string()
            .await
            .unwrap();
        assert_eq!(output, "hi");
    }

    #[tokio::test]
    async fn test_output_success() {
        let output = Command::new("echo")
            .argument("hello")
            .output()
            .await
            .unwrap();
        assert!(output.success());
        assert_eq!(output.stdout, b"hello\n");
        assert!(output.stderr.is_empty());
    }

    #[tokio::test]
    async fn test_output_failure_with_stderr() {
        let output = Command::new("sh")
            .arguments(["-c", "echo error >&2; exit 1"])
            .output()
            .await
            .unwrap();
        assert!(!output.success());
        assert_eq!(output.into_stderr_string().unwrap(), "error\n");
    }

    #[tokio::test]
    async fn test_output_io_error() {
        let error = Command::new("./nonexistent").output().await.unwrap_err();
        assert!(error.io_error.is_some());
        assert_eq!(error.io_error.unwrap().kind(), std::io::ErrorKind::NotFound);
    }

    #[tokio::test]
    async fn test_spawn_with_piped_stdout() {
        use tokio::io::AsyncReadExt;

        let mut child = Command::new("echo")
            .argument("hello")
            .spawn()
            .stdout(Stdio::Piped)
            .run()
            .unwrap();

        let mut output = String::new();
        child
            .stdout()
            .unwrap()
            .read_to_string(&mut output)
            .await
            .unwrap();
        assert_eq!(output, "hello\n");

        let status = child.wait().await.unwrap();
        assert!(status.success());
    }

    #[tokio::test]
    async fn test_spawn_with_piped_stdin() {
        use tokio::io::AsyncWriteExt;

        let mut child = Command::new("cat")
            .spawn()
            .stdin(Stdio::Piped)
            .stdout(Stdio::Piped)
            .run()
            .unwrap();

        child.stdin().unwrap().write_all(b"hello").await.unwrap();
        drop(child.take_stdin());

        let output = child.wait_with_output().await.unwrap();
        assert!(output.success());
        assert_eq!(output.stdout, b"hello");
    }

    #[tokio::test]
    async fn test_spawn_wait() {
        let child = Command::new("true").spawn().run().unwrap();

        let status = child.wait().await.unwrap();
        assert!(status.success());
    }

    #[tokio::test]
    async fn test_spawn_wait_with_output() {
        let child = Command::new("sh")
            .arguments(["-c", "echo out; echo err >&2"])
            .spawn()
            .stdout(Stdio::Piped)
            .stderr(Stdio::Piped)
            .run()
            .unwrap();

        let output = child.wait_with_output().await.unwrap();
        assert!(output.success());
        assert_eq!(output.stdout, b"out\n");
        assert_eq!(output.stderr, b"err\n");
    }

    #[tokio::test]
    async fn test_spawn_io_error() {
        let error = Command::new("./nonexistent").spawn().run().unwrap_err();
        assert!(error.io_error.is_some());
        assert_eq!(error.io_error.unwrap().kind(), std::io::ErrorKind::NotFound);
    }

    #[tokio::test]
    async fn test_spawn_take_handles() {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};

        let mut child = Command::new("cat")
            .spawn()
            .stdin(Stdio::Piped)
            .stdout(Stdio::Piped)
            .run()
            .unwrap();

        let mut stdin = child.take_stdin().unwrap();
        stdin.write_all(b"test").await.unwrap();
        drop(stdin);

        let mut stdout = child.take_stdout().unwrap();
        let mut output = String::new();
        stdout.read_to_string(&mut output).await.unwrap();
        assert_eq!(output, "test");

        child.wait().await.unwrap();
    }

    #[tokio::test]
    async fn test_option() {
        let output = Command::new("echo")
            .option("-n", "hello")
            .capture_stdout()
            .string()
            .await
            .unwrap();
        assert_eq!(output, "hello");
    }

    #[tokio::test]
    async fn test_optional_option_some() {
        let output = Command::new("echo")
            .optional_option("-n", Some("hello"))
            .capture_stdout()
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
            .capture_stdout()
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
            .capture_stdout()
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
            .capture_stdout()
            .string()
            .await
            .unwrap();
        assert_eq!(output, "hello\n");
    }
}
