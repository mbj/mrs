mod common;

/// Keys whose values change between the caching container and the final container
/// (password is regenerated, port is reassigned on each boot).
const UNSTABLE_ENV_KEYS: &[&str] = &["DATABASE_URL", "PGPASSWORD", "PGPORT"];

async fn assert_environment_matches(
    container: &pg_ephemeral::Container,
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
    let pg_env = container.pg_env();
    let mut expected: Vec<(String, String)> = pg_env
        .iter()
        .map(|(key, value)| (key.to_string(), value.to_string()))
        .collect();
    expected.push(("DATABASE_URL".to_string(), container.database_url()));
    expected.sort();

    // Verify all expected keys are present
    let actual_keys: Vec<&str> = actual.iter().map(|(k, _)| k.as_str()).collect();
    let expected_keys: Vec<&str> = expected.iter().map(|(k, _)| k.as_str()).collect();
    assert_eq!(expected_keys, actual_keys);

    // Verify stable values match (password and port change between cache and boot)
    let actual_stable: Vec<&(String, String)> = actual
        .iter()
        .filter(|(k, _)| !UNSTABLE_ENV_KEYS.contains(&k.as_str()))
        .collect();
    let expected_stable: Vec<&(String, String)> = expected
        .iter()
        .filter(|(k, _)| !UNSTABLE_ENV_KEYS.contains(&k.as_str()))
        .collect();
    assert_eq!(expected_stable, actual_stable);
}

#[tokio::test]
async fn test_command_seed_receives_environment() {
    let backend = ociman::test_backend_setup!();

    let definition = common::test_definition(backend)
        .apply_file(
            "create-table".parse().unwrap(),
            "tests/fixtures/create_seed_env_table.sql".into(),
        )
        .unwrap()
        .apply_command(
            "capture-env".parse().unwrap(),
            pg_ephemeral::Command::new(
                "sh",
                [
                    "-c",
                    "(env | grep '^PG' && echo DATABASE_URL=$DATABASE_URL) | sed 's/=/,/' | psql -c \"\\copy seed_env FROM STDIN WITH (FORMAT csv)\"",
                ],
            ),
            pg_ephemeral::CommandCacheConfig::None,
        )
        .unwrap();

    definition
        .with_container(async |container| {
            container
                .with_connection(async |connection| {
                    assert_environment_matches(container, connection).await;
                })
                .await
        })
        .await
        .unwrap()
}

#[tokio::test]
async fn test_script_seed_receives_environment() {
    let backend = ociman::test_backend_setup!();

    let definition = common::test_definition(backend)
        .apply_file(
            "create-table".parse().unwrap(),
            "tests/fixtures/create_seed_env_table.sql".into(),
        )
        .unwrap()
        .apply_script(
            "capture-env".parse().unwrap(),
            "(env | grep '^PG' && echo DATABASE_URL=$DATABASE_URL) | sed 's/=/,/' | psql -c \"\\copy seed_env FROM STDIN WITH (FORMAT csv)\"",
        )
        .unwrap();

    definition
        .with_container(async |container| {
            container
                .with_connection(async |connection| {
                    assert_environment_matches(container, connection).await;
                })
                .await
        })
        .await
        .unwrap()
}

#[tokio::test]
async fn test_git_revision_seed() {
    let _backend = ociman::test_backend_setup!();

    let repo = common::TestGitRepo::new("git-revision-test").await;

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

    // Get path to pg-ephemeral binary using the canonical Cargo test environment variable
    let pg_ephemeral_bin = env!("CARGO_BIN_EXE_pg-ephemeral");

    // Create pipes for the integration protocol
    let (result_read, result_write) = std::io::pipe().unwrap();
    let (control_read, control_write) = std::io::pipe().unwrap();

    use std::os::fd::AsRawFd;
    let result_write_fd = result_write.as_raw_fd();
    let control_read_fd = control_read.as_raw_fd();

    // Start pg-ephemeral integration-server with pipe FDs
    let mut cmd = cmd_proc::Command::new(pg_ephemeral_bin)
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
    // This runs after fork() but before exec().
    unsafe {
        cmd.pre_exec(move || {
            let flags = libc::fcntl(result_write_fd, libc::F_GETFD);
            libc::fcntl(result_write_fd, libc::F_SETFD, flags & !libc::FD_CLOEXEC);
            let flags = libc::fcntl(control_read_fd, libc::F_GETFD);
            libc::fcntl(control_read_fd, libc::F_SETFD, flags & !libc::FD_CLOEXEC);
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
    let output = cmd_proc::Command::new(pg_ephemeral_bin)
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
    assert_eq!(String::from_utf8(output.stdout).unwrap().trim(), "id\n1");

    // Stop the server by closing the control pipe write end and wait for it to finish
    drop(control_write);
    server.wait().await.unwrap();
}
