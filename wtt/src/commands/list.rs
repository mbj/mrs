use crate::{Command, Config, Error, RepoName, detect_repo_from_cwd, git, list_repos};

#[derive(Debug, clap::Parser)]
pub struct List {
    /// Repository name [default: auto-detected, or list all if outside worktree]
    #[clap(long)]
    repo: Option<RepoName>,
}

impl List {
    pub fn run(self, config: &Config) -> Result<(), Error> {
        let repo = match self.repo {
            Some(repo) => Some(repo),
            None => detect_repo_from_cwd(config).ok(),
        };

        match repo {
            Some(repo) => list_repo(config, &repo),
            None => list_all(config),
        }
    }
}

fn list_repo(config: &Config, repo: &RepoName) -> Result<(), Error> {
    let bare_path = config.bare_repo_path(repo);

    if !bare_path.exists() {
        return Err(Error::RepoNotFound(repo.clone()));
    }

    println!("{repo}:");

    let output = Command::new("git")
        .argument("-C")
        .argument(&bare_path)
        .argument("worktree")
        .argument("list")
        .stdout()
        .string()?;

    for line in git::parse_worktree_list(&output) {
        println!("  {line}");
    }

    Ok(())
}

fn list_all(config: &Config) -> Result<(), Error> {
    let repos = list_repos(config)?;

    if repos.is_empty() {
        log::info!("No repositories found");
        return Ok(());
    }

    for (index, repo) in repos.iter().enumerate() {
        list_repo(config, repo)?;
        if index + 1 < repos.len() {
            println!();
        }
    }

    Ok(())
}
