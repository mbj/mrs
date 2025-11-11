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

// Run a detached container with automatic cleanup
let definition = Definition::new(backend, Image::from("postgres:latest"))
    .env("POSTGRES_PASSWORD", "secret")
    .env("POSTGRES_USER", "admin")
    .publish(Publish::from("127.0.0.1::5432/tcp"));

definition.with_container(|container| {
    // Execute commands in the running container
    let output = container.exec_capture_only_stdout(
        [],
        "psql",
        ["-U", "admin", "-c", "SELECT version()"]
    );
    // Container automatically stopped and cleaned up
});

// Or manually manage lifecycle
let mut container = definition.run_detached();
let output = container.exec_capture_only_stdout([], "psql", ["-U", "admin", "-c", "SELECT version()"]);
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

### Command with Stdin

Commands can accept input via stdin:

```rust
use cbt::Command;

let input = b"Hello, World!";
let output = Command::new("cat")
    .stdin_bytes(input.to_vec())
    .capture_only_stdout();

assert_eq!(output, input);
```

### Building Images

CBT provides an API for building container images from Dockerfiles:

```rust
use cbt::{BuildDefinition, Image};

let backend = cbt::backend::autodetect::run().unwrap();

// Build from a directory containing a Dockerfile
let image = Image::from("myapp:latest");
let definition = BuildDefinition::from_directory(image, "./app");
cbt::image::build(backend, &definition).expect("Build failed");

// Build from inline Dockerfile instructions
let dockerfile = "FROM alpine:latest\nRUN apk add curl";
let image = Image::from("myimage:latest");
let definition = BuildDefinition::from_instructions(image, dockerfile);
cbt::image::build(backend, &definition).expect("Build failed");

// Build only if not already present
cbt::image::build_if_absent(backend, &definition).expect("Build failed");
```

### Content-Based Image Hashing

CBT supports automatic tag generation based on content hashing (SHA256). This ensures deterministic builds where the same content always produces the same image tag:

```rust
use cbt::BuildDefinition;

let backend = cbt::backend::autodetect::run().unwrap();

// Build from directory with automatic hash-based tag
// Produces: "myapp:a1b2c3d4..." where hash is SHA256 of directory contents
let definition = BuildDefinition::from_directory_hash("myapp", "./app");
cbt::image::build(backend, &definition).expect("Build failed");

// Build from inline Dockerfile with automatic hash-based tag
let dockerfile = "FROM alpine:latest\nRUN echo 'hello'";
let definition = BuildDefinition::from_instructions_hash("myapp", dockerfile);
// Produces: "myapp:e3b0c44..." where hash is SHA256 of Dockerfile content
cbt::image::build(backend, &definition).expect("Build failed");

// Same content always produces same hash - perfect for caching
let definition2 = BuildDefinition::from_instructions_hash("myapp", dockerfile);
assert_eq!(definition.image, definition2.image);
```

Benefits of content-based hashing:
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

### Image Management Operations

```rust
let backend = cbt::backend::autodetect::run().unwrap();
let image = Image::from("alpine:latest");

// Pull an image only if not already present locally
cbt::image::pull_if_absent(backend, &image);

// Check if an image is present locally
if cbt::image::is_present(backend, &image) {
    println!("Image is available");
}

// Pull an image from a registry
cbt::image::pull(backend, &image);

// Tag an image
let source = Image::from("alpine:latest");
let target = Image::from("myapp:v1.0");
cbt::image::tag(backend, &source, &target);

// Push to a registry
cbt::image::push(backend, &target);

// Remove an image
backend.remove_image(&image);
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
