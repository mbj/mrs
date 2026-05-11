//! Cargo/nextest entry point for `pg-ephemeral`'s `meta test` suite.
//!
//! Cargo's test discovery requires a `[[test]]` target; this wrapper is the
//! smallest possible one. Its only job is to translate cargo's
//! `<test-binary> [libtest-args]` invocation shape into
//! `pg-ephemeral meta test [libtest-args]`, so each test runs inside the
//! actual production binary's process — exercising main, CLI parsing, and
//! any startup hooks that a normal cargo-test library build would not.
//!
//! Intentionally references no `pg_ephemeral::*` symbols so the wrapper
//! itself links nothing from the library; only `std` is pulled in.

fn main() {
    let bin = env!("CARGO_BIN_EXE_pg-ephemeral");
    let mut command = std::process::Command::new(bin);
    command.arg("meta").arg("test");
    command.args(std::env::args().skip(1));
    let status = command.status().unwrap();
    std::process::exit(status.code().unwrap_or(1));
}
