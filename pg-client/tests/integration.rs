use std::num::NonZeroUsize;

#[tokio::test]
async fn test_with_sqlx_connection() {
    let backend = ociman::test_backend_setup!();

    // CI environments may be slow, use 30s instead of default 10s
    let definition = pg_ephemeral::Definition::new(backend, pg_ephemeral::Image::default())
        .wait_available_timeout(std::time::Duration::from_secs(30));

    definition
        .with_container(async |container| {
            let result = container
                .client_config()
                .with_sqlx_connection(async |connection| {
                    let row = sqlx::query("SELECT true")
                        .fetch_one(connection)
                        .await
                        .unwrap();

                    sqlx::Row::get::<bool, usize>(&row, 0)
                })
                .await;

            assert!(result.is_ok(), "Connection should succeed: {result:?}");
            assert!(result.unwrap(), "Query should return true");
        })
        .await
}

#[tokio::test]
async fn test_with_sqlx_connection_error_on_unavailable_database() {
    let config = pg_client::Config {
        application_name: None,
        database: "test_db".parse().unwrap(),
        endpoint: pg_client::Endpoint::Network {
            host: "localhost".parse().unwrap(),
            channel_binding: None,
            host_addr: None,
            port: Some(pg_client::Port::new(0)), // Port 0 is reserved and never available
        },
        password: Some("test".parse().unwrap()),
        ssl_mode: pg_client::SslMode::Disable,
        ssl_root_cert: None,
        user: "test".parse().unwrap(),
    };

    let result = config
        .with_sqlx_connection(async |connection| {
            let row = sqlx::query("SELECT true")
                .fetch_one(connection)
                .await
                .unwrap();

            sqlx::Row::get::<bool, usize>(&row, 0)
        })
        .await;

    assert!(result.is_err(), "Connection should fail");

    let error = result.unwrap_err();
    match error {
        pg_client::sqlx::ConnectionError::Connect(_) => {
            // Expected error variant
        }
        other => panic!("Expected Connect error, got: {other:?}"),
    }
}

#[tokio::test]
async fn test_analyze_all_tables() {
    let backend = ociman::test_backend_setup!();

    let definition = pg_ephemeral::Definition::new(backend, pg_ephemeral::Image::default())
        .wait_available_timeout(std::time::Duration::from_secs(30));

    definition
        .with_container(async |container| {
            let config = container.client_config();

            // Create a test table to analyze
            config
                .with_sqlx_connection(async |connection| {
                    sqlx::query("CREATE TABLE test_table (id INT PRIMARY KEY, name TEXT)")
                        .execute(connection)
                        .await
                        .unwrap();
                })
                .await
                .unwrap();

            // Run analyze on public schema
            let result = pg_client::sqlx::analyze::run_all(
                config,
                &pg_client::sqlx::analyze::Schemas::Specific(
                    [pg_client::identifier::Schema::PUBLIC].into(),
                ),
                NonZeroUsize::new(1).unwrap(),
            )
            .await;

            assert!(result.is_ok(), "Analyze should succeed: {result:?}");

            let result = result.unwrap();
            assert_eq!(result.table_count, 1, "Should have 1 table to analyze");
            assert!(!result.elapsed.is_zero(), "Elapsed time should be non-zero");
        })
        .await
}
