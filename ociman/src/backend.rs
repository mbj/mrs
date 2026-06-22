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

// `image::Reference` is ~176 bytes (Name + Vec of PathComponents + Tag +
// Digest), so carrying it inline pushes this enum past the 128-byte
// `clippy::result_large_err` threshold. We carry it inline anyway and allow
// the lint at the call sites: these errors only arise from cold subprocess
// paths (spawning docker/podman, awaiting a registry round-trip), never a
// hot loop where moving a large `Result` by value would matter. Boxing the
// reference would buy nothing here but an allocation and `Box::new`/deref
// ceremony.
#[derive(Debug, thiserror::Error)]
pub enum PullError {
    #[error("image not found in registry: {reference}")]
    NotFound { reference: crate::image::Reference },
    #[error("pull failed for {reference}: {message}")]
    Other {
        reference: crate::image::Reference,
        message: String,
    },
    /// The pull subprocess could not be spawned or failed at the IO layer
    /// (e.g. the docker/podman binary is missing, or the daemon socket is
    /// unreachable) — distinct from a registry-level pull failure. Surfaced
    /// rather than aborting the process so `?`-propagating callers can react.
    #[error("pull command failed to run for {reference}")]
    Command {
        reference: crate::image::Reference,
        #[source]
        source: cmd_proc::CommandError,
    },
    /// `pull_image_if_absent` consults [`Backend::is_image_present`] to
    /// decide whether to skip the pull. If that probe itself fails,
    /// surface the failure here rather than masking it as a successful
    /// "absent → pull".
    #[error(transparent)]
    ImagePresent(#[from] ImagePresentError),
}

/// Turn a pull subprocess exit status + captured stderr into a
/// [`PullError`] (or [`Ok`] on success).
///
/// Split out from [`Backend::pull_image`] so it can be unit-tested with
/// canned stderr bytes — no network, no daemon, no registry.
#[allow(
    clippy::result_large_err,
    reason = "PullError carries the image Reference inline; only constructed on cold subprocess error paths (see the type's comment)"
)]
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
            reference: reference.clone(),
        })
    } else {
        Err(PullError::Other {
            reference: reference.clone(),
            message: stderr.trim().to_string(),
        })
    }
}

// Carries `image::Reference` inline and is allowed past
// `clippy::result_large_err` for the same reason as [`PullError`] — see
// that type's comment.
#[derive(Debug, thiserror::Error)]
pub enum PushError {
    #[error("push failed for {reference}: {message}")]
    Failed {
        reference: crate::image::Reference,
        message: String,
    },
    /// The push subprocess could not be spawned or failed at the IO layer
    /// (e.g. the docker/podman binary is missing, or the daemon socket is
    /// unreachable) — distinct from a registry-level push failure. Surfaced
    /// rather than aborting the process so `?`-propagating callers can react.
    #[error("push command failed to run for {reference}")]
    Command {
        reference: crate::image::Reference,
        #[source]
        source: cmd_proc::CommandError,
    },
}

/// Turn a push subprocess exit status + captured stderr into a
/// [`PushError`] (or [`Ok`] on success).
///
/// Split out from [`Backend::push_image`] for the same reason as
/// [`classify_pull_result`] — unit-testable without a network or daemon.
#[allow(
    clippy::result_large_err,
    reason = "PushError carries the image Reference inline; only constructed on cold subprocess error paths (see the type's comment)"
)]
fn classify_push_result(
    reference: &crate::image::Reference,
    success: bool,
    stderr: &[u8],
) -> Result<(), PushError> {
    if success {
        return Ok(());
    }

    Err(PushError::Failed {
        reference: reference.clone(),
        message: String::from_utf8_lossy(stderr).trim().to_string(),
    })
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
    Docker {
        version: semver::Version,
        rootless: bool,
    },
    Podman {
        version: semver::Version,
        rootless: bool,
    },
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
                        "container",
                        "list",
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

    /// Whether the backend is running in rootless mode.
    ///
    /// The value is captured once during [`resolve`](resolve::auto) /
    /// [`resolve::docker`] / [`resolve::podman`] and held on the
    /// [`Backend`] for its lifetime — rootful↔rootless requires a daemon
    /// restart for Docker or a different user session for Podman, so
    /// re-probing per call would be wasted work.
    #[must_use]
    pub const fn is_rootless(&self) -> bool {
        match self {
            Self::Docker { rootless, .. } | Self::Podman { rootless, .. } => *rootless,
        }
    }

    /// Tag an image with a new name
    pub async fn tag_image(
        &self,
        source: &crate::image::Reference,
        target: &crate::image::Reference,
    ) {
        self.command()
            .arguments(["image", "tag", &source.to_string(), &target.to_string()])
            .status()
            .await
            .unwrap();
    }

    /// Pull an image from a registry.
    ///
    /// Stdout streams to the parent so users see layer progress. Stderr is
    /// captured and parsed to distinguish [`PullError::NotFound`] (registry
    /// reports `manifest unknown`) from other failures.
    #[allow(
        clippy::result_large_err,
        reason = "PullError carries the image Reference inline; cold subprocess path (see the type's comment)"
    )]
    pub async fn pull_image(&self, reference: &crate::image::Reference) -> Result<(), PullError> {
        let output = self
            .command()
            .arguments(["image", "pull", &reference.to_string()])
            .stderr_capture()
            .accept_nonzero_exit()
            .run()
            .await
            .map_err(|source| PullError::Command {
                reference: reference.clone(),
                source,
            })?;

        classify_pull_result(reference, output.status.success(), &output.bytes)
    }

    /// Pull an image only if it's not already present locally.
    ///
    /// If the local-presence probe itself fails, the [`ImagePresentError`]
    /// is propagated through [`PullError::ImagePresent`] rather than
    /// silently falling through to a pull attempt.
    #[allow(
        clippy::result_large_err,
        reason = "PullError carries the image Reference inline; cold subprocess path (see the type's comment)"
    )]
    pub async fn pull_image_if_absent(
        &self,
        reference: &crate::image::Reference,
    ) -> Result<(), PullError> {
        if self.is_image_present(reference).await? {
            Ok(())
        } else {
            self.pull_image(reference).await
        }
    }

    /// Push an image to a registry.
    ///
    /// Stdout streams to the parent so users see upload progress. Stderr
    /// is captured and surfaced as [`PushError`] on non-zero exit. Unlike
    /// pull, there's no useful sub-discrimination here: every push failure
    /// (auth, network, rate limit, missing local image) collapses into the
    /// same "it didn't upload" outcome as far as callers are concerned.
    #[allow(
        clippy::result_large_err,
        reason = "PushError carries the image Reference inline; cold subprocess path (see the type's comment)"
    )]
    pub async fn push_image(&self, reference: &crate::image::Reference) -> Result<(), PushError> {
        let output = self
            .command()
            .arguments(["image", "push", &reference.to_string()])
            .stderr_capture()
            .accept_nonzero_exit()
            .run()
            .await
            .map_err(|source| PushError::Command {
                reference: reference.clone(),
                source,
            })?;

        classify_push_result(reference, output.status.success(), &output.bytes)
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
                "image",
                "list",
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

    /// Inspect one or more containers by ID, mapping 1:1 to
    /// `docker inspect --type container <id>...`.
    ///
    /// Returns the full array the runtime emits — one element per ID, in
    /// order. A missing target (for *any* of the requested IDs) surfaces as
    /// [`crate::InspectError::NotFound`], detected by matching the
    /// runtime's stderr; any other non-zero exit becomes
    /// [`crate::InspectError::Subprocess`] with the captured stderr.
    pub async fn inspect_container<'id, I>(
        &self,
        ids: I,
    ) -> Result<Vec<serde_json::Value>, crate::InspectError>
    where
        I: IntoIterator<Item = &'id crate::ContainerId>,
    {
        let mut command = self.command().arguments(["container", "inspect"]);
        for id in ids {
            command = command.argument(id);
        }
        let result = command
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
            return Err(crate::InspectError::classify_failure(
                result.status,
                &result.stderr,
            ));
        }

        Ok(serde_json::from_slice(&result.stdout)?)
    }

    /// Run `inspect --format` against a container and return the rendered
    /// stdout (with trailing newline stripped).
    ///
    /// Run `inspect --format` against a container and return the rendered
    /// stdout (with trailing newline stripped). Failure classification is
    /// the same stderr-substring scheme as [`Self::inspect_container`].
    pub async fn inspect_container_format(
        &self,
        id: &crate::ContainerId,
        format: &str,
    ) -> Result<String, crate::InspectError> {
        let result = self
            .command()
            .arguments(["container", "inspect", "--format"])
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
            return Err(crate::InspectError::classify_failure(
                result.status,
                &result.stderr,
            ));
        }

        Ok(std::str::from_utf8(crate::container::strip_nl_end(&result.stdout))?.to_string())
    }

    /// Read the labels on a container by id.
    pub async fn container_labels(
        &self,
        id: &crate::ContainerId,
    ) -> Result<crate::label::ContainerLabels, crate::label::ContainerError> {
        // Single-id batched inspect: docker returns a length-1 array on
        // success; peel the singleton here.
        let value = self
            .inspect_container([id])
            .await?
            .into_iter()
            .next()
            .unwrap();
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
        let value = self
            .inspect_container([id])
            .await?
            .into_iter()
            .next()
            .unwrap();
        let raw = value
            .get("Name")
            .and_then(|name_value| name_value.as_str())
            .ok_or(crate::container::ReadContainerNameError::NameNotString)?;
        Ok(self.parse_container_name(raw)?)
    }

    /// Inspect an image by reference, mapping 1:1 to
    /// `docker inspect --type image <ref>`. Returns the single entry from
    /// the runtime's response array.
    ///
    /// A missing image surfaces as [`crate::InspectError::NotFound`],
    /// detected by stderr substring; any other non-zero exit becomes
    /// [`crate::InspectError::Subprocess`].
    pub async fn inspect_image(
        &self,
        reference: &crate::image::Reference,
    ) -> Result<serde_json::Value, crate::InspectError> {
        let result = self
            .command()
            .arguments(["image", "inspect"])
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
            return Err(crate::InspectError::classify_failure(
                result.status,
                &result.stderr,
            ));
        }

        // Single-ref inspect: docker always returns a length-1 array on
        // success; peel the singleton.
        let array: Vec<serde_json::Value> = serde_json::from_slice(&result.stdout)?;
        Ok(array.into_iter().next().unwrap())
    }

    /// Read the labels on an image by reference.
    pub async fn image_labels(
        &self,
        reference: &crate::image::Reference,
    ) -> Result<crate::label::ImageLabels, crate::label::ImageError> {
        let value = self.inspect_image(reference).await?;
        crate::label::decode_labels(&value)
    }

    /// Foundation `container list` primitive: runs
    /// `container list --all --no-trunc --format <format> [--filter label=…]…`
    /// and returns the captured stdout as a `String`.
    ///
    /// Higher-level listing helpers parse this stdout into typed values.
    /// Non-zero exits surface the captured stderr through
    /// [`ContainerListError::Subprocess`] so callers can diagnose failures
    /// (e.g. backend down) without re-running the command.
    pub async fn container_list<'a>(
        &self,
        format: &str,
        label_filters: impl IntoIterator<Item = crate::label::Filter<'a>>,
    ) -> Result<String, ContainerListError> {
        let mut command = self.command().arguments([
            "container",
            "list",
            "--all",
            "--no-trunc",
            "--format",
            format,
        ]);

        for filter in label_filters {
            command = command.argument("--filter").argument(filter.to_string());
        }

        let result = command
            .stdout_capture()
            .stderr_capture()
            .accept_nonzero_exit()
            .run()
            .await
            .map_err(|error| match error {
                cmd_proc::CommandError::Io(io) => ContainerListError::Io(io),
                cmd_proc::CommandError::ExitStatus(exit_status) => ContainerListError::Subprocess {
                    exit_status,
                    stderr: String::new(),
                },
            })?;

        let stderr = std::str::from_utf8(&result.stderr).map_err(ContainerListError::StderrUtf8)?;

        if !result.status.success() {
            return Err(ContainerListError::Subprocess {
                exit_status: result.status,
                stderr: stderr.to_owned(),
            });
        }

        let stdout = std::str::from_utf8(&result.stdout).map_err(ContainerListError::StdoutUtf8)?;
        Ok(stdout.to_owned())
    }

    /// List containers with both [`crate::Container`] handle and runtime
    /// [`crate::ContainerName`], in one `container list` call.
    ///
    /// Layered on top of [`Self::container_list`] — formats `{{.ID}}\t{{.Names}}`
    /// and parses each line. The container name comes back validated as
    /// [`crate::ContainerName`]; callers reading a naming scheme prefix off the
    /// name itself don't need to follow up with an `inspect`.
    pub async fn container_list_with_name<'a>(
        &self,
        label_filters: impl IntoIterator<Item = crate::label::Filter<'a>>,
    ) -> Result<Vec<(crate::Container, crate::ContainerName)>, ContainerListWithNameError> {
        let stdout = self
            .container_list("{{.ID}}\t{{.Names}}", label_filters)
            .await?;

        stdout
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| {
                let (raw_id, raw_name) = line
                    .split_once('\t')
                    .ok_or_else(|| ContainerListWithNameError::MalformedLine(line.to_owned()))?;
                let name: crate::ContainerName = raw_name
                    .parse()
                    .map_err(ContainerListWithNameError::InvalidContainerName)?;
                let container = crate::Container {
                    backend: self.clone(),
                    id: crate::ContainerId(raw_id.to_owned()),
                };
                Ok((container, name))
            })
            .collect()
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
    ) -> Result<Vec<crate::Container>, ContainerListError> {
        let filter = match value {
            None => crate::label::Filter::key_only(key),
            Some(value) => crate::label::Filter::exact(key, value),
        };

        let stdout = self.container_list("{{.ID}}", [filter]).await?;

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
pub enum RootlessProbeError {
    /// Subprocess could not be started or failed at the IO layer.
    #[error("rootless probe failed")]
    Command(#[source] cmd_proc::CommandError),
    /// Subprocess exited non-zero. The captured stderr is preserved for
    /// diagnostics.
    #[error("rootless probe exited with {exit_status}: {stderr}")]
    Subprocess {
        exit_status: std::process::ExitStatus,
        stderr: String,
    },
    /// Probe stdout (or stderr on failure) was not valid UTF-8.
    #[error("rootless probe output was not valid UTF-8")]
    Utf8(#[source] std::str::Utf8Error),
    /// Probe succeeded but the output did not match the expected
    /// `true` / `false` (podman) or `rootless` line (docker). Captured
    /// trimmed stdout is preserved for diagnostics.
    #[error("rootless probe returned unexpected output: {0:?}")]
    UnexpectedOutput(String),
}

#[derive(Debug, thiserror::Error)]
pub enum ContainerListWithNameError {
    #[error(transparent)]
    List(#[from] ContainerListError),
    /// `container list` produced a row that did not contain the expected
    /// `\t` separator between ID and Name.
    #[error("`container list` produced a malformed line: {0:?}")]
    MalformedLine(String),
    #[error("`container list` produced an invalid container name")]
    InvalidContainerName(#[source] crate::ContainerNameError),
}

#[derive(Debug, thiserror::Error)]
pub enum ContainerListError {
    #[error("`container list` command IO failure")]
    Io(#[source] std::io::Error),
    #[error("`container list` exited with {exit_status}: {stderr}")]
    Subprocess {
        exit_status: std::process::ExitStatus,
        stderr: String,
    },
    #[error("`container list` stdout was not valid UTF-8")]
    StdoutUtf8(#[source] std::str::Utf8Error),
    #[error("`container list` stderr was not valid UTF-8")]
    StderrUtf8(#[source] std::str::Utf8Error),
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
            .arguments(["container", "run"])
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
    use super::{Backend, Command, RootlessProbeError};

    const ENV_VARIABLE_NAME: cmd_proc::EnvVariableName =
        cmd_proc::EnvVariableName::from_static_or_panic("OCIMAN_BACKEND");

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
        #[error("Failed to probe {executable} rootless mode")]
        RootlessProbeFailed {
            executable: &'static str,
            #[source]
            source: RootlessProbeError,
        },
    }

    /// Resolve backend automatically based on env var, config file, or available tools
    pub async fn auto() -> Result {
        match ENV_VARIABLE_NAME.read() {
            Err(cmd_proc::EnvVariableReadError::NotPresent { .. }) => {
                let config = crate::config::Config::load().map_err(Error::ConfigLoad)?;
                from_present_tool(config.default_backend).await
            }
            Err(error) => {
                panic!("{error}")
            }
            Ok(value) => from_env_value(value.as_str()).await,
        }
    }

    /// Resolve docker backend with version detection
    pub async fn docker() -> Result {
        let version = detect_version(Backend::DOCKER_EXECUTABLE).await?;
        let rootless =
            probe_rootless_docker()
                .await
                .map_err(|source| Error::RootlessProbeFailed {
                    executable: Backend::DOCKER_EXECUTABLE,
                    source,
                })?;
        Ok(Backend::Docker { version, rootless })
    }

    /// Resolve podman backend with version detection
    pub async fn podman() -> Result {
        let version = detect_version(Backend::PODMAN_EXECUTABLE).await?;
        let rootless =
            probe_rootless_podman()
                .await
                .map_err(|source| Error::RootlessProbeFailed {
                    executable: Backend::PODMAN_EXECUTABLE,
                    source,
                })?;
        Ok(Backend::Podman { version, rootless })
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
    ) -> std::result::Result<semver::Version, Error> {
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

        Ok(version)
    }

    async fn probe_rootless_docker() -> std::result::Result<bool, RootlessProbeError> {
        let result = Command::new(Backend::DOCKER_EXECUTABLE)
            .arguments([
                "info",
                "--format",
                "{{range .SecurityOptions}}{{println .}}{{end}}",
            ])
            .stdout_capture()
            .stderr_capture()
            .accept_nonzero_exit()
            .run()
            .await
            .map_err(RootlessProbeError::Command)?;

        if result.status.success() {
            let stdout = std::str::from_utf8(&result.stdout).map_err(RootlessProbeError::Utf8)?;
            Ok(stdout.lines().any(|line| line.trim() == "rootless"))
        } else {
            let stderr = std::str::from_utf8(&result.stderr).map_err(RootlessProbeError::Utf8)?;
            Err(RootlessProbeError::Subprocess {
                exit_status: result.status,
                stderr: stderr.to_string(),
            })
        }
    }

    async fn probe_rootless_podman() -> std::result::Result<bool, RootlessProbeError> {
        let result = Command::new(Backend::PODMAN_EXECUTABLE)
            .arguments(["info", "--format", "{{.Host.Security.Rootless}}"])
            .stdout_capture()
            .stderr_capture()
            .accept_nonzero_exit()
            .run()
            .await
            .map_err(RootlessProbeError::Command)?;

        if !result.status.success() {
            let stderr = std::str::from_utf8(&result.stderr).map_err(RootlessProbeError::Utf8)?;
            return Err(RootlessProbeError::Subprocess {
                exit_status: result.status,
                stderr: stderr.to_string(),
            });
        }

        let stdout = std::str::from_utf8(&result.stdout).map_err(RootlessProbeError::Utf8)?;
        match stdout.trim() {
            "true" => Ok(true),
            "false" => Ok(false),
            other => Err(RootlessProbeError::UnexpectedOutput(other.to_string())),
        }
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
            Err(PullError::NotFound {
                reference: error_reference,
            }) => assert_eq!(error_reference, reference),
            other => panic!("expected PullError::NotFound, got {other:?}"),
        }
    }

    #[test]
    fn test_classify_pull_result_not_found_docker() {
        let reference = pull_test_reference();
        // Representative docker stderr for a non-existent tag.
        let stderr = b"Error response from daemon: manifest for ghcr.io/myorg/pg-ephemeral/main:abc123 not found: manifest unknown: manifest unknown";
        match classify_pull_result(&reference, false, stderr) {
            Err(PullError::NotFound {
                reference: error_reference,
            }) => assert_eq!(error_reference, reference),
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
                reference: error_reference,
                message,
            }) => {
                assert_eq!(error_reference, reference);
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

    #[test]
    fn test_classify_push_result_success() {
        let reference = pull_test_reference();
        assert!(classify_push_result(&reference, true, b"").is_ok());
    }

    #[test]
    fn test_classify_push_result_failure_captures_stderr() {
        let reference = pull_test_reference();
        let stderr = b"unauthorized: authentication required\n";
        match classify_push_result(&reference, false, stderr) {
            Err(PushError::Failed {
                reference: error_reference,
                message,
            }) => {
                assert_eq!(error_reference, reference);
                assert_eq!(message, "unauthorized: authentication required");
            }
            other => panic!("expected PushError::Failed, got {other:?}"),
        }
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
