# Changelog

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
