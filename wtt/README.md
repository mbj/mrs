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
wtt setup <REPO> <URL>
```

- `<REPO>` - Local name for the repository
- `<URL>` - Git remote URL to clone
- Clones bare repo to `~/.wtt/bare/<repo>.git`
- Creates empty `~/devel/<repo>/` directory

### add

Create a worktree.

```sh
wtt add [OPTIONS] <BRANCH>
```

- `<BRANCH>` - Branch name for the new worktree
- `--base <BASE>` - Base ref for new branches (default: remote default branch)
- `--repo <REPO>` - Repository name (default: auto-detected from current directory)
- Auto-detects existing vs new branch:
  - If branch exists (local/remote): checkout
  - If branch doesn't exist: create from base

### list

List worktrees.

```sh
wtt list [OPTIONS]
```

- `--repo <REPO>` - Repository name (default: auto-detected, or list all if outside worktree)

### remove

Remove a worktree.

```sh
wtt remove [OPTIONS] <BRANCH>
```

- `<BRANCH>` - Branch name of the worktree to remove
- `--repo <REPO>` - Repository name (default: auto-detected from current directory)
- Deletes worktree directory only, does not delete the branch
