//! Parse check for every `database.toml` under `pg-ephemeral/examples/`.
//!
//! Walks every example directory and asserts that its `database.toml`
//! parses against the current `Config` schema. Catches renamed fields,
//! removed seed types, and `deny_unknown_fields` regressions the moment
//! someone touches `config.rs`, with no Docker required.
//!
//! The examples directory is embedded at build time via [`include_dir!`]
//! and materialized to a temp directory at trial run time, so the trial
//! works on installed binaries with no source tree present.

use include_dir::{Dir, include_dir};
use libtest_mimic::{Failed, Trial};

use super::common::{TestDir, materialize};

static EXAMPLES: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/examples");

#[must_use]
pub fn trials() -> Vec<Trial> {
    vec![Trial::test(
        "every_example_database_toml_parses",
        every_example_database_toml_parses,
    )]
}

fn every_example_database_toml_parses() -> Result<(), Failed> {
    let examples_dir = TestDir::new("meta-examples-parse");
    materialize(&EXAMPLES, &examples_dir.path);

    let mut checked: Vec<std::path::PathBuf> = Vec::new();
    let mut failures: Vec<String> = Vec::new();

    let mut entries: Vec<_> = std::fs::read_dir(&examples_dir.path)
        .unwrap()
        .map(Result::unwrap)
        .collect();
    entries.sort_by_key(std::fs::DirEntry::file_name);

    for entry in entries {
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }

        let toml_path = path.join("database.toml");
        if !toml_path.exists() {
            // Examples without a TOML (e.g. `01-default`) are intentional.
            continue;
        }

        match crate::Config::load_toml_file(&toml_path, &crate::config::InstanceDefinition::empty())
        {
            Ok(_) => checked.push(toml_path),
            Err(error) => failures.push(format!("{}: {error}", toml_path.display())),
        }
    }

    assert!(
        !checked.is_empty(),
        "expected at least one example database.toml under {}",
        examples_dir.path.display()
    );

    assert!(
        failures.is_empty(),
        "example database.toml files failed to parse:\n  {}",
        failures.join("\n  ")
    );

    Ok(())
}
