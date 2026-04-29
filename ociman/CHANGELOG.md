# Changelog

## 0.5.0

### Breaking Changes

- `Container::inspect()` now returns `Result<serde_json::Value, InspectError>`
  instead of panicking on command or JSON parse failures.
- `Container::inspect_format()` now returns `Result<String, InspectError>`
  instead of panicking on command or UTF-8 decode failures.
- `Container::read_host_tcp_port()` now returns `Result<u16,
  ReadHostTcpPortError>` with typed `NotPublished { container_port }` and
  `InvalidHostPort { value, source }` variants. Previously it returned
  `Option<u16>`, collapsing real inspect failures into the same `None` as a
  legitimately unpublished port.

### Added

- `InspectError` enum covering command, JSON parse, and UTF-8 failure modes of
  the `inspect` code path.
- `ReadHostTcpPortError` enum for `Container::read_host_tcp_port()`.

## 0.4.0

### Breaking Changes

- Update to `cmd-proc` 0.5.0. `cmd_proc::CommandError` is re-exposed through
  `Backend::Error::CommandFailed` and through `Command::run()` / `Command::status()`
  return types, so the struct-to-enum shape change in `cmd-proc` 0.5.0 is a
  breaking change for ociman consumers as well.

### Added

- Configuration file support (`~/.config/ociman.toml`) to set preferred backend
  via `default_backend = "docker"` or `default_backend = "podman"`

### Changed

- Default auto-detection order changed from Podman-first to Docker-first

## 0.3.1

### Added

- `Backend::bridge_subnets()` method to inspect the default bridge network subnets
- `Domain::is_registry()` method following distribution/reference convention
- Improved `Name` parsing: single-component segments before `/` are now treated as path
  components (not domains) unless they contain dots, ports, or are `localhost`

### Changed

- Update to `cmd-proc` 0.4.0

### Fixed

- Resolve flaky Docker image build tests with unique isolation layers
- Fix stale doctest for async `resolve` example

## 0.3.0

### Breaking Changes

- Switch to async `cmd-proc` 0.3.0: all container execution methods are now `async fn`
- `Definition::run_detached()` is now `async fn`
- `Definition::with_container()` now takes `AsyncFnMut` and is `async fn`
- `Container::stop()`, `Container::remove()`, `Container::commit()` are now `async fn`
- `Container::exec()` returns builder whose terminal methods are now async
- `Container::read_host_tcp_port()` is now `async fn`
- All `Backend` methods are now `async fn`
- Image building and pulling methods are now `async fn`

## 0.2.0

### Removed

- Remove unsound `Drop` implementation for `Container`. The Drop impl performed
  blocking I/O (shelling out to `docker`/`podman`) and called `unwrap()`, which
  aborts the process when panicking during unwind. Container cleanup is now the
  caller's responsibility via explicit `stop()`/`remove()` calls or the `--rm` flag.
- Remove `stop_on_drop()` and `remove_on_drop()` builder methods from `Definition`
- Remove `stopped` and `removed` fields from `Container`

### Changed

- `Definition::with_container` now stops the container explicitly after the closure
  instead of relying on Drop

## 0.1.0

### Changed

- Migrate to `cmd-proc` for command execution with consistent stream handling
- Change to consistent formatting of the stability warning

### Fixed

- Fix environment variable handling to enforce validation via `cmd-proc`

## 0.0.1

Initial release.

### Added

- Unified API for OCI container runtimes (Docker, Podman)
- Backend autodetection with `Backend::autodetect()`
- Container lifecycle management (run, stop, remove, commit)
- Container exec with composable builder API
- Image building from Dockerfile, directory, or instructions
- Image tagging and pulling
- Content-addressed image builds with hash-based tags
- Port publishing configuration
- Environment variables and mounts
- Explicit container stop and remove methods
- OCI reference parsing (domain, path, tag, digest)
- Container hostname resolution for inter-container networking
- Platform support detection (amd64, arm64)

### Backend Support

- Docker (with version detection for API differences)
- Podman (rootless supported)
