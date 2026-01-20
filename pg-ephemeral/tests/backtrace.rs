const RUST_BACKTRACE: cmd_proc::EnvVariableName<'static> =
    cmd_proc::EnvVariableName::from_static("RUST_BACKTRACE");

#[test]
fn test_backtrace_contains_file_paths() {
    let pg_ephemeral_bin = env!("CARGO_BIN_EXE_pg-ephemeral");

    let output = cmd_proc::Command::new(pg_ephemeral_bin)
        .arguments(["platform", "test-backtrace"])
        .env(&RUST_BACKTRACE, "1")
        .output()
        .expect("failed to execute pg-ephemeral");

    assert!(
        !output.success(),
        "test-backtrace should exit with non-zero status"
    );

    let stderr = String::from_utf8_lossy(&output.stderr);

    // Verify the panic message is present
    assert!(
        stderr.contains("intentional panic for backtrace testing"),
        "stderr should contain panic message, got:\n{stderr}"
    );

    // Verify function names are present in backtrace
    assert!(
        stderr.contains("inner_function_for_backtrace_test"),
        "backtrace should contain inner_function_for_backtrace_test, got:\n{stderr}"
    );
    assert!(
        stderr.contains("trigger_test_panic"),
        "backtrace should contain trigger_test_panic, got:\n{stderr}"
    );

    // Verify file paths with line numbers are present in backtrace frames
    // This checks that debug info is available, not just the panic message location
    // Look for pattern like "at pg-ephemeral/src/cli.rs:123" or similar in stack frames
    let has_file_line_in_backtrace = stderr.lines().any(|line| {
        // Skip the panic message line itself
        !line.contains("panicked at") && line.contains("cli.rs:") && line.contains("at ")
    });

    assert!(
        has_file_line_in_backtrace,
        "backtrace frames should contain file paths with line numbers (e.g., 'at cli.rs:123'), got:\n{stderr}"
    );
}
