# Changelog

## 0.0.2

- Fix `wtt add` to use `origin/<branch>` as base when creating new worktrees,
  ensuring branches are based on the latest fetched remote state rather than
  potentially stale local refs
- Suppress confusing git stderr output when checking if a branch exists locally
