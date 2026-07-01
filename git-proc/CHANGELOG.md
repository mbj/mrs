# Changelog

## 0.6.0

### Breaking Changes

- `Build::build` now returns `Result<cmd_proc::Command, EnvError>` instead of
  `cmd_proc::Command`. Building a command resolves git's repository-local
  environment, which can fail; the failure is now propagated rather than
  panicked or silently ignored. Call sites must add `?` (or otherwise handle the
  `Result`).
- The `status()` convenience methods (`add`, `clone`, `commit`, `config`,
  `init`, `tag`, `clean`, `reset`, `push`, `checkout`, `fetch`, `worktree`) now
  return `Result<(), Error>` — a unified error wrapping both `EnvError`
  (environment resolution) and `cmd_proc::CommandError` (process execution) —
  instead of `Result<(), cmd_proc::CommandError>`.

### Changed

- Every command now clears git's repository-local environment variables — the
  set reported by `git rev-parse --local-env-vars` (`GIT_DIR`, `GIT_INDEX_FILE`,
  `GIT_WORK_TREE`, …) — before spawning `git`, so `git-proc` is never hijacked
  onto a surrounding git operation's repository or index (e.g. when run from a
  hook or `git rebase -x`, where `GIT_DIR`/`GIT_INDEX_FILE` would otherwise
  override `-C`; `GIT_INDEX_FILE` has no command-line equivalent at all). The
  set is queried from the installed `git` at runtime, so it always matches that
  git's notion of repository-local state. Global config and auth variables
  outside that set (`GIT_SSH_COMMAND`, `GIT_ASKPASS`, `GIT_CONFIG_GLOBAL`, …) —
  and all non-`GIT_*` configuration (`HOME`, `SSH_AUTH_SOCK`, ssh config) — are
  preserved, so authentication and global config keep working.

### Added

- `EnvPolicy` (`ClearLocal` (default), `ClearAll`, `Inherit`), `EnvError`
  describing the environment-clearing failure, and `Error` unifying `EnvError`
  with `cmd_proc::CommandError`.
- `.env_policy(EnvPolicy)` builder method on every command, selecting the
  clearing policy per invocation (e.g. `ClearAll` for a fully hermetic run,
  `Inherit` to opt out).
- `init` and `clone` now clear the git environment as well (previously they
  spawned `git` with the ambient environment intact, so an ambient `GIT_DIR` /
  `GIT_INDEX_FILE` could hijack them too).
- `symbolic_ref` module (`git symbolic-ref <name>`), `config::list()`
  (`git config --list`), and `--verify` / `--quiet` flags on `rev_parse`.

## 0.5.1

### Changed

- Raised minimum supported Rust version to 1.95.

## 0.5.0

### Breaking Changes

- `Commit::env` now accepts `cmd_proc::EnvVariableName` (no lifetime parameter)
  for the key. The `<'a>` parameter was dropped at the `cmd_proc` level since
  all public constructors produced `'static`-bound names anyway. `Commit<'a>`
  itself still carries `'a` for the borrowed `&'a OsStr` env value.

## 0.4.0

### Breaking Changes

- Update to `cmd-proc` 0.5.0. `cmd_proc::Command` and `cmd_proc::CommandError`
  are re-exposed through public API (`status()` return types on `add`,
  `clone`, `config`, `init`, `worktree`, and `build()` / `test_eq()` accessors),
  so the struct-to-enum shape change in `cmd-proc` 0.5.0 is a breaking change
  for git-proc consumers as well.

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
