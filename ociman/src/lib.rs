#![doc = include_str!("../README.md")]

pub mod backend;
pub mod command;
pub mod image;
pub mod platform;
pub mod testing;

pub use backend::{Backend, ContainerHostnameResolver, ResolveHostnameError};
pub use command::Command;
pub use image::{
    BuildArgumentKey, BuildArgumentKeyError, BuildArgumentValue, BuildDefinition, BuildSource,
    ImageName,
};
use std::ffi::OsStr;

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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Image(String);

string_newtype!(Image);

impl Apply for Image {
    fn apply(&self, command: Command) -> Command {
        command.argument(self)
    }
}

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
            write!(formatter, "{}", port)
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
            write!(formatter, "{}:", host_binding)?;
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
pub struct EnvironmentVariables(std::collections::BTreeMap<String, String>);

impl EnvironmentVariables {
    fn new() -> Self {
        Self(std::collections::BTreeMap::new())
    }

    fn insert(&mut self, key: String, value: String) {
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
    image: Image,
    remove: Remove,
    stop_on_drop: bool,
    remove_on_drop: bool,
    mounts: Vec<Mount>,
    publish: Vec<Publish>,
    workdir: Option<Workdir>,
}

impl Definition {
    pub fn new(backend: Backend, image: Image) -> Definition {
        Definition {
            backend,
            container_arguments: vec![],
            detach: Detach::NoDetach,
            entrypoint: None,
            environment_variables: EnvironmentVariables::new(),
            image,
            mounts: vec![],
            publish: vec![],
            remove: Remove::NoRemove,
            stop_on_drop: false,
            remove_on_drop: false,
            workdir: None,
        }
    }

    /// Runs a detached container and passes it to the provided closure.
    ///
    /// The container is automatically stopped when dropped (after the closure returns
    /// or on panic).
    pub fn with_container<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut Container) -> R,
    {
        let mut container = self.clone().stop_on_drop().run_detached();
        f(&mut container)
    }

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

    pub fn environment_variable(self, key: &str, value: &str) -> Self {
        let mut environment_variables = self.environment_variables;

        environment_variables.insert(key.to_string(), value.to_string());

        Self {
            environment_variables,
            ..self
        }
    }

    pub fn environment_variables<K: ToString, V: ToString>(
        self,
        values: impl IntoIterator<Item = (K, V)>,
    ) -> Self {
        let mut environment_variables = self.environment_variables;

        for (key, value) in values {
            environment_variables.insert(key.to_string(), value.to_string());
        }

        Self {
            environment_variables,
            ..self
        }
    }

    pub fn remove(self) -> Self {
        Self {
            remove: Remove::Remove,
            ..self
        }
    }

    pub fn no_remove(self) -> Self {
        Self {
            remove: Remove::NoRemove,
            ..self
        }
    }

    /// Marks the container to be stopped when the Container handle is dropped.
    ///
    /// By default containers are not stopped on drop. Use this when you want
    /// automatic cleanup of running containers when the handle goes out of scope.
    pub fn stop_on_drop(self) -> Self {
        Self {
            stop_on_drop: true,
            ..self
        }
    }

    /// Marks the container for removal when the Container handle is dropped.
    ///
    /// This is different from `remove()` which uses `--rm` flag:
    /// - `remove()` → docker/podman removes container when it stops (can't commit)
    /// - `remove_on_drop()` → Rust removes container on Drop (can commit stopped container)
    ///
    /// Use this when you need to stop a container, commit it, then clean up.
    pub fn remove_on_drop(self) -> Self {
        Self {
            remove_on_drop: true,
            ..self
        }
    }

    pub fn detach(self) -> Self {
        Self {
            detach: Detach::Detach,
            ..self
        }
    }

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

    pub fn run_detached(&self) -> Container {
        let stdout = self.clone().detach().run_output();

        Container {
            backend: self.backend,
            id: ContainerId::try_from(strip_nl_end(&stdout)).unwrap(),
            stopped: false,
            removed: false,
            stop_on_drop: self.stop_on_drop,
            remove_on_drop: self.remove_on_drop,
        }
    }

    pub fn run_capture_only_stdout(&self) -> Vec<u8> {
        self.clone().no_detach().run_output()
    }

    /// Runs the container and returns the exit status.
    pub fn run_status(&self) -> std::process::ExitStatus {
        self.build_run_command().status()
    }

    /// Runs the container and panics on non-zero exit.
    pub fn run_status_success(&self) {
        let status = self.run_status();

        if !status.success() {
            panic!("Container execution failed with status: {status}");
        }
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
        let command = self.image.apply(command);

        self.container_arguments.apply(command)
    }

    fn run_output(&self) -> Vec<u8> {
        self.build_run_command().capture_only_stdout()
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
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug)]
pub struct Container {
    backend: Backend,
    id: ContainerId,
    stopped: bool,
    removed: bool,
    stop_on_drop: bool,
    remove_on_drop: bool,
}

impl Container {
    pub fn stop(&mut self) {
        self.backend_command()
            .arguments(["container", "stop"])
            .argument(&self.id)
            .capture_only_stdout();

        self.stopped = true;
    }

    pub fn remove(&mut self) {
        self.backend_command()
            .arguments(["container", "rm"])
            .argument(&self.id)
            .capture_only_stdout();

        self.removed = true;
    }

    pub fn exec_capture_only_stdout<T: AsRef<OsStr>>(
        &self,
        environment: impl IntoIterator<Item = (&'static str, String)>,
        executable: T,
        arguments: impl IntoIterator<Item = T>,
    ) -> Vec<u8> {
        self.backend_command()
            .argument("exec")
            .arguments(
                environment
                    .into_iter()
                    .flat_map(|(key, value)| ["--env".to_string(), format!("{key}={value}")]),
            )
            .argument(&self.id)
            .argument(executable)
            .arguments(arguments)
            .capture_only_stdout()
    }

    pub fn exec_interactive<T: AsRef<OsStr>>(
        &self,
        environment: impl IntoIterator<Item = (&'static str, String)>,
        executable: T,
        arguments: impl IntoIterator<Item = T>,
    ) {
        let _status = self
            .backend_command()
            .argument("exec")
            .argument("--tty")
            .argument("--interactive")
            .arguments(
                environment
                    .into_iter()
                    .flat_map(|(key, value)| ["--env".to_string(), format!("{key}={value}")]),
            )
            .argument(&self.id)
            .argument(executable)
            .arguments(arguments)
            .status();
    }

    pub fn inspect(&self) -> serde_json::Value {
        let stdout = self
            .backend_command()
            .argument("inspect")
            .argument(&self.id)
            .capture_only_stdout();

        serde_json::from_slice(&stdout).expect("invalid json")
    }

    pub fn inspect_format(&self, format: &str) -> String {
        let bytes = self
            .backend_command()
            .argument("inspect")
            .argument("--format")
            .argument(format)
            .argument(&self.id)
            .capture_only_stdout();

        std::str::from_utf8(strip_nl_end(&bytes))
            .expect("invalid utf8")
            .to_string()
    }

    pub fn read_host_tcp_port(&self, container_port: u16) -> Option<u16> {
        let json = self.inspect();

        json.get(0)?
            .get("NetworkSettings")?
            .get("Ports")?
            .get(format!("{}/tcp", container_port))?
            .get(0)?
            .get("HostPort")?
            .as_str()?
            .parse()
            .ok()
    }

    pub fn commit(&self, image: &Image, pause: bool) {
        let pause_argument = match (self.backend, pause) {
            (Backend::Docker, true) => None,
            (Backend::Docker, false) => Some("--no-pause"),
            (Backend::Podman, true) => Some("--pause"),
            (Backend::Podman, false) => None,
        };

        self.backend_command()
            .argument("commit")
            .optional_argument(pause_argument)
            .argument(&self.id)
            .argument(image.as_str())
            .status();
    }

    fn backend_command(&self) -> Command {
        self.backend.command()
    }
}

impl Drop for Container {
    fn drop(&mut self) {
        if self.stop_on_drop && !self.stopped {
            self.stop()
        }

        if self.remove_on_drop && !self.removed {
            self.remove()
        }
    }
}
