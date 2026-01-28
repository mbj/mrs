use std::str::FromStr;

const ENV_POSTGRES_PASSWORD: cmd_proc::EnvVariableName<'static> =
    cmd_proc::EnvVariableName::from_static_or_panic("POSTGRES_PASSWORD");
const ENV_POSTGRES_USER: cmd_proc::EnvVariableName<'static> =
    cmd_proc::EnvVariableName::from_static_or_panic("POSTGRES_USER");
const ENV_PGDATA: cmd_proc::EnvVariableName<'static> =
    cmd_proc::EnvVariableName::from_static_or_panic("PGDATA");

#[tokio::test]
async fn test_run_container_definition() {
    if ociman::testing::platform_not_supported() {
        return;
    }

    let backend = ociman::test_backend_setup!();
    let static_password = "testpass123";
    let static_user = "postgres";
    let static_database = "postgres";
    let snapshot_image: ociman::image::Reference =
        ociman::testing::test_reference("pg-ephemeral-test:snapshot");

    let mut ociman_container = ociman::Definition::new(
        backend.clone(),
        "docker.io/library/postgres:17"
            .parse::<ociman::image::Reference>()
            .unwrap(),
    )
    .remove_on_drop()
    .environment_variable(ENV_POSTGRES_PASSWORD, static_password)
    .environment_variable(ENV_POSTGRES_USER, static_user)
    .environment_variable(ENV_PGDATA, pg_ephemeral::container::PGDATA)
    .publish(ociman::Publish::tcp(5432))
    .run_detached();

    let port = ociman_container.read_host_tcp_port(5432).unwrap();

    let client_config = pg_client::Config {
        application_name: None,
        database: pg_client::Database::from_str(static_database).unwrap(),
        endpoint: pg_client::Endpoint::Network {
            host: pg_client::Host::IpAddr(std::net::IpAddr::V4(std::net::Ipv4Addr::LOCALHOST)),
            channel_binding: None,
            host_addr: None,
            port: Some(port.into()),
        },
        password: Some(pg_client::Password::from_str(static_password).unwrap()),
        ssl_mode: pg_client::SslMode::Disable,
        ssl_root_cert: None,
        user: pg_client::User::from_str(static_user).unwrap(),
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

    let definition = pg_ephemeral::container::Definition {
        image: snapshot_image.clone(),
        password: pg_client::Password::from_str(static_password).unwrap(),
        user: pg_client::User::from_str(static_user).unwrap(),
        database: pg_client::Database::from_str(static_database).unwrap(),
        backend: backend.clone(),
        cross_container_access: false,
        application_name: None,
        ssl_config: None,
        // CI environments may be slow, use 30s instead of default 10s
        wait_available_timeout: std::time::Duration::from_secs(30),
    };

    let mut container = pg_ephemeral::container::Container::run_container_definition(&definition);
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
