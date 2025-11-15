use super::command::*;

#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Deserialize, clap::ValueEnum)]
#[serde(rename_all = "snake_case")]
pub enum Backend {
    Docker,
    Podman,
}

impl Backend {
    pub fn command(&self) -> Command {
        match self {
            Self::Docker => Command::new("docker"),
            Self::Podman => Command::new("podman"),
        }
    }

    /// Check if an image is present in the local registry
    pub fn is_image_present(&self, image: &crate::Image) -> bool {
        match self {
            Backend::Docker => self
                .command()
                .arguments(["inspect", "--type", "image", image.as_str()])
                .capture_only_stdout_result()
                .is_ok(),
            Backend::Podman => {
                // For Podman, image exists returns 0 if present, 1 if not
                // We use status() instead of capture because we don't need output
                let status = self
                    .command()
                    .arguments(["image", "exists", image.as_str()])
                    .status();
                status.success()
            }
        }
    }

    /// Tag an image with a new name
    pub fn tag_image(&self, source: &crate::Image, target: &crate::Image) {
        self.command()
            .arguments(["tag", source.as_str(), target.as_str()])
            .capture_only_stdout();
    }

    /// Pull an image from a registry
    pub fn pull_image(&self, image: &crate::Image) {
        self.command()
            .arguments(["pull", image.as_str()])
            .capture_only_stdout();
    }

    /// Pull an image only if it's not already present
    pub fn pull_image_if_absent(&self, image: &crate::Image) {
        if !self.is_image_present(image) {
            self.pull_image(image);
        }
    }

    /// Push an image to a registry
    pub fn push_image(&self, image: &crate::Image) {
        self.command()
            .arguments(["push", image.as_str()])
            .capture_only_stdout();
    }

    pub fn remove_image(&self, image: &crate::Image) {
        self.command()
            .arguments(["image", "rm", image.as_str()])
            .capture_only_stdout();
    }

    /// Create a hostname resolver that runs inside a container
    ///
    /// This is useful for resolving DNS names that only work inside containers
    /// (e.g., host.docker.internal) or when you need to see how DNS resolves
    /// from within a containerized environment.
    ///
    /// # Example
    /// ```no_run
    /// let ip = cbt::backend::autodetect::run()
    ///     .unwrap()
    ///     .container_resolver()
    ///     .add_host("host.docker.internal:host-gateway")
    ///     .resolve("host.docker.internal")
    ///     .unwrap();
    /// ```
    pub fn container_resolver(&self) -> ContainerHostnameResolver {
        ContainerHostnameResolver::new(*self)
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
    /// let ip = cbt::backend::autodetect::run()
    ///     .unwrap()
    ///     .resolve_container_host()
    ///     .unwrap();
    /// ```
    pub fn resolve_container_host(&self) -> Result<std::net::IpAddr, ResolveHostnameError> {
        match self {
            Backend::Podman => {
                // Podman provides host.containers.internal natively
                self.container_resolver()
                    .resolve("host.containers.internal")
            }
            Backend::Docker => {
                // Docker needs --add-host on Linux
                self.container_resolver()
                    .add_host("host.docker.internal:host-gateway")
                    .resolve("host.docker.internal")
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
    pub fn resolve(self, hostname: &str) -> Result<std::net::IpAddr, ResolveHostnameError> {
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
            .capture_only_stdout_result()
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

pub mod autodetect {
    use super::Backend;

    const ENV_VARIABLE_NAME: &str = "CBT_BACKEND";

    pub type Result = std::result::Result<super::Backend, Error>;

    #[derive(Clone, Debug, thiserror::Error, PartialEq)]
    pub enum Error {
        #[error(
            "Invalid env variable for {ENV_VARIABLE_NAME}, expected \"podman\" or \"docker\", got: {0}"
        )]
        InvalidEnvVariable(String),
        #[error("No container tool detected in $PATH, searched for podman and docker")]
        NoContainerToolDetected,
    }

    pub fn run() -> Result {
        match std::env::var(ENV_VARIABLE_NAME) {
            Err(std::env::VarError::NotPresent) => from_present_tool(),
            Err(std::env::VarError::NotUnicode(_)) => {
                panic!("{ENV_VARIABLE_NAME} env variable exist but is not unicode!")
            }
            Ok(value) => from_env_value(&value),
        }
    }

    fn from_env_value(value: &str) -> Result {
        if value == "docker" {
            Ok(Backend::Docker)
        } else if value == "podman" {
            Ok(Backend::Podman)
        } else {
            Err(Error::InvalidEnvVariable(value.to_string()))
        }
    }

    fn from_present_tool() -> Result {
        fn attempt(backend: Backend) -> Option<Backend> {
            match backend
                .command()
                .argument("--version")
                .capture_only_stdout_result()
            {
                Err(_) => None,
                Ok(version) => {
                    log::debug!("cbt using: {}", std::str::from_utf8(&version).unwrap());
                    Some(backend)
                }
            }
        }

        attempt(Backend::Podman)
            .or_else(|| attempt(Backend::Docker))
            .ok_or(Error::NoContainerToolDetected)
    }

    pub struct Lazy(std::cell::OnceCell<Result>);

    impl Default for Lazy {
        fn default() -> Self {
            Self::new()
        }
    }

    impl Lazy {
        pub fn new() -> Self {
            Self(std::cell::OnceCell::new())
        }

        pub fn result(&self) -> &Result {
            self.0.get_or_init(run)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_container_resolver_localhost() {
        let backend = crate::test_backend_setup!();

        let ip = backend.container_resolver().resolve("localhost").unwrap();

        assert!(ip.is_loopback());
    }

    #[test]
    fn test_container_resolver_with_add_host() {
        let backend = crate::test_backend_setup!();

        let ip = backend
            .container_resolver()
            .add_host("host.docker.internal:host-gateway")
            .resolve("host.docker.internal")
            .unwrap();

        // Should resolve to some IP address
        assert!(ip.is_ipv4() || ip.is_ipv6());
    }

    #[test]
    fn test_container_resolver_nonexistent() {
        let backend = crate::test_backend_setup!();

        let result = backend
            .container_resolver()
            .resolve("this-definitely-does-not-exist-12345.local");

        assert!(result.is_err());
        match result {
            Err(ResolveHostnameError::CommandFailed(_)) => {
                // Expected: hostname resolution will fail for nonexistent hostname
            }
            other => panic!("Expected CommandFailed error, got: {:?}", other),
        }
    }

    #[test]
    fn test_container_resolver_with_multiple_arguments() {
        let backend = crate::test_backend_setup!();

        let ip = backend
            .container_resolver()
            .add_host("custom-host:192.168.1.100")
            .resolve("custom-host")
            .unwrap();

        assert_eq!(
            ip,
            std::net::IpAddr::V4(std::net::Ipv4Addr::new(192, 168, 1, 100))
        );
    }

    #[test]
    fn test_container_resolver_builder_pattern() {
        let backend = crate::test_backend_setup!();

        let resolver = backend
            .container_resolver()
            .argument("--add-host")
            .argument("test-host:10.0.0.1");

        let ip = resolver.resolve("test-host").unwrap();

        assert_eq!(
            ip,
            std::net::IpAddr::V4(std::net::Ipv4Addr::new(10, 0, 0, 1))
        );
    }

    #[test]
    fn test_resolve_container_host() {
        let backend = crate::test_backend_setup!();

        let ip = backend.resolve_container_host().unwrap();

        // Should resolve to some IP address
        assert!(ip.is_ipv4() || ip.is_ipv6());
    }
}
