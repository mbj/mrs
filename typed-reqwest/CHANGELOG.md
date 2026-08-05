# Changelog

## 0.2.0

### Added

- `ContentTypes::add_raw()` and `ContentTypes::add_raw_match()` register a
  decoder that hands the `reqwest::Response` back to the caller with its body
  unread, once the status code and content type have been negotiated as usual.
  Everything past that negotiation is the caller's responsibility — the body is
  neither bounded by the configured maximum buffered size, nor buffered, nor
  drained, so a response that is dropped unread discards its connection rather
  than returning it to the pool. Intended for bodies that should not be
  buffered at all, such as a streamed download handed to another consumer. Pass
  `Ok` as the decode function for a request whose `Response` type is
  `reqwest::Response` itself.

### Breaking Changes

- `BodyDecoder`, its `new()` / `body_only()` / `constant()` constructors, and
  `ContentTypes::get()` / `ContentTypes::default()` are no longer public. They
  were reachable from the closure passed to `ResponseBuilder::status_code()`,
  but inert: no public API accepts a `BodyDecoder`, so one could only be
  constructed or borrowed and then discarded. Decoders continue to be
  registered through the `ContentTypes` methods that take closures.

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
