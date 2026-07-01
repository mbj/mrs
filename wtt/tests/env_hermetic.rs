//! Regression test: `wtt`'s git invocations must not be hijacked by an ambient
//! git environment (the `GIT_DIR` / `GIT_INDEX_FILE` a `git rebase` or hook
//! exports). Before `git_proc` cleared `GIT_*`, running `wtt` from inside a
//! rebase made its `git` calls operate on the surrounding repository instead of
//! the one `wtt` targeted — corrupting it.
//!
//! Following the established pattern (see `greenhell/tests/cli_token.rs`): spawn
//! the real `wtt` binary and set the poisoned environment on the *child* via
//! `cmd_proc::Command::env` — never a process-global `std::env::set_var`.

const GIT_DIR: cmd_proc::EnvVariableName =
    cmd_proc::EnvVariableName::from_static_or_panic("GIT_DIR");
const GIT_INDEX_FILE: cmd_proc::EnvVariableName =
    cmd_proc::EnvVariableName::from_static_or_panic("GIT_INDEX_FILE");

#[tokio::test]
async fn list_ignores_ambient_git_environment() {
    // Isolated wtt layout in a temp directory.
    let root = std::env::temp_dir().join(format!("wtt-env-hermetic-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    let bare_clone_dir = root.join("bare");
    let worktree_dir = root.join("worktrees");
    std::fs::create_dir_all(&bare_clone_dir).unwrap();
    std::fs::create_dir_all(&worktree_dir).unwrap();

    // A bare repo for `wtt list` to inspect: <bare_clone_dir>/testrepo.git
    let bare_repo = bare_clone_dir.join("testrepo.git");
    git_proc::init::new()
        .bare()
        .directory(&bare_repo)
        .status()
        .await
        .unwrap();

    let config_path = root.join("wtt.toml");
    std::fs::write(
        &config_path,
        format!("bare_clone_dir = {bare_clone_dir:?}\nworktree_dir = {worktree_dir:?}\n"),
    )
    .unwrap();

    // Spawn the real binary with an ambient git context pinning a *different*
    // repo and a bogus index — set on the child only. `wtt list` runs
    // `git -C <bare_repo> worktree list`; without the `GIT_*` scrub, `GIT_DIR`
    // redirects git to /nonexistent and the command fails.
    let output = cmd_proc::Command::new(env!("CARGO_BIN_EXE_wtt"))
        .argument("--config-file")
        .argument(&config_path)
        .argument("list")
        .argument("--repo")
        .argument("testrepo")
        .env(&GIT_DIR, "/nonexistent/.git")
        .env(&GIT_INDEX_FILE, root.join("BOGUS_INDEX"))
        .stdout_capture()
        .stderr_capture()
        .accept_nonzero_exit()
        .run()
        .await
        .expect("failed to spawn wtt");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        output.status.success(),
        "wtt was hijacked by the ambient git environment; stderr: {stderr}"
    );
    assert!(
        stdout.contains("testrepo"),
        "expected the targeted repo in output, got: {stdout:?}"
    );

    let _ = std::fs::remove_dir_all(&root);
}
