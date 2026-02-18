use crate::{Config, Error, RepoName, detect_repo_from_cwd, git};
use git_proc::Build;

#[derive(Debug, clap::Parser)]
pub struct List {
    /// Repository name [default: auto-detected, or list all if outside worktree]
    #[clap(long)]
    repo: Option<RepoName>,
}

impl List {
    pub async fn run(self, config: &Config) -> Result<(), Error> {
        let repo = match self.repo {
            Some(repo) => Some(repo),
            None => detect_repo_from_cwd(config).ok(),
        };

        match repo {
            Some(repo) => list_repo(config, &repo).await,
            None => list_all(config).await,
        }
    }
}

async fn list_repo(config: &Config, repo: &RepoName) -> Result<(), Error> {
    let repo_path = config.repo_path(repo);

    if !repo_path.exists() {
        return Err(Error::RepoNotFound(repo.clone()));
    }

    println!("{repo}:");

    let output = git_proc::worktree::list()
        .repo_path(&repo_path)
        .build()
        .stdout_capture()
        .string()
        .await?;

    for line in git::parse_worktree_list(&output) {
        println!("  {line}");
    }

    Ok(())
}

async fn list_all(config: &Config) -> Result<(), Error> {
    if !config.base_dir.exists() {
        log::info!("No repositories found");
        return Ok(());
    }

    let entries = std::fs::read_dir(&config.base_dir)?;

    let mut repos: Vec<RepoName> = Vec::new();

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir()
            && path.join("HEAD").exists()
            && let Some(name) = path.file_name()
            && let Some(name_str) = name.to_str()
            && let Ok(repo) = name_str.parse::<RepoName>()
        {
            repos.push(repo);
        }
    }

    repos.sort_by(|a, b| a.as_str().cmp(b.as_str()));

    for repo in repos {
        list_repo(config, &repo).await?;
        println!();
    }

    Ok(())
}
