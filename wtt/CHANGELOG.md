# Changelog

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
