//! Container definitions and runtime handles.
//!
//! - [`Definition`] is the builder used to launch a container.
//! - [`Container`] is the runtime handle returned after a launch (or attached
//!   to via lookup primitives on [`crate::Backend`]).

use std::borrow::Cow;

use cmd_proc::Command;
use cmd_proc::CommandError;

use crate::Apply;
use crate::Backend;
use crate::image;
use crate::label;

/// Maximum permitted length of a [`ContainerName`], in bytes. Docker has no
/// documented maximum; this is a defensive ceiling far above any plausible
/// real-world name.
pub const MAX_CONTAINER_NAME_LEN: usize = 256;

/// Validated container name per docker / podman acceptance rules.
///
/// The first byte must be ASCII alphanumeric; subsequent bytes must be ASCII
/// alphanumeric or one of `_`, `.`, `-`. Length is bounded by
/// [`MAX_CONTAINER_NAME_LEN`].
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ContainerName(Cow<'static, str>);

impl ContainerName {
    /// Validated container name for `'static` inputs.
    ///
    /// # Panics
    ///
    /// Panics at compile time when used in a `const` context, or at runtime
    /// otherwise, if the input does not satisfy the container-name rules.
    #[must_use]
    pub const fn from_static_or_panic(input: &'static str) -> Self {
        match validate_container_name(input) {
            Ok(()) => {}
            Err(ContainerNameError::Empty) => panic!("Container name cannot be empty"),
            Err(ContainerNameError::InvalidStartCharacter) => {
                panic!("Container name must start with an ASCII alphanumeric")
            }
            Err(ContainerNameError::InvalidCharacter) => {
                panic!("Container name may only contain ASCII alphanumerics, '_', '.', '-'")
            }
            Err(ContainerNameError::TooLong) => panic!("Container name exceeds maximum length"),
        }
        Self(Cow::Borrowed(input))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::str::FromStr for ContainerName {
    type Err = ContainerNameError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        validate_container_name(input).map(|()| Self(Cow::Owned(input.to_string())))
    }
}

impl std::fmt::Display for ContainerName {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl AsRef<std::ffi::OsStr> for ContainerName {
    fn as_ref(&self) -> &std::ffi::OsStr {
        self.0.as_ref().as_ref()
    }
}

#[derive(Clone, Debug, thiserror::Error)]
pub enum ContainerNameError {
    #[error("Container name cannot be empty")]
    Empty,
    #[error("Container name must start with an ASCII alphanumeric")]
    InvalidStartCharacter,
    #[error("Container name may only contain ASCII alphanumerics, '_', '.', '-'")]
    InvalidCharacter,
    #[error("Container name exceeds maximum length of {MAX_CONTAINER_NAME_LEN} bytes")]
    TooLong,
}

const fn validate_container_name(input: &str) -> Result<(), ContainerNameError> {
    let bytes = input.as_bytes();
    if bytes.is_empty() {
        return Err(ContainerNameError::Empty);
    }
    if bytes.len() > MAX_CONTAINER_NAME_LEN {
        return Err(ContainerNameError::TooLong);
    }

    if !bytes[0].is_ascii_alphanumeric() {
        return Err(ContainerNameError::InvalidStartCharacter);
    }

    let mut i = 1;
    while i < bytes.len() {
        let byte = bytes[i];
        let is_alnum = byte.is_ascii_alphanumeric();
        let is_separator = byte == b'_' || byte == b'.' || byte == b'-';
        if !(is_alnum || is_separator) {
            return Err(ContainerNameError::InvalidCharacter);
        }
        i += 1;
    }
    Ok(())
}

impl Apply for ContainerName {
    fn apply(&self, command: Command) -> Command {
        command.argument("--name").argument(self)
    }
}

/// Macro to generate standard implementations for string wrapper newtypes
macro_rules! string_newtype {
    ($name:ident) => {
        impl From<String> for $name {
            fn from(value: String) -> Self {
                Self(value)
            }
        }

        impl From<&str> for $name {
            fn from(value: &str) -> Self {
                Self(value.to_string())
            }
        }

        impl AsRef<std::ffi::OsStr> for $name {
            fn as_ref(&self) -> &std::ffi::OsStr {
                self.0.as_ref()
            }
        }

        impl $name {
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }
    };
}

/// Macro to generate implementations for string wrapper newtypes with static flag + dynamic argument in Apply
macro_rules! apply_argument {
    ($name:ident, $flag:expr) => {
        string_newtype!($name);

        impl Apply for $name {
            fn apply(&self, command: Command) -> Command {
                command.argument($flag).argument(self)
            }
        }
    };
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContainerArgument(String);

string_newtype!(ContainerArgument);

impl Apply for ContainerArgument {
    fn apply(&self, command: Command) -> Command {
        command.argument(self)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Detach {
    Detach,
    NoDetach,
}

impl Apply for Detach {
    fn apply(&self, command: Command) -> Command {
        match self {
            Self::Detach => command.argument("--detach"),
            Self::NoDetach => command,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Remove {
    Remove,
    NoRemove,
}

impl Apply for Remove {
    fn apply(&self, command: Command) -> Command {
        match self {
            Self::Remove => command.argument("--rm"),
            Self::NoRemove => command,
        }
    }
}

/// Image pull policy for `docker run` / `podman run`, mapping 1:1 to
/// `--pull <always|missing|never>`.
///
/// Both runtimes default to `missing` when the flag is omitted; ociman
/// preserves that default by leaving [`Definition`] without a pull policy
/// unless [`Definition::pull_policy`] is called.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PullPolicy {
    /// `--pull always` — always pull the image, even if a local copy exists.
    Always,
    /// `--pull missing` — pull only if the image is absent locally
    /// (runtime default).
    Missing,
    /// `--pull never` — never pull; fail the run if the image is absent
    /// locally.
    Never,
}

impl PullPolicy {
    fn as_value(self) -> &'static str {
        match self {
            Self::Always => "always",
            Self::Missing => "missing",
            Self::Never => "never",
        }
    }
}

impl Apply for PullPolicy {
    fn apply(&self, command: Command) -> Command {
        command.argument("--pull").argument(self.as_value())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Mount(String);

apply_argument!(Mount, "--mount");

const UNSPECIFIED_IP: std::net::IpAddr = std::net::IpAddr::V4(std::net::Ipv4Addr::UNSPECIFIED);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Protocol {
    Tcp,
    Udp,
}

impl Protocol {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Tcp => "tcp",
            Self::Udp => "udp",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct HostBinding {
    ip: std::net::IpAddr,
    port: Option<u16>,
}

impl std::fmt::Display for HostBinding {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}:", self.ip)?;

        if let Some(port) = self.port {
            write!(formatter, "{port}")
        } else {
            Ok(())
        }
    }
}

/// Port publishing configuration for container networking.
///
/// Specifies how container ports are exposed to the host system.
/// The format follows Docker's `--publish` flag: `[[ip:][hostPort]:]containerPort[/protocol]`
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Publish {
    host_binding: Option<HostBinding>,
    container_port: u16,
    protocol: Protocol,
}

impl Publish {
    /// Creates a TCP port publish configuration.
    ///
    /// # Example
    ///
    /// ```
    /// let publish = ociman::Publish::tcp(8080);
    /// assert_eq!(publish.to_string(), "8080/tcp");
    /// ```
    #[must_use]
    pub fn tcp(container_port: u16) -> Self {
        Self {
            host_binding: None,
            container_port,
            protocol: Protocol::Tcp,
        }
    }

    /// Creates a UDP port publish configuration.
    ///
    /// # Example
    ///
    /// ```
    /// let publish = ociman::Publish::udp(53);
    /// assert_eq!(publish.to_string(), "53/udp");
    /// ```
    #[must_use]
    pub fn udp(container_port: u16) -> Self {
        Self {
            host_binding: None,
            container_port,
            protocol: Protocol::Udp,
        }
    }

    /// Sets the host IP address to bind to.
    ///
    /// # Examples
    ///
    /// ```
    /// let publish = ociman::Publish::tcp(8080)
    ///     .host_ip(std::net::Ipv4Addr::LOCALHOST.into());
    /// assert_eq!(publish.to_string(), "127.0.0.1::8080/tcp");
    /// ```
    ///
    /// With unspecified address:
    ///
    /// ```
    /// let publish = ociman::Publish::tcp(5432)
    ///     .host_ip(std::net::Ipv4Addr::UNSPECIFIED.into());
    /// assert_eq!(publish.to_string(), "0.0.0.0::5432/tcp");
    /// ```
    ///
    /// With IPv6:
    ///
    /// ```
    /// let publish = ociman::Publish::tcp(8080)
    ///     .host_ip(std::net::Ipv6Addr::LOCALHOST.into());
    /// assert_eq!(publish.to_string(), "::1::8080/tcp");
    /// ```
    ///
    /// Preserves previously set host port:
    ///
    /// ```
    /// let publish = ociman::Publish::tcp(80)
    ///     .host_port(8080)
    ///     .host_ip(std::net::Ipv4Addr::LOCALHOST.into());
    /// assert_eq!(publish.to_string(), "127.0.0.1:8080:80/tcp");
    /// ```
    #[must_use]
    pub fn host_ip(self, ip: std::net::IpAddr) -> Self {
        Self {
            host_binding: Some(HostBinding {
                ip,
                port: self.host_binding.and_then(|binding| binding.port),
            }),
            ..self
        }
    }

    /// Sets the host port to map to the container port.
    ///
    /// If no host IP has been set, defaults to `0.0.0.0`.
    ///
    /// # Examples
    ///
    /// ```
    /// let publish = ociman::Publish::tcp(80).host_port(8080);
    /// assert_eq!(publish.to_string(), "0.0.0.0:8080:80/tcp");
    /// ```
    ///
    /// Preserves previously set host IP:
    ///
    /// ```
    /// let publish = ociman::Publish::tcp(80)
    ///     .host_ip(std::net::Ipv4Addr::LOCALHOST.into())
    ///     .host_port(8080);
    /// assert_eq!(publish.to_string(), "127.0.0.1:8080:80/tcp");
    /// ```
    #[must_use]
    pub fn host_port(self, port: u16) -> Self {
        Self {
            host_binding: Some(HostBinding {
                ip: self
                    .host_binding
                    .map(|binding| binding.ip)
                    .unwrap_or(UNSPECIFIED_IP),
                port: Some(port),
            }),
            ..self
        }
    }

    /// Sets both host IP and port in a single call.
    ///
    /// # Example
    ///
    /// ```
    /// let publish = ociman::Publish::tcp(80)
    ///     .host_ip_port(std::net::Ipv4Addr::LOCALHOST.into(), 8080);
    /// assert_eq!(publish.to_string(), "127.0.0.1:8080:80/tcp");
    /// ```
    #[must_use]
    pub fn host_ip_port(self, ip: std::net::IpAddr, port: u16) -> Self {
        Self {
            host_binding: Some(HostBinding {
                ip,
                port: Some(port),
            }),
            ..self
        }
    }
}

impl std::fmt::Display for Publish {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(host_binding) = self.host_binding {
            write!(formatter, "{host_binding}:")?;
        }

        write!(
            formatter,
            "{}/{}",
            self.container_port,
            self.protocol.as_str()
        )
    }
}

impl Apply for Publish {
    fn apply(&self, command: Command) -> Command {
        command.argument("--publish").argument(self.to_string())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Entrypoint(String);

apply_argument!(Entrypoint, "--entrypoint");

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Workdir(String);

apply_argument!(Workdir, "--workdir");

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EnvironmentVariables(
    std::collections::BTreeMap<cmd_proc::EnvVariableName, cmd_proc::EnvVariableValue>,
);

impl EnvironmentVariables {
    fn new() -> Self {
        Self(std::collections::BTreeMap::new())
    }

    fn insert(&mut self, key: cmd_proc::EnvVariableName, value: cmd_proc::EnvVariableValue) {
        self.0.insert(key, value);
    }
}

impl Apply for EnvironmentVariables {
    fn apply(&self, command: Command) -> Command {
        self.0.iter().fold(command, |command, (key, value)| {
            command.argument("--env").argument(format!("{key}={value}"))
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Definition {
    backend: Backend,
    container_arguments: Vec<ContainerArgument>,
    detach: Detach,
    entrypoint: Option<Entrypoint>,
    environment_variables: EnvironmentVariables,
    interactive: Interactive,
    labels: label::Map,
    name: Option<ContainerName>,
    pull_policy: Option<PullPolicy>,
    reference: image::Reference,
    remove: Remove,
    mounts: Vec<Mount>,
    publish: Vec<Publish>,
    tty: Tty,
    workdir: Option<Workdir>,
}

impl Definition {
    #[must_use]
    pub fn new(backend: Backend, reference: image::Reference) -> Definition {
        Definition {
            backend,
            container_arguments: vec![],
            detach: Detach::NoDetach,
            entrypoint: None,
            environment_variables: EnvironmentVariables::new(),
            interactive: Interactive::NoInteractive,
            labels: label::Map::new(),
            name: None,
            pull_policy: None,
            reference,
            mounts: vec![],
            publish: vec![],
            remove: Remove::NoRemove,
            tty: Tty::NoTty,
            workdir: None,
        }
    }

    /// Runs a detached container and passes it to the provided async closure.
    ///
    /// The container is automatically stopped after the closure returns.
    /// Returns the closure's value, a [`WithContainerError::Run`] if the
    /// container could not be started, or a [`WithContainerError::Stop`] if
    /// the post-action stop fails.
    pub async fn with_container<R>(
        &self,
        mut action: impl AsyncFnMut(&mut Container) -> R,
    ) -> Result<R, WithContainerError> {
        let mut container = self
            .clone()
            .run_detached()
            .await
            .map_err(WithContainerError::Run)?;
        let result = action(&mut container).await;
        container.stop().await.map_err(WithContainerError::Stop)?;
        Ok(result)
    }

    #[must_use]
    pub fn backend(self, backend: Backend) -> Self {
        Self { backend, ..self }
    }

    pub fn entrypoint(self, command: impl Into<Entrypoint>) -> Self {
        Self {
            entrypoint: Some(command.into()),
            ..self
        }
    }

    pub fn workdir(self, path: impl Into<Workdir>) -> Self {
        Self {
            workdir: Some(path.into()),
            ..self
        }
    }

    pub fn arguments(
        self,
        arguments: impl IntoIterator<Item = impl Into<ContainerArgument>>,
    ) -> Self {
        Self {
            container_arguments: arguments.into_iter().map(Into::into).collect(),
            ..self
        }
    }

    pub fn argument(self, argument: impl Into<ContainerArgument>) -> Self {
        let mut container_arguments = self.container_arguments;
        container_arguments.push(argument.into());
        Self {
            container_arguments,
            ..self
        }
    }

    /// Uses validated [`cmd_proc::EnvVariableName`] keys to prevent invalid env names.
    #[must_use]
    pub fn environment_variable(
        self,
        key: cmd_proc::EnvVariableName,
        value: impl Into<cmd_proc::EnvVariableValue>,
    ) -> Self {
        let mut environment_variables = self.environment_variables;

        environment_variables.insert(key, value.into());

        Self {
            environment_variables,
            ..self
        }
    }

    /// Uses validated [`cmd_proc::EnvVariableName`] keys to prevent invalid env names.
    pub fn environment_variables<V: Into<cmd_proc::EnvVariableValue>>(
        self,
        values: impl IntoIterator<Item = (cmd_proc::EnvVariableName, V)>,
    ) -> Self {
        let mut environment_variables = self.environment_variables;

        for (key, value) in values {
            environment_variables.insert(key, value.into());
        }

        Self {
            environment_variables,
            ..self
        }
    }

    /// Set a fixed name on the container (`docker run --name <NAME>`).
    ///
    /// The backend enforces uniqueness — launching another container with the
    /// same name fails. Useful for long-lived "session" containers that
    /// callers want to look up by name later.
    #[must_use]
    pub fn container_name(self, name: &ContainerName) -> Self {
        Self {
            name: Some(name.clone()),
            ..self
        }
    }

    #[must_use]
    pub fn label(self, key: &label::Key, value: &label::Value) -> Self {
        let mut labels = self.labels;
        labels.insert(key.clone(), value.clone());
        Self { labels, ..self }
    }

    #[must_use]
    pub fn labels<'a>(
        self,
        values: impl IntoIterator<Item = (&'a label::Key, &'a label::Value)>,
    ) -> Self {
        let mut labels = self.labels;
        for (key, value) in values {
            labels.insert(key.clone(), value.clone());
        }
        Self { labels, ..self }
    }

    #[must_use]
    pub fn remove(self) -> Self {
        Self {
            remove: Remove::Remove,
            ..self
        }
    }

    #[must_use]
    pub fn no_remove(self) -> Self {
        Self {
            remove: Remove::NoRemove,
            ..self
        }
    }

    #[must_use]
    pub fn detach(self) -> Self {
        Self {
            detach: Detach::Detach,
            ..self
        }
    }

    #[must_use]
    pub fn no_detach(self) -> Self {
        Self {
            detach: Detach::NoDetach,
            ..self
        }
    }

    /// Set the image pull policy (`docker run --pull=...` /
    /// `podman run --pull=...`). Omitting this leaves the runtime's default
    /// (`missing`).
    #[must_use]
    pub fn pull_policy(self, value: PullPolicy) -> Self {
        Self {
            pull_policy: Some(value),
            ..self
        }
    }

    pub fn publish(self, value: impl Into<Publish>) -> Self {
        let mut publish = self.publish;

        publish.push(value.into());

        Self { publish, ..self }
    }

    pub fn publishes(self, values: impl IntoIterator<Item = impl Into<Publish>>) -> Self {
        let mut publish = self.publish;
        publish.extend(values.into_iter().map(Into::into));
        Self { publish, ..self }
    }

    pub fn mount(self, value: impl Into<Mount>) -> Self {
        let mut mounts = self.mounts;

        mounts.push(value.into());

        Self { mounts, ..self }
    }

    pub fn mounts(self, values: impl IntoIterator<Item = impl Into<Mount>>) -> Self {
        let mut mounts = self.mounts;
        mounts.extend(values.into_iter().map(Into::into));
        Self { mounts, ..self }
    }

    /// Allocate a pseudo-TTY for the container process (`docker run --tty` /
    /// `podman run --tty`). Avoid for binary stdout capture — `--tty`
    /// line-buffers and CRLF-translates the stream.
    #[must_use]
    pub fn tty(self) -> Self {
        Self {
            tty: Tty::Tty,
            ..self
        }
    }

    /// Keep stdin open for the container process (`docker run --interactive` /
    /// `podman run --interactive`), forwarding host stdin into the container.
    #[must_use]
    pub fn interactive(self) -> Self {
        Self {
            interactive: Interactive::Interactive,
            ..self
        }
    }

    /// Run the container detached, returning a handle to it.
    ///
    /// A configured container name that is already taken surfaces as
    /// [`RunError::NameInUse`]; any other non-zero exit (image absent,
    /// network error, etc.) surfaces as [`RunError::Subprocess`].
    pub async fn run_detached(&self) -> Result<Container, RunDetachedError> {
        let result = self
            .clone()
            .detach()
            .to_cmd_proc_command()
            .stdout_capture()
            .stderr_capture()
            .accept_nonzero_exit()
            .run()
            .await
            .map_err(RunError::from_command_error)?;

        if !result.status.success() {
            return Err(self.classify_run_failure(&result).into());
        }

        let id = ContainerId::try_from(strip_nl_end(&result.stdout))
            .map_err(RunDetachedError::ContainerIdUtf8)?;
        Ok(Container {
            backend: self.backend.clone(),
            id,
        })
    }

    /// Runs the container in the foreground and returns its captured stdout.
    pub async fn run_foreground_capture_only_stdout(&self) -> Result<Vec<u8>, CommandError> {
        self.clone()
            .no_detach()
            .to_cmd_proc_command()
            .stdout_capture()
            .bytes()
            .await
    }

    /// Runs the container in the foreground and returns success or a
    /// classified [`RunError`].
    pub async fn run(&self) -> Result<(), RunError> {
        let result = self
            .to_cmd_proc_command()
            .stdout_capture()
            .stderr_capture()
            .accept_nonzero_exit()
            .run()
            .await
            .map_err(RunError::from_command_error)?;

        if result.status.success() {
            Ok(())
        } else {
            Err(self.classify_run_failure(&result))
        }
    }

    /// Classify a non-zero `run` exit into a [`RunError`]. A configured
    /// container name that the runtime reports as taken becomes
    /// [`RunError::NameInUse`]; everything else is [`RunError::Subprocess`].
    /// Invalid-UTF-8 stderr becomes [`RunError::StderrUtf8`] rather than
    /// being silently lossy-decoded.
    fn classify_run_failure(&self, result: &cmd_proc::CaptureAllResult) -> RunError {
        let stderr = match std::str::from_utf8(&result.stderr) {
            Ok(stderr) => stderr,
            Err(source) => return RunError::StderrUtf8(source),
        };
        // Neither Docker nor Podman expose a structured signal for a name
        // collision — `run` exits 125 for every failure mode — so it is
        // detected by the English stderr substring. Only emit `NameInUse`
        // when a name was actually configured; otherwise a collision is
        // impossible and any match would be spurious.
        if let Some(name) = &self.name
            && stderr.contains("is already in use")
        {
            return RunError::NameInUse { name: name.clone() };
        }
        RunError::Subprocess {
            exit_status: result.status,
            stderr: stderr.to_string(),
        }
    }

    /// Lower this run definition into a runnable [`cmd_proc::Command`] without
    /// executing it.
    ///
    /// Borrows the definition and hands back the `docker run` / `podman run`
    /// command, so the caller composes the stdio disposition itself
    /// (`.status()`, `.stdout_capture().bytes()`, …) rather than picking a
    /// pre-baked run method. Symmetric with
    /// [`ExecCommand::to_cmd_proc_command`].
    #[must_use]
    pub fn to_cmd_proc_command(&self) -> Command {
        let command = self.backend.command().arguments(["container", "run"]);

        let command = self.detach.apply(command);
        let command = self.remove.apply(command);
        let command = self.tty.apply(command);
        let command = self.interactive.apply(command);
        let command = self.pull_policy.apply(command);
        let command = self.name.apply(command);
        let command = self.environment_variables.apply(command);
        let command = self.labels.apply(command);
        let command = self.publish.apply(command);
        let command = self.mounts.apply(command);
        let command = self.workdir.apply(command);
        let command = self.entrypoint.apply(command);
        let command = self.reference.apply(command);

        self.container_arguments.apply(command)
    }
}

pub(crate) fn strip_nl_end(value: &[u8]) -> &[u8] {
    match value.split_last() {
        Some((last, prefix)) => {
            if *last == b'\n' {
                prefix
            } else {
                panic!("last byte not a newline")
            }
        }
        None => panic!("empty slice"),
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContainerId(pub(crate) String);

impl std::convert::TryFrom<&[u8]> for ContainerId {
    type Error = std::str::Utf8Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        std::str::from_utf8(value).map(|str| ContainerId(str.to_string()))
    }
}

impl AsRef<std::ffi::OsStr> for ContainerId {
    fn as_ref(&self) -> &std::ffi::OsStr {
        self.0.as_ref()
    }
}

impl ContainerId {
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug)]
pub struct Container {
    pub(crate) backend: Backend,
    pub(crate) id: ContainerId,
}

#[derive(Debug, thiserror::Error)]
pub enum InspectError {
    /// At least one inspect target is absent.
    ///
    /// Detected by matching the runtime's stderr against `no such ` (case
    /// insensitive) — neither Docker nor Podman expose a structured signal
    /// distinguishing "target absent" from other failures of `inspect`, and
    /// `inspect` exits non-zero for every failure mode. The English stderr
    /// substring is the only thing specific to absence. This is consistent
    /// with how [`RunError::NameInUse`] is detected, justified by the same
    /// gap in the runtime contract.
    #[error("inspect target not found")]
    NotFound,
    /// Subprocess could not be started (binary missing, permissions, etc.).
    #[error("inspect command IO failure")]
    Io(#[source] std::io::Error),
    /// Subprocess exited non-zero for some reason other than absence (daemon
    /// down, permission, etc.). The captured stderr is preserved for
    /// diagnostics.
    #[error("inspect command exited with {exit_status}: {stderr}")]
    Subprocess {
        exit_status: std::process::ExitStatus,
        stderr: String,
    },
    #[error("inspect output was not valid JSON")]
    Parse(#[from] serde_json::Error),
    #[error("inspect output was not valid UTF-8")]
    Utf8(#[from] std::str::Utf8Error),
}

impl InspectError {
    /// Classify a non-zero `inspect` subprocess result. Stderr substring
    /// match against known absence signatures → [`Self::NotFound`];
    /// anything else → [`Self::Subprocess`].
    ///
    /// Patterns this catches (case-insensitive):
    /// - `"no such "` — Docker (`No such object`, `No such container`,
    ///   `No such image`) and Podman containers (`no such container`).
    /// - `"not known"` — Podman images (`<ref>: image not known`).
    pub(crate) fn classify_failure(
        exit_status: std::process::ExitStatus,
        stderr_bytes: &[u8],
    ) -> Self {
        let stderr = match std::str::from_utf8(stderr_bytes) {
            Ok(stderr) => stderr,
            Err(source) => return Self::Utf8(source),
        };
        let lower = stderr.to_ascii_lowercase();
        if lower.contains("no such ") || lower.contains("not known") {
            return Self::NotFound;
        }
        Self::Subprocess {
            exit_status,
            stderr: stderr.to_owned(),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ReadContainerNameError {
    #[error(transparent)]
    Inspect(#[from] InspectError),
    #[error("inspect returned no string Name field")]
    NameNotString,
    #[error(transparent)]
    InvalidName(#[from] ContainerNameError),
}

#[derive(Debug, thiserror::Error)]
pub enum ReadHostTcpPortError {
    #[error(transparent)]
    Inspect(#[from] InspectError),
    #[error("container port {container_port}/tcp is not published")]
    NotPublished { container_port: u16 },
    #[error("host port {value:?} is not a valid u16")]
    InvalidHostPort {
        value: String,
        #[source]
        source: std::num::ParseIntError,
    },
}

/// Shared classification of a `docker run` / `podman run` failure, used by
/// both [`Definition::run`] and [`Definition::run_detached`].
#[derive(Debug, thiserror::Error)]
pub enum RunError {
    /// Subprocess could not be started (binary missing, permissions, etc.).
    #[error("docker/podman run command IO failure")]
    Io(#[source] std::io::Error),
    /// The configured container name is already taken by another container.
    ///
    /// Neither Docker nor Podman expose a structured signal for this — `run`
    /// exits 125 for *every* failure mode — so it is detected by matching
    /// the runtime's stderr. This is the one place in ociman that matches on
    /// stderr content, justified by the absence of any documented exit code
    /// or probe for the condition.
    #[error("container name {name} is already in use")]
    NameInUse { name: ContainerName },
    /// Subprocess exited non-zero for some other reason (image absent,
    /// network error, etc.). The captured stderr is preserved for
    /// diagnostics.
    #[error("docker/podman run exited with {exit_status}: {stderr}")]
    Subprocess {
        exit_status: std::process::ExitStatus,
        stderr: String,
    },
    /// Subprocess exited non-zero and its stderr was not valid UTF-8 — so
    /// no name-collision detection or human-readable diagnostic could be
    /// performed.
    #[error("docker/podman run stderr was not valid UTF-8")]
    StderrUtf8(#[source] std::str::Utf8Error),
}

impl RunError {
    /// Map a [`CommandError`] from the run subprocess into a [`RunError`].
    ///
    /// With `accept_nonzero_exit` set on the command, the `ExitStatus` arm
    /// is not reached in practice — a non-zero exit comes back as
    /// `Ok(result)` and is classified from its captured stderr instead. It
    /// is still mapped honestly here rather than panicking, in case the
    /// command is ever run without that flag.
    fn from_command_error(error: CommandError) -> Self {
        match error {
            CommandError::Io(io) => Self::Io(io),
            CommandError::ExitStatus(exit_status) => Self::Subprocess {
                exit_status,
                stderr: String::new(),
            },
        }
    }
}

/// Errors produced by [`Definition::run_detached`].
#[derive(Debug, thiserror::Error)]
pub enum RunDetachedError {
    /// The run itself failed — see [`RunError`].
    #[error(transparent)]
    Run(#[from] RunError),
    /// The container ID returned on stdout was not valid UTF-8.
    #[error("container ID returned by docker/podman run was not valid UTF-8")]
    ContainerIdUtf8(#[source] std::str::Utf8Error),
}

/// Errors produced by [`Definition::with_container`].
#[derive(Debug, thiserror::Error)]
pub enum WithContainerError {
    /// The container could not be started.
    #[error("failed to start container")]
    Run(#[source] RunDetachedError),
    /// The post-action stop failed.
    #[error("failed to stop container after action")]
    Stop(#[source] CommandError),
}

/// `--user UID:GID` value for an exec'd process.
struct User {
    uid: rustix::process::Uid,
    gid: rustix::process::Gid,
}

impl Apply for User {
    fn apply(&self, command: Command) -> Command {
        command
            .argument("--user")
            .argument(format!("{}:{}", self.uid.as_raw(), self.gid.as_raw()))
    }
}

/// Whether to allocate a pseudo-TTY for the exec'd process (`--tty`).
#[derive(Clone, Debug, Eq, PartialEq)]
enum Tty {
    Tty,
    NoTty,
}

impl Apply for Tty {
    fn apply(&self, command: Command) -> Command {
        match self {
            Self::Tty => command.argument("--tty"),
            Self::NoTty => command,
        }
    }
}

/// Whether to keep stdin open for the exec'd process (`--interactive`).
#[derive(Clone, Debug, Eq, PartialEq)]
enum Interactive {
    Interactive,
    NoInteractive,
}

impl Apply for Interactive {
    fn apply(&self, command: Command) -> Command {
        match self {
            Self::Interactive => command.argument("--interactive"),
            Self::NoInteractive => command,
        }
    }
}

pub struct ExecCommand<'a> {
    container: &'a Container,
    executable: String,
    arguments: Vec<String>,
    environment_variables: EnvironmentVariables,
    tty: Tty,
    interactive: Interactive,
    stdin_data: Option<Vec<u8>>,
    user: Option<User>,
    workdir: Option<Workdir>,
}

impl<'a> ExecCommand<'a> {
    fn new(container: &'a Container, executable: impl Into<String>) -> Self {
        Self {
            container,
            executable: executable.into(),
            arguments: Vec::new(),
            environment_variables: EnvironmentVariables::new(),
            tty: Tty::NoTty,
            interactive: Interactive::NoInteractive,
            stdin_data: None,
            user: None,
            workdir: None,
        }
    }

    /// Add a single argument.
    pub fn argument(mut self, value: impl Into<String>) -> Self {
        self.arguments.push(value.into());
        self
    }

    /// Add multiple arguments.
    pub fn arguments(mut self, values: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.arguments.extend(values.into_iter().map(Into::into));
        self
    }

    /// Add an environment variable.
    ///
    /// Uses validated [`cmd_proc::EnvVariableName`] keys to prevent invalid env names.
    pub fn environment_variable(
        mut self,
        key: cmd_proc::EnvVariableName,
        value: impl Into<cmd_proc::EnvVariableValue>,
    ) -> Self {
        self.environment_variables.insert(key, value.into());
        self
    }

    /// Add multiple environment variables.
    ///
    /// Uses validated [`cmd_proc::EnvVariableName`] keys to prevent invalid env names.
    pub fn environment_variables<V: Into<cmd_proc::EnvVariableValue>>(
        mut self,
        variables: impl IntoIterator<Item = (cmd_proc::EnvVariableName, V)>,
    ) -> Self {
        for (key, value) in variables {
            self.environment_variables.insert(key, value.into());
        }
        self
    }

    /// Allocate a pseudo-TTY (runtime `--tty` flag).
    ///
    /// Pair with [`Self::interactive`] for an interactive shell-style session.
    /// Avoid for binary stream capture — `--tty` line-buffers and CRLF-translates
    /// stdout.
    #[must_use]
    pub fn tty(mut self) -> Self {
        self.tty = Tty::Tty;
        self
    }

    /// Keep stdin open in the container (runtime `--interactive` flag).
    ///
    /// Without this, the container sees stdin as closed. With it, host stdin
    /// flows through to the container — enabling shell pipes like
    /// `cat host.sql | container.exec("psql").interactive()...`.
    ///
    /// `interactive` and `tty` are independent, mirroring the runtime CLI:
    /// `interactive()` alone enables stdin passthrough on a clean byte stream
    /// (suitable for `pg_dump`/`pg_restore`); pair with [`Self::tty`] for an
    /// interactive shell. Implied automatically when [`Self::stdin`] supplies
    /// preloaded data.
    #[must_use]
    pub fn interactive(mut self) -> Self {
        self.interactive = Interactive::Interactive;
        self
    }

    /// Set stdin data to send to the command.
    ///
    /// Implies [`Self::interactive`] so host stdin is kept open for the data.
    pub fn stdin(mut self, data: impl Into<Vec<u8>>) -> Self {
        self.stdin_data = Some(data.into());
        self.interactive = Interactive::Interactive;
        self
    }

    /// Run the exec'd process as the given uid:gid (runtime `--user` flag).
    ///
    /// Maps 1:1 to `docker exec --user UID:GID` / `podman exec --user UID:GID`.
    /// Changes only the exec'd process's identity inside the container — the
    /// container's main process keeps its own user.
    ///
    /// Pair with a host-cwd bind mount to make file ownership of reads/writes
    /// match the host user without configuring user-namespace remapping.
    #[must_use]
    pub fn user(mut self, uid: rustix::process::Uid, gid: rustix::process::Gid) -> Self {
        self.user = Some(User { uid, gid });
        self
    }

    /// Set the working directory inside the container (runtime `--workdir` flag).
    ///
    /// The path is resolved against the container's filesystem, not the
    /// host's. When the container was launched with a bind mount that
    /// mirrors a host path, pointing `workdir` at the mirrored path makes
    /// relative file references in the exec'd command resolve naturally.
    #[must_use]
    pub fn workdir(mut self, path: impl Into<Workdir>) -> Self {
        self.workdir = Some(path.into());
        self
    }

    /// Lower this exec builder into a runnable [`cmd_proc::Command`] without
    /// executing it.
    ///
    /// Borrows the builder and hands back the command, so the caller composes
    /// the stdio disposition itself (`.status()`, `.stdout_capture().bytes()`,
    /// …) rather than picking a pre-baked run method.
    #[must_use]
    pub fn to_cmd_proc_command(&self) -> Command {
        let command = self
            .container
            .backend_command()
            .arguments(["container", "exec"]);

        let command = self.tty.apply(command);
        let command = self.interactive.apply(command);
        let command = self.user.apply(command);
        let command = self.workdir.apply(command);
        let command = self.environment_variables.apply(command);

        let mut command = command
            .argument(&self.container.id)
            .argument(self.executable.as_str())
            .arguments(self.arguments.iter().cloned());

        if let Some(data) = &self.stdin_data {
            command = command.stdin_bytes(data.clone());
        }

        command
    }

    /// Execute the command and return success or an error.
    pub async fn status(self) -> Result<(), CommandError> {
        self.to_cmd_proc_command().status().await
    }
}

impl Container {
    /// The runtime-assigned container ID.
    #[must_use]
    pub fn id(&self) -> &ContainerId {
        &self.id
    }

    /// Create an exec command builder for running commands inside this container.
    pub fn exec(&self, executable: impl Into<String>) -> ExecCommand<'_> {
        ExecCommand::new(self, executable)
    }

    /// Stop the container (`docker container stop` / `podman container stop`).
    /// Returns the subprocess outcome rather than panicking so callers can
    /// decide how to handle failure (best-effort cleanup paths typically
    /// log and continue; transactional callers propagate).
    pub async fn stop(&mut self) -> Result<(), CommandError> {
        self.backend_command()
            .arguments(["container", "stop"])
            .argument(&self.id)
            .stdout_capture()
            .bytes()
            .await
            .map(drop)
    }

    /// Remove the stopped container (`docker container rm` / `podman container rm`).
    /// Returns the subprocess outcome rather than panicking; see [`Self::stop`].
    pub async fn remove(&mut self) -> Result<(), CommandError> {
        self.do_remove(false).await
    }

    /// Force-remove the container (`docker container rm --force` /
    /// `podman container rm --force`). SIGKILLs a running container then
    /// removes it in one call. Designed for best-effort cleanup paths
    /// where waiting for graceful shutdown is not appropriate.
    pub async fn remove_force(&mut self) -> Result<(), CommandError> {
        self.do_remove(true).await
    }

    async fn do_remove(&mut self, force: bool) -> Result<(), CommandError> {
        let mut command = self.backend_command().arguments(["container", "rm"]);
        if force {
            command = command.argument("--force");
        }
        command
            .argument(&self.id)
            .stdout_capture()
            .bytes()
            .await
            .map(drop)
    }

    pub async fn inspect(&self) -> Result<serde_json::Value, InspectError> {
        // Single-id batched inspect: docker returns a length-1 array on
        // success; peel the singleton.
        Ok(self
            .backend
            .inspect_container([&self.id])
            .await?
            .into_iter()
            .next()
            .unwrap())
    }

    pub async fn inspect_format(&self, format: &str) -> Result<String, InspectError> {
        self.backend
            .inspect_container_format(&self.id, format)
            .await
    }

    pub async fn labels(&self) -> Result<label::ContainerLabels, label::ContainerError> {
        self.backend.container_labels(&self.id).await
    }

    pub async fn name(&self) -> Result<ContainerName, ReadContainerNameError> {
        self.backend.container_name(&self.id).await
    }

    pub async fn read_host_tcp_port(
        &self,
        container_port: u16,
    ) -> Result<u16, ReadHostTcpPortError> {
        let value = self.inspect().await?;

        let host_port_str = value
            .get("NetworkSettings")
            .and_then(|network_settings| network_settings.get("Ports"))
            .and_then(|ports| ports.get(format!("{container_port}/tcp")))
            .and_then(|bindings| bindings.get(0))
            .and_then(|binding| binding.get("HostPort"))
            .and_then(|host_port| host_port.as_str())
            .ok_or(ReadHostTcpPortError::NotPublished { container_port })?;

        host_port_str
            .parse::<u16>()
            .map_err(|source| ReadHostTcpPortError::InvalidHostPort {
                value: host_port_str.to_string(),
                source,
            })
    }

    pub async fn commit(
        &self,
        reference: &image::Reference,
        pause: bool,
    ) -> Result<(), CommandError> {
        let pause_argument = match (&self.backend, pause) {
            (Backend::Docker { .. }, true) => None,
            (Backend::Docker { version, .. }, false) => {
                // Docker 29.0 replaced --pause with --no-pause
                // https://docs.docker.com/engine/release-notes/29/
                if version.major >= 29 {
                    Some("--no-pause")
                } else {
                    Some("--pause=false")
                }
            }
            (Backend::Podman { .. }, true) => Some("--pause"),
            (Backend::Podman { .. }, false) => None,
        };

        self.backend_command()
            .argument("commit")
            .optional_argument(pause_argument)
            .argument(&self.id)
            .argument(reference.to_string())
            .status()
            .await
    }

    fn backend_command(&self) -> Command {
        self.backend.command()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn container_name_accepts_valid() {
        assert!("session-main".parse::<ContainerName>().is_ok());
        assert!("0".parse::<ContainerName>().is_ok());
        assert!("a".parse::<ContainerName>().is_ok());
        assert!("pg-ephemeral.session_42".parse::<ContainerName>().is_ok());
    }

    #[test]
    fn container_name_rejects_empty() {
        assert!(matches!(
            "".parse::<ContainerName>(),
            Err(ContainerNameError::Empty)
        ));
    }

    #[test]
    fn container_name_rejects_invalid_start() {
        assert!(matches!(
            "-leading".parse::<ContainerName>(),
            Err(ContainerNameError::InvalidStartCharacter)
        ));
        assert!(matches!(
            ".leading".parse::<ContainerName>(),
            Err(ContainerNameError::InvalidStartCharacter)
        ));
        assert!(matches!(
            "_leading".parse::<ContainerName>(),
            Err(ContainerNameError::InvalidStartCharacter)
        ));
    }

    #[test]
    fn container_name_rejects_invalid_inner_char() {
        assert!(matches!(
            "session main".parse::<ContainerName>(),
            Err(ContainerNameError::InvalidCharacter)
        ));
        assert!(matches!(
            "session/main".parse::<ContainerName>(),
            Err(ContainerNameError::InvalidCharacter)
        ));
        assert!(matches!(
            "session=main".parse::<ContainerName>(),
            Err(ContainerNameError::InvalidCharacter)
        ));
    }

    #[test]
    fn container_name_rejects_too_long() {
        let input = "a".repeat(MAX_CONTAINER_NAME_LEN + 1);
        assert!(matches!(
            input.parse::<ContainerName>(),
            Err(ContainerNameError::TooLong)
        ));
    }

    #[test]
    fn container_name_accepts_at_max_length() {
        let input = "a".repeat(MAX_CONTAINER_NAME_LEN);
        assert!(input.parse::<ContainerName>().is_ok());
    }

    #[test]
    fn container_name_from_static_or_panic_is_const() {
        const NAME: ContainerName = ContainerName::from_static_or_panic("pg-ephemeral.main");
        assert_eq!(NAME.as_str(), "pg-ephemeral.main");
    }
}
