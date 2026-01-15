use crate::{Base, Branch, Command, Config, Error, RepoName, detect_repo_from_cwd, git};

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

        Command::new("git")
            .argument("-C")
            .argument(&bare_path)
            .argument("fetch")
            .argument("--all")
            .status()?;

        let branch_exists = branch_exists(&bare_path, &self.branch)?;

        if branch_exists {
            log::info!(
                "Creating worktree for existing branch '{}' at {}",
                self.branch,
                worktree_path.display()
            );

            Command::new("git")
                .argument("-C")
                .argument(&bare_path)
                .argument("worktree")
                .argument("add")
                .argument(&worktree_path)
                .argument(&self.branch)
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

            Command::new("git")
                .argument("-C")
                .argument(&bare_path)
                .argument("worktree")
                .argument("add")
                .argument("-b")
                .argument(&self.branch)
                .argument(&worktree_path)
                .argument(&base)
                .status()?;
        }

        log::info!("Worktree created at {}", worktree_path.display());

        Ok(())
    }
}

fn branch_exists(
    bare_path: &std::path::Path,
    branch: &Branch,
) -> Result<bool, crate::CommandError> {
    let local_result = Command::new("git")
        .argument("-C")
        .argument(bare_path)
        .argument("show-ref")
        .argument("--verify")
        .argument(format!("refs/heads/{branch}"))
        .stdout()
        .bytes();

    if local_result.is_ok() {
        return Ok(true);
    }

    let remote_output = Command::new("git")
        .argument("-C")
        .argument(bare_path)
        .argument("ls-remote")
        .argument("--heads")
        .argument("origin")
        .argument(branch)
        .stdout()
        .string()?;

    Ok(!remote_output.trim().is_empty())
}

fn get_remote_default_branch(bare_path: &std::path::Path) -> Result<Base, Error> {
    let output = Command::new("git")
        .argument("-C")
        .argument(bare_path)
        .argument("ls-remote")
        .argument("--symref")
        .argument("origin")
        .argument("HEAD")
        .stdout()
        .string()?;

    let branch = git::parse_default_branch(&output).map_err(|_| Error::DefaultBranchNotFound)?;

    format!("origin/{branch}")
        .parse()
        .map_err(|_| Error::DefaultBranchNotFound)
}
