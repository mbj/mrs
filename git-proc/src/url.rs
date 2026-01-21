use std::borrow::Cow;
use std::path::{Path, PathBuf};

/// A validated git repository URL.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GitUrl {
    /// SSH URL: `git@host:path` or `ssh://user@host/path`
    Ssh(SshUrl),
    /// HTTPS URL: `https://host/path`
    Https(HttpsUrl),
    /// Git protocol URL: `git://host/path`
    Git(GitProtocolUrl),
    /// Local file path: `/path/to/repo` or `file:///path/to/repo`
    Path(PathUrl),
}

impl GitUrl {
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Ssh(url) => url.as_str(),
            Self::Https(url) => url.as_str(),
            Self::Git(url) => url.as_str(),
            Self::Path(url) => url.as_str(),
        }
    }
}

impl std::fmt::Display for GitUrl {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.as_str())
    }
}

impl AsRef<std::ffi::OsStr> for GitUrl {
    fn as_ref(&self) -> &std::ffi::OsStr {
        self.as_str().as_ref()
    }
}

impl std::str::FromStr for GitUrl {
    type Err = GitUrlError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if input.is_empty() {
            return Err(GitUrlError::Empty);
        }

        // Try SCP-style SSH first (not a valid URL, so must be checked before URL parsing)
        if let Some(url) = SshUrl::from_scp(input) {
            return Ok(Self::Ssh(url));
        }

        // Try parsing as a URL
        if let Ok(parsed) = url::Url::parse(input) {
            match parsed.scheme() {
                "https" => return Ok(Self::Https(HttpsUrl::from_parsed(input, parsed)?)),
                "ssh" => return Ok(Self::Ssh(SshUrl::from_parsed(input, parsed)?)),
                "git" => return Ok(Self::Git(GitProtocolUrl::from_parsed(input, parsed)?)),
                "file" => return Ok(Self::Path(PathUrl::from_parsed(input, parsed)?)),
                _ => {}
            }
        }

        // Try absolute path
        if let Ok(url) = input.parse::<PathUrl>() {
            return Ok(Self::Path(url));
        }

        Err(GitUrlError::InvalidFormat)
    }
}

/// A git remote reference: either a named remote or a URL.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Remote {
    /// A named remote (e.g., `origin`, `upstream`).
    Name(RemoteName),
    /// A repository URL.
    Url(GitUrl),
}

impl Remote {
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Name(name) => name.as_str(),
            Self::Url(url) => url.as_str(),
        }
    }
}

impl std::fmt::Display for Remote {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.as_str())
    }
}

impl AsRef<std::ffi::OsStr> for Remote {
    fn as_ref(&self) -> &std::ffi::OsStr {
        self.as_str().as_ref()
    }
}

impl std::str::FromStr for Remote {
    type Err = RemoteError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if input.is_empty() {
            return Err(RemoteError::Empty);
        }

        // Try parsing as URL first
        if let Ok(url) = input.parse::<GitUrl>() {
            return Ok(Self::Url(url));
        }

        // Fall back to remote name
        input.parse::<RemoteName>().map(Self::Name)
    }
}

impl From<RemoteName> for Remote {
    fn from(name: RemoteName) -> Self {
        Self::Name(name)
    }
}

impl From<GitUrl> for Remote {
    fn from(url: GitUrl) -> Self {
        Self::Url(url)
    }
}

/// A named git remote (e.g., `origin`, `upstream`).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RemoteName(Cow<'static, str>);

impl RemoteName {
    const fn validate(input: &str) -> Result<(), RemoteError> {
        if input.is_empty() {
            return Err(RemoteError::Empty);
        }

        let bytes = input.as_bytes();
        let mut index = 0;
        // Using index loop because iterators are not const-compatible.
        while index < bytes.len() {
            if bytes[index].is_ascii_whitespace() {
                return Err(RemoteError::InvalidRemoteName);
            }
            index += 1;
        }

        Ok(())
    }

    /// Create a `RemoteName` from a static string, panicking if invalid.
    ///
    /// Use this for known-valid static remote names like `"origin"`.
    ///
    /// # Panics
    ///
    /// Panics if the input is empty or contains whitespace.
    #[must_use]
    pub const fn from_static_or_panic(input: &'static str) -> Self {
        assert!(Self::validate(input).is_ok(), "invalid remote name");
        Self(Cow::Borrowed(input))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for RemoteName {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

impl AsRef<std::ffi::OsStr> for RemoteName {
    fn as_ref(&self) -> &std::ffi::OsStr {
        self.as_str().as_ref()
    }
}

impl std::str::FromStr for RemoteName {
    type Err = RemoteError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Self::validate(input)?;
        Ok(Self(Cow::Owned(input.to_string())))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RemoteError {
    #[error("Remote cannot be empty")]
    Empty,
    #[error("Invalid remote name")]
    InvalidRemoteName,
}

/// SSH URL: `git@host:path` (SCP-style) or `ssh://user@host/path`
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SshUrl {
    raw: String,
    user: String,
    host: String,
    path: String,
}

impl SshUrl {
    fn from_parsed(raw: &str, parsed: url::Url) -> Result<Self, GitUrlError> {
        let user = parsed.username();
        let host = parsed.host_str().ok_or(GitUrlError::InvalidSshUrl)?;
        let path = parsed.path();

        if user.is_empty() || host.is_empty() {
            return Err(GitUrlError::InvalidSshUrl);
        }

        Ok(Self {
            raw: raw.to_string(),
            user: user.to_string(),
            host: host.to_string(),
            path: path.to_string(),
        })
    }

    /// Parse SCP-style SSH URL: `user@host:path`
    ///
    /// Path must not start with `/` to distinguish from URL-style.
    fn from_scp(input: &str) -> Option<Self> {
        let (user_host, path) = input.split_once(':')?;
        let (user, host) = user_host.split_once('@')?;

        if path.starts_with('/') || path.starts_with("//") {
            return None;
        }

        if user.is_empty() || host.is_empty() || path.is_empty() {
            return None;
        }

        Some(Self {
            raw: input.to_string(),
            user: user.to_string(),
            host: host.to_string(),
            path: path.to_string(),
        })
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.raw
    }

    #[must_use]
    pub fn user(&self) -> &str {
        &self.user
    }

    #[must_use]
    pub fn host(&self) -> &str {
        &self.host
    }

    #[must_use]
    pub fn path(&self) -> &str {
        &self.path
    }
}

/// HTTPS URL: `https://host/path`
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HttpsUrl {
    raw: String,
    host: String,
    path: String,
}

impl HttpsUrl {
    fn from_parsed(raw: &str, parsed: url::Url) -> Result<Self, GitUrlError> {
        let host = parsed.host_str().ok_or(GitUrlError::InvalidHttpsUrl)?;

        if host.is_empty() {
            return Err(GitUrlError::InvalidHttpsUrl);
        }

        Ok(Self {
            raw: raw.to_string(),
            host: host.to_string(),
            path: parsed.path().to_string(),
        })
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.raw
    }

    #[must_use]
    pub fn host(&self) -> &str {
        &self.host
    }

    #[must_use]
    pub fn path(&self) -> &str {
        &self.path
    }
}

/// Git protocol URL: `git://host/path`
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GitProtocolUrl {
    raw: String,
    host: String,
    path: String,
}

impl GitProtocolUrl {
    fn from_parsed(raw: &str, parsed: url::Url) -> Result<Self, GitUrlError> {
        let host = parsed
            .host_str()
            .ok_or(GitUrlError::InvalidGitProtocolUrl)?;

        if host.is_empty() {
            return Err(GitUrlError::InvalidGitProtocolUrl);
        }

        Ok(Self {
            raw: raw.to_string(),
            host: host.to_string(),
            path: parsed.path().to_string(),
        })
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.raw
    }

    #[must_use]
    pub fn host(&self) -> &str {
        &self.host
    }

    #[must_use]
    pub fn path(&self) -> &str {
        &self.path
    }
}

/// Local file path URL: `/path/to/repo` or `file:///path/to/repo`
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PathUrl {
    raw: String,
    path: PathBuf,
}

impl PathUrl {
    fn from_parsed(raw: &str, parsed: url::Url) -> Result<Self, GitUrlError> {
        let path = parsed
            .to_file_path()
            .map_err(|()| GitUrlError::InvalidPathUrl)?;

        Ok(Self {
            raw: raw.to_string(),
            path,
        })
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.raw
    }

    #[must_use]
    pub fn path(&self) -> &Path {
        &self.path
    }
}

impl std::str::FromStr for PathUrl {
    type Err = GitUrlError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let path = PathBuf::from(input);

        if path.is_absolute() {
            return Ok(Self {
                raw: input.to_string(),
                path,
            });
        }

        Err(GitUrlError::InvalidPathUrl)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum GitUrlError {
    #[error("Git URL cannot be empty")]
    Empty,
    #[error("Invalid git URL format")]
    InvalidFormat,
    #[error("Invalid SSH URL format (expected user@host:path or ssh://user@host/path)")]
    InvalidSshUrl,
    #[error("Invalid HTTPS URL format (expected https://host/path)")]
    InvalidHttpsUrl,
    #[error("Invalid git protocol URL format (expected git://host/path)")]
    InvalidGitProtocolUrl,
    #[error("Invalid path URL format (expected absolute path or file:// URL)")]
    InvalidPathUrl,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ssh_scp_style() {
        let url: GitUrl = "git@github.com:user/repo.git".parse().unwrap();
        assert!(matches!(url, GitUrl::Ssh(_)));
        if let GitUrl::Ssh(ssh) = url {
            assert_eq!(ssh.user(), "git");
            assert_eq!(ssh.host(), "github.com");
            assert_eq!(ssh.path(), "user/repo.git");
        }
    }

    #[test]
    fn test_ssh_url_style() {
        let url: GitUrl = "ssh://git@github.com/user/repo.git".parse().unwrap();
        assert!(matches!(url, GitUrl::Ssh(_)));
        if let GitUrl::Ssh(ssh) = url {
            assert_eq!(ssh.user(), "git");
            assert_eq!(ssh.host(), "github.com");
            assert_eq!(ssh.path(), "/user/repo.git");
        }
    }

    #[test]
    fn test_https() {
        let url: GitUrl = "https://github.com/user/repo.git".parse().unwrap();
        assert!(matches!(url, GitUrl::Https(_)));
        if let GitUrl::Https(https) = url {
            assert_eq!(https.host(), "github.com");
            assert_eq!(https.path(), "/user/repo.git");
        }
    }

    #[test]
    fn test_git_protocol() {
        let url: GitUrl = "git://github.com/user/repo.git".parse().unwrap();
        assert!(matches!(url, GitUrl::Git(_)));
        if let GitUrl::Git(git) = url {
            assert_eq!(git.host(), "github.com");
            assert_eq!(git.path(), "/user/repo.git");
        }
    }

    #[test]
    fn test_file_url() {
        let url: GitUrl = "file:///home/user/repo".parse().unwrap();
        assert!(matches!(url, GitUrl::Path(_)));
        if let GitUrl::Path(path) = url {
            assert_eq!(path.path(), Path::new("/home/user/repo"));
        }
    }

    #[test]
    fn test_absolute_path() {
        let url: GitUrl = "/home/user/repo".parse().unwrap();
        assert!(matches!(url, GitUrl::Path(_)));
        if let GitUrl::Path(path) = url {
            assert_eq!(path.path(), Path::new("/home/user/repo"));
        }
    }

    #[test]
    fn test_empty() {
        assert!(matches!("".parse::<GitUrl>(), Err(GitUrlError::Empty)));
    }

    #[test]
    fn test_invalid() {
        assert!(matches!(
            "not-a-valid-url".parse::<GitUrl>(),
            Err(GitUrlError::InvalidFormat)
        ));
    }

    #[test]
    fn test_display() {
        let url: GitUrl = "git@github.com:user/repo.git".parse().unwrap();
        assert_eq!(url.to_string(), "git@github.com:user/repo.git");
        assert_eq!(url.as_str(), "git@github.com:user/repo.git");
    }

    #[test]
    fn test_as_ref_os_str() {
        let url: GitUrl = "git@github.com:user/repo.git".parse().unwrap();
        let os_str: &std::ffi::OsStr = url.as_ref();
        assert_eq!(os_str, "git@github.com:user/repo.git");
    }

    #[test]
    fn test_scp_empty_user() {
        assert!(matches!(
            "@github.com:path".parse::<GitUrl>(),
            Err(GitUrlError::InvalidFormat)
        ));
    }

    #[test]
    fn test_scp_empty_host() {
        assert!(matches!(
            "git@:path".parse::<GitUrl>(),
            Err(GitUrlError::InvalidFormat)
        ));
    }

    #[test]
    fn test_scp_empty_path() {
        assert!(matches!(
            "git@github.com:".parse::<GitUrl>(),
            Err(GitUrlError::InvalidFormat)
        ));
    }

    #[test]
    fn test_scp_path_with_leading_slash_rejected() {
        // git@host:/path is not valid SCP (path starts with /) and not a valid URL
        assert!(matches!(
            "git@github.com:/user/repo".parse::<GitUrl>(),
            Err(GitUrlError::InvalidFormat)
        ));
    }

    #[test]
    fn test_ssh_url_missing_user() {
        assert!(matches!(
            "ssh://github.com/user/repo.git".parse::<GitUrl>(),
            Err(GitUrlError::InvalidSshUrl)
        ));
    }

    #[test]
    fn test_relative_path() {
        assert!(matches!(
            "./relative/path".parse::<GitUrl>(),
            Err(GitUrlError::InvalidFormat)
        ));
    }

    #[test]
    fn test_unknown_scheme() {
        assert!(matches!(
            "ftp://example.com/repo".parse::<GitUrl>(),
            Err(GitUrlError::InvalidFormat)
        ));
    }

    #[test]
    fn test_remote_name() {
        let remote: Remote = "origin".parse().unwrap();
        assert!(matches!(remote, Remote::Name(_)));
        assert_eq!(remote.as_str(), "origin");
    }

    #[test]
    fn test_remote_url() {
        let remote: Remote = "git@github.com:user/repo.git".parse().unwrap();
        assert!(matches!(remote, Remote::Url(_)));
        assert_eq!(remote.as_str(), "git@github.com:user/repo.git");
    }

    #[test]
    fn test_remote_https_url() {
        let remote: Remote = "https://github.com/user/repo.git".parse().unwrap();
        assert!(matches!(remote, Remote::Url(_)));
    }

    #[test]
    fn test_remote_empty() {
        assert!(matches!("".parse::<Remote>(), Err(RemoteError::Empty)));
    }

    #[test]
    fn test_remote_name_with_whitespace() {
        assert!(matches!(
            "origin upstream".parse::<Remote>(),
            Err(RemoteError::InvalidRemoteName)
        ));
    }

    #[test]
    fn test_remote_name_display() {
        let name: RemoteName = "origin".parse().unwrap();
        assert_eq!(name.to_string(), "origin");
    }

    #[test]
    fn test_remote_from_remote_name() {
        let name: RemoteName = "upstream".parse().unwrap();
        let remote: Remote = name.into();
        assert!(matches!(remote, Remote::Name(_)));
    }

    #[test]
    fn test_remote_from_git_url() {
        let url: GitUrl = "git@github.com:user/repo.git".parse().unwrap();
        let remote: Remote = url.into();
        assert!(matches!(remote, Remote::Url(_)));
    }
}
