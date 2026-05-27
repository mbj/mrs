mod common;

use common::{TestDir, run_pg_ephemeral};

/// `bin` runs a tool from the image against the bind-mounted host cwd, with no
/// PostgreSQL booted. Writing a marker file into the working directory and
/// `cat`-ing it back through the container proves the cwd is bind-mounted at
/// the same path and the tool executes there as the entrypoint.
#[tokio::test]
async fn test_bin_runs_image_tool_in_bind_mounted_cwd() {
    let _backend = ociman::test_backend_setup!();

    let dir = TestDir::new("bin-test");
    dir.write_file("marker.txt", "bin-marker\n");

    let stdout = run_pg_ephemeral(&["bin", "--", "cat", "marker.txt"], &dir.path).await;

    assert_eq!(stdout, "bin-marker\n");
}

/// `bin` runs the *image's* tool, not the host's. Pinning a specific postgres
/// major via `--image` and capturing `pg_dump --version` proves provenance:
/// the reported major can only match the pinned image if the binary ran from
/// inside that container.
///
/// Pins the same major as `common::POSTGRES_IMAGE` (pre-pulled by `base.rs`)
/// to reuse an image the suite already standardizes on rather than fetching a
/// new tag.
#[tokio::test]
async fn test_bin_uses_pinned_image_tool_version() {
    let _backend = ociman::test_backend_setup!();

    let dir = TestDir::new("bin-version-test");

    let stdout = run_pg_ephemeral(
        &["--image", "17", "bin", "--", "pg_dump", "--version"],
        &dir.path,
    )
    .await;

    // Output is "pg_dump (PostgreSQL) 17.<patch> (Debian ...)\n"; the patch and
    // Debian suffix float with image rebuilds, so assert on the pinned major.
    let major = stdout
        .strip_prefix("pg_dump (PostgreSQL) ")
        .unwrap()
        .split('.')
        .next()
        .unwrap();

    assert_eq!(major, "17");
}

/// `bin` forwards the host `PG*` environment into the container. Proven
/// without a database: set `PGSSLMODE` to a bogus value, which libpq validates
/// eagerly (before connecting) and echoes verbatim as
/// `invalid sslmode value: "<marker>"`. The marker can only reach the
/// container's psql if the variable crossed the host→container boundary, so
/// finding it in stderr proves forwarding.
#[tokio::test]
async fn test_bin_forwards_pg_env() {
    let _backend = ociman::test_backend_setup!();

    let dir = TestDir::new("bin-pgenv-test");
    let pg_ephemeral_bin = env!("CARGO_BIN_EXE_pg-ephemeral");

    const MARKER: &str = "pg-ephemeral-pgenv-forward-proof";

    let output = cmd_proc::Command::new(pg_ephemeral_bin)
        .arguments(["--image", "17", "bin", "--", "psql", "-c", "SELECT 1"])
        .working_directory(&dir.path)
        .env(
            &cmd_proc::EnvVariableName::from_static_or_panic("PGSSLMODE"),
            cmd_proc::EnvVariableValue::from_static_or_panic(MARKER),
        )
        .stdout_capture()
        .stderr_capture()
        .accept_nonzero_exit()
        .run()
        .await
        .unwrap();

    let stderr = String::from_utf8(output.stderr).unwrap();

    assert!(
        stderr.contains(MARKER),
        "expected forwarded PGSSLMODE={MARKER:?} to surface in the libpq error; stderr:\n{stderr}"
    );
}
