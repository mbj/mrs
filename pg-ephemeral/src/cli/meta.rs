//! `pg-ephemeral meta <subcommand>` — diagnostics about this binary itself.
//!
//! The `meta` namespace hosts subcommands that interrogate or test the
//! production binary: `meta test` (full integration suite), `meta smoke`
//! (curated fast subset for downstream-user diagnosis), later potentially
//! `meta info`, `meta diagnose`, etc. The `meta` name keeps the
//! user-facing `test` verb free for things like SQL / schema testing
//! later.

use libtest_mimic::{Arguments, Trial};

#[derive(Clone, Debug, clap::Parser)]
pub enum Command {
    /// Run the full integration test suite hosted by this binary.
    ///
    /// Each trial runs inside the production binary's process — main(),
    /// CLI parsing, and any startup hooks are exercised, unlike a normal
    /// cargo-test library build. All arguments after `test` are forwarded
    /// to libtest-mimic (e.g. `--list`, `--exact <name>`,
    /// `--filter <pattern>`, `--nocapture`).
    ///
    /// Heavy — includes language integration (Ruby/Prisma image pulls,
    /// container builds) and all example-boot checks. For a fast
    /// downstream-diagnostic subset, use `meta smoke`.
    Test {
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },
    /// Run the curated smoke subset of the integration suite.
    ///
    /// Verifies pg-ephemeral works on this host: binary integrity,
    /// example schema parses, basic container lifecycle, label
    /// round-trip, cache populate. Avoids the multi-hundred-MB image
    /// pulls and minutes-long boot sequences in the full `meta test`.
    /// All arguments after `smoke` are forwarded to libtest-mimic.
    Smoke {
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },
}

impl Command {
    /// Execute the meta subcommand. Each arm runs libtest-mimic and
    /// terminates the process with libtest's conventional exit code (0
    /// on pass, 101 on fail) — it does not return.
    pub fn run(&self) {
        match self {
            Self::Test { args } => run_trials("test", args, crate::meta::test::trials()),
            Self::Smoke { args } => run_trials("smoke", args, crate::meta::test::smoke_trials()),
        }
    }
}

fn run_trials(subcommand: &str, args: &[String], trials: Vec<Trial>) {
    let arg0 = format!("pg-ephemeral meta {subcommand}");
    let parse_args = std::iter::once(arg0).chain(args.iter().cloned());
    let arguments = Arguments::from_iter(parse_args);
    libtest_mimic::run(&arguments, trials).exit();
}
