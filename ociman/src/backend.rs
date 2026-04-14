use cmd_proc::*;

/// Substring used to detect the OCI distribution-spec `MANIFEST_UNKNOWN`
/// error code as rendered on stderr by docker, podman, and skopeo when a
/// registry reports that a tag does not exist.
///
/// **This is load-bearing string matching and we do not like it.** We fall
/// back on it because there is no better option available today:
///
/// - Neither `docker pull` nor `podman pull` has a `--json` / `--format`
///   flag. The CLIs only expose human-readable stderr.
/// - Exit codes are useless: both tools return `1` (or `125` for podman)
///   for every failure mode — not-found, auth, network, tls — without
///   discrimination.
/// - The docker/podman engine REST APIs do stream NDJSON, but the error
///   `message` / `errorDetail.message` fields contain the same human
///   string (`... manifest unknown`) — the daemons do not surface the
///   registry's structured error code — so switching to the socket would
///   just move the substring match from stderr to a JSON field, at the
///   cost of a ~400KB HTTP client dependency. No actual signal gain.
/// - `docker manifest inspect` still requires `experimental: enabled` as
///   of Docker 28, and `podman manifest inspect` is local-only. `skopeo
///   inspect` is clean but is a separate binary not always installed
///   (Docker Desktop ships without it).
/// - The only path to a clean spec-defined signal is talking to the
///   registry HTTP API directly, which means reimplementing bearer-token
///   auth, cred-helper integration, and auth challenges — substantial
///   work for a library that otherwise just shells out to the CLI.
///
/// The OCI Distribution Spec v1.1.0 defines the `MANIFEST_UNKNOWN` error
/// code (code-7) that registries MUST return when a manifest is absent:
/// <https://github.com/opencontainers/distribution-spec/blob/v1.1.0/spec.md#error-codes>
///
/// **However the spec does not mandate this stderr string.** The spec only
/// mandates the uppercase `code` field in the registry's JSON response;
/// the human-readable `message` field is OPTIONAL and its content is
/// unspecified. The lowercase `"manifest unknown"` substring this constant
/// matches is a de-facto convention that docker, podman, and skopeo all
/// happen to use when rendering the error to stderr. If a future CLI
/// version changes its wording, this constant must be updated and a
/// corresponding test will break.
const MANIFEST_UNKNOWN_STDERR_SIGNAL: &str = "manifest unknown";

#[derive(Debug, thiserror::Error)]
pub enum PullError {
    // The `reference` field is boxed because `image::Reference` is ~176
    // bytes (Name + Vec of PathComponents + Tag + Digest), and
    // `clippy::result-large-err` trips on anything >128 bytes inside a
    // `Result::Err`.
    #[error("image not found in registry: {reference}")]
    NotFound {
        reference: Box<crate::image::Reference>,
    },
    #[error("pull failed for {reference}: {message}")]
    Other {
        reference: Box<crate::image::Reference>,
        message: String,
    },
}

/// Turn a pull subprocess exit status + captured stderr into a
/// [`PullError`] (or [`Ok`] on success).
///
/// Split out from [`Backend::pull_image`] so it can be unit-tested with
/// canned stderr bytes — no network, no daemon, no registry.
fn classify_pull_result(
    reference: &crate::image::Reference,
    success: bool,
    stderr: &[u8],
) -> Result<(), PullError> {
    if success {
        return Ok(());
    }

    let stderr = String::from_utf8_lossy(stderr);
    if stderr.contains(MANIFEST_UNKNOWN_STDERR_SIGNAL) {
        Err(PullError::NotFound {
            reference: Box::new(reference.clone()),
        })
    } else {
        Err(PullError::Other {
            reference: Box::new(reference.clone()),
            message: stderr.trim().to_string(),
        })
    }
}

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

    /// Check if an image is present in the local registry
    pub async fn is_image_present(&self, reference: &crate::image::Reference) -> bool {
        let reference_string = reference.to_string();

        match self {
            Backend::Docker { .. } => self
                .command()
                .arguments(["inspect", "--type", "image", &reference_string])
                .stdout_capture()
                .bytes()
                .await
                .is_ok(),
            Backend::Podman { .. } => {
                // For Podman, image exists returns 0 if present, 1 if not
                // We use status() instead of capture because we don't need output
                self.command()
                    .arguments(["image", "exists", &reference_string])
                    .status()
                    .await
                    .is_ok()
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

    /// Pull an image from a registry.
    ///
    /// Stdout streams to the parent so users see layer progress. Stderr is
    /// captured and parsed to distinguish [`PullError::NotFound`] (registry
    /// reports `manifest unknown`) from other failures.
    pub async fn pull_image(&self, reference: &crate::image::Reference) -> Result<(), PullError> {
        let output = self
            .command()
            .arguments(["pull", &reference.to_string()])
            .stderr_capture()
            .accept_nonzero_exit()
            .run()
            .await
            .unwrap();

        classify_pull_result(reference, output.status.success(), &output.bytes)
    }

    /// Pull an image only if it's not already present locally.
    pub async fn pull_image_if_absent(
        &self,
        reference: &crate::image::Reference,
    ) -> Result<(), PullError> {
        if self.is_image_present(reference).await {
            Ok(())
        } else {
            self.pull_image(reference).await
        }
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

    fn pull_test_reference() -> crate::image::Reference {
        "ghcr.io/myorg/pg-ephemeral/main:abc123".parse().unwrap()
    }

    #[test]
    fn test_classify_pull_result_success() {
        let reference = pull_test_reference();
        assert!(classify_pull_result(&reference, true, b"").is_ok());
    }

    #[test]
    fn test_classify_pull_result_not_found_podman() {
        let reference = pull_test_reference();
        // Representative podman stderr for a non-existent tag.
        let stderr = b"Error: initializing source docker://ghcr.io/myorg/pg-ephemeral/main:abc123: reading manifest abc123 in ghcr.io/myorg/pg-ephemeral/main: manifest unknown";
        match classify_pull_result(&reference, false, stderr) {
            Err(PullError::NotFound { reference: r }) => assert_eq!(*r, reference),
            other => panic!("expected PullError::NotFound, got {other:?}"),
        }
    }

    #[test]
    fn test_classify_pull_result_not_found_docker() {
        let reference = pull_test_reference();
        // Representative docker stderr for a non-existent tag.
        let stderr = b"Error response from daemon: manifest for ghcr.io/myorg/pg-ephemeral/main:abc123 not found: manifest unknown: manifest unknown";
        match classify_pull_result(&reference, false, stderr) {
            Err(PullError::NotFound { reference: r }) => assert_eq!(*r, reference),
            other => panic!("expected PullError::NotFound, got {other:?}"),
        }
    }

    #[test]
    fn test_classify_pull_result_auth_failure_is_other() {
        let reference = pull_test_reference();
        // Auth failure must NOT be misclassified as NotFound.
        let stderr = b"Error response from daemon: pull access denied for ghcr.io/myorg/pg-ephemeral/main, repository does not exist or may require 'docker login': denied: requested access to the resource is denied";
        match classify_pull_result(&reference, false, stderr) {
            Err(PullError::Other {
                reference: r,
                message,
            }) => {
                assert_eq!(*r, reference);
                assert!(message.contains("denied"));
            }
            other => panic!("expected PullError::Other, got {other:?}"),
        }
    }

    #[test]
    fn test_classify_pull_result_network_error_is_other() {
        let reference = pull_test_reference();
        let stderr = b"Error response from daemon: Get https://ghcr.io/v2/: dial tcp: lookup ghcr.io: no such host";
        let result = classify_pull_result(&reference, false, stderr);
        assert!(matches!(result, Err(PullError::Other { .. })));
    }

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
