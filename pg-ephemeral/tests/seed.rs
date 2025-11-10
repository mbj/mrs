mod common;

async fn assert_environment_matches(
    container: &pg_ephemeral::Container<'_>,
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
    if common::platform_not_supported() {
        return;
    }

    let definition = pg_ephemeral::Definition::new(
            pg_ephemeral::BackendSelection::Auto,
            pg_ephemeral::Image::default(),
        )
        .apply_file(
            "create-table".parse().unwrap(),
            "tests/fixtures/create_seed_env_table.sql".into(),
        )
        .unwrap()
        .apply_command(
            "capture-env".parse().unwrap(),
            pg_ephemeral::Command::new(
                "sh".to_string(),
                vec![
                    "-c".to_string(),
                    "(env | grep '^PG' && echo DATABASE_URL=$DATABASE_URL) | sed 's/=/,/' | psql -c \"\\copy seed_env FROM STDIN WITH (FORMAT csv)\"".to_string(),
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
    if common::platform_not_supported() {
        return;
    }

    let definition = pg_ephemeral::Definition::new(
            pg_ephemeral::BackendSelection::Auto,
            pg_ephemeral::Image::default(),
        )
        .apply_file(
            "create-table".parse().unwrap(),
            "tests/fixtures/create_seed_env_table.sql".into(),
        )
        .unwrap()
        .apply_script(
            "capture-env".parse().unwrap(),
            "(env | grep '^PG' && echo DATABASE_URL=$DATABASE_URL) | sed 's/=/,/' | psql -c \"\\copy seed_env FROM STDIN WITH (FORMAT csv)\"".to_string(),
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
