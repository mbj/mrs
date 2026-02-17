use std::borrow::Cow;
use std::path::{Path, PathBuf};

/// A validated git repository address.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Address {
    /// SSH address: `git@host:path` (SCP-style) or `ssh://user@host/path`
    Ssh(SshAddress),
    /// HTTPS URL: `https://host/path`
    Https(HttpsUrl),
    /// Git protocol URL: `git://host/path`
    Git(GitUrl),
    /// Local file path: `/path/to/repo` or `file:///path/to/repo`
    Path(PathAddress),
}

impl Address {
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Ssh(address) => address.as_str(),
            Self::Https(address) => address.as_str(),
            Self::Git(address) => address.as_str(),
            Self::Path(address) => address.as_str(),
        }
    }
}

impl std::fmt::Display for Address {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.as_str())
    }
}

impl AsRef<std::ffi::OsStr> for Address {
    fn as_ref(&self) -> &std::ffi::OsStr {
        self.as_str().as_ref()
    }
}

impl std::str::FromStr for Address {
    type Err = AddressError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if input.is_empty() {
            return Err(AddressError::Empty);
        }

        // Try SCP-style SSH first (not a valid URL, so must be checked before URL parsing)
        if let Some(address) = SshAddress::from_scp(input) {
            return Ok(Self::Ssh(address));
        }

        // Try parsing as a URL
        if let Ok(parsed) = url::Url::parse(input) {
            match parsed.scheme() {
                "https" => return Ok(Self::Https(HttpsUrl::from_parsed(input, parsed)?)),
                "ssh" => return Ok(Self::Ssh(SshAddress::from_parsed(input, parsed)?)),
                "git" => return Ok(Self::Git(GitUrl::from_parsed(input, parsed)?)),
                "file" => return Ok(Self::Path(PathAddress::from_parsed(input, parsed)?)),
                _ => {}
            }
        }

        // Try absolute path
        if let Ok(address) = input.parse::<PathAddress>() {
            return Ok(Self::Path(address));
        }

        Err(AddressError::InvalidFormat)
    }
}

/// A git remote reference: either a named remote or an address.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Remote {
    /// A named remote (e.g., `origin`, `upstream`).
    Name(RemoteName),
    /// A repository address.
    RepositoryAddress(Address),
}

impl Remote {
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Name(name) => name.as_str(),
            Self::RepositoryAddress(address) => address.as_str(),
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

        // Try parsing as address first
        if let Ok(address) = input.parse::<Address>() {
            return Ok(Self::RepositoryAddress(address));
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

impl From<Address> for Remote {
    fn from(address: Address) -> Self {
        Self::RepositoryAddress(address)
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

/// SSH address: `git@host:path` (SCP-style) or `ssh://user@host/path`
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SshAddress {
    raw: String,
    user: String,
    host: String,
    path: String,
}

impl SshAddress {
    fn from_parsed(raw: &str, parsed: url::Url) -> Result<Self, AddressError> {
        let user = parsed.username();
        let host = parsed.host_str().ok_or(AddressError::InvalidSshAddress)?;
        let path = parsed.path();

        if user.is_empty() || host.is_empty() {
            return Err(AddressError::InvalidSshAddress);
        }

        Ok(Self {
            raw: raw.to_string(),
            user: user.to_string(),
            host: host.to_string(),
            path: path.to_string(),
        })
    }

    /// Parse SCP-style SSH address: `user@host:path`
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
    fn from_parsed(raw: &str, parsed: url::Url) -> Result<Self, AddressError> {
        let host = parsed.host_str().ok_or(AddressError::InvalidHttpsUrl)?;

        if host.is_empty() {
            return Err(AddressError::InvalidHttpsUrl);
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
pub struct GitUrl {
    raw: String,
    host: String,
    path: String,
}

impl GitUrl {
    fn from_parsed(raw: &str, parsed: url::Url) -> Result<Self, AddressError> {
        let host = parsed.host_str().ok_or(AddressError::InvalidGitUrl)?;

        if host.is_empty() {
            return Err(AddressError::InvalidGitUrl);
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

/// Local file path address: `/path/to/repo` or `file:///path/to/repo`
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PathAddress {
    raw: String,
    path: PathBuf,
}

impl PathAddress {
    fn from_parsed(raw: &str, parsed: url::Url) -> Result<Self, AddressError> {
        let path = parsed
            .to_file_path()
            .map_err(|()| AddressError::InvalidPathAddress)?;

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

impl std::str::FromStr for PathAddress {
    type Err = AddressError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let path = PathBuf::from(input);

        if path.is_absolute() {
            return Ok(Self {
                raw: input.to_string(),
                path,
            });
        }

        Err(AddressError::InvalidPathAddress)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum AddressError {
    #[error("Repository address cannot be empty")]
    Empty,
    #[error("Invalid repository address format")]
    InvalidFormat,
    #[error("Invalid SSH address format (expected user@host:path or ssh://user@host/path)")]
    InvalidSshAddress,
    #[error("Invalid HTTPS URL format (expected https://host/path)")]
    InvalidHttpsUrl,
    #[error("Invalid git:// URL format (expected git://host/path)")]
    InvalidGitUrl,
    #[error("Invalid path address format (expected absolute path or file:// URL)")]
    InvalidPathAddress,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ssh_scp_style() {
        let address: Address = "git@github.com:user/repo.git".parse().unwrap();
        assert!(matches!(address, Address::Ssh(_)));
        if let Address::Ssh(ssh) = address {
            assert_eq!(ssh.user(), "git");
            assert_eq!(ssh.host(), "github.com");
            assert_eq!(ssh.path(), "user/repo.git");
        }
    }

    #[test]
    fn test_ssh_url_style() {
        let address: Address = "ssh://git@github.com/user/repo.git".parse().unwrap();
        assert!(matches!(address, Address::Ssh(_)));
        if let Address::Ssh(ssh) = address {
            assert_eq!(ssh.user(), "git");
            assert_eq!(ssh.host(), "github.com");
            assert_eq!(ssh.path(), "/user/repo.git");
        }
    }

    #[test]
    fn test_https() {
        let address: Address = "https://github.com/user/repo.git".parse().unwrap();
        assert!(matches!(address, Address::Https(_)));
        if let Address::Https(https) = address {
            assert_eq!(https.host(), "github.com");
            assert_eq!(https.path(), "/user/repo.git");
        }
    }

    #[test]
    fn test_git_protocol() {
        let address: Address = "git://github.com/user/repo.git".parse().unwrap();
        assert!(matches!(address, Address::Git(_)));
        if let Address::Git(git) = address {
            assert_eq!(git.host(), "github.com");
            assert_eq!(git.path(), "/user/repo.git");
        }
    }

    #[test]
    fn test_file_url() {
        let address: Address = "file:///home/user/repo".parse().unwrap();
        assert!(matches!(address, Address::Path(_)));
        if let Address::Path(path) = address {
            assert_eq!(path.path(), Path::new("/home/user/repo"));
        }
    }

    #[test]
    fn test_absolute_path() {
        let address: Address = "/home/user/repo".parse().unwrap();
        assert!(matches!(address, Address::Path(_)));
        if let Address::Path(path) = address {
            assert_eq!(path.path(), Path::new("/home/user/repo"));
        }
    }

    #[test]
    fn test_empty() {
        assert!(matches!("".parse::<Address>(), Err(AddressError::Empty)));
    }

    #[test]
    fn test_invalid() {
        assert!(matches!(
            "not-a-valid-url".parse::<Address>(),
            Err(AddressError::InvalidFormat)
        ));
    }

    #[test]
    fn test_display() {
        let address: Address = "git@github.com:user/repo.git".parse().unwrap();
        assert_eq!(address.to_string(), "git@github.com:user/repo.git");
        assert_eq!(address.as_str(), "git@github.com:user/repo.git");
    }

    #[test]
    fn test_as_ref_os_str() {
        let address: Address = "git@github.com:user/repo.git".parse().unwrap();
        let os_str: &std::ffi::OsStr = address.as_ref();
        assert_eq!(os_str, "git@github.com:user/repo.git");
    }

    #[test]
    fn test_scp_empty_user() {
        assert!(matches!(
            "@github.com:path".parse::<Address>(),
            Err(AddressError::InvalidFormat)
        ));
    }

    #[test]
    fn test_scp_empty_host() {
        assert!(matches!(
            "git@:path".parse::<Address>(),
            Err(AddressError::InvalidFormat)
        ));
    }

    #[test]
    fn test_scp_empty_path() {
        assert!(matches!(
            "git@github.com:".parse::<Address>(),
            Err(AddressError::InvalidFormat)
        ));
    }

    #[test]
    fn test_scp_path_with_leading_slash_rejected() {
        // git@host:/path is not valid SCP (path starts with /) and not a valid URL
        assert!(matches!(
            "git@github.com:/user/repo".parse::<Address>(),
            Err(AddressError::InvalidFormat)
        ));
    }

    #[test]
    fn test_ssh_url_missing_user() {
        assert!(matches!(
            "ssh://github.com/user/repo.git".parse::<Address>(),
            Err(AddressError::InvalidSshAddress)
        ));
    }

    #[test]
    fn test_relative_path() {
        assert!(matches!(
            "./relative/path".parse::<Address>(),
            Err(AddressError::InvalidFormat)
        ));
    }

    #[test]
    fn test_unknown_scheme() {
        assert!(matches!(
            "ftp://example.com/repo".parse::<Address>(),
            Err(AddressError::InvalidFormat)
        ));
    }

    #[test]
    fn test_remote_name() {
        let remote: Remote = "origin".parse().unwrap();
        assert!(matches!(remote, Remote::Name(_)));
        assert_eq!(remote.as_str(), "origin");
    }

    #[test]
    fn test_remote_repository_address() {
        let remote: Remote = "git@github.com:user/repo.git".parse().unwrap();
        assert!(matches!(remote, Remote::RepositoryAddress(_)));
        assert_eq!(remote.as_str(), "git@github.com:user/repo.git");
    }

    #[test]
    fn test_remote_https_url() {
        let remote: Remote = "https://github.com/user/repo.git".parse().unwrap();
        assert!(matches!(remote, Remote::RepositoryAddress(_)));
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
    fn test_remote_from_address() {
        let address: Address = "git@github.com:user/repo.git".parse().unwrap();
        let remote: Remote = address.into();
        assert!(matches!(remote, Remote::RepositoryAddress(_)));
    }
}
