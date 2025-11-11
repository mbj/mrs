# cbt - Container Backend Tool

A Rust library providing a unified API for Docker and Podman container runtimes.

## ⚠️ Stability Warning

**CBT is likely to change a lot and provide more and more structured builder support over time, pushing back string-based APIs incrementally.** Its primary use case is to implement `pg-ephemeral`, so do expect lots of API changes.

This library is in active development and should be considered unstable. Breaking changes may occur frequently as we improve the API design.

## Features

- **Unified API**: Single interface for both Docker and Podman
- **Auto-detection**: Automatically detects available container runtime
- **Environment override**: Control backend selection via `CBT_BACKEND` environment variable
- **Fluent interface**: Builder pattern for constructing container definitions
- **Container lifecycle**: Run, execute commands, inspect, and manage containers
- **Command execution**: Execute commands both inside and outside containers

## Usage

### Basic Example

```rust
use cbt::{Backend, Definition, Image};

// Auto-detect available backend (Podman or Docker)
let backend = cbt::backend::autodetect::run()
    .expect("No container backend available");

// Run a simple container
let definition = Definition::new(backend, Image::from("alpine:latest"))
    .entrypoint("echo".to_string(), vec!["Hello, World!".to_string()])
    .remove();

let output = definition.run_capture_only_stdout();
println!("{}", std::str::from_utf8(&output).unwrap());
```

### Running Detached Containers

```rust
use cbt::{Backend, Definition, Image, Publish};

let backend = cbt::backend::autodetect::run().unwrap();

// Run a detached container with port publishing
let mut container = Definition::new(backend, Image::from("postgres:latest"))
    .env("POSTGRES_PASSWORD", "secret")
    .env("POSTGRES_USER", "admin")
    .publish(Publish::from("127.0.0.1::5432/tcp"))
    .run_detached();

// Execute commands in the running container
let output = container.exec_capture_only_stdout(
    [],
    "psql",
    ["-U", "admin", "-c", "SELECT version()"]
);

// Stop the container when done
container.stop();
```

### Environment Variables and Mounts

```rust
use cbt::{Backend, Definition, Image, Mount};

let backend = cbt::backend::autodetect::run().unwrap();

let definition = Definition::new(backend, Image::from("alpine:latest"))
    .env("KEY1", "value1")
    .env("KEY2", "value2")
    .mount(Mount::from("type=bind,source=/host/path,target=/container/path".to_string()))
    .entrypoint("sh".to_string(), vec!["-c".to_string(), "env".to_string()]);

let output = definition.run_capture_only_stdout();
```

### Backend Selection

```rust
use cbt::Backend;

// Explicit backend selection
let backend = Backend::Docker;
let backend = Backend::Podman;

// Auto-detection (prefers Podman over Docker)
let backend = cbt::backend::autodetect::run().unwrap();

// Use CBT_BACKEND environment variable to override
// export CBT_BACKEND=docker
// export CBT_BACKEND=podman
```

### Running External Commands

```rust
use cbt::Command;

let output = Command::new("git")
    .argument("rev-parse")
    .argument("HEAD")
    .capture_only_stdout_string();

println!("Current commit: {}", output.trim());
```

## Testing Utilities

The crate provides a testing module with utilities for writing container-based tests:

```rust
#[test]
fn my_container_test() {
    if cbt::testing::platform_not_supported() {
        return;
    }

    // Test code that requires containers
    let backend = cbt::backend::autodetect::run().unwrap();
    // ... rest of test
}
```

The `platform_not_supported()` function returns `true` on macOS in GitHub Actions environments where container runtimes are not reliably available, allowing tests to gracefully skip in those environments.

## License

See workspace root for license information.
