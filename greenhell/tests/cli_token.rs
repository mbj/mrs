//! Integration tests for `greenhell cli-token` command.
//!
//! These tests spawn the actual binary with controlled environment
//! to verify token discovery behavior.

const MOCK_GH_TOKEN: &str = "ghp_mockTokenFromGhAuth1234567890abcdef";
const ENV_GH_TOKEN: &str = "ghp_fromGhTokenEnv12345678901234567890";
const ENV_GITHUB_TOKEN: &str = "ghp_fromGithubTokenEnv123456789012345";

const GH_TOKEN: cmd_proc::EnvVariableName<'static> =
    cmd_proc::EnvVariableName::from_static_or_panic("GH_TOKEN");
const GITHUB_TOKEN: cmd_proc::EnvVariableName<'static> =
    cmd_proc::EnvVariableName::from_static_or_panic("GITHUB_TOKEN");
const PATH: cmd_proc::EnvVariableName<'static> =
    cmd_proc::EnvVariableName::from_static_or_panic("PATH");

fn binary_path() -> std::path::PathBuf {
    env!("CARGO_BIN_EXE_greenhell").into()
}

fn mock_gh_path() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("mock-gh")
}

/// Creates a base command with clean environment for testing.
fn base_command() -> cmd_proc::Command {
    let original_path = std::env::var("PATH").unwrap_or_default();
    let path_with_mock = format!("{}:{}", mock_gh_path().display(), original_path);

    cmd_proc::Command::new(binary_path())
        .argument("cli-token")
        .env_remove(&GH_TOKEN)
        .env_remove(&GITHUB_TOKEN)
        .env(&PATH, path_with_mock)
}

#[tokio::test]
async fn gh_token_env_takes_precedence() {
    let output = base_command()
        .env(&GH_TOKEN, ENV_GH_TOKEN)
        .env(&GITHUB_TOKEN, ENV_GITHUB_TOKEN)
        .stdout_capture()
        .stderr_capture()
        .accept_nonzero_exit()
        .run()
        .await
        .expect("failed to execute");

    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout).trim(), ENV_GH_TOKEN);
}

#[tokio::test]
async fn github_token_env_fallback() {
    let output = base_command()
        .env(&GITHUB_TOKEN, ENV_GITHUB_TOKEN)
        .stdout_capture()
        .stderr_capture()
        .accept_nonzero_exit()
        .run()
        .await
        .expect("failed to execute");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stdout).trim(),
        ENV_GITHUB_TOKEN
    );
}

#[tokio::test]
async fn gh_auth_token_fallback() {
    let output = base_command()
        .stdout_capture()
        .stderr_capture()
        .accept_nonzero_exit()
        .run()
        .await
        .expect("failed to execute");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stdout).trim(),
        MOCK_GH_TOKEN
    );
}

#[tokio::test]
async fn fails_when_no_token_source() {
    let output = cmd_proc::Command::new(binary_path())
        .argument("cli-token")
        .env_remove(&GH_TOKEN)
        .env_remove(&GITHUB_TOKEN)
        .env(&PATH, "/nonexistent")
        .stdout_capture()
        .stderr_capture()
        .accept_nonzero_exit()
        .run()
        .await
        .expect("failed to execute");

    assert!(!output.status.success());
}
