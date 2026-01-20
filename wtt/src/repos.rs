use crate::{Config, Error, RepoName};

pub fn list_repos(config: &Config) -> Result<Vec<RepoName>, Error> {
    if !config.bare_clone_dir.exists() {
        return Ok(Vec::new());
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

    Ok(repos)
}
