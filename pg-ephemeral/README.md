# pg-ephemeral

Ephemeral PostgreSQL instances for testing.

## Requirements

For cross-container access (integration tests):
- Docker Desktop 4.34+ (macOS/Windows), or
- Podman 5.3+, or
- Docker Engine 20.10+ with `--add-host` support (Linux, GitHub Actions)

Note: Older versions of Docker Engine and Podman are supported via automatic `--add-host=host.docker.internal:host-gateway` flag.

## License

See workspace root for license information.
