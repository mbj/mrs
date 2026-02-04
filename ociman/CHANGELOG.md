# Changelog

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
- Stop-on-drop and remove-on-drop container cleanup
- OCI reference parsing (domain, path, tag, digest)
- Container hostname resolution for inter-container networking
- Platform support detection (amd64, arm64)

### Backend Support

- Docker (with version detection for API differences)
- Podman (rootless supported)
