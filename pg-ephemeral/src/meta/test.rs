//! Integration test trial registry for `pg-ephemeral meta test` (full
//! suite) and `pg-ephemeral meta smoke` (curated fast subset for
//! downstream-user diagnosis).
//!
//! Trials live as library code (not `tests/*.rs` cargo-test targets) so
//! they run inside the actual production binary's process — exercising
//! main(), CLI parsing, and any startup hooks that a normal cargo-test
//! library build would not. The registry is driven by `libtest-mimic`,
//! with the wrapper at `tests/meta.rs` routing cargo-test invocations
//! through the `pg-ephemeral meta test` subcommand.
//!
//! Per-file submodules (e.g. [`backtrace`]) each expose their own
//! `pub fn trials() -> Vec<Trial>`; this module aggregates them into a
//! single registry and offers two surfaces:
//!
//! - [`trials`] — every trial; used by `meta test`.
//! - [`smoke_trials`] — a small, fast, downstream-relevant subset
//!   identified by name in [`SMOKE`]; used by `meta smoke`.
//!
//! When the host platform doesn't support container runtimes (per
//! [`ociman::testing::platform_not_supported`]), every trial is marked
//! ignored — surfacing as "ignored" in libtest output rather than
//! falsely passing as the prior `test_backend_setup!()` early-return
//! pattern did. Use `--include-ignored` to force-run on a host where
//! you've manually set up the runtime.

pub mod backtrace;
pub mod base;
pub mod cache;
pub mod common;
pub mod container;
pub mod examples;
pub mod examples_boot;
pub mod integration;
pub mod labels;
pub mod seed;

use libtest_mimic::Trial;

/// The smoke subset — a small, fast set of trials that gives a downstream
/// user enough signal to know whether their pg-ephemeral installation
/// works on this host. Avoids the multi-hundred-MB image pulls and
/// minutes-long boot sequences in the full suite.
const SMOKE: &[&str] = &[
    "backtrace_contains_file_paths",
    "every_example_database_toml_parses",
    "set_superuser_password",
    "labels_written_minimal_container",
    "populate_cache",
];

/// Every registered trial.
#[must_use]
pub fn trials() -> Vec<Trial> {
    mark_ignored_when_unsupported(all_trials_raw())
}

/// The smoke subset of [`trials`] — filtered to trials whose name is
/// listed in [`SMOKE`].
#[must_use]
pub fn smoke_trials() -> Vec<Trial> {
    mark_ignored_when_unsupported(
        all_trials_raw()
            .into_iter()
            .filter(|trial| SMOKE.contains(&trial.name()))
            .collect(),
    )
}

fn all_trials_raw() -> Vec<Trial> {
    let mut trials = Vec::new();
    trials.extend(backtrace::trials());
    trials.extend(base::trials());
    trials.extend(cache::trials());
    trials.extend(container::trials());
    trials.extend(examples::trials());
    trials.extend(examples_boot::trials());
    trials.extend(integration::trials());
    trials.extend(labels::trials());
    trials.extend(seed::trials());
    trials
}

fn mark_ignored_when_unsupported(trials: Vec<Trial>) -> Vec<Trial> {
    if ociman::testing::platform_not_supported() {
        trials
            .into_iter()
            .map(|trial| trial.with_ignored_flag(true))
            .collect()
    } else {
        trials
    }
}
