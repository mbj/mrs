use crate::{Command, Config, Error, RepoName, detect_repo_from_cwd, git};

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
    if !config.bare_clone_dir.exists() {
        log::info!("No repositories found");
        return Ok(());
    }

    let entries = std::fs::read_dir(&config.bare_clone_dir)?;

    let mut repos: Vec<RepoName> = Vec::new();

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir()
            && let Some(name) = path.file_name()
            && let Some(name_str) = name.to_str()
            && let Some(repo_name) = name_str.strip_suffix(".git")
            && let Ok(repo) = repo_name.parse::<RepoName>()
        {
            repos.push(repo);
        }
    }

    repos.sort_by(|a, b| a.as_str().cmp(b.as_str()));

    for repo in repos {
        list_repo(config, &repo)?;
        println!();
    }

    Ok(())
}
