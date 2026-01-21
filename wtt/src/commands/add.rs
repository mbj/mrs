use crate::{
    Base, Branch, CommandError, Config, Error, ORIGIN, RepoName, detect_repo_from_cwd, git,
};

#[derive(Debug, clap::Parser)]
pub struct Add {
    /// Branch name for the new worktree
    branch: Branch,

    /// Base ref for new branches [default: remote default branch]
    #[clap(long)]
    base: Option<Base>,

    /// Repository name [default: auto-detected from current directory]
    #[clap(long)]
    repo: Option<RepoName>,
}

impl Add {
    pub fn run(self, config: &Config) -> Result<(), Error> {
        let repo = match self.repo {
            Some(repo) => repo,
            None => detect_repo_from_cwd(config)?,
        };

        let bare_path = config.bare_repo_path(&repo);

        if !bare_path.exists() {
            return Err(Error::RepoNotFound(repo));
        }

        let worktree_path = config.worktree_path(&repo, &self.branch);

        if worktree_path.exists() {
            return Err(Error::WorktreeExists(worktree_path));
        }

        log::info!("Fetching latest from remote");

        git_proc::fetch::new()
            .repo_path(&bare_path)
            .all()
            .status()?;

        if branch_exists(&bare_path, &self.branch)? {
            log::info!(
                "Creating worktree for existing branch '{}' at {}",
                self.branch,
                worktree_path.display()
            );

            git_proc::worktree::add(&worktree_path)
                .repo_path(&bare_path)
                .branch(self.branch.as_str())
                .status()?;
        } else {
            let base = match self.base {
                Some(base) => base,
                None => get_remote_default_branch(&bare_path)?,
            };

            log::info!(
                "Creating worktree for new branch '{}' from '{}' at {}",
                self.branch,
                base,
                worktree_path.display()
            );

            git_proc::worktree::add(&worktree_path)
                .repo_path(&bare_path)
                .new_branch(self.branch.as_str())
                .commit_ish(base.as_str())
                .status()?;
        }

        set_upstream(&worktree_path, &self.branch)?;

        log::info!("Worktree created at {}", worktree_path.display());

        Ok(())
    }
}

fn branch_exists(bare_path: &std::path::Path, branch: &Branch) -> Result<bool, CommandError> {
    let local_result = git_proc::show_ref::new()
        .repo_path(bare_path)
        .verify()
        .pattern(&format!("refs/heads/{branch}"))
        .stdout()
        .bytes();

    if local_result.is_ok() {
        return Ok(true);
    }

    let remote_output = git_proc::ls_remote::new()
        .repo_path(bare_path)
        .heads()
        .remote(&ORIGIN)
        .pattern(branch.as_str())
        .stdout()
        .string()?;

    Ok(!remote_output.trim().is_empty())
}

fn get_remote_default_branch(bare_path: &std::path::Path) -> Result<Base, Error> {
    let output = git_proc::ls_remote::new()
        .repo_path(bare_path)
        .symref()
        .remote(&ORIGIN)
        .pattern("HEAD")
        .stdout()
        .string()?;

    let branch = git::parse_default_branch(&output).map_err(|_| Error::DefaultBranchNotFound)?;

    format!("origin/{branch}")
        .parse()
        .map_err(|_| Error::DefaultBranchNotFound)
}

fn set_upstream(worktree_path: &std::path::Path, branch: &Branch) -> Result<(), CommandError> {
    log::info!("Setting upstream to origin/{branch}");

    git_proc::config::new(&format!("branch.{branch}.remote"))
        .repo_path(worktree_path)
        .value("origin")
        .status()?;

    git_proc::config::new(&format!("branch.{branch}.merge"))
        .repo_path(worktree_path)
        .value(&format!("refs/heads/{branch}"))
        .status()
}
