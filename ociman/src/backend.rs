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

    /// Returns the arguments for building an image.
    ///
    /// For Docker, uses BuildKit via `buildx build` which has proper
    /// concurrency handling. For Podman, uses the standard `build` command
    /// since Podman already uses Buildah (BuildKit-like) natively.
    #[must_use]
    pub fn build_command_args(&self) -> Vec<String> {
        match self {
            Self::Docker { .. } => vec!["buildx".into(), "build".into(), "--load".into()],
            Self::Podman { .. } => vec!["build".into()],
        }
    }

    /// Check if an image is present in the local registry
    pub async fn is_image_present(&self, reference: &crate::image::Reference) -> bool {
        let reference_string = reference.to_string();

        match self {
            Backend::Docker { .. } => self
                .command()
                .arguments(["inspect", "--type", "image", &reference_string])
                .capture_stdout()
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

    /// Pull an image from a registry
    pub async fn pull_image(&self, reference: &crate::image::Reference) {
        self.command()
            .arguments(["pull", &reference.to_string()])
            .status()
            .await
            .unwrap();
    }

    /// Pull an image only if it's not already present
    pub async fn pull_image_if_absent(&self, reference: &crate::image::Reference) {
        if !self.is_image_present(reference).await {
            self.pull_image(reference).await;
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
            .capture_stdout()
            .string()
            .await
            .unwrap();

        output
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| line.parse::<crate::image::Reference>().unwrap())
            .filter(|reference| &reference.name == name)
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
            .capture_stdout()
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

    #[derive(Clone, Debug, thiserror::Error, PartialEq)]
    pub enum Error {
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

    /// Resolve backend automatically based on env var or available tools
    pub async fn auto() -> Result {
        match std::env::var(ENV_VARIABLE_NAME) {
            Err(std::env::VarError::NotPresent) => from_present_tool().await,
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

    async fn from_present_tool() -> Result {
        match podman().await {
            Ok(backend) => Ok(backend),
            Err(_) => docker().await.map_err(|_| Error::NoContainerToolDetected),
        }
    }

    async fn detect_version(
        executable: &'static str,
        constructor: impl FnOnce(semver::Version) -> Backend,
    ) -> Result {
        let output = Command::new(executable)
            .argument("--version")
            .capture_stdout()
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

        // Create test images by tagging alpine:latest
        let source: crate::image::Reference = "alpine:latest".parse().unwrap();
        backend.pull_image_if_absent(&source).await;

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
