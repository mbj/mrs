#![doc = include_str!("../README.md")]

pub mod backend;
pub mod image;
pub mod platform;
pub mod reference;
pub mod testing;

pub use backend::{Backend, ContainerHostnameResolver, ResolveHostnameError};
use cmd_proc::Command;
use cmd_proc::CommandError;
pub use image::{
    BuildArgumentKey, BuildArgumentKeyError, BuildArgumentValue, BuildDefinition, BuildSource,
    BuildTarget, Reference,
};

trait Apply {
    fn apply(&self, command: Command) -> Command;
}

impl<T: Apply> Apply for Vec<T> {
    fn apply(&self, command: Command) -> Command {
        self.iter()
            .fold(command, |command, item| item.apply(command))
    }
}

impl<T: Apply> Apply for Option<T> {
    fn apply(&self, command: Command) -> Command {
        match self {
            Some(item) => item.apply(command),
            None => command,
        }
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

impl Apply for image::Reference {
    fn apply(&self, command: Command) -> Command {
        command.argument(self.to_string())
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
    std::collections::BTreeMap<cmd_proc::EnvVariableName<'static>, String>,
);

impl EnvironmentVariables {
    fn new() -> Self {
        Self(std::collections::BTreeMap::new())
    }

    fn insert(&mut self, key: cmd_proc::EnvVariableName<'static>, value: String) {
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
    reference: image::Reference,
    remove: Remove,
    mounts: Vec<Mount>,
    publish: Vec<Publish>,
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
            reference,
            mounts: vec![],
            publish: vec![],
            remove: Remove::NoRemove,
            workdir: None,
        }
    }

    /// Runs a detached container and passes it to the provided async closure.
    ///
    /// The container is automatically stopped after the closure returns.
    pub async fn with_container<R>(&self, mut action: impl AsyncFnMut(&mut Container) -> R) -> R {
        let mut container = self.clone().run_detached().await;
        let result = action(&mut container).await;
        container.stop().await;
        result
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
        key: cmd_proc::EnvVariableName<'static>,
        value: &str,
    ) -> Self {
        let mut environment_variables = self.environment_variables;

        environment_variables.insert(key, value.to_string());

        Self {
            environment_variables,
            ..self
        }
    }

    /// Uses validated [`cmd_proc::EnvVariableName`] keys to prevent invalid env names.
    pub fn environment_variables<V: Into<String>>(
        self,
        values: impl IntoIterator<Item = (cmd_proc::EnvVariableName<'static>, V)>,
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

    pub async fn run_detached(&self) -> Container {
        let stdout = self.clone().detach().run_output().await;

        Container {
            backend: self.backend.clone(),
            id: ContainerId::try_from(strip_nl_end(&stdout)).unwrap(),
        }
    }

    pub async fn run_capture_only_stdout(&self) -> Vec<u8> {
        self.clone().no_detach().run_output().await
    }

    /// Runs the container and returns success or an error.
    pub async fn run(&self) -> Result<(), CommandError> {
        self.build_run_command().status().await
    }

    fn build_run_command(&self) -> Command {
        let command = self.backend.command().argument("run");

        let command = self.detach.apply(command);
        let command = self.remove.apply(command);
        let command = self.environment_variables.apply(command);
        let command = self.publish.apply(command);
        let command = self.mounts.apply(command);
        let command = self.workdir.apply(command);
        let command = self.entrypoint.apply(command);
        let command = self.reference.apply(command);

        self.container_arguments.apply(command)
    }

    async fn run_output(&self) -> Vec<u8> {
        self.build_run_command()
            .stdout_capture()
            .bytes()
            .await
            .unwrap()
    }
}

fn strip_nl_end(value: &[u8]) -> &[u8] {
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
pub struct ContainerId(String);

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
    backend: Backend,
    id: ContainerId,
}

/// Builder for executing commands inside a container.
pub struct ExecCommand<'a> {
    container: &'a Container,
    executable: String,
    arguments: Vec<String>,
    environment: Vec<(cmd_proc::EnvVariableName<'static>, String)>,
    interactive: bool,
    stdin_data: Option<Vec<u8>>,
}

impl<'a> ExecCommand<'a> {
    fn new(container: &'a Container, executable: impl Into<String>) -> Self {
        Self {
            container,
            executable: executable.into(),
            arguments: Vec::new(),
            environment: Vec::new(),
            interactive: false,
            stdin_data: None,
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
        key: cmd_proc::EnvVariableName<'static>,
        value: impl Into<String>,
    ) -> Self {
        self.environment.push((key, value.into()));
        self
    }

    /// Add multiple environment variables.
    ///
    /// Uses validated [`cmd_proc::EnvVariableName`] keys to prevent invalid env names.
    pub fn environment_variables(
        mut self,
        variables: impl IntoIterator<Item = (cmd_proc::EnvVariableName<'static>, impl Into<String>)>,
    ) -> Self {
        self.environment.extend(
            variables
                .into_iter()
                .map(|(key, value)| (key, value.into())),
        );
        self
    }

    /// Enable interactive mode (--tty --interactive).
    #[must_use]
    pub fn interactive(mut self) -> Self {
        self.interactive = true;
        self
    }

    /// Set stdin data to send to the command.
    pub fn stdin(mut self, data: impl Into<Vec<u8>>) -> Self {
        self.stdin_data = Some(data.into());
        self
    }

    /// Build the command without executing it.
    ///
    /// Use this to access stream configuration methods on [`cmd_proc::Command`].
    #[must_use]
    pub fn build(self) -> Command {
        let mut command = self.container.backend_command().argument("exec");

        if self.interactive {
            command = command.argument("--tty").argument("--interactive");
        } else if self.stdin_data.is_some() {
            command = command.argument("--interactive");
        }

        for (key, value) in self.environment {
            command = command.argument("--env").argument(format!("{key}={value}"));
        }

        command = command
            .argument(&self.container.id)
            .argument(self.executable)
            .arguments(self.arguments);

        if let Some(data) = self.stdin_data {
            command = command.stdin_bytes(data);
        }

        command
    }

    /// Execute the command and return success or an error.
    pub async fn status(self) -> Result<(), CommandError> {
        self.build().status().await
    }
}

impl Container {
    /// Create an exec command builder for running commands inside this container.
    pub fn exec(&self, executable: impl Into<String>) -> ExecCommand<'_> {
        ExecCommand::new(self, executable)
    }

    pub async fn stop(&mut self) {
        self.backend_command()
            .arguments(["container", "stop"])
            .argument(&self.id)
            .stdout_capture()
            .bytes()
            .await
            .unwrap();
    }

    pub async fn remove(&mut self) {
        self.backend_command()
            .arguments(["container", "rm"])
            .argument(&self.id)
            .stdout_capture()
            .bytes()
            .await
            .unwrap();
    }

    pub async fn inspect(&self) -> serde_json::Value {
        let stdout = self
            .backend_command()
            .argument("inspect")
            .argument(&self.id)
            .stdout_capture()
            .bytes()
            .await
            .unwrap();

        serde_json::from_slice(&stdout).expect("invalid json")
    }

    pub async fn inspect_format(&self, format: &str) -> String {
        let bytes = self
            .backend_command()
            .argument("inspect")
            .argument("--format")
            .argument(format)
            .argument(&self.id)
            .stdout_capture()
            .bytes()
            .await
            .unwrap();

        std::str::from_utf8(strip_nl_end(&bytes))
            .expect("invalid utf8")
            .to_string()
    }

    pub async fn read_host_tcp_port(&self, container_port: u16) -> Option<u16> {
        let json = self.inspect().await;

        json.get(0)?
            .get("NetworkSettings")?
            .get("Ports")?
            .get(format!("{container_port}/tcp"))?
            .get(0)?
            .get("HostPort")?
            .as_str()?
            .parse()
            .ok()
    }

    pub async fn commit(
        &self,
        reference: &image::Reference,
        pause: bool,
    ) -> Result<(), CommandError> {
        let pause_argument = match (&self.backend, pause) {
            (Backend::Docker { .. }, true) => None,
            (Backend::Docker { version }, false) => {
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
