# Changelog

## 0.1.0

- **Breaking**: Consolidate bare clone and worktree directories. Repositories
  are now bare-cloned directly into `<base-dir>/<repo>/` with worktrees checked
  out as subdirectories inside the bare clone. This eliminates the separate
  `bare_clone_dir`.
- **Breaking**: Config fields `bare_clone_dir` and `worktree_dir` replaced by
  single `base_dir` field (default: `~/devel`)
- `git branch` and other non-checkout commands now work inside
  `<base-dir>/<repo>/` since it is the bare clone itself

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
