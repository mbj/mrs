use crate::{Branch, Command, Config, Error, RepoName, detect_repo_from_cwd};

#[derive(Debug, clap::Parser)]
pub struct Remove {
    /// Branch name of the worktree to remove
    branch: Branch,

    /// Repository name [default: auto-detected from current directory]
    #[clap(long)]
    repo: Option<RepoName>,

    #[clap(long)]
    force: bool,
}

impl Remove {
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

        if !worktree_path.exists() {
            return Err(Error::WorktreeNotFound(worktree_path));
        }

        log::info!("Removing worktree at {}", worktree_path.display());

        let mut command = Command::new("git")
            .argument("-C")
            .argument(&bare_path)
            .argument("worktree")
            .argument("remove");

        if self.force {
            command = command.argument("--force");
        }

        command.argument(&worktree_path).status()?;

        cleanup_empty_parents(config, &repo, &self.branch)?;

        log::info!("Worktree removed");

        Ok(())
    }
}

fn cleanup_empty_parents(config: &Config, repo: &RepoName, branch: &Branch) -> Result<(), Error> {
    if !branch.has_parents() {
        return Ok(());
    }

    let worktree_base = config.worktree_base_path(repo);
    let mut current = config.worktree_path(repo, branch);

    while current != worktree_base {
        if let Some(parent) = current.parent() {
            if parent == worktree_base {
                break;
            }

            if parent.exists() {
                let is_empty = parent
                    .read_dir()
                    .is_ok_and(|mut entries| entries.next().is_none());

                if is_empty {
                    log::debug!("Removing empty directory {}", parent.display());
                    std::fs::remove_dir(parent)?;
                } else {
                    break;
                }
            }

            current = parent.to_path_buf();
        } else {
            break;
        }
    }

    Ok(())
}
