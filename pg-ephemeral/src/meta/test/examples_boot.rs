//! Boot check for every example under `pg-ephemeral/examples/`.
//!
//! For every example with a `database.toml`, load it, build a definition for
//! every declared instance, boot a container, and assert a trivial query
//! works. Catches semantic drift the parse-only [`super::examples`] trial
//! cannot see: e.g. a seed file that references a function the new image
//! removed, or a config that parses but produces a definition that fails
//! to boot.
//!
//! `06-container-script-pg-cron` is excluded because it does
//! `apt-get update && apt-get install` from inside the container, which
//! requires network access that not every CI environment provides. Set
//! `PG_EPHEMERAL_TEST_NETWORK=1` to opt into running it.

use std::path::Path;

use include_dir::{Dir, include_dir};
use libtest_mimic::{Failed, Trial};

use super::common::{TestDir, materialize};

static EXAMPLES: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/examples");

const NETWORK_OPT_IN_VAR: &str = "PG_EPHEMERAL_TEST_NETWORK";
const NETWORK_GATED_EXAMPLES: &[&str] = &["06-container-script-pg-cron"];

#[must_use]
pub fn trials() -> Vec<Trial> {
    vec![Trial::test("every_example_boots", every_example_boots)]
}

fn every_example_boots() -> Result<(), Failed> {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    runtime.block_on(async {
        let _backend = ociman::backend::resolve::auto().await.unwrap();

        let examples_dir = TestDir::new("meta-examples-boot");
        materialize(&EXAMPLES, &examples_dir.path);

        let network_opt_in = std::env::var(NETWORK_OPT_IN_VAR).is_ok();

        let mut entries: Vec<_> = std::fs::read_dir(&examples_dir.path)
            .unwrap()
            .map(Result::unwrap)
            .collect();
        entries.sort_by_key(std::fs::DirEntry::file_name);

        let mut booted: Vec<String> = Vec::new();

        for entry in entries {
            let path = entry.path();
            if !path.is_dir() {
                continue;
            }

            let dir_name = path
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap()
                .to_string();

            let toml_path = path.join("database.toml");
            if !toml_path.exists() {
                continue;
            }

            if NETWORK_GATED_EXAMPLES.contains(&dir_name.as_str()) && !network_opt_in {
                log::info!(
                    "skipping {dir_name}: requires network access; set {NETWORK_OPT_IN_VAR}=1 to enable"
                );
                continue;
            }

            boot_example(&toml_path, &dir_name).await;
            booted.push(dir_name);
        }

        assert!(
            !booted.is_empty(),
            "expected at least one example to boot from {}",
            examples_dir.path.display()
        );

        Ok(())
    })
}

async fn boot_example(toml_path: &Path, example_name: &str) {
    let instance_map =
        crate::Config::load_toml_file(toml_path, &crate::config::InstanceDefinition::empty())
            .unwrap_or_else(|error| panic!("{example_name}: parse failed: {error}"));

    for (instance_name, instance) in &instance_map {
        let definition = instance
            .definition(instance_name)
            .await
            .unwrap_or_else(|error| {
                panic!("{example_name}/{instance_name}: definition build failed: {error}")
            })
            .wait_available_timeout(std::time::Duration::from_secs(30));

        definition
            .with_container(async |container| {
                container
                    .with_connection(async |connection| {
                        let row = sqlx::query("SELECT 1::int AS one")
                            .fetch_one(connection)
                            .await
                            .unwrap_or_else(|error| {
                                panic!("{example_name}/{instance_name}: SELECT 1 failed: {error}")
                            });
                        let one: i32 = sqlx::Row::get(&row, "one");
                        assert_eq!(
                            one, 1,
                            "{example_name}/{instance_name}: SELECT 1 returned {one}"
                        );
                    })
                    .await;
            })
            .await
            .unwrap_or_else(|error| {
                panic!("{example_name}/{instance_name}: with_container failed: {error:?}")
            });
    }
}
