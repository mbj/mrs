# Changelog

## 0.0.2

- URL parsing now accepts a `&str` and uses RFC 3986 semantics; `+` is treated as a literal plus
  (use `%20` for spaces). The prior behavior treated `+` as space, which was incorrect for URIs.
- URL parsing errors now report field-specific detail via `ParseError::Field`.

## 0.0.1

- Initial release
- URL parsing support
- `FromStr` instance for `SslMode`
- Optional `sqlx` feature flag
