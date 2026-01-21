use std::borrow::Cow;
use std::ffi::OsStr;

#[derive(Debug, thiserror::Error)]
#[error("Command execution failed: io_error={io_error:?}, exit_status={exit_status:?}")]
pub struct CommandError {
    pub io_error: Option<std::io::Error>,
    pub exit_status: Option<std::process::ExitStatus>,
}

fn write_stdin(
    child: &mut std::process::Child,
    stdin_data: Option<Vec<u8>>,
) -> Result<(), CommandError> {
    use std::io::Write;

    if let Some(data) = stdin_data {
        child
            .stdin
            .take()
            .unwrap()
            .write_all(&data)
            .map_err(|io_error| CommandError {
                io_error: Some(io_error),
                exit_status: None,
            })?;
    }

    Ok(())
}

fn run_and_wait(
    mut child: std::process::Child,
    stdin_data: Option<Vec<u8>>,
    start: std::time::Instant,
) -> Result<std::process::Output, CommandError> {
    write_stdin(&mut child, stdin_data)?;

    let output = child.wait_with_output().map_err(|io_error| CommandError {
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

fn run_and_wait_status(
    mut child: std::process::Child,
    stdin_data: Option<Vec<u8>>,
    start: std::time::Instant,
) -> Result<std::process::ExitStatus, CommandError> {
    write_stdin(&mut child, stdin_data)?;

    let status = child.wait().map_err(|io_error| CommandError {
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
enum CaptureStream {
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
    inner: std::process::Child,
}

impl Child {
    /// Returns a mutable reference to the child's stdin handle.
    pub fn stdin(&mut self) -> Option<&mut std::process::ChildStdin> {
        self.inner.stdin.as_mut()
    }

    /// Returns a mutable reference to the child's stdout handle.
    pub fn stdout(&mut self) -> Option<&mut std::process::ChildStdout> {
        self.inner.stdout.as_mut()
    }

    /// Returns a mutable reference to the child's stderr handle.
    pub fn stderr(&mut self) -> Option<&mut std::process::ChildStderr> {
        self.inner.stderr.as_mut()
    }

    /// Takes ownership of the child's stdin handle.
    pub fn take_stdin(&mut self) -> Option<std::process::ChildStdin> {
        self.inner.stdin.take()
    }

    /// Takes ownership of the child's stdout handle.
    pub fn take_stdout(&mut self) -> Option<std::process::ChildStdout> {
        self.inner.stdout.take()
    }

    /// Takes ownership of the child's stderr handle.
    pub fn take_stderr(&mut self) -> Option<std::process::ChildStderr> {
        self.inner.stderr.take()
    }

    /// Waits for the child to exit and returns its exit status.
    pub fn wait(mut self) -> Result<std::process::ExitStatus, CommandError> {
        self.inner.wait().map_err(|io_error| CommandError {
            io_error: Some(io_error),
            exit_status: None,
        })
    }

    /// Simultaneously waits for the child to exit and collects all output.
    pub fn wait_with_output(self) -> Result<Output, CommandError> {
        let output = self
            .inner
            .wait_with_output()
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
    stream: CaptureStream,
}

impl Capture {
    fn new(command: Command, stream: CaptureStream) -> Self {
        Self { command, stream }
    }

    /// Execute the command and return captured output as bytes.
    pub fn bytes(mut self) -> Result<Vec<u8>, CommandError> {
        use std::process::Stdio;

        log::debug!("{:#?}", self.command.inner);

        self.command.inner.stdout(Stdio::piped());
        self.command.inner.stderr(Stdio::piped());

        if self.command.stdin_data.is_some() {
            self.command.inner.stdin(Stdio::piped());
        }

        let start = std::time::Instant::now();

        let child = self
            .command
            .inner
            .spawn()
            .map_err(|io_error| CommandError {
                io_error: Some(io_error),
                exit_status: None,
            })?;

        let output = run_and_wait(child, self.command.stdin_data, start)?;

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

    /// Asserts that two commands are equal by comparing their `Debug` representations.
    ///
    /// This is useful for testing that a command builder produces the expected command
    /// without actually executing it.
    ///
    /// # Panics
    ///
    /// Panics if the `Debug` output of the two commands' inner `std::process::Command` differ.
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
    pub fn stdout(self) -> Capture {
        Capture::new(self, CaptureStream::Stdout)
    }

    /// Capture stderr from this command.
    #[must_use]
    pub fn stderr(self) -> Capture {
        Capture::new(self, CaptureStream::Stderr)
    }

    /// Spawn the command as a child process.
    ///
    /// Returns a [`Spawn`] builder to configure stdin/stdout/stderr before running.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use cmd_proc::{Command, Stdio};
    /// use std::io::BufRead;
    ///
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
    /// let line = std::io::BufReader::new(child.stdout().unwrap())
    ///     .lines()
    ///     .next()
    ///     .unwrap()
    ///     .unwrap();
    ///
    /// // Close stdin to signal the child to exit
    /// drop(child.take_stdin());
    /// child.wait().unwrap();
    /// ```
    #[must_use]
    pub fn spawn(self) -> Spawn {
        Spawn::new(self)
    }

    /// Execute the command and return full output regardless of exit status.
    ///
    /// Unlike `stdout()` and `stderr()`, this does not treat non-zero exit as an error.
    /// Use this when you need to inspect both streams or handle failure cases with stderr.
    pub fn output(mut self) -> Result<Output, CommandError> {
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

        let output = run_and_wait(child, self.stdin_data, start)?;

        Ok(Output {
            stdout: output.stdout,
            stderr: output.stderr,
            status: output.status,
        })
    }

    /// Execute the command and return success or an error.
    pub fn status(mut self) -> Result<(), CommandError> {
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

        let exit_status = run_and_wait_status(child, self.stdin_data, start)?;

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
        assert!(Command::new("true").status().is_ok());
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

    #[test]
    fn test_env_with_variable() {
        let name: EnvVariableName = "MY_VAR".parse().unwrap();
        let output = Command::new("sh")
            .arguments(["-c", "echo $MY_VAR"])
            .env(&name, "hello")
            .stdout()
            .string()
            .unwrap();
        assert_eq!(output, "hello\n");
    }

    #[test]
    fn test_stdin_bytes() {
        let output = Command::new("cat")
            .stdin_bytes(b"hello world".as_slice())
            .stdout()
            .string()
            .unwrap();
        assert_eq!(output, "hello world");
    }

    #[test]
    fn test_stdin_bytes_vec() {
        let output = Command::new("cat")
            .stdin_bytes(vec![104, 105])
            .stdout()
            .string()
            .unwrap();
        assert_eq!(output, "hi");
    }

    #[test]
    fn test_output_success() {
        let output = Command::new("echo").argument("hello").output().unwrap();
        assert!(output.success());
        assert_eq!(output.stdout, b"hello\n");
        assert!(output.stderr.is_empty());
    }

    #[test]
    fn test_output_failure_with_stderr() {
        let output = Command::new("sh")
            .arguments(["-c", "echo error >&2; exit 1"])
            .output()
            .unwrap();
        assert!(!output.success());
        assert_eq!(output.into_stderr_string().unwrap(), "error\n");
    }

    #[test]
    fn test_output_io_error() {
        let error = Command::new("./nonexistent").output().unwrap_err();
        assert!(error.io_error.is_some());
        assert_eq!(error.io_error.unwrap().kind(), std::io::ErrorKind::NotFound);
    }

    #[test]
    fn test_spawn_with_piped_stdout() {
        use std::io::Read;

        let mut child = Command::new("echo")
            .argument("hello")
            .spawn()
            .stdout(Stdio::Piped)
            .run()
            .unwrap();

        let mut output = String::new();
        child.stdout().unwrap().read_to_string(&mut output).unwrap();
        assert_eq!(output, "hello\n");

        let status = child.wait().unwrap();
        assert!(status.success());
    }

    #[test]
    fn test_spawn_with_piped_stdin() {
        use std::io::Write;

        let mut child = Command::new("cat")
            .spawn()
            .stdin(Stdio::Piped)
            .stdout(Stdio::Piped)
            .run()
            .unwrap();

        child.stdin().unwrap().write_all(b"hello").unwrap();
        drop(child.take_stdin());

        let output = child.wait_with_output().unwrap();
        assert!(output.success());
        assert_eq!(output.stdout, b"hello");
    }

    #[test]
    fn test_spawn_wait() {
        let child = Command::new("true").spawn().run().unwrap();

        let status = child.wait().unwrap();
        assert!(status.success());
    }

    #[test]
    fn test_spawn_wait_with_output() {
        let child = Command::new("sh")
            .arguments(["-c", "echo out; echo err >&2"])
            .spawn()
            .stdout(Stdio::Piped)
            .stderr(Stdio::Piped)
            .run()
            .unwrap();

        let output = child.wait_with_output().unwrap();
        assert!(output.success());
        assert_eq!(output.stdout, b"out\n");
        assert_eq!(output.stderr, b"err\n");
    }

    #[test]
    fn test_spawn_io_error() {
        let error = Command::new("./nonexistent").spawn().run().unwrap_err();
        assert!(error.io_error.is_some());
        assert_eq!(error.io_error.unwrap().kind(), std::io::ErrorKind::NotFound);
    }

    #[test]
    fn test_spawn_take_handles() {
        use std::io::{Read, Write};

        let mut child = Command::new("cat")
            .spawn()
            .stdin(Stdio::Piped)
            .stdout(Stdio::Piped)
            .run()
            .unwrap();

        let mut stdin = child.take_stdin().unwrap();
        stdin.write_all(b"test").unwrap();
        drop(stdin);

        let mut stdout = child.take_stdout().unwrap();
        let mut output = String::new();
        stdout.read_to_string(&mut output).unwrap();
        assert_eq!(output, "test");

        child.wait().unwrap();
    }

    #[test]
    fn test_option() {
        let output = Command::new("echo")
            .option("-n", "hello")
            .stdout()
            .string()
            .unwrap();
        assert_eq!(output, "hello");
    }

    #[test]
    fn test_optional_option_some() {
        let output = Command::new("echo")
            .optional_option("-n", Some("hello"))
            .stdout()
            .string()
            .unwrap();
        assert_eq!(output, "hello");
    }

    #[test]
    fn test_optional_option_none() {
        let output = Command::new("echo")
            .optional_option("-n", None::<&str>)
            .argument("hello")
            .stdout()
            .string()
            .unwrap();
        assert_eq!(output, "hello\n");
    }
}
