# ociman - OCI Manager

A Rust library providing a unified API for OCI container runtimes (Docker, Podman).

> **Status**: Pre-1.0 - exists to serve [mbj/mrs](https://github.com/mbj/mrs) monorepo, expect breaking changes without notice.

## Goals

- **Unified API**: Single interface for OCI-compliant container runtimes
- **Auto-detection**: Automatically detects available container runtime
- **Environment override**: Control backend selection via `OCIMAN_BACKEND` environment variable
- **Container lifecycle management**: Run, execute commands, inspect, and manage containers
- **Image building**: Build images from Dockerfiles or inline instructions
- **Content-based hashing**: Automatic tag generation based on SHA256 of build context/instructions for deterministic builds

## Executing Commands in Containers

Use `container.exec()` to run commands inside a running container:

```rust,ignore
// Simple command execution
container.exec("echo")
    .argument("hello")
    .status().await?;

// Capture stdout
let output = container.exec("cat")
    .argument("/etc/os-release")
    .build()
    .capture_stdout()
    .string().await?;

// With environment variables and stdin
let result = container.exec("psql")
    .argument("--dbname=mydb")
    .environment_variable(PG_PASSWORD, "secret")
    .stdin(b"SELECT 1;")
    .build()
    .capture_stdout()
    .bytes().await?;
```

The `ExecCommand` builder focuses on container exec configuration. For stream capture,
use `.build()` to get a `cmd_proc::Command`, then use its stream methods.

## Container Stopping and Removal

Containers must be stopped and removed explicitly by the caller. `ociman::Container`
intentionally does **not** implement `Drop` for automatic cleanup. This is a deliberate
design decision:

- **Blocking I/O in Drop is unsafe**: Stopping/removing a container shells out to
  `docker`/`podman`. If the subprocess fails, an `unwrap()` inside Drop causes a panic,
  which aborts the process when unwinding from another panic.
- **`--rm` is the correct cleanup mechanism**: Use `.remove()` on the `Definition` to pass
  `--rm` to the container runtime. This ensures the runtime removes the container when it
  stops, even if the Rust process is killed.
- **Explicit lifecycle is clearer**: Callers always know when stop/remove happens. There
  are no hidden side effects on scope exit.

Typical usage patterns:

```rust,ignore
// Pattern 1: Use --rm flag + with_container (most common)
// Container is explicitly stopped after the closure, and --rm handles removal.
let definition = Definition::new(backend, image).remove();
definition.with_container(async |container| {
    // use container
}).await;

// Pattern 2: Stop, commit, then remove (for snapshotting)
// Cannot use --rm here because the container must survive stop for commit.
let mut container = definition.run_detached().await;
container.stop().await;
container.commit(&snapshot_image, false).await?;
container.remove().await;
```

## Content-Based Image Hashing

ociman supports automatic tag generation based on content hashing (SHA256). This ensures deterministic builds where the same content always produces the same image tag.

Benefits:
- **Deterministic**: Same content always produces the same tag
- **Automatic cache invalidation**: Content changes automatically produce a new tag
- **No manual tag management**: Hash is computed automatically
- **Reproducibility**: Easy to verify if an image matches its source

**Important**: Content-based hashing only captures the Dockerfile and build context, not the base images. Using unspecific tags like `FROM alpine:latest` reduces reproducibility since `latest` can point to different images over time. For fully reproducible builds, use specific base image digests:

```dockerfile
# Less reproducible - tag can change
FROM alpine:latest

# More reproducible - specific version tag
FROM alpine:3.19

# Most reproducible - pinned to specific digest
FROM alpine@sha256:6457d53fb065d6f250e1504b9bc42d5b6c65941d57532c072d929dd0628977d0
```