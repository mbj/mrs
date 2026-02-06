# Changelog

## 0.4.0

### Breaking Changes

- Typestate stream handling API: `Command::capture_stdout()` renamed to `Command::stdout_capture()`,
  `Command::capture_stderr()` renamed to `Command::stderr_capture()`
- Remove `Command::capture_stderr_stdout()` (use `.stdout_capture().stderr_capture()` instead)
- Remove `Command::output()` (use `.stdout_capture().stderr_capture().run()` instead)
- Replace `Capture` with `CaptureSingle<S>` using sealed `StreamMarker` trait (`Stdout`/`Stderr`)
- Replace `CaptureAllStreams` with `CaptureAll`
- Replace `Output` with `CaptureAllResult` (fields: `stdout`, `stderr`, `status`)
- Add `CaptureSingleResult` (fields: `bytes`, `status`)
- Remove `Output::success()`, `Output::into_stdout_string()`, `Output::into_stderr_string()`
- Remove `Spawn` builder, `Stdio` enum, `Child` struct
- Remove `Command::spawn()` (use `.build()` for native `tokio::process` access)

### Added

- `Command::stdout_null()`, `Command::stderr_null()` to redirect streams to /dev/null
- `Command::stdout_inherit()`, `Command::stderr_inherit()` for explicit inherit
- Bidirectional typestate transitions: `CaptureAll` -> `CaptureSingle<S>` -> `Command`
- `CaptureSingle<S>::run()` returning `CaptureSingleResult`
- `CaptureAll::run()` returning `CaptureAllResult`
- `Command::build()` to access the underlying `tokio::process::Command`

## 0.3.0

### Breaking Changes

- Switch from `std::process` to `tokio::process` for async process execution
- `Command::status()`, `Command::output()` are now `async fn`
- `Capture::bytes()`, `Capture::string()` are now `async fn`
- `CaptureAllStreams::output()` is now `async fn`
- `Child::wait()`, `Child::wait_with_output()` are now `async fn`
- `Child` stream accessors now return `tokio::process` types
- `Spawn` doc example uses `tokio::io::AsyncBufReadExt` instead of `std::io::BufRead`

### Added

- `tokio` dependency with `process` and `io-util` features

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
