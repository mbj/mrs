mod common;

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

    assert_eq!(expected, actual);
}

#[tokio::test]
async fn test_command_seed_receives_environment() {
    let backend = ociman::test_backend_setup!();

    let definition = pg_ephemeral::Definition::new(backend, pg_ephemeral::Image::default())
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
}

#[tokio::test]
async fn test_script_seed_receives_environment() {
    let backend = ociman::test_backend_setup!();

    let definition = pg_ephemeral::Definition::new(backend, pg_ephemeral::Image::default())
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
}

#[test]
fn test_git_revision_seed() {
    let _backend = ociman::test_backend_setup!();

    let repo = common::TestGitRepo::new("git_revision_seed");

    // Create seed.sql with table creation and insert for commit 1
    repo.write_file(
        "seed.sql",
        indoc::indoc! {r#"
            CREATE TABLE users (id INTEGER PRIMARY KEY);
            INSERT INTO users (id) VALUES (1);
        "#},
    );

    // Commit v1
    let commit1_hash = repo.commit("Initial data");

    // Modify seed.sql to insert different data for commit 2
    repo.write_file(
        "seed.sql",
        indoc::indoc! {r#"
            CREATE TABLE users (id INTEGER PRIMARY KEY);
            INSERT INTO users (id) VALUES (2);
        "#},
    );

    // Commit v2
    let _ = repo.commit("Different data");

    // Create TOML config that references commit1
    let config_path = repo.path.join("database.toml");
    let config_content = indoc::formatdoc! {r#"
        image = "17.1"

        [instances.main.seeds.schema]
        type = "sql-file"
        path = "seed.sql"
        git_revision = "{commit1_hash}"
    "#};
    std::fs::write(&config_path, config_content).unwrap();

    // Get path to pg-ephemeral binary using the canonical Cargo test environment variable
    let pg_ephemeral_bin = env!("CARGO_BIN_EXE_pg-ephemeral");

    // Start pg-ephemeral integration-server
    let mut server = std::process::Command::new(pg_ephemeral_bin)
        .arg("integration-server")
        .arg("--protocol")
        .arg("v0")
        .current_dir(&repo.path)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::inherit())
        .spawn()
        .unwrap();

    // Read the JSON output with connection details
    use std::io::BufRead;
    std::io::BufReader::new(server.stdout.as_mut().unwrap())
        .lines()
        .next()
        .unwrap()
        .unwrap();

    // Run psql command to query the data
    let output = std::process::Command::new(pg_ephemeral_bin)
        .arg("run-env")
        .arg("--")
        .arg("psql")
        .arg("--csv")
        .arg("--command=SELECT id FROM users ORDER BY id")
        .current_dir(&repo.path)
        .stderr(std::process::Stdio::inherit())
        .output()
        .unwrap();

    assert!(output.status.success(), "psql command failed");

    // Verify we have the data from commit 1 (id=1), not commit 2 (id=2)
    assert_eq!(String::from_utf8(output.stdout).unwrap().trim(), "id\n1");

    // Stop the server by closing stdin and wait for it to finish
    drop(server.stdin.take());
    server.wait().unwrap();
}
