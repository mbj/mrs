# Changelog

## 0.3.1

### Added

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
