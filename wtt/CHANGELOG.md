# Changelog

## 0.1.0

### Fixed

- `wtt` no longer operates on the wrong repository when run from inside
  another git operation, such as a hook or `git rebase -x`. Its git commands
  now ignore the repository-local environment that operation exports
  (`GIT_DIR`, `GIT_INDEX_FILE`, `GIT_WORK_TREE`, …), which previously
  redirected them onto the surrounding repository and could corrupt it.
  Global git configuration and authentication variables are still honoured.

### Changed

- Removed the pre-1.0 stability warning from the README.
- Raised minimum supported Rust version to 1.99.
- Dropped the `dirs` dependency. The home directory behind the default
  configuration and worktree paths now comes from `std::env::home_dir()`,
  which resolves it identically.

## 0.0.7

- Switch to async `cmd-proc` and `git-proc`: all command execution is now async
- Add `tokio` runtime with `#[tokio::main(flavor = "current_thread")]`

## 0.0.6

- Change `wtt setup` to make the repo argument optional, auto-extracting the
  repository name from the URL when `--repo` is not specified

## 0.0.5

- Add `teardown` command to remove a repository completely (inverse of setup)
- Fix `wtt add` upstream tracking for new branches by using git config directly,
  allowing `git push` to work before the remote branch exists

## 0.0.4

- Add automatic upstream configuration on `wtt add`, setting the branch to track
  `origin/<branch>` for proper `git push` and `git pull` functionality

## 0.0.3

- Add `--force` option to `wtt remove` to bypass dirty checkout limitations
- Add `rm` alias for the `remove` command

## 0.0.2

- Fix `wtt add` to use `origin/<branch>` as base when creating new worktrees,
  ensuring branches are based on the latest fetched remote state rather than
  potentially stale local refs
- Suppress confusing git stderr output when checking if a branch exists locally
