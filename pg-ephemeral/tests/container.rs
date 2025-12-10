use std::str::FromStr;

#[tokio::test]
async fn test_run_container_definition() {
    if ociman::testing::platform_not_supported() {
        return;
    }

    let backend = ociman::test_backend_setup!();
    let static_password = "testpass123";
    let static_username = "postgres";
    let static_database = "postgres";
    let snapshot_image: ociman::image::Reference = "pg-ephemeral-test:snapshot".parse().unwrap();

    // Create a container, populate it with data, and commit it as a snapshot image
    let mut ociman_container = ociman::Definition::new(
        backend.clone(),
        "docker.io/library/postgres:17"
            .parse::<ociman::image::Reference>()
            .unwrap(),
    )
    .remove_on_drop()
    .environment_variable("POSTGRES_PASSWORD", static_password)
    .environment_variable("POSTGRES_USER", static_username)
    .environment_variable("PGDATA", pg_ephemeral::container::PGDATA)
    .publish(ociman::Publish::tcp(5432))
    .run_detached();

    let port = ociman_container.read_host_tcp_port(5432).unwrap();

    let client_config = pg_client::Config {
        application_name: None,
        database: pg_client::Database::from_str(static_database).unwrap(),
        endpoint: pg_client::Endpoint::Network {
            host: pg_client::Host::IpAddr(std::net::IpAddr::V4(std::net::Ipv4Addr::LOCALHOST)),
            host_addr: None,
            port: Some(pg_client::Port(port)),
        },
        password: Some(pg_client::Password::from_str(static_password).unwrap()),
        ssl_mode: pg_client::SslMode::Disable,
        ssl_root_cert: None,
        username: pg_client::Username::from_str(static_username).unwrap(),
    };

    wait_for_postgres(&client_config).await;

    client_config
        .with_sqlx_connection(async |conn| {
            sqlx::query("CREATE TABLE test_data (id INT, value TEXT)")
                .execute(&mut *conn)
                .await
                .unwrap();
            sqlx::query("INSERT INTO test_data VALUES (1, 'snapshot_test')")
                .execute(&mut *conn)
                .await
                .unwrap();
        })
        .await
        .unwrap();

    ociman_container.stop();
    ociman_container.commit(&snapshot_image, false).unwrap();
    drop(ociman_container);

    // Now use pg_ephemeral to run from this snapshot image using container::Definition
    let definition = pg_ephemeral::container::Definition {
        image: snapshot_image.clone(),
        password: pg_client::Password::from_str(static_password).unwrap(),
        username: pg_client::Username::from_str(static_username).unwrap(),
        database: pg_client::Database::from_str(static_database).unwrap(),
        backend: backend.clone(),
        cross_container_access: false,
        application_name: None,
        ssl_config: None,
    };

    let mut container = pg_ephemeral::Container::run_container_definition(&definition);
    container.wait_available().await;

    container
        .with_connection(async |conn| {
            let row: (i32, String) = sqlx::query_as("SELECT id, value FROM test_data")
                .fetch_one(&mut *conn)
                .await
                .unwrap();
            assert_eq!(row.0, 1);
            assert_eq!(row.1, "snapshot_test");
        })
        .await;

    container.stop();

    // Force remove needed: container stop returns before container removal completes,
    // so a non-force remove may fail with "image is in use by stopped container".
    backend.remove_image_force(&snapshot_image);
}

#[tokio::test]
async fn test_set_superuser_password() {
    let backend = ociman::test_backend_setup!();

    let definition = pg_ephemeral::Definition::new(backend, pg_ephemeral::Image::default());

    definition
        .with_container(async |container| {
            let new_password = pg_client::Password::from_str("new_password_123").unwrap();
            container.set_superuser_password(&new_password);

            let new_client_config = pg_client::Config {
                password: Some(new_password),
                ..container.client_config().clone()
            };

            new_client_config
                .with_sqlx_connection(async |_| {})
                .await
                .unwrap();
        })
        .await;
}

async fn wait_for_postgres(config: &pg_client::Config) {
    let sqlx_config = config.to_sqlx_connect_options().unwrap();

    let start = std::time::Instant::now();
    let max_duration = std::time::Duration::from_secs(30);
    let sleep_duration = std::time::Duration::from_millis(100);

    while start.elapsed() <= max_duration {
        match sqlx::ConnectOptions::connect(&sqlx_config).await {
            Ok(conn) => {
                sqlx::Connection::close(conn).await.unwrap();
                return;
            }
            Err(_) => {
                tokio::time::sleep(sleep_duration).await;
            }
        }
    }

    panic!("Postgres did not become available within 30 seconds");
}
