use crate::{Config, Error, RepoName};

#[derive(Debug, clap::Parser)]
pub struct Teardown {
    /// Repository name to remove completely
    repo: RepoName,

    /// Force removal of worktrees with uncommitted changes
    #[clap(long)]
    force: bool,
}

impl Teardown {
    pub fn run(self, config: &Config) -> Result<(), Error> {
        let bare_clone = config.bare_clone(&self.repo)?;

        for branch in bare_clone.list_worktrees()? {
            config.worktree(&self.repo, &branch)?.remove(self.force)?;
        }

        log::info!("Removing bare repository at {}", bare_clone.path().display());
        std::fs::remove_dir_all(bare_clone.path())?;

        let worktree_base = config.base_dir_path(&self.repo);

        if worktree_base.exists() {
            log::info!("Removing worktree directory at {}", worktree_base.display());
            std::fs::remove_dir_all(&worktree_base)?;
        }

        log::info!("Teardown complete for repository '{}'", self.repo);

        Ok(())
    }
}
