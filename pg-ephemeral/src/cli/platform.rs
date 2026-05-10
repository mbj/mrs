//! `pg-ephemeral platform <subcommand>` — host platform diagnostics.

#[derive(Clone, Debug, clap::Parser)]
pub enum Command {
    /// Check if the current platform is supported
    ///
    /// Exits with status 0 if platform is supported.
    /// Exits with status 1 if platform is not supported.
    Support,
    /// Trigger a panic to test backtrace quality
    ///
    /// Used by integration tests to verify that backtraces
    /// contain file paths and line numbers in release builds.
    TestBacktrace,
}

impl Command {
    pub fn run(&self) {
        match self {
            Self::Support => match ociman::platform::support() {
                Ok(()) => {
                    std::process::exit(0);
                }
                Err(error) => {
                    log::info!("pg-ephemeral is not supported on this platform: {error}");
                    std::process::exit(1);
                }
            },
            Self::TestBacktrace => {
                trigger_test_panic();
            }
        }
    }
}

#[inline(never)]
fn trigger_test_panic() {
    inner_function_for_backtrace_test();
}

#[inline(never)]
fn inner_function_for_backtrace_test() {
    panic!("intentional panic for backtrace testing");
}
