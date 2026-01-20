use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::{Base, Branch, Command, Config, Error, RepoName, git, list_repos};

use super::remove::{cleanup_empty_parents, remove_worktree};

#[derive(Debug, clap::Parser)]
pub struct Gc {
    /// Repository name [default: all repos]
    #[clap(long)]
    repo: Option<RepoName>,

    /// Force removal of worktrees with uncommitted changes
    #[clap(long)]
    force: bool,
}

impl Gc {
    pub fn run(self, config: &Config) -> Result<(), Error> {
        let repos = match self.repo {
            Some(repo) => vec![repo],
            None => list_repos(config)?,
        };

        if repos.is_empty() {
            log::info!("No repositories found");
            return Ok(());
        }

        for repo in repos {
            gc_repo(config, &repo, self.force)?;
        }

        Ok(())
    }
}

fn gc_repo(config: &Config, repo: &RepoName, force: bool) -> Result<(), Error> {
    let bare_path = config.bare_repo_path(repo);

    if !bare_path.exists() {
        return Err(Error::RepoNotFound(repo.clone()));
    }

    log::info!("Fetching latest from remote for {}", repo);

    Command::new("git")
        .argument("-C")
        .argument(&bare_path)
        .argument("fetch")
        .argument("--all")
        .argument("--prune")
        .status()?;

    let default_branch = get_remote_default_branch(&bare_path)?;
    let default_ref = format!("origin/{default_branch}");

    let merged_branches = merged_branches(&bare_path, &default_ref, default_branch.as_str())?;

    if merged_branches.is_empty() {
        log::info!("No merged branches found for {}", repo);
        return Ok(());
    }

    let worktrees_by_branch = worktrees_by_branch(&bare_path)?;

    let mut removed_worktrees = 0;
    let mut deleted_branches = 0;

    for branch in merged_branches {
        if let Some(path) = worktrees_by_branch.get(branch.as_str()) {
            remove_worktree(&bare_path, path, force)?;
            cleanup_empty_parents(config, repo, &branch)?;
            removed_worktrees += 1;
        }

        delete_branch(&bare_path, &branch)?;
        deleted_branches += 1;
    }

    log::info!(
        "GC complete for {}: removed {} worktrees, deleted {} branches",
        repo,
        removed_worktrees,
        deleted_branches
    );

    Ok(())
}

fn get_remote_default_branch(bare_path: &Path) -> Result<Base, Error> {
    let output = Command::new("git")
        .argument("-C")
        .argument(bare_path)
        .argument("ls-remote")
        .argument("--symref")
        .argument("origin")
        .argument("HEAD")
        .stdout()
        .string()?;

    git::parse_default_branch(&output).map_err(|_| Error::DefaultBranchNotFound)
}

fn merged_branches(
    bare_path: &Path,
    default_ref: &str,
    default_branch: &str,
) -> Result<Vec<Branch>, Error> {
    let output = Command::new("git")
        .argument("-C")
        .argument(bare_path)
        .argument("for-each-ref")
        .argument("--format=%(refname:short)\t%(upstream:short)")
        .argument("refs/heads")
        .stdout()
        .string()?;

    let mut branches = Vec::new();

    for line in output.lines() {
        let (name, upstream) = line.split_once('\t').unwrap_or((line, ""));

        if name.is_empty() || name == default_branch {
            continue;
        }

        if upstream.is_empty() {
            continue;
        }

        if !upstream_ref_exists(bare_path, upstream)? {
            continue;
        }

        if !is_upstream_merged(bare_path, upstream, default_ref)? {
            continue;
        }

        if is_branch_ahead_of_upstream(bare_path, name, upstream)? {
            continue;
        }

        if let Ok(branch) = name.parse::<Branch>() {
            branches.push(branch);
        }
    }

    Ok(branches)
}

fn upstream_ref_exists(bare_path: &Path, upstream: &str) -> Result<bool, crate::CommandError> {
    let refname = format!("refs/remotes/{upstream}");

    let output = Command::new("git")
        .argument("-C")
        .argument(bare_path)
        .argument("show-ref")
        .argument("--verify")
        .argument("--quiet")
        .argument(refname)
        .output()?;

    Ok(output.success())
}

fn is_upstream_merged(
    bare_path: &Path,
    upstream: &str,
    default_ref: &str,
) -> Result<bool, crate::CommandError> {
    let output = Command::new("git")
        .argument("-C")
        .argument(bare_path)
        .argument("merge-base")
        .argument("--is-ancestor")
        .argument(upstream)
        .argument(default_ref)
        .output()?;

    Ok(output.success())
}

fn is_branch_ahead_of_upstream(
    bare_path: &Path,
    branch: &str,
    upstream: &str,
) -> Result<bool, crate::CommandError> {
    let output = Command::new("git")
        .argument("-C")
        .argument(bare_path)
        .argument("rev-list")
        .argument("--left-right")
        .argument("--count")
        .argument(format!("{upstream}...{branch}"))
        .stdout()
        .string()?;

    let mut counts = output.split_whitespace();
    let _upstream_ahead = counts.next().unwrap_or("0");
    let branch_ahead = counts.next().unwrap_or("0");

    Ok(branch_ahead != "0")
}

fn worktrees_by_branch(bare_path: &Path) -> Result<HashMap<String, PathBuf>, crate::CommandError> {
    let output = Command::new("git")
        .argument("-C")
        .argument(bare_path)
        .argument("worktree")
        .argument("list")
        .argument("--porcelain")
        .stdout()
        .string()?;

    Ok(parse_worktrees_porcelain(&output))
}

fn parse_worktrees_porcelain(input: &str) -> HashMap<String, PathBuf> {
    let mut worktrees = HashMap::new();
    let mut current_path: Option<PathBuf> = None;
    let mut current_branch: Option<String> = None;
    let mut current_bare = false;

    for line in input.lines() {
        if let Some(path) = line.strip_prefix("worktree ") {
            flush_worktree(
                &mut worktrees,
                current_path.take(),
                current_branch.take(),
                current_bare,
            );
            current_path = Some(PathBuf::from(path));
            current_branch = None;
            current_bare = false;
            continue;
        }

        if line == "bare" {
            current_bare = true;
            continue;
        }

        if let Some(branch) = line.strip_prefix("branch refs/heads/") {
            current_branch = Some(branch.to_string());
        }
    }

    flush_worktree(
        &mut worktrees,
        current_path.take(),
        current_branch.take(),
        current_bare,
    );

    worktrees
}

fn flush_worktree(
    worktrees: &mut HashMap<String, PathBuf>,
    path: Option<PathBuf>,
    branch: Option<String>,
    is_bare: bool,
) {
    if is_bare {
        return;
    }

    if let (Some(path), Some(branch)) = (path, branch) {
        worktrees.insert(branch, path);
    }
}

fn delete_branch(bare_path: &Path, branch: &Branch) -> Result<(), Error> {
    log::info!("Deleting branch {}", branch);

    Command::new("git")
        .argument("-C")
        .argument(bare_path)
        .argument("branch")
        .argument("-d")
        .argument(branch)
        .status()?;

    Ok(())
}
