//! Verifies that `pg-ephemeral platform test-backtrace` produces a panic
//! whose backtrace contains real file paths and function names — the
//! property that breaks if the release build strips debug info or the
//! panic handler is misconfigured.

use libtest_mimic::{Failed, Trial};

const RUST_BACKTRACE: cmd_proc::EnvVariableName =
    cmd_proc::EnvVariableName::from_static_or_panic("RUST_BACKTRACE");

#[must_use]
pub fn trials() -> Vec<Trial> {
    vec![Trial::test(
        "backtrace_contains_file_paths",
        backtrace_contains_file_paths,
    )]
}

fn backtrace_contains_file_paths() -> Result<(), Failed> {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    runtime.block_on(async {
        let pg_ephemeral_bin = std::env::current_exe().unwrap();

        let output = cmd_proc::Command::new(pg_ephemeral_bin)
            .arguments(["platform", "test-backtrace"])
            .env(&RUST_BACKTRACE, "1")
            .stdout_capture()
            .stderr_capture()
            .accept_nonzero_exit()
            .run()
            .await
            .unwrap();

        assert!(
            !output.status.success(),
            "test-backtrace should exit with non-zero status"
        );

        let stderr = std::str::from_utf8(&output.stderr).unwrap();

        assert!(
            stderr.contains("intentional panic for backtrace testing"),
            "stderr should contain panic message, got:\n{stderr}"
        );
        assert!(
            stderr.contains("inner_function_for_backtrace_test"),
            "backtrace should contain inner_function_for_backtrace_test, got:\n{stderr}"
        );
        assert!(
            stderr.contains("trigger_test_panic"),
            "backtrace should contain trigger_test_panic, got:\n{stderr}"
        );

        let has_file_line_in_backtrace = stderr.lines().any(|line| {
            !line.contains("panicked at") && line.contains("cli") && line.contains("at ")
        });

        assert!(
            has_file_line_in_backtrace,
            "backtrace frames should contain file paths with line numbers, got:\n{stderr}"
        );

        Ok(())
    })
}
