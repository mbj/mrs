//! Integration tests for `greenhell cli-token` command.
//!
//! These tests spawn the actual binary with controlled environment
//! to verify token discovery behavior.

use std::process::Command;

const MOCK_GH_TOKEN: &str = "ghp_mock_token_from_gh_auth";
const ENV_GH_TOKEN: &str = "ghp_from_gh_token_env";
const ENV_GITHUB_TOKEN: &str = "ghp_from_github_token_env";

fn binary_path() -> std::path::PathBuf {
    env!("CARGO_BIN_EXE_greenhell").into()
}

fn mock_gh_path() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("mock-gh")
}

/// Creates a command with clean environment for testing.
fn cli_token_command() -> Command {
    let mut command = Command::new(binary_path());
    command.arg("cli-token");

    command.env_remove("GH_TOKEN");
    command.env_remove("GITHUB_TOKEN");

    let original_path = std::env::var("PATH").unwrap_or_default();
    command.env(
        "PATH",
        format!("{}:{}", mock_gh_path().display(), original_path),
    );

    command
}

#[test]
fn gh_token_env_takes_precedence() {
    let output = cli_token_command()
        .env("GH_TOKEN", ENV_GH_TOKEN)
        .env("GITHUB_TOKEN", ENV_GITHUB_TOKEN)
        .output()
        .expect("failed to execute");

    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout).trim(), ENV_GH_TOKEN);
}

#[test]
fn github_token_env_fallback() {
    let output = cli_token_command()
        .env("GITHUB_TOKEN", ENV_GITHUB_TOKEN)
        .output()
        .expect("failed to execute");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stdout).trim(),
        ENV_GITHUB_TOKEN
    );
}

#[test]
fn gh_auth_token_fallback() {
    let output = cli_token_command().output().expect("failed to execute");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stdout).trim(),
        MOCK_GH_TOKEN
    );
}

#[test]
fn fails_when_no_token_source() {
    let mut command = Command::new(binary_path());
    command.arg("cli-token");
    command.env_remove("GH_TOKEN");
    command.env_remove("GITHUB_TOKEN");
    command.env("PATH", "/nonexistent");

    let output = command.output().expect("failed to execute");

    assert!(!output.status.success());
}
