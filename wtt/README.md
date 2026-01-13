# wtt - Work Tree Tool

## Overview

CLI tool for managing git worktrees using bare clones.

## Installation

From a repository checkout:

```sh
cargo install --path wtt
```

## Paths

| Type | Location |
|------|----------|
| Bare clones | `~/.wtt/bare/<repo>.git` |
| Worktrees | `~/devel/<repo>/<branch>/` |

Branch names containing `/` become subdirectories (e.g., `feature/login` → `~/devel/myrepo/feature/login/`).

## Commands

### setup

Create bare clone and prepare worktree directory.

```sh
wtt setup --repo <repo> --url <url>
```

- Clones bare repo to `~/.wtt/bare/<repo>.git`
- Creates empty `~/devel/<repo>/` directory
- Does not create any worktrees

### add

Create a worktree.

```sh
wtt add <branch> [--base <ref>] [--repo <repo>]
```

- `--repo` auto-detected from cwd when inside a worktree
- `--base` defaults to repo's configured default branch
- Auto-detects existing vs new branch:
  - If branch exists (local/remote): checkout
  - If branch doesn't exist: create from base
- Errors if `--repo` not provided and can't detect from cwd

### list

List worktrees.

```sh
wtt list [--repo <repo>]
```

- No `--repo`, outside worktree: list all repos and their worktrees
- No `--repo`, inside worktree: list worktrees for detected repo
- `--repo` provided: list worktrees for specified repo

### remove

Remove a worktree.

```sh
wtt remove <branch> [--repo <repo>]
```

- Deletes worktree directory only
- Does not delete the branch
- `--repo` auto-detected from cwd when inside a worktree
- Errors if `--repo` not provided and can't detect from cwd
