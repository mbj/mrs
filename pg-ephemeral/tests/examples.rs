//! Parse check for every `database.toml` under `pg-ephemeral/examples/`.
//!
//! Walks every example directory and asserts that its `database.toml`
//! parses against the current `Config` schema. Catches renamed fields,
//! removed seed types, and `deny_unknown_fields` regressions the moment
//! someone touches `config.rs`, with no Docker required.

#[test]
fn every_example_database_toml_parses() {
    let examples_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("examples");

    assert!(
        examples_dir.is_dir(),
        "examples directory missing: {}",
        examples_dir.display()
    );

    let mut checked: Vec<std::path::PathBuf> = Vec::new();
    let mut failures: Vec<String> = Vec::new();

    let mut entries: Vec<_> = std::fs::read_dir(&examples_dir)
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

        match pg_ephemeral::Config::load_toml_file(
            &toml_path,
            &pg_ephemeral::config::InstanceDefinition::empty(),
        ) {
            Ok(_) => checked.push(toml_path),
            Err(error) => failures.push(format!("{}: {error}", toml_path.display())),
        }
    }

    assert!(
        !checked.is_empty(),
        "expected at least one example database.toml under {}",
        examples_dir.display()
    );

    assert!(
        failures.is_empty(),
        "example database.toml files failed to parse:\n  {}",
        failures.join("\n  ")
    );
}
