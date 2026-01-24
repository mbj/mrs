use std::path::{Path, PathBuf};

use crate::Branch;

#[derive(Debug)]
pub struct BareClone(PathBuf);

impl BareClone {
    #[must_use]
    pub fn path(&self) -> &Path {
        &self.0
    }

    pub fn branch_exists(&self, branch: &Branch) -> Result<bool, git_proc::CommandError> {
        let local_result = git_proc::show_ref::new()
            .repo_path(self.path())
            .verify()
            .pattern(&format!("refs/heads/{branch}"))
            .stdout()
            .bytes();

        if local_result.is_ok() {
            return Ok(true);
        }

        let remote_output = git_proc::ls_remote::new()
            .repo_path(self.path())
            .heads()
            .remote(&crate::ORIGIN)
            .pattern(branch.as_str())
            .stdout()
            .string()?;

        Ok(!remote_output.trim().is_empty())
    }

    pub fn list_worktrees(&self) -> Result<Vec<Branch>, git_proc::CommandError> {
        let output = git_proc::worktree::list()
            .repo_path(self.path())
            .porcelain()
            .stdout()
            .string()?;

        Ok(crate::git::parse_worktree_list(&output))
    }

    pub fn remote_default_branch(&self) -> Result<crate::Base, crate::Error> {
        let output = git_proc::ls_remote::new()
            .repo_path(self.path())
            .symref()
            .remote(&crate::ORIGIN)
            .pattern("HEAD")
            .stdout()
            .string()?;

        let branch = crate::git::parse_default_branch(&output)
            .map_err(|_| crate::Error::DefaultBranchNotFound)?;

        format!("origin/{branch}")
            .parse()
            .map_err(|_| crate::Error::DefaultBranchNotFound)
    }

    pub fn fetch(&self) -> Result<(), git_proc::CommandError> {
        log::info!("Fetching latest from remote");

        git_proc::fetch::new()
            .repo_path(self.path())
            .all()
            .status()
    }
}

#[derive(Debug)]
pub struct Worktree {
    bare_clone: BareClone,
    base_dir: PathBuf,
    checkout_dir: PathBuf,
    branch: Branch,
}

impl Worktree {
    pub fn remove(&self, force: bool) -> Result<(), crate::Error> {
        log::info!("Removing worktree at {}", self.checkout_dir.display());

        git_proc::worktree::remove(&self.checkout_dir)
            .repo_path(self.bare_clone.path())
            .force_if(force)
            .status()?;

        self.cleanup_empty_parents()?;

        log::info!("Worktree removed");

        Ok(())
    }

    pub fn set_upstream(&self) -> Result<(), git_proc::CommandError> {
        log::info!("Setting upstream to origin/{}", self.branch);

        git_proc::config::new(&format!("branch.{}.remote", self.branch))
            .repo_path(&self.checkout_dir)
            .value("origin")
            .status()?;

        git_proc::config::new(&format!("branch.{}.merge", self.branch))
            .repo_path(&self.checkout_dir)
            .value(&format!("refs/heads/{}", self.branch))
            .status()
    }

    fn cleanup_empty_parents(&self) -> Result<(), std::io::Error> {
        if !self.branch.has_parents() {
            return Ok(());
        }

        let mut current = self.checkout_dir.clone();

        while current != self.base_dir {
            if let Some(parent) = current.parent() {
                if parent == self.base_dir {
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
}

#[derive(Debug, thiserror::Error)]
#[error("Bare clone not found for '{repo}' at '{}'", .path.display())]
pub struct BareCloneNotFound {
    pub repo: crate::RepoName,
    pub path: PathBuf,
}

#[derive(Clone, Debug)]
pub struct Config {
    pub bare_clone_dir: PathBuf,
    pub worktree_dir: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        let home = dirs::home_dir().expect("HOME directory not found");
        Self {
            bare_clone_dir: home.join(".local/share/wtt/bare"),
            worktree_dir: home.join("devel"),
        }
    }
}

impl Config {
    pub fn bare_clone(&self, repo: &crate::RepoName) -> Result<BareClone, BareCloneNotFound> {
        let path = self.bare_repo_path(repo);
        if path.exists() {
            Ok(BareClone(path))
        } else {
            Err(BareCloneNotFound {
                repo: repo.clone(),
                path,
            })
        }
    }

    pub fn worktree(
        &self,
        repo: &crate::RepoName,
        branch: &Branch,
    ) -> Result<Worktree, crate::Error> {
        let bare_clone = self.bare_clone(repo)?;
        let path = self.worktree_path(repo, branch);

        if !path.exists() {
            return Err(crate::Error::WorktreeNotFound(path));
        }

        Ok(Worktree {
            bare_clone,
            base_dir: self.base_dir_path(repo),
            checkout_dir: path,
            branch: branch.clone(),
        })
    }

    #[must_use]
    pub fn bare_repo_path(&self, repo: &crate::RepoName) -> PathBuf {
        self.bare_clone_dir.join(format!("{}.git", repo.as_str()))
    }

    #[must_use]
    pub fn base_dir_path(&self, repo: &crate::RepoName) -> PathBuf {
        self.worktree_dir.join(repo.as_str())
    }

    #[must_use]
    pub fn worktree_path(&self, repo: &crate::RepoName, branch: &Branch) -> PathBuf {
        self.base_dir_path(repo).join(branch.as_str())
    }

    pub fn load(source: &Source) -> Result<Self, Error> {
        match source {
            Source::None => Ok(Self::default()),
            Source::File(path) => Self::load_file(path),
            Source::Implicit => Self::load_implicit(),
        }
    }

    fn load_file(path: &PathBuf) -> Result<Self, Error> {
        let contents = std::fs::read_to_string(path).map_err(|error| Error::IO {
            path: path.clone(),
            kind: error.kind(),
        })?;
        Self::load_toml(&contents)
    }

    fn load_implicit() -> Result<Self, Error> {
        let path = Self::default_config_path();

        match std::fs::read_to_string(&path) {
            Ok(contents) => Self::load_toml(&contents),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(Self::default()),
            Err(error) => Err(Error::IO {
                path,
                kind: error.kind(),
            }),
        }
    }

    fn load_toml(contents: &str) -> Result<Self, Error> {
        let file_config: FileConfig = toml::from_str(contents)?;
        Ok(file_config.into())
    }

    #[must_use]
    pub fn default_config_path() -> PathBuf {
        dirs::home_dir()
            .expect("HOME directory not found")
            .join(".config/wtt.toml")
    }
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct FileConfig {
    bare_clone_dir: Option<PathBuf>,
    worktree_dir: Option<PathBuf>,
}

impl From<FileConfig> for Config {
    fn from(file_config: FileConfig) -> Self {
        let default = Self::default();
        Self {
            bare_clone_dir: file_config.bare_clone_dir.unwrap_or(default.bare_clone_dir),
            worktree_dir: file_config.worktree_dir.unwrap_or(default.worktree_dir),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Could not load config file {path}: {kind}")]
    IO {
        path: PathBuf,
        kind: std::io::ErrorKind,
    },
    #[error("Decoding as toml failed: {0}")]
    TomlDecode(#[from] toml::de::Error),
}

pub enum Source {
    None,
    File(PathBuf),
    Implicit,
}
