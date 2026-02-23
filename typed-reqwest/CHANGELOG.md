# Changelog

## 0.0.1

- Initial release

### Added

- `Request<API>` trait for type-safe HTTP requests with associated response types
- `BaseUrl` for origin encapsulation with path segment building and percent-encoding
- `decoder` module for response decoding based on status code and content type
- `decoder!` macro for declarative response decoder definitions
- `link` module with `Paginated<T>` and `PaginatedRequest` for Link header pagination
- `testing` module (`test-utils` feature) with `TestRequest` for asserting request construction
