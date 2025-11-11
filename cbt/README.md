# cbt - Container Backend Tool

A Rust library providing a unified API for Docker and Podman container runtimes.

## ⚠️ Status

**CBT is highly unstable and exists solely to serve pg-ephemeral.** The API is changing frequently and nothing should be considered stable. Breaking changes occur without notice as the library evolves to meet pg-ephemeral's needs.

Do not use this library for other projects at this time.

## Goals

- **Unified API**: Single interface for both Docker and Podman
- **Auto-detection**: Automatically detects available container runtime
- **Environment override**: Control backend selection via `CBT_BACKEND` environment variable
- **Container lifecycle management**: Run, execute commands, inspect, and manage containers
- **Image building**: Build images from Dockerfiles or inline instructions
- **Content-based hashing**: Automatic tag generation based on SHA256 of build context/instructions for deterministic builds

## Content-Based Image Hashing

CBT supports automatic tag generation based on content hashing (SHA256). This ensures deterministic builds where the same content always produces the same image tag.

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

## License

See workspace root for license information.
