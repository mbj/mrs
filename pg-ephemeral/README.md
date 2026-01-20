# pg-ephemeral - Ephemeral PostgreSQL for Testing

> **Status**: Pre-1.0 - exists to serve [mbj/mrs](https://github.com/mbj/mrs) monorepo, expect breaking changes without notice.

Ephemeral PostgreSQL instances for testing.

## Requirements

For cross-container access (integration tests):
- Docker Desktop 4.34+ (macOS/Windows), or
- Podman 5.3+, or
- Docker Engine 20.10+ with `--add-host` support (Linux, GitHub Actions)

Note: Older versions of Docker Engine and Podman are supported via automatic `--add-host=host.docker.internal:host-gateway` flag.

## Release Build Configuration

Release builds use `split-debuginfo = "packed"` to separate debug information from the binary:

- **Linux**: Debug info stored in `.dwp` file alongside the binary
- **macOS**: Debug info stored in `.dSYM` bundle alongside the binary

This configuration provides:
- Smaller binary size (~8.6MB vs ~16MB with embedded debug info)
- Full backtraces with file paths and line numbers (requires debug files present)
- Cross-platform consistency

The Ruby gem includes both the binary and its debug info files to ensure useful panic backtraces in production.