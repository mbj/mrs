# wtt - Work Tree Tool

> **Status**: Pre-1.0 - exists to serve [mbj/mrs](https://github.com/mbj/mrs) monorepo, expect breaking changes without notice.

## Overview

CLI tool for managing git worktrees using bare clones.

## Installation

From a repository checkout:

```bash
cargo install --path wtt
```

## Paths

| Type        | Default Location                       |
|-------------|----------------------------------------|
| Config      | `~/.config/wtt.toml`                   |
| Bare clones | `~/.local/share/wtt/bare/<repo>.git`   |
| Worktrees   | `~/devel/<repo>/<branch>/`             |

Branch names containing `/` become subdirectories (e.g., `feature/login` → `~/devel/myrepo/feature/login/`).

## Configuration

Configuration is loaded from `~/.config/wtt.toml` by default. All fields are optional.

```toml
bare_clone_dir = "/path/to/bare/clones"
worktree_dir = "/path/to/worktrees"
```

### CLI Flags

| Flag                    | Description                            |
|-------------------------|----------------------------------------|
| `--config-file <PATH>`  | Load configuration from specified file |
| `--no-config-file`      | Disable configuration file loading     |

## Commands

### setup

Create bare clone and prepare worktree directory.

```bash
wtt setup <URL> [--repo <REPO>]
```

- `<URL>` - Git remote URL to clone
- `--repo <REPO>` - Optional: Local name for the repository (defaults to repo name extracted from URL)
- Clones bare repo to `~/.local/share/wtt/bare/<repo>.git`
- Creates empty `~/devel/<repo>/` directory

**Examples:**

```bash
# Auto-extract repo name from URL (will use 'my-repo')
wtt setup git@github.com:user/my-repo.git

# Specify custom repo name
wtt setup git@github.com:user/my-repo.git --repo custom-name
```

### teardown

Remove a repository completely (inverse of setup).

```bash
wtt teardown [OPTIONS] <REPO>
```

- `<REPO>` - Repository name to remove
- `--force` - Force removal of worktrees with uncommitted changes
- Removes all worktrees
- Removes bare clone at `~/.local/share/wtt/bare/<repo>.git`
- Removes worktree directory at `~/devel/<repo>/`

### add

Create a worktree.

```bash
wtt add [OPTIONS] <BRANCH>
```

- `<BRANCH>` - Branch name for the new worktree
- `--base <BASE>` - Base ref for new branches (default: remote default branch)
- `--repo <REPO>` - Repository name (default: auto-detected from current directory)
- Auto-detects existing vs new branch:
  - If branch exists (local/remote): checkout
  - If branch doesn't exist: create from base
- Configures upstream tracking to `origin/<branch>` via git config, so `git push`
  and `git pull` work without additional flags, even for new branches that don't
  exist on the remote yet

### list

List worktrees.

```bash
wtt list [OPTIONS]
```

- `--repo <REPO>` - Repository name (default: auto-detected, or list all if outside worktree)

### remove

Remove a worktree.

```bash
wtt remove [OPTIONS] <BRANCH>
```

- `<BRANCH>` - Branch name of the worktree to remove
- `--repo <REPO>` - Repository name (default: auto-detected from current directory)
- Deletes worktree directory only, does not delete the branch
