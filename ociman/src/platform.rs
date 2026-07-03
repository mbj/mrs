//! Platform support detection for container-based tools

use crate::backend::Selection;

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

const GITHUB_ACTIONS: cmd_proc::EnvVariableName =
    cmd_proc::EnvVariableName::from_static_or_panic("GITHUB_ACTIONS");

const OCIMAN_BACKEND: cmd_proc::EnvVariableName =
    cmd_proc::EnvVariableName::from_static_or_panic("OCIMAN_BACKEND");

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
    let backend = OCIMAN_BACKEND.read().ok().and_then(|value| {
        if value.as_str() == "apple" {
            Some(Selection::Apple)
        } else {
            None
        }
    });

    support_for(std::env::consts::OS, GITHUB_ACTIONS.is_present(), backend)
}

fn support_for(os: &str, github_actions: bool, backend: Option<Selection>) -> Result<(), Error> {
    if os == "macos" && github_actions && !matches!(backend, Some(Selection::Apple)) {
        Err(Error::GitHubActionsMacOs)
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn github_actions_macos_is_skipped_for_docker() {
        assert_eq!(
            support_for("macos", true, Some(Selection::Docker)),
            Err(Error::GitHubActionsMacOs)
        );
    }

    #[test]
    fn github_actions_macos_is_not_skipped_for_explicit_apple() {
        assert_eq!(support_for("macos", true, Some(Selection::Apple)), Ok(()));
    }

    #[test]
    fn local_macos_is_supported() {
        assert_eq!(support_for("macos", false, None), Ok(()));
    }
}
