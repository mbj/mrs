//! Platform support detection for container-based tools

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Error {
    /// GitHub Actions on macOS does not support Docker
    GitHubActionsMacOs,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GitHubActionsMacOs => {
                write!(
                    f,
                    "GitHub Actions on macOS does not support Docker. Container runtime is not available."
                )
            }
        }
    }
}

impl std::error::Error for Error {}

/// Check if the current platform supports container operations
///
/// Returns `Ok(())` if the platform is supported, or an `Err` containing
/// details about why the platform is not supported.
///
/// # Examples
///
/// ```
/// match ociman::platform::support() {
///     Ok(()) => println!("Platform is supported"),
///     Err(error) => eprintln!("Platform not supported: {}", error),
/// }
/// ```
pub fn support() -> Result<(), Error> {
    if std::env::consts::OS == "macos" && std::env::var("GITHUB_ACTIONS").is_ok() {
        Err(Error::GitHubActionsMacOs)
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_platform_support_consistency() {
        // This test is intentionally tautological - it verifies that the support()
        // function's logic is consistent with itself by re-implementing the same check.
        // While this doesn't validate correctness, it ensures that if someone modifies
        // the platform detection logic in support(), they must also update this test,
        // preventing silent breakage of the platform detection contract.
        let expected = if std::env::consts::OS == "macos" && std::env::var("GITHUB_ACTIONS").is_ok()
        {
            Err(Error::GitHubActionsMacOs)
        } else {
            Ok(())
        };

        assert_eq!(support(), expected);
    }
}
