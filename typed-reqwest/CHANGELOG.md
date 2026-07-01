# Changelog

## 0.1.0

### Added

- Response decoding emits `tracing` spans: `decode` (INFO) with child `buffer`
  and `deserialize` spans (DEBUG). They nest under the caller's request span, so
  buffering and deserialization timing are visible without instrumenting the
  decoder yourself.
- Buffered response bodies are now capped at a configurable maximum size
  (`ResponseBuilder::buffered_body_max_size`, defaulting to 10 MiB to match the
  response payload ceiling of common API gateways). When the
  response advertises a body size, that size becomes the read limit, capped at
  the maximum; otherwise the maximum applies. Oversized bodies are rejected with
  `ErrorReason::DeclaredBodyTooLarge` (when the response advertises a size over
  the maximum, rejected before reading) or `ErrorReason::BufferedBodyTooLarge`
  (when the body exceeds the limit while being read), and on rejection the
  connection is dropped rather than drained.

### Changed

- Decoders that ignore the response body (such as constant decoders) now drain
  and discard the body one chunk at a time instead of buffering it entirely,
  keeping memory usage bounded while still consuming the body so the connection
  can be reused.

## 0.0.3

### Changed

- Raised minimum supported Rust version to 1.95.

## 0.0.2

### Changed

- Response decoding negotiates on the `Content-Type` header, rejecting
  unexpected media types instead of assuming JSON.
- Renamed the JSON-specific decode error to a content-type-agnostic body
  decode error.

## 0.0.1

- Initial release

### Added

- `Request<API>` trait for type-safe HTTP requests with associated response types
- `BaseUrl` for origin encapsulation with path segment building and percent-encoding
- `decoder` module for response decoding based on status code and content type
- `decoder!` macro for declarative response decoder definitions
- `link` module with `Paginated<T>` and `PaginatedRequest` for Link header pagination
- `testing` module (`test-utils` feature) with `TestRequest` for asserting request construction
