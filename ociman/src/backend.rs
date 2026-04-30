use cmd_proc::*;

#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Deserialize, clap::ValueEnum)]
#[serde(rename_all = "snake_case")]
pub enum Selection {
    Auto,
    Docker,
    Podman,
}

impl Selection {
    pub async fn resolve(&self) -> resolve::Result {
        match self {
            Self::Auto => resolve::auto().await,
            Self::Docker => resolve::docker().await,
            Self::Podman => resolve::podman().await,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Backend {
    Docker { version: semver::Version },
    Podman { version: semver::Version },
}

impl Backend {
    const DOCKER_EXECUTABLE: &'static str = "docker";
    const PODMAN_EXECUTABLE: &'static str = "podman";

    #[must_use]
    pub fn command(&self) -> Command {
        match self {
            Self::Docker { .. } => Command::new(Self::DOCKER_EXECUTABLE),
            Self::Podman { .. } => Command::new(Self::PODMAN_EXECUTABLE),
        }
    }

    /// Check if an image is present in the local registry.
    ///
    /// Uses each runtime's documented existence probe so "image absent" is
    /// properly distinguishable from "real failure" (binary missing, daemon
    /// down, storage error):
    ///
    /// - Docker: `docker image ls --filter reference=<ref> --quiet`. Exit 0
    ///   with non-empty stdout = present; exit 0 with empty stdout = absent;
    ///   any non-zero exit = real failure surfaced as
    ///   [`ImagePresentError::Subprocess`]. Reference:
    ///   <https://docs.docker.com/reference/cli/docker/image/ls/>.
    /// - Podman: `podman image exists -- <ref>`. Documented exits: 0 =
    ///   found, 1 = absent, 125 = storage error (mapped to
    ///   [`ImagePresentError::Subprocess`]). Reference:
    ///   <https://docs.podman.io/en/latest/markdown/podman-image-exists.1.html>.
    pub async fn is_image_present(
        &self,
        reference: &crate::image::Reference,
    ) -> Result<bool, ImagePresentError> {
        let reference_string = reference.to_string();

        match self {
            Self::Docker { .. } => {
                let result = self
                    .command()
                    .arguments([
                        "image",
                        "ls",
                        "--filter",
                        &format!("reference={reference_string}"),
                        "--quiet",
                    ])
                    .stdout_capture()
                    .stderr_capture()
                    .accept_nonzero_exit()
                    .run()
                    .await
                    .map_err(ImagePresentError::Command)?;

                if result.status.success() {
                    let stdout =
                        std::str::from_utf8(&result.stdout).map_err(ImagePresentError::Utf8)?;
                    Ok(!stdout.trim().is_empty())
                } else {
                    Err(ImagePresentError::Subprocess {
                        exit_status: result.status,
                        stderr: String::from_utf8_lossy(&result.stderr).into_owned(),
                    })
                }
            }
            Self::Podman { .. } => {
                let result = self
                    .command()
                    .arguments(["image", "exists", "--", &reference_string])
                    .stdout_capture()
                    .stderr_capture()
                    .accept_nonzero_exit()
                    .run()
                    .await
                    .map_err(ImagePresentError::Command)?;

                match result.status.code() {
                    Some(0) => Ok(true),
                    Some(1) => Ok(false),
                    _ => Err(ImagePresentError::Subprocess {
                        exit_status: result.status,
                        stderr: String::from_utf8_lossy(&result.stderr).into_owned(),
                    }),
                }
            }
        }
    }

    /// Check if a container is present in the local runtime.
    ///
    /// Uses each runtime's documented existence probe so "container absent"
    /// is properly distinguishable from "real failure":
    ///
    /// - Docker: `docker ps --all --quiet --filter id=<id>`. Exit 0 with
    ///   non-empty stdout = present; exit 0 with empty stdout = absent; any
    ///   non-zero exit = real failure surfaced as
    ///   [`ContainerPresentError::Subprocess`]. Reference:
    ///   <https://docs.docker.com/reference/cli/docker/container/ls/>.
    /// - Podman: `podman container exists -- <id>`. Documented exits: 0 =
    ///   found, 1 = absent, 125 = storage error (mapped to
    ///   [`ContainerPresentError::Subprocess`]). Reference:
    ///   <https://docs.podman.io/en/latest/markdown/podman-container-exists.1.html>.
    pub async fn is_container_present(
        &self,
        id: &crate::ContainerId,
    ) -> Result<bool, ContainerPresentError> {
        match self {
            Self::Docker { .. } => {
                let result = self
                    .command()
                    .arguments([
                        "ps",
                        "--all",
                        "--quiet",
                        "--filter",
                        &format!("id={}", id.as_str()),
                    ])
                    .stdout_capture()
                    .stderr_capture()
                    .accept_nonzero_exit()
                    .run()
                    .await
                    .map_err(ContainerPresentError::Command)?;

                if result.status.success() {
                    let stdout =
                        std::str::from_utf8(&result.stdout).map_err(ContainerPresentError::Utf8)?;
                    Ok(!stdout.trim().is_empty())
                } else {
                    Err(ContainerPresentError::Subprocess {
                        exit_status: result.status,
                        stderr: String::from_utf8_lossy(&result.stderr).into_owned(),
                    })
                }
            }
            Self::Podman { .. } => {
                let result = self
                    .command()
                    .arguments(["container", "exists", "--", id.as_str()])
                    .stdout_capture()
                    .stderr_capture()
                    .accept_nonzero_exit()
                    .run()
                    .await
                    .map_err(ContainerPresentError::Command)?;

                match result.status.code() {
                    Some(0) => Ok(true),
                    Some(1) => Ok(false),
                    _ => Err(ContainerPresentError::Subprocess {
                        exit_status: result.status,
                        stderr: String::from_utf8_lossy(&result.stderr).into_owned(),
                    }),
                }
            }
        }
    }

    /// Tag an image with a new name
    pub async fn tag_image(
        &self,
        source: &crate::image::Reference,
        target: &crate::image::Reference,
    ) {
        self.command()
            .arguments(["tag", &source.to_string(), &target.to_string()])
            .status()
            .await
            .unwrap();
    }

    /// Pull an image from a registry
    pub async fn pull_image(&self, reference: &crate::image::Reference) {
        self.command()
            .arguments(["pull", &reference.to_string()])
            .status()
            .await
            .unwrap();
    }

    /// Pull an image only if it's not already present.
    pub async fn pull_image_if_absent(
        &self,
        reference: &crate::image::Reference,
    ) -> Result<(), ImagePresentError> {
        if !self.is_image_present(reference).await? {
            self.pull_image(reference).await;
        }
        Ok(())
    }

    /// Push an image to a registry
    pub async fn push_image(&self, reference: &crate::image::Reference) {
        self.command()
            .arguments(["push", &reference.to_string()])
            .status()
            .await
            .unwrap();
    }

    pub async fn remove_image(&self, reference: &crate::image::Reference) {
        self.do_remove_image(reference, false).await;
    }

    pub async fn remove_image_force(&self, reference: &crate::image::Reference) {
        self.do_remove_image(reference, true).await;
    }

    async fn do_remove_image(&self, reference: &crate::image::Reference, force: bool) {
        let command = self.command().arguments(["image", "rm"]);
        let command = if force {
            command.argument("--force")
        } else {
            command
        };
        command
            .argument(reference.to_string())
            .status()
            .await
            .unwrap();
    }

    /// List image references by name (e.g., "pg-ephemeral/main")
    ///
    /// Note: Podman's `--filter reference=` with wildcards returns all tags sharing the same
    /// image ID, not just matching references. This is a known issue partially addressed in
    /// <https://github.com/containers/common/pull/2413>, but only for single fully-qualified
    /// references ("query mode"), not wildcard patterns ("search mode"). We filter results
    /// client-side to ensure only matching names are returned.
    pub async fn image_references_by_name(
        &self,
        name: &crate::reference::Name,
    ) -> std::collections::BTreeSet<crate::image::Reference> {
        let output = self
            .command()
            .arguments([
                "images",
                "--format",
                "{{.Repository}}:{{.Tag}}",
                "--filter",
                &format!("reference={name}:*"),
            ])
            .stdout_capture()
            .string()
            .await
            .unwrap();

        output
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| line.parse::<crate::image::Reference>().unwrap())
            .filter(|reference| reference.name.path == name.path)
            .collect()
    }

    /// Create a hostname resolver that runs inside a container
    ///
    /// This is useful for resolving DNS names that only work inside containers
    /// (e.g., host.docker.internal) or when you need to see how DNS resolves
    /// from within a containerized environment.
    ///
    /// # Example
    /// ```no_run
    /// async fn example() {
    ///     let ip = ociman::backend::resolve::auto()
    ///         .await
    ///         .unwrap()
    ///         .container_resolver()
    ///         .add_host("host.docker.internal:host-gateway")
    ///         .resolve("host.docker.internal")
    ///         .await
    ///         .unwrap();
    /// }
    /// ```
    #[must_use]
    pub fn container_resolver(&self) -> ContainerHostnameResolver {
        ContainerHostnameResolver::new(self.clone())
    }

    /// Resolve the container host to an IP address
    ///
    /// This is a convenience method that resolves the special hostname used to
    /// connect back to services running on the host machine from within containers.
    ///
    /// Uses host.containers.internal for Podman and host.docker.internal for Docker
    /// (requires --add-host on Linux).
    ///
    /// # Example
    /// ```no_run
    /// async fn example() {
    ///     let ip = ociman::backend::resolve::auto()
    ///         .await
    ///         .unwrap()
    ///         .resolve_container_host()
    ///         .await
    ///         .unwrap();
    /// }
    /// ```
    pub async fn resolve_container_host(&self) -> Result<std::net::IpAddr, ResolveHostnameError> {
        match self {
            Backend::Podman { .. } => {
                // Podman provides host.containers.internal natively
                self.container_resolver()
                    .resolve("host.containers.internal")
                    .await
            }
            Backend::Docker { .. } => {
                // Docker needs --add-host on Linux
                self.container_resolver()
                    .add_host("host.docker.internal:host-gateway")
                    .resolve("host.docker.internal")
                    .await
            }
        }
    }

    /// Inspect a container by id and return the raw JSON payload.
    ///
    /// On a non-zero subprocess exit, re-probes via
    /// [`Self::is_container_present`] to remap the error: a confirmed-absent
    /// target becomes [`crate::InspectError::NotFound`]; a still-present
    /// target produces [`crate::InspectError::Subprocess`] carrying the
    /// captured stderr; a probe failure surfaces as
    /// [`crate::InspectError::ContainerPresent`].
    pub async fn inspect_container(
        &self,
        id: &crate::ContainerId,
    ) -> Result<serde_json::Value, crate::InspectError> {
        let result = self
            .command()
            .arguments(["inspect", "--type", "container"])
            .argument(id)
            .stdout_capture()
            .stderr_capture()
            .accept_nonzero_exit()
            .run()
            .await
            .map_err(|error| match error {
                cmd_proc::CommandError::Io(io) => crate::InspectError::Io(io),
                cmd_proc::CommandError::ExitStatus(exit_status) => {
                    crate::InspectError::Subprocess {
                        exit_status,
                        stderr: String::new(),
                    }
                }
            })?;

        if !result.status.success() {
            return Err(match self.is_container_present(id).await {
                Ok(false) => crate::InspectError::NotFound,
                Ok(true) => crate::InspectError::Subprocess {
                    exit_status: result.status,
                    stderr: String::from_utf8_lossy(&result.stderr).into_owned(),
                },
                Err(probe) => crate::InspectError::ContainerPresent(probe),
            });
        }

        Ok(serde_json::from_slice(&result.stdout)?)
    }

    /// Run `inspect --format` against a container and return the rendered
    /// stdout (with trailing newline stripped).
    ///
    /// Same probe-based remapping as [`Self::inspect_container`].
    pub async fn inspect_container_format(
        &self,
        id: &crate::ContainerId,
        format: &str,
    ) -> Result<String, crate::InspectError> {
        let result = self
            .command()
            .arguments(["inspect", "--format"])
            .argument(format)
            .argument(id)
            .stdout_capture()
            .stderr_capture()
            .accept_nonzero_exit()
            .run()
            .await
            .map_err(|error| match error {
                cmd_proc::CommandError::Io(io) => crate::InspectError::Io(io),
                cmd_proc::CommandError::ExitStatus(exit_status) => {
                    crate::InspectError::Subprocess {
                        exit_status,
                        stderr: String::new(),
                    }
                }
            })?;

        if !result.status.success() {
            return Err(match self.is_container_present(id).await {
                Ok(false) => crate::InspectError::NotFound,
                Ok(true) => crate::InspectError::Subprocess {
                    exit_status: result.status,
                    stderr: String::from_utf8_lossy(&result.stderr).into_owned(),
                },
                Err(probe) => crate::InspectError::ContainerPresent(probe),
            });
        }

        Ok(std::str::from_utf8(crate::container::strip_nl_end(&result.stdout))?.to_string())
    }

    /// Read the labels on a container by id.
    pub async fn container_labels(
        &self,
        id: &crate::ContainerId,
    ) -> Result<crate::label::ContainerLabels, crate::label::ContainerError> {
        let value = self.inspect_container(id).await?;
        crate::label::decode_labels(&value)
    }

    /// Parse a backend-supplied container name string into a validated
    /// [`crate::ContainerName`], normalising backend-specific quirks.
    ///
    /// On Docker the inspect `Name` field is conventionally prefixed with `/`;
    /// that prefix is stripped here so the returned name matches what was
    /// originally passed via `--name`. Podman emits the bare name and is
    /// left untouched.
    fn parse_container_name(
        &self,
        raw: &str,
    ) -> Result<crate::ContainerName, crate::ContainerNameError> {
        let normalised = match self {
            Backend::Docker { .. } => raw.strip_prefix('/').unwrap_or(raw),
            Backend::Podman { .. } => raw,
        };
        normalised.parse()
    }

    /// Read the name of a container by id.
    pub async fn container_name(
        &self,
        id: &crate::ContainerId,
    ) -> Result<crate::ContainerName, crate::container::ReadContainerNameError> {
        let value = self.inspect_container(id).await?;
        let raw = value
            .get(0)
            .and_then(|entry| entry.get("Name"))
            .and_then(|value| value.as_str())
            .ok_or(crate::container::ReadContainerNameError::NameNotString)?;
        Ok(self.parse_container_name(raw)?)
    }

    /// Inspect an image by reference and return the raw JSON payload.
    ///
    /// On a non-zero subprocess exit, re-probes via
    /// [`Self::is_image_present`] to remap the error: a confirmed-absent
    /// target becomes [`crate::InspectError::NotFound`]; a still-present
    /// target produces [`crate::InspectError::Subprocess`] carrying the
    /// captured stderr; a probe failure surfaces as
    /// [`crate::InspectError::ImagePresent`].
    pub async fn inspect_image(
        &self,
        reference: &crate::image::Reference,
    ) -> Result<serde_json::Value, crate::InspectError> {
        let result = self
            .command()
            .arguments(["inspect", "--type", "image"])
            .argument(reference.to_string())
            .stdout_capture()
            .stderr_capture()
            .accept_nonzero_exit()
            .run()
            .await
            .map_err(|error| match error {
                cmd_proc::CommandError::Io(io) => crate::InspectError::Io(io),
                cmd_proc::CommandError::ExitStatus(exit_status) => {
                    crate::InspectError::Subprocess {
                        exit_status,
                        stderr: String::new(),
                    }
                }
            })?;

        if !result.status.success() {
            return Err(match self.is_image_present(reference).await {
                Ok(false) => crate::InspectError::NotFound,
                Ok(true) => crate::InspectError::Subprocess {
                    exit_status: result.status,
                    stderr: String::from_utf8_lossy(&result.stderr).into_owned(),
                },
                Err(probe) => crate::InspectError::ImagePresent(probe),
            });
        }

        Ok(serde_json::from_slice(&result.stdout)?)
    }

    /// Read the labels on an image by reference.
    pub async fn image_labels(
        &self,
        reference: &crate::image::Reference,
    ) -> Result<crate::label::ImageLabels, crate::label::ImageError> {
        let value = self.inspect_image(reference).await?;
        crate::label::decode_labels(&value)
    }

    /// List all containers (running or stopped) carrying a given label.
    ///
    /// When `value` is `None`, matches any container with the key set,
    /// regardless of value. When `Some`, matches only containers where the key
    /// has exactly that value.
    pub async fn list_containers_by_label(
        &self,
        key: &crate::label::Key,
        value: Option<&crate::label::Value>,
    ) -> Result<Vec<crate::Container>, ListContainersError> {
        let filter = match value {
            None => format!("label={key}"),
            Some(value) => format!("label={key}={value}"),
        };

        let stdout = self
            .command()
            .arguments([
                "ps",
                "--all",
                "--no-trunc",
                "--format",
                "{{.ID}}",
                "--filter",
                &filter,
            ])
            .stdout_capture()
            .string()
            .await?;

        Ok(stdout
            .lines()
            .filter(|line| !line.is_empty())
            .map(|id| crate::Container {
                backend: self.clone(),
                id: crate::ContainerId(id.to_string()),
            })
            .collect())
    }

    /// Inspect the default bridge network and return its subnets.
    ///
    /// Returns the subnet CIDRs of the default bridge network:
    /// - Docker: inspects the `bridge` network
    /// - Podman: inspects the `podman` network
    pub async fn bridge_subnets(&self) -> Result<Vec<ipnet::IpNet>, BridgeSubnetError> {
        let stdout = self
            .command()
            .arguments(match self {
                Self::Docker { .. } => ["network", "inspect", "bridge"],
                Self::Podman { .. } => ["network", "inspect", "podman"],
            })
            .stdout_capture()
            .bytes()
            .await
            .map_err(BridgeSubnetError::CommandFailed)?;

        match self {
            Self::Docker { .. } => {
                let networks: Vec<DockerNetworkInspect> =
                    serde_json::from_slice(&stdout).map_err(BridgeSubnetError::JsonParseFailed)?;

                Ok(networks
                    .into_iter()
                    .flat_map(|n| n.ipam.config)
                    .map(|c| c.subnet)
                    .collect())
            }
            Self::Podman { .. } => {
                let networks: Vec<PodmanNetworkInspect> =
                    serde_json::from_slice(&stdout).map_err(BridgeSubnetError::JsonParseFailed)?;

                Ok(networks
                    .into_iter()
                    .flat_map(|n| n.subnets)
                    .map(|s| s.subnet)
                    .collect())
            }
        }
    }
}

#[derive(serde::Deserialize)]
struct DockerNetworkInspect {
    #[serde(rename = "IPAM")]
    ipam: DockerIpam,
}

#[derive(serde::Deserialize)]
struct DockerIpam {
    #[serde(rename = "Config")]
    config: Vec<DockerIpamConfig>,
}

#[derive(serde::Deserialize)]
struct DockerIpamConfig {
    #[serde(rename = "Subnet")]
    subnet: ipnet::IpNet,
}

#[derive(serde::Deserialize)]
struct PodmanNetworkInspect {
    subnets: Vec<PodmanSubnet>,
}

#[derive(serde::Deserialize)]
struct PodmanSubnet {
    subnet: ipnet::IpNet,
}

#[derive(Debug, thiserror::Error)]
pub enum ImagePresentError {
    /// Subprocess could not be started or failed at the IO layer.
    #[error("image existence probe failed")]
    Command(#[source] cmd_proc::CommandError),
    /// Subprocess exited non-zero with an unrecognised status — not the
    /// runtime's documented "absent" signal, so treated as a real failure.
    /// The captured stderr is preserved for diagnostics.
    #[error("image existence probe exited with {exit_status}: {stderr}")]
    Subprocess {
        exit_status: std::process::ExitStatus,
        stderr: String,
    },
    /// Probe stdout was not valid UTF-8 (only relevant on Docker, where the
    /// probe parses image IDs from stdout).
    #[error("image existence probe stdout was not valid UTF-8")]
    Utf8(#[source] std::str::Utf8Error),
}

#[derive(Debug, thiserror::Error)]
pub enum ContainerPresentError {
    /// Subprocess could not be started or failed at the IO layer.
    #[error("container existence probe failed")]
    Command(#[source] cmd_proc::CommandError),
    /// Subprocess exited non-zero with an unrecognised status — not the
    /// runtime's documented "absent" signal, so treated as a real failure.
    /// The captured stderr is preserved for diagnostics.
    #[error("container existence probe exited with {exit_status}: {stderr}")]
    Subprocess {
        exit_status: std::process::ExitStatus,
        stderr: String,
    },
    /// Probe stdout was not valid UTF-8 (only relevant on Docker, where the
    /// probe parses container IDs from stdout).
    #[error("container existence probe stdout was not valid UTF-8")]
    Utf8(#[source] std::str::Utf8Error),
}

#[derive(Debug, thiserror::Error)]
pub enum ListContainersError {
    #[error("`docker ps` / `podman ps` command failed")]
    Command(#[from] cmd_proc::CommandError),
}

#[derive(Debug, thiserror::Error)]
pub enum BridgeSubnetError {
    #[error("network inspect command failed")]
    CommandFailed(#[source] cmd_proc::CommandError),

    #[error("failed to parse network inspect JSON")]
    JsonParseFailed(#[source] serde_json::Error),
}

#[derive(Clone, Debug, thiserror::Error)]
pub enum ResolveHostnameError {
    #[error("hostname resolution command failed: {0}")]
    CommandFailed(String),

    #[error("Invalid UTF-8 in resolution output")]
    InvalidUtf8,

    #[error("No IP address found in resolution output for hostname: {0}")]
    NoIpAddressFound(String),

    #[error("Failed to parse IP address from resolution output: {source}")]
    ParseError {
        output: String,
        #[source]
        source: std::net::AddrParseError,
    },
}

/// Resolves hostnames from within a container environment
///
/// This allows you to resolve DNS names as they would appear from within
/// a container, which is useful for names like host.docker.internal or
/// service names in custom Docker networks.
pub struct ContainerHostnameResolver {
    backend: Backend,
    container_arguments: Vec<String>,
}

impl ContainerHostnameResolver {
    fn new(backend: Backend) -> Self {
        Self {
            backend,
            container_arguments: vec![],
        }
    }

    /// Add a custom host mapping (--add-host)
    pub fn add_host(mut self, mapping: impl Into<String>) -> Self {
        self.container_arguments.push("--add-host".to_string());
        self.container_arguments.push(mapping.into());
        self
    }

    /// Specify a Docker/Podman network to use (--network)
    pub fn network(mut self, network: impl Into<String>) -> Self {
        self.container_arguments.push("--network".to_string());
        self.container_arguments.push(network.into());
        self
    }

    /// Add a custom container argument
    pub fn argument(mut self, argument: impl Into<String>) -> Self {
        self.container_arguments.push(argument.into());
        self
    }

    /// Add multiple custom container arguments
    pub fn arguments(mut self, arguments: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.container_arguments
            .extend(arguments.into_iter().map(Into::into));
        self
    }

    /// Resolve the hostname to an IP address
    ///
    /// If multiple IP addresses are available for the hostname, returns the first one.
    ///
    /// # Arguments
    /// * `hostname` - The hostname to resolve
    ///
    /// # Returns
    /// The resolved IP address (supports both IPv4 and IPv6)
    pub async fn resolve(self, hostname: &str) -> Result<std::net::IpAddr, ResolveHostnameError> {
        const ALPINE_IMAGE: &str = "alpine:latest";

        let output = self
            .backend
            .command()
            .argument("run")
            .argument("--rm")
            .arguments(&self.container_arguments)
            .argument(ALPINE_IMAGE)
            .argument("getent")
            .argument("hosts")
            .argument(hostname)
            .stdout_capture()
            .bytes()
            .await
            .map_err(|error| ResolveHostnameError::CommandFailed(error.to_string()))?;

        // Parse output: "IP_ADDRESS HOSTNAME [ALIASES...]"
        // Extract the first IP address from the output
        let output_str =
            std::str::from_utf8(&output).map_err(|_| ResolveHostnameError::InvalidUtf8)?;

        let ip_str = output_str
            .split_whitespace()
            .next()
            .ok_or_else(|| ResolveHostnameError::NoIpAddressFound(hostname.to_string()))?;

        ip_str
            .parse()
            .map_err(|parse_error| ResolveHostnameError::ParseError {
                output: output_str.to_string(),
                source: parse_error,
            })
    }
}

pub mod resolve {
    use super::{Backend, Command};

    const ENV_VARIABLE_NAME: &str = "OCIMAN_BACKEND";

    pub type Result = std::result::Result<Backend, Error>;

    #[derive(Debug, thiserror::Error)]
    pub enum Error {
        #[error("Failed to load config")]
        ConfigLoad(#[source] crate::config::Error),
        #[error(
            "Invalid env variable for {ENV_VARIABLE_NAME}, expected \"podman\" or \"docker\", got: {0}"
        )]
        InvalidEnvVariable(String),
        #[error("No container tool detected in $PATH, searched for podman and docker")]
        NoContainerToolDetected,
        #[error("Failed to detect {executable} version: {message}")]
        VersionDetectionFailed {
            executable: &'static str,
            message: String,
        },
        #[error("Failed to parse {executable} version from output '{output}': {message}")]
        VersionParseFailed {
            executable: &'static str,
            output: String,
            message: String,
        },
    }

    /// Resolve backend automatically based on env var, config file, or available tools
    pub async fn auto() -> Result {
        match std::env::var(ENV_VARIABLE_NAME) {
            Err(std::env::VarError::NotPresent) => {
                let config = crate::config::Config::load().map_err(Error::ConfigLoad)?;
                from_present_tool(config.default_backend).await
            }
            Err(std::env::VarError::NotUnicode(_)) => {
                panic!("{ENV_VARIABLE_NAME} env variable exist but is not unicode!")
            }
            Ok(value) => from_env_value(&value).await,
        }
    }

    /// Resolve docker backend with version detection
    pub async fn docker() -> Result {
        detect_version(Backend::DOCKER_EXECUTABLE, |version| Backend::Docker {
            version,
        })
        .await
    }

    /// Resolve podman backend with version detection
    pub async fn podman() -> Result {
        detect_version(Backend::PODMAN_EXECUTABLE, |version| Backend::Podman {
            version,
        })
        .await
    }

    async fn from_env_value(value: &str) -> Result {
        match value {
            "docker" => docker().await,
            "podman" => podman().await,
            _ => Err(Error::InvalidEnvVariable(value.to_string())),
        }
    }

    async fn from_present_tool(preferred: super::Selection) -> Result {
        match preferred {
            super::Selection::Podman => match podman().await {
                Ok(backend) => Ok(backend),
                Err(_) => docker().await.map_err(|_| Error::NoContainerToolDetected),
            },
            super::Selection::Docker | super::Selection::Auto => match docker().await {
                Ok(backend) => Ok(backend),
                Err(_) => podman().await.map_err(|_| Error::NoContainerToolDetected),
            },
        }
    }

    async fn detect_version(
        executable: &'static str,
        constructor: impl FnOnce(semver::Version) -> Backend,
    ) -> Result {
        let output = Command::new(executable)
            .argument("--version")
            .stdout_capture()
            .bytes()
            .await
            .map_err(|error| Error::VersionDetectionFailed {
                executable,
                message: error.to_string(),
            })?;

        let output_str =
            std::str::from_utf8(&output).map_err(|_| Error::VersionDetectionFailed {
                executable,
                message: "invalid UTF-8 in version output".to_string(),
            })?;

        let version = parse_version(executable, output_str)?;

        log::debug!("ociman using: {executable} {version}");

        Ok(constructor(version))
    }

    fn parse_version(
        executable: &'static str,
        output: &str,
    ) -> std::result::Result<semver::Version, Error> {
        // Extract version string from output like:
        // Docker: "Docker version 29.0.0, build abcdef1"
        // Podman: "podman version 4.9.0"
        let version_str = output
            .split_whitespace()
            .find(|word| word.chars().next().is_some_and(|c| c.is_ascii_digit()))
            .map(|s| s.trim_end_matches(','))
            .ok_or_else(|| Error::VersionDetectionFailed {
                executable,
                message: format!("no version found in output: {output}"),
            })?;

        semver::Version::parse(version_str).map_err(|error| Error::VersionParseFailed {
            executable,
            output: output.to_string(),
            message: error.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_container_resolver_localhost() {
        let backend = crate::test_backend_setup!();

        let ip = backend
            .container_resolver()
            .resolve("localhost")
            .await
            .unwrap();

        assert!(ip.is_loopback());
    }

    #[tokio::test]
    async fn test_container_resolver_with_add_host() {
        let backend = crate::test_backend_setup!();

        let ip = backend
            .container_resolver()
            .add_host("host.docker.internal:host-gateway")
            .resolve("host.docker.internal")
            .await
            .unwrap();

        // Should resolve to some IP address
        assert!(ip.is_ipv4() || ip.is_ipv6());
    }

    #[tokio::test]
    async fn test_container_resolver_nonexistent() {
        let backend = crate::test_backend_setup!();

        let result = backend
            .container_resolver()
            .resolve("this-definitely-does-not-exist-12345.local")
            .await;

        assert!(result.is_err());
        match result {
            Err(ResolveHostnameError::CommandFailed(_)) => {
                // Expected: hostname resolution will fail for nonexistent hostname
            }
            other => panic!("Expected CommandFailed error, got: {other:?}"),
        }
    }

    #[tokio::test]
    async fn test_container_resolver_with_multiple_arguments() {
        let backend = crate::test_backend_setup!();

        let ip = backend
            .container_resolver()
            .add_host("custom-host:192.168.1.100")
            .resolve("custom-host")
            .await
            .unwrap();

        assert_eq!(
            ip,
            std::net::IpAddr::V4(std::net::Ipv4Addr::new(192, 168, 1, 100))
        );
    }

    #[tokio::test]
    async fn test_container_resolver_builder_pattern() {
        let backend = crate::test_backend_setup!();

        let resolver = backend
            .container_resolver()
            .argument("--add-host")
            .argument("test-host:10.0.0.1");

        let ip = resolver.resolve("test-host").await.unwrap();

        assert_eq!(
            ip,
            std::net::IpAddr::V4(std::net::Ipv4Addr::new(10, 0, 0, 1))
        );
    }

    #[tokio::test]
    async fn test_resolve_container_host() {
        let backend = crate::test_backend_setup!();

        let ip = backend.resolve_container_host().await.unwrap();

        // Should resolve to some IP address
        assert!(ip.is_ipv4() || ip.is_ipv6());
    }

    #[test]
    fn test_docker_bridge_json_parsing() {
        let json = r#"[{
            "Name": "bridge",
            "IPAM": {
                "Config": [{"Subnet": "172.17.0.0/16", "Gateway": "172.17.0.1"}]
            }
        }]"#;

        let networks: Vec<DockerNetworkInspect> = serde_json::from_str(json).unwrap();
        assert_eq!(
            networks[0].ipam.config[0].subnet.to_string(),
            "172.17.0.0/16"
        );
    }

    #[test]
    fn test_podman_bridge_json_parsing() {
        let json = r#"[{
            "name": "podman",
            "subnets": [{"subnet": "10.88.0.0/16", "gateway": "10.88.0.1"}]
        }]"#;

        let networks: Vec<PodmanNetworkInspect> = serde_json::from_str(json).unwrap();
        assert_eq!(networks[0].subnets[0].subnet.to_string(), "10.88.0.0/16");
    }

    #[tokio::test]
    async fn test_image_references_by_name() {
        use std::collections::BTreeSet;

        let backend = crate::test_backend_setup!();

        // Use localhost prefix to match how Podman canonicalizes local images
        let name: crate::reference::Name = "localhost/ociman-test/image-references-by-name"
            .parse()
            .unwrap();

        // Clean up any existing images with this name
        for image in backend.image_references_by_name(&name).await {
            backend.remove_image_force(&image).await;
        }

        // Create test images by tagging alpine
        let source = crate::testing::ALPINE_LATEST_IMAGE.clone();
        backend.pull_image_if_absent(&source).await.unwrap();

        let target_a: crate::image::Reference = "localhost/ociman-test/image-references-by-name:a"
            .parse()
            .unwrap();
        let target_b: crate::image::Reference = "localhost/ociman-test/image-references-by-name:b"
            .parse()
            .unwrap();

        backend.tag_image(&source, &target_a).await;
        backend.tag_image(&source, &target_b).await;

        assert_eq!(
            backend.image_references_by_name(&name).await,
            BTreeSet::from([target_a.clone(), target_b.clone()])
        );

        // Clean up
        backend.remove_image_force(&target_a).await;
        backend.remove_image_force(&target_b).await;
    }
}
