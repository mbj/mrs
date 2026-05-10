//! Verifies seed application across every seed type (file, statement, command,
//! script, csv-file, git-revision) — including the environment surface delivered
//! to command/script seeds and the column-reorder / header-mismatch behavior
//! of csv-file seeds.

use include_dir::{Dir, include_dir};
use libtest_mimic::{Failed, Trial};

use super::common::{TestDir, TestGitRepo, test_definition};

static FIXTURES: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/tests/fixtures");

/// Keys whose values change between the caching container and the final container
/// (password is regenerated, port is reassigned on each boot).
const UNSTABLE_ENV_KEYS: &[&str] = &["DATABASE_URL", "PGPASSWORD", "PGPORT"];

#[must_use]
pub fn trials() -> Vec<Trial> {
    vec![
        Trial::test(
            "command_seed_receives_environment",
            command_seed_receives_environment,
        ),
        Trial::test(
            "script_seed_receives_environment",
            script_seed_receives_environment,
        ),
        Trial::test(
            "sql_statement_seed_multi_statement",
            sql_statement_seed_multi_statement,
        ),
        Trial::test("csv_file_seed", csv_file_seed),
        Trial::test("csv_file_seed_column_reorder", csv_file_seed_column_reorder),
        Trial::test(
            "csv_file_seed_header_mismatch",
            csv_file_seed_header_mismatch,
        ),
        Trial::test("git_revision_seed", git_revision_seed),
    ]
}

async fn assert_environment_matches(
    container: &crate::Container,
    connection: &mut sqlx::postgres::PgConnection,
) {
    // Read environment variables from database
    let rows = sqlx::query("SELECT key, value FROM seed_env ORDER BY key")
        .fetch_all(connection)
        .await
        .unwrap();

    let actual: Vec<(String, String)> = rows
        .iter()
        .map(|row| {
            (
                sqlx::Row::get::<String, _>(row, "key"),
                sqlx::Row::get::<String, _>(row, "value"),
            )
        })
        .collect();

    // Generate expected output from config
    let pg_env = container.pg_env().unwrap();
    let mut expected: Vec<(String, String)> = pg_env
        .iter()
        .map(|(key, value)| (key.to_string(), value.to_string()))
        .collect();
    expected.push(("DATABASE_URL".to_string(), container.database_url()));
    expected.sort();

    // Verify all expected keys are present
    let actual_keys: Vec<&str> = actual.iter().map(|(key, _)| key.as_str()).collect();
    let expected_keys: Vec<&str> = expected.iter().map(|(key, _)| key.as_str()).collect();
    assert_eq!(expected_keys, actual_keys);

    // Verify stable values match (password and port change between cache and boot)
    let actual_stable: Vec<&(String, String)> = actual
        .iter()
        .filter(|(key, _)| !UNSTABLE_ENV_KEYS.contains(&key.as_str()))
        .collect();
    let expected_stable: Vec<&(String, String)> = expected
        .iter()
        .filter(|(key, _)| !UNSTABLE_ENV_KEYS.contains(&key.as_str()))
        .collect();
    assert_eq!(expected_stable, actual_stable);
}

fn command_seed_receives_environment() -> Result<(), Failed> {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    runtime.block_on(async {
        let backend = ociman::backend::resolve::auto().await.unwrap();
        let fixtures_dir = TestDir::new("seed-command-env-fixtures");
        super::common::materialize(&FIXTURES, &fixtures_dir.path);

        let definition = test_definition(backend)
            .apply_file(
                "create-table".parse().unwrap(),
                fixtures_dir.path.join("create_seed_env_table.sql"),
            )
            .unwrap()
            .apply_command(
                "capture-env".parse().unwrap(),
                crate::Command::new(
                    "sh",
                    [
                        "-c",
                        "(env | grep '^PG' && echo DATABASE_URL=$DATABASE_URL) | sed 's/=/,/' | psql -c \"\\copy seed_env FROM STDIN WITH (FORMAT csv)\"",
                    ],
                ),
                crate::SeedCacheConfig::None,
            )
            .unwrap();

        definition
            .with_container(async |container| {
                container
                    .with_connection(async |connection| {
                        assert_environment_matches(container, connection).await;
                    })
                    .await;
            })
            .await
            .unwrap();

        Ok(())
    })
}

fn script_seed_receives_environment() -> Result<(), Failed> {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    runtime.block_on(async {
        let backend = ociman::backend::resolve::auto().await.unwrap();
        let fixtures_dir = TestDir::new("seed-script-env-fixtures");
        super::common::materialize(&FIXTURES, &fixtures_dir.path);

        let definition = test_definition(backend)
            .apply_file(
                "create-table".parse().unwrap(),
                fixtures_dir.path.join("create_seed_env_table.sql"),
            )
            .unwrap()
            .apply_script(
                "capture-env".parse().unwrap(),
                "(env | grep '^PG' && echo DATABASE_URL=$DATABASE_URL) | sed 's/=/,/' | psql -c \"\\copy seed_env FROM STDIN WITH (FORMAT csv)\"",
                crate::SeedCacheConfig::CommandHash,
            )
            .unwrap();

        definition
            .with_container(async |container| {
                container
                    .with_connection(async |connection| {
                        assert_environment_matches(container, connection).await;
                    })
                    .await;
            })
            .await
            .unwrap();

        Ok(())
    })
}

fn sql_statement_seed_multi_statement() -> Result<(), Failed> {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    runtime.block_on(async {
        let backend = ociman::backend::resolve::auto().await.unwrap();

        let definition = test_definition(backend)
            .apply_sql_statement(
                "schema-and-data".parse().unwrap(),
                indoc::indoc! {r#"
                    CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT NOT NULL);
                    INSERT INTO users (id, name) VALUES (1, 'alice');
                    INSERT INTO users (id, name) VALUES (2, 'bob');
                "#},
            )
            .unwrap();

        definition
            .with_container(async |container| {
                container
                    .with_connection(async |connection| {
                        let rows: Vec<(i32, String)> =
                            sqlx::query_as("SELECT id, name FROM users ORDER BY id")
                                .fetch_all(&mut *connection)
                                .await
                                .unwrap();

                        assert_eq!(rows, vec![(1, "alice".to_string()), (2, "bob".to_string())]);
                    })
                    .await;
            })
            .await
            .unwrap();

        Ok(())
    })
}

fn csv_file_seed() -> Result<(), Failed> {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    runtime.block_on(async {
        let backend = ociman::backend::resolve::auto().await.unwrap();
        let fixtures_dir = TestDir::new("seed-csv-file-fixtures");
        super::common::materialize(&FIXTURES, &fixtures_dir.path);

        let definition = test_definition(backend)
            .apply_file(
                "create-table".parse().unwrap(),
                fixtures_dir.path.join("create_users_table.sql"),
            )
            .unwrap()
            .apply_csv_file(
                "import-users".parse().unwrap(),
                fixtures_dir.path.join("users.csv"),
                pg_client::QualifiedTable {
                    schema: pg_client::identifier::Schema::PUBLIC,
                    table: "users".parse().unwrap(),
                },
            )
            .unwrap();

        definition
            .with_container(async |container| {
                container
                    .with_connection(async |connection| {
                        let rows: Vec<(i32, String)> =
                            sqlx::query_as("SELECT id, name FROM public.users ORDER BY id")
                                .fetch_all(&mut *connection)
                                .await
                                .unwrap();

                        assert_eq!(
                            rows,
                            vec![
                                (1, "alice".to_string()),
                                (2, "bob".to_string()),
                                (3, "charlie".to_string()),
                            ]
                        );
                    })
                    .await;
            })
            .await
            .unwrap();

        Ok(())
    })
}

fn csv_file_seed_column_reorder() -> Result<(), Failed> {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    runtime.block_on(async {
        let backend = ociman::backend::resolve::auto().await.unwrap();
        let fixtures_dir = TestDir::new("seed-csv-reorder-fixtures");
        super::common::materialize(&FIXTURES, &fixtures_dir.path);

        let definition = test_definition(backend)
            .apply_file(
                "create-table".parse().unwrap(),
                fixtures_dir.path.join("create_users_table_serial.sql"),
            )
            .unwrap()
            .apply_csv_file(
                "import-users".parse().unwrap(),
                fixtures_dir.path.join("users_name_only.csv"),
                pg_client::QualifiedTable {
                    schema: pg_client::identifier::Schema::PUBLIC,
                    table: "users".parse().unwrap(),
                },
            )
            .unwrap();

        definition
            .with_container(async |container| {
                container
                    .with_connection(async |connection| {
                        let rows: Vec<(i32, String)> =
                            sqlx::query_as("SELECT id, name FROM public.users ORDER BY id")
                                .fetch_all(&mut *connection)
                                .await
                                .unwrap();

                        assert_eq!(
                            rows,
                            vec![
                                (1, "alice".to_string()),
                                (2, "bob".to_string()),
                                (3, "charlie".to_string()),
                            ]
                        );
                    })
                    .await;
            })
            .await
            .unwrap();

        Ok(())
    })
}

fn csv_file_seed_header_mismatch() -> Result<(), Failed> {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    runtime.block_on(async {
        let backend = ociman::backend::resolve::auto().await.unwrap();
        let fixtures_dir = TestDir::new("seed-csv-header-mismatch-fixtures");
        super::common::materialize(&FIXTURES, &fixtures_dir.path);

        let definition = test_definition(backend)
            .apply_file(
                "create-table".parse().unwrap(),
                fixtures_dir.path.join("create_users_table_serial.sql"),
            )
            .unwrap()
            .apply_csv_file(
                "import-users".parse().unwrap(),
                fixtures_dir.path.join("users_wrong_column.csv"),
                pg_client::QualifiedTable {
                    schema: pg_client::identifier::Schema::PUBLIC,
                    table: "users".parse().unwrap(),
                },
            )
            .unwrap();

        let error = definition.with_container(async |_| {}).await.unwrap_err();

        assert!(
            format!("{error:?}").contains("wrong_column"),
            "Expected error mentioning wrong_column, got: {error:?}"
        );

        Ok(())
    })
}

fn git_revision_seed() -> Result<(), Failed> {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    runtime.block_on(async {
        let _backend = ociman::backend::resolve::auto().await.unwrap();

        let repo = TestGitRepo::new("git-revision-test").await;

        // Create seed.sql with table creation and insert for commit 1
        repo.write_file(
            "seed.sql",
            indoc::indoc! {r#"
                CREATE TABLE users (id INTEGER PRIMARY KEY);
                INSERT INTO users (id) VALUES (1);
            "#},
        );
        let commit1_hash = repo.commit("Initial data").await;

        // Modify seed.sql to insert different data for commit 2
        repo.write_file(
            "seed.sql",
            indoc::indoc! {r#"
                CREATE TABLE users (id INTEGER PRIMARY KEY);
                INSERT INTO users (id) VALUES (2);
            "#},
        );

        // Commit v2
        let _ = repo.commit("Different data").await;

        // Create TOML config that references commit1
        let config_content = indoc::formatdoc! {r#"
            image = "17.1"

            [instances.main.seeds.schema]
            type = "sql-file"
            path = "seed.sql"
            git_revision = "{commit1_hash}"
        "#};
        repo.write_file("database.toml", &config_content);

        // Get path to pg-ephemeral binary
        let pg_ephemeral_bin = std::env::current_exe().unwrap();

        // Create pipes for the integration protocol
        let (result_read, result_write) = std::io::pipe().unwrap();
        let (control_read, control_write) = std::io::pipe().unwrap();

        use std::os::fd::AsRawFd;
        let result_write_fd = result_write.as_raw_fd();
        let control_read_fd = control_read.as_raw_fd();

        // Start pg-ephemeral integration-server with pipe FDs
        let mut cmd = cmd_proc::Command::new(&pg_ephemeral_bin)
            .arguments([
                "integration-server",
                "--result-fd",
                &result_write_fd.to_string(),
                "--control-fd",
                &control_read_fd.to_string(),
            ])
            .working_directory(&repo.path)
            .build();

        // SAFETY: Clear CLOEXEC on the pipe FDs so the child inherits them.
        // This runs after fork() but before exec(); the borrowed FDs alias
        // the pipe ends owned by the parent and are only used here to flip
        // a flag — they outlive this closure on both sides of the fork.
        unsafe {
            cmd.pre_exec(move || {
                for raw_fd in [result_write_fd, control_read_fd] {
                    let borrowed = std::os::fd::BorrowedFd::borrow_raw(raw_fd);
                    let flags = rustix::io::fcntl_getfd(borrowed)?;
                    rustix::io::fcntl_setfd(borrowed, flags - rustix::io::FdFlags::CLOEXEC)?;
                }
                Ok(())
            });
        }

        let mut server = cmd.spawn().unwrap();

        // Close parent's copies of the child's pipe ends
        drop(result_write);
        drop(control_read);

        // Read the JSON output from the result pipe
        use std::io::BufRead;
        let mut reader = std::io::BufReader::new(result_read);
        let mut line = String::new();
        reader.read_line(&mut line).unwrap();

        // Run psql command to query the data
        let output = cmd_proc::Command::new(&pg_ephemeral_bin)
            .arguments([
                "run-env",
                "--",
                "psql",
                "--csv",
                "--command=SELECT id FROM users ORDER BY id",
            ])
            .working_directory(&repo.path)
            .stdout_capture()
            .stderr_capture()
            .accept_nonzero_exit()
            .run()
            .await
            .unwrap();

        assert!(output.status.success(), "psql command failed");

        // Verify we have the data from commit 1 (id=1), not commit 2 (id=2)
        assert_eq!(std::str::from_utf8(&output.stdout).unwrap().trim(), "id\n1");

        // Stop the server by closing the control pipe write end and wait for it to finish
        drop(control_write);
        server.wait().await.unwrap();

        Ok(())
    })
}
