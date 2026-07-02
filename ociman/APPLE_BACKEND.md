# Apple Container Backend

- macOS-only; requires macOS 26 and Apple Container 1.0.0. The version is detected and recorded but not gated — no behavior branches on it.
- Goal: `pg-ephemeral` runs correctly on the Apple backend.
- Uses the released `container` CLI, parses Apple JSON, and fails early on unsupported features.

## Compatibility Boundaries

- Do not reshape Apple inspect JSON as a public or internal contract.
- Do not implement Go-template rendering for Apple commands.

## Relationship to Containerization Framework

Apple `container` is built on the Swift
[`apple/containerization`](https://github.com/apple/containerization) framework, which grounds two assumptions here: it requires macOS 26 (matching the ociman Apple backend gate), and its EXT4/archive/OCI APIs are what make the commit emulation (below) workable.

The backend targets the released `container` CLI; the framework is
treated as background evidence, not a dependency.

## Backend Shape

The backend is a single enum. Apple carries only a version; it has no
`rootless` field because Apple Container has no rootless/rootful split:

```rust
pub enum Backend {
    Docker { version: semver::Version, rootless: bool },
    Podman { version: semver::Version, rootless: bool },
    Apple { version: semver::Version },
}
```

There is no separate backend-kind type. Every unsupported-feature error
is only ever constructed on the Apple path, so those messages hardcode
`"apple"` rather than carrying a backend value, and backend checks use
`matches!(backend, Backend::Apple { .. })`.

## Selection

Extend backend selection with `apple` only:

```sh
OCIMAN_BACKEND=apple
```

and config:

```toml
default_backend = "apple"
```

- `OCIMAN_BACKEND=apple` — strict: resolve Apple or return the Apple error.
- `default_backend = "apple"` — preferred: Apple, then Docker, then Podman.
- `default_backend = "auto"` — Docker, then Podman, then Apple last.

Apple is tried last in auto-detection: discoverable when it's the only runtime, never displacing an available Docker/Podman.

## Supported Surface

The Apple backend supports:

- **Resolution & readiness** — macOS-only resolution/version detection, with clear errors for non-macOS selection and macOS older than 26; `container system status --format json` readiness check with a not-running error and start hint.
- **Containers** — run, exec, stop, and delete/force-delete (canonical Apple `container delete`); inspect (raw Apple JSON); typed labels, name/id (preserving user-provided names), and published-host-port lookup; host directory/file mounts where the released CLI exposes supported semantics; list via Apple JSON, including by-label via client-side filtering (key-only and exact-match).
- **Images** — inspect, labels, presence check, tag, delete (canonical Apple command); pull with Apple-aware not-found classification; push via `container image push`, surfacing runtime/registry errors clearly; build from a directory and from inline instructions via a temporary-directory fallback (see "Build").
- **Networking** — default network subnet inspection on macOS 26.

## Explicitly Unsupported

The backend fails early with domain errors for:

- `Container::inspect_format` on Apple Container: returns `InspectError::FormatUnsupported`. `pg-ephemeral` does not need it.
- Docker/Podman rootless semantics; `is_rootless()` returns `false`
- host aliases such as `host.docker.internal`; Apple host reachability should use the default network gateway from Apple network inspect output
- unsupported pull policy flags, if Apple CLI does not accept equivalent flags
- list format templates on Apple commands
- label filters on Apple CLI commands

## API/Error Changes

Where a subprocess-level error could not distinguish an unsupported feature from a command failure, the API carries an explicit public domain error variant instead.

### Commit

`commit` returns a domain error instead of a raw `CommandError`:

```rust
pub enum CommitError {
    Inspect(InspectError),
    MissingAppleBaseImage,
    Command(CommandError),
}
```

On Apple, `commit` dispatches to an export-and-build emulation (see
"Commit emulation" below); the `Inspect` and `MissingAppleBaseImage`
variants surface its failure modes.

### Inspect Format

`inspect_format` on Apple returns an explicit unsupported variant:

```rust
pub enum InspectError {
    FormatUnsupported,
    // existing variants...
}
```

No Go-template rendering. If a caller needs a value from `inspect_format`, add a typed ociman API for it instead.

### Host Resolution

Apple host reachability is resolved through the default network gateway (`.status.ipv4Gateway`, see "Network subnets").

## Implementation Seams

### 1. Command construction

`Backend::command()` returns `container` for Apple; individual operations construct the flags they need.

The run-definition-to-command translation can fail for unsupported Apple options before subprocess execution, so the public run command builder is fallible:

```rust
pub fn to_cmd_proc_command(&self) -> Result<Command, RunError>
```

`run()` and `run_detached()` use `self.to_cmd_proc_command()?`, and Apple returns `RunError::UnsupportedOption { option }` when a configured run option has no supported Apple equivalent (currently `pull_policy`).

Command differences:

- Docker/Podman remove: `container rm`; Apple remove: `container delete`.
- Docker/Podman list format: Go templates; Apple list format: `json`, `yaml`, `toml`, `table`.

### 2. Inspect parsing

Raw inspect stays backend-specific; portability comes from typed APIs. Docker/Podman share common-field parsing; Apple uses its own field names, parsed separately.

Contracts preserved:

- `Backend::inspect_container([id])` → JSON array (Apple objects wrapped and looped per id — no batch inspect).
- `Container::inspect()` → the single raw Apple object from that array.
- `Backend::inspect_image(reference)` → the one raw Apple image object.

Typed APIs parse by backend:

- Docker/Podman labels: `.Config.Labels`
- Apple container labels: `.configuration.labels`
- Docker/Podman ports: `.NetworkSettings.Ports["5432/tcp"][0].HostPort`
- Apple ports: `.configuration.publishedPorts[]`
- Docker name: `.Name` with leading slash stripped
- Podman name: `.Name`
- Apple name/id: prefer an explicit user-provided name field if Apple exposes one; only fall back to ID if Apple has no separate name concept in inspect/list

### 3. Container listing

`Backend::container_list(format, filters)` remains the Docker/Podman list seam.

Apple uses a typed internal primitive parsed from `container list --all
--format json`:

```rust
struct AppleRecord {
    id: ContainerId,
    name: Option<String>,
    labels: BTreeMap<String, String>,
}
```

This is Apple-specific; Docker/Podman keep their existing `container_list`
seam.

Apple label filtering is client-side. Both ociman label filter forms are supported: key-only existence and exact key/value matching. Apple list JSON includes labels, so filtering runs directly on list output rather than inspecting each container.

### 4. Image and container presence

Apple image presence uses:

```sh
container image inspect <reference>
```

Success means present, missing image means absent, and other failures surface as errors. Apple container presence uses container inspect the same way.

Apple has no distinct exit codes for absence, so missing images/containers are classified from stderr (`apple_stderr_is_not_found`), covered by inline stderr tests.

### 5. Build

Directory builds use:

```sh
container build -t <reference> <context-dir>
```

The target release rejects `-` (stdin) as a context directory, so inline instruction builds use a temporary-context fallback: ociman writes the instructions to a temporary `Dockerfile`, builds that directory, and deletes it after the command completes.

### 6. Port publishing

Apple rejects empty host-port auto-assignment syntax (`127.0.0.1::5432`). When `ociman::Publish` allows "any host port", the Apple backend allocates a concrete loopback port itself (`Publish::apple_argument`): it binds a socket to port 0, reads back the assigned ephemeral port, releases it, and passes an explicit `ip:port:containerPort/proto` spec to `container run`. This preserves the ociman API contract. An explicit user-supplied host port is passed through unchanged.

There is a race between allocating the port and `container run` binding it — the port is free when ociman picks it but could be taken by the time the container starts. This is currently **not** retried; a lost race surfaces as a normal bind failure.

Dedicated container IPs via vmnet do not by themselves satisfy ociman's published-host-port API through the released CLI, so the CLI port-publishing behavior is kept explicit and tested.

### 7. Network subnets

On macOS 26, `bridge_subnets()` runs:

```sh
container network inspect default
```

and parses the JSON into `Vec<ipnet::IpNet>`, failing explicitly if the command is unavailable or the default network is absent.

## Integration Testing

The Apple acceptance suite is the existing integration corpus run under `OCIMAN_BACKEND=apple`:

```sh
OCIMAN_BACKEND=apple cargo nextest run --test integration --features test-utils
```

Apple integration requires macOS 26, Apple `container`, and a running `container system`. `test_backend_setup!()` stays the single entry point; tests that branch on backend match the resolved backend, not the env var.

The existing macOS Actions skip (`platform::support()`) is Docker-specific: it returns `Ok(())` when `OCIMAN_BACKEND=apple`, so an Apple job attempts resolution rather than silently skipping.

Apple integration CI runs on a local self-hosted GitHub runner labeled `aarch64-darwin` so Apple Container can use real local macOS virtualization. Hosted macOS runners already run in VMs and do not allow Apple Container to run containers in their own isolated VMs. Tests run serially to reduce VM/runtime flakiness.

## Known Risks

### Inspect schema drift

Apple inspect JSON is not Docker-shaped and may evolve. Typed parser tests should use fixtures from the supported release.

### List semantics

Apple `container list --format` accepts output format names. Any internal API that accepts template strings is a Docker/Podman-specific seam and should not be reused for Apple.

### Label filtering

Apple `container list` documentation does not advertise `--filter label=...`. Client-side filtering is required unless this becomes supported.

### Missing image classification

`container image inspect <ref>` missing-image stderr may not match Docker/Podman. Add explicit tests for the target release.

### Mount semantics

Bind mounts use virtiofs. Directory mounts are natural; single-file mounts share the file's parent dir into the VM, then bind-mount the file — so sibling files are visible in the VM-level holding mount. Not Docker's isolation boundary: test the public mount behavior, don't promise stronger isolation.

### Build compatibility

Known open Apple Container issues where build behavior does not fully match Docker (beyond the stdin-context fallback in "Build"):

- apple/container#1766 — default build args in `FROM` may not resolve like Docker/Podman.
- apple/container#1800 — `.dockerignore` resolution differs from Docker.
- apple/container#1825 — BuildKit may require Rosetta even for arm64 builds.

Keep build tests narrow and document backend-specific behavior.

### Registry behavior

Known open Apple Container registry/auth issues:

- apple/container#1707 — ECR push can fail on manifest PUT with 401.
- apple/container#1733 — private registry keychain query failure.

Pull/push surface errors as-is (no over-normalizing). Loopback refs (`localhost:5001/...`) pass `--scheme http` so pg-ephemeral can use a throwaway plain-HTTP registry.

### Runtime reliability / test flakiness

Relevant open Apple Container issues:

- apple/container#1876 — foreground TTY/signal hang.
- apple/container#1747 — SIGWINCH forwarding errors.
- apple/container#1722 — containers running while `container list` reports empty.
- apple/container#1873 — `container cp` hangs with mounted volumes.
- apple/container#1875 — storage/snapshot bloat affecting backups.
- apple/container#1767 — rootfs journaling concern after ungraceful termination.

These do not block the backend but argue for serial tests and aggressive cleanup.

## Commit emulation

`pg-ephemeral` cache population requires commit before Apple ships built-in
`container commit`, so ociman implements a limited, Apple-specific
export-and-build emulation. The Containerization framework makes this
workable because it includes EXT4 export, archive writing, OCI
image/config, and content-store APIs.

The emulation:

1. inspects the container and original image,
2. exports the container filesystem,
3. generates a temporary build context,
4. builds a new image from `scratch` with the exported rootfs (using `container build --no-cache` to bound builder cache growth across repeated commits),
5. reconstructs only the OCI image config fields required by `pg-ephemeral`.

It is limited to the semantics it claims to preserve: environment
variables, entrypoint/command, workdir, user, labels used by
ociman/pg-ephemeral, and filesystem contents/ownership relevant to
PostgreSQL data directories.

When apple/container#1762 (`add commit command`) ships in a release,
revisit whether to replace the emulation with built-in `container commit`.

## `pg-ephemeral` Cache Shape

On Apple, `pg-ephemeral` uses one committed PostgreSQL cache layer, then
runs later seeds live, keeping cache chains shallow while built-in commit
is unavailable.
