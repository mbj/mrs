# git-proc

Process-based git CLI wrapper with builder API.

## Why

Shelling out to `git` is often the right choice: it's battle-tested, handles edge cases, and
matches what users run manually. But raw `Command::new("git")` calls scatter string arguments
throughout your codebase, making them hard to review, refactor, and test.

git-proc provides:

- **Discoverable API**: Method names match `git --help` flags exactly. IDE autocomplete guides you.
- **Compile-time structure**: Builders ensure required arguments are present and ordered correctly.
- **Testable commands**: Compare built commands without executing them using `test_eq()`.
- **Consistent `-C` handling**: Every command supports `.repo_path()` for repository context.

This is not a reimplementation of git. It shells out to the `git` binary. For embedded git
functionality, use `gitoxide` or `git2`.

## Coverage

Command and flag coverage is not exhaustive. Only what was needed by direct dependencies in the
`mbj/mrs` workspace has been implemented. Similar crates exist (e.g., `git-cmd`), but it was
cheaper to centralize a minimal library here than to upstream all missing fields to external
projects.

## Usage

Each git command has a corresponding builder struct in its own module. Builders use method names
matching the git CLI options from `git <command> --help`.

```rust,ignore
use git_proc::{rev_parse, status, clone, fetch, worktree};
use git_proc::url::GitUrl;
use std::path::Path;

// git rev-parse --abbrev-ref HEAD
let branch = rev_parse::new()
    .abbrev_ref()
    .rev("HEAD")
    .stdout().string()?;

// git -C /path/to/repo status --porcelain
let status = status::new()
    .repo_path(Path::new("/path/to/repo"))
    .porcelain()
    .stdout().string()?;

// git clone --bare <url> <path>
let url: GitUrl = "https://github.com/user/repo.git".parse()?;
clone::new(&url)
    .bare()
    .directory(Path::new("/path/to/bare.git"))
    .status()?;

// git -C /path/to/repo fetch --all
fetch::new()
    .repo_path(Path::new("/path/to/repo"))
    .all()
    .status()?;

// git -C /path/to/repo worktree add -b new-branch /path/to/worktree origin/main
worktree::add(Path::new("/path/to/worktree"))
    .repo_path(Path::new("/path/to/repo"))
    .new_branch("new-branch")
    .commit_ish("origin/main")
    .status()?;
```

## Available Commands

| Module       | Command              | Description                          |
|--------------|----------------------|--------------------------------------|
| `add`        | `git add`            | Add file contents to the index       |
| `clone`      | `git clone`          | Clone a repository                   |
| `commit`     | `git commit`         | Record changes to the repository     |
| `config`     | `git config`         | Get and set repository options       |
| `fetch`      | `git fetch`          | Download objects and refs            |
| `init`       | `git init`           | Create an empty repository           |
| `ls_remote`  | `git ls-remote`      | List references in a remote          |
| `push`       | `git push`           | Update remote refs                   |
| `remote`     | `git remote`         | Manage remotes (get-url)             |
| `rev_list`   | `git rev-list`       | List commit objects                  |
| `rev_parse`  | `git rev-parse`      | Parse revision specifications        |
| `show`       | `git show`           | Show objects                         |
| `show_ref`   | `git show-ref`       | List references                      |
| `status`     | `git status`         | Show working tree status             |
| `url`        | -                    | URL and remote types (`GitUrl`, `Remote`, `RemoteName`) |
| `worktree`   | `git worktree`       | Manage worktrees (list, add, remove) |

## Testing

Enable the `test-utils` feature to access the `test_eq()` method on builders, which compares
the built command against an expected `cmd_proc::Command` using debug representation.

```rust,ignore
#[cfg(feature = "test-utils")]
#[test]
fn test_fetch_all() {
    use std::path::Path;

    let expected = cmd_proc::Command::new("git")
        .argument("-C")
        .argument("/repo")
        .argument("fetch")
        .argument("--all");

    git_proc::fetch::new()
        .repo_path(Path::new("/repo"))
        .all()
        .test_eq(&expected);
}
```
