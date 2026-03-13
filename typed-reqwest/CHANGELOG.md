# Changelog

## 0.0.2

### Changed

- Content-type matching now uses `mime::Mime` instead of raw `HeaderValue` strings; matching is exact including parameters (e.g. `application/json` and `application/json; charset=utf-8` are distinct registrations) so that body decoding can be specific to each content type variant
- `ContentTypes::add` and `add_with_headers` now take `mime::Mime` instead of `&'static str`
- `ContentTypes::json` and `json_map` now register a single `APPLICATION_JSON` entry instead of two
- Renamed `ErrorReason::JsonDecodeError` to `ErrorReason::BodyDecodeError`
- `ErrorReason::UnexpectedContentType` now holds a `String` instead of `HeaderValue`
- Builder methods (`status_code`, `status_code_json`, etc.) now return `Result<Self, ResponseBuilderError>`
- `ContentTypes::add`, `add_with_headers`, `json`, `json_map` now return `Result<(), ResponseBuilderError>`
- `decoder!` macro now panics with a descriptive message on builder errors

### Added

- `mime` dependency for structured content-type handling
- `ResponseBuilderError` enum for builder configuration errors
- Duplicate content-type detection when registering decoders

## 0.0.1

- Initial release

### Added

- `Request<API>` trait for type-safe HTTP requests with associated response types
- `BaseUrl` for origin encapsulation with path segment building and percent-encoding
- `decoder` module for response decoding based on status code and content type
- `decoder!` macro for declarative response decoder definitions
- `link` module with `Paginated<T>` and `PaginatedRequest` for Link header pagination
- `testing` module (`test-utils` feature) with `TestRequest` for asserting request construction
