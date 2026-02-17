# Changelog

## 0.3.0

### Breaking Changes

- Rename module `url` to `repository`
- Rename `GitUrl` to `repository::Address`
- Rename `GitUrlError` to `repository::AddressError`
- Rename `SshUrl` to `SshAddress`
- Rename `PathUrl` to `PathAddress`
- Rename `Remote::Url` variant to `Remote::RepositoryAddress`

`GitUrl` was inaccurate: SCP-style SSH addresses (`git@host:path`) and bare
local paths (`/path/to/repo`) are not URLs. The `Git` prefix was redundant
within the `git_proc` crate. The module rename from `url` to `repository`
allows shorter qualified type names (e.g. `repository::Address`).

`HttpsUrl` is unchanged as it represents an actual URL. `GitProtocolUrl` is
renamed to `GitUrl` for consistency with `HttpsUrl` (`{Scheme}Url` pattern).

## 0.2.1

### Changed

- Update to `cmd-proc` 0.4.0

## 0.2.0

### Breaking Changes

- Switch to async `cmd-proc` 0.3.0: `Build::status()` and capture chain terminal
  methods (`.string()`, `.bytes()`, `.output()`) are now `async fn`

### Added

- `tokio` dependency with `process` feature

## 0.1.0

### Breaking Changes

- Remove `.stdout()` wrapper methods from all builder types (`Config`, `LsRemote`, `Push`, `Remote`, `RevList`, `RevParse`, `Show`, `ShowRef`, `Status`, `worktree::List`)
- Remove `.stderr()` wrapper methods from builder types
- Remove `.output()` wrapper methods from builder types (`Push`, `Remote`, `RevList`, `RevParse`, `Show`)

### Migration

Use `.build()` to get a `cmd_proc::Command`, then call stream methods on it:

```rust
// Before
let output = git_proc::rev_parse::new()
    .rev("HEAD")
    .stdout()
    .string()?;

// After
use git_proc::Build;

let output = git_proc::rev_parse::new()
    .rev("HEAD")
    .build()
    .capture_stdout()
    .string()?;
```

This change delegates all stream configuration to `cmd_proc::Command`, providing access to the full stream API including `.accept_nonzero_exit()` and `.capture_stderr_stdout()`.

## 0.0.1

- Initial release
