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
            host_addr: None,
            port: Some(pg_client::Port::new(0)), // Port 0 is reserved and never available
        },
        password: Some("test".parse().unwrap()),
        ssl_mode: pg_client::SslMode::Disable,
        ssl_root_cert: None,
        username: "test".parse().unwrap(),
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
