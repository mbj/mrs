# Changelog

## 0.1.0

### Breaking Changes

- Rename `EnvVariableName::from_static` to `from_static_or_panic`

### Added

- `Display` implementation for `EnvVariableName`
- `PartialOrd` and `Ord` derives for `EnvVariableName`

### Fixed

- `env` and `envs` methods now require `EnvVariableName` instead of raw strings, enforcing validation

## 0.0.1

- Initial release
