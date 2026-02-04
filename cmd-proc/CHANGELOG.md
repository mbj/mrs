# Changelog

## 0.2.0

### Breaking Changes

- Rename `Command::stdout()` to `Command::capture_stdout()`
- Rename `Command::stderr()` to `Command::capture_stderr()`
- Rename internal `CaptureStream` to `CaptureSingleStream`

### Added

- `Command::capture_stderr_stdout()` method returning `CaptureAllStreams` for capturing both streams
- `Capture::accept_nonzero_exit()` to allow non-zero exit codes without error
- `CaptureAllStreams::accept_nonzero_exit()` to allow non-zero exit codes without error

### Changed

- `Capture` now only pipes the targeted stream; the other stream inherits to the terminal instead of being silently discarded

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
