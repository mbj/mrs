mod common;

use std::str::FromStr;

#[tokio::test]
async fn test_set_superuser_password() {
    if ociman::testing::platform_not_supported() {
        return;
    }

    let backend = ociman::test_backend_setup!();

    let definition = pg_ephemeral::Definition::new(
        backend,
        pg_ephemeral::Image::default(),
        "test".parse().unwrap(),
    )
    .wait_available_timeout(std::time::Duration::from_secs(30));

    definition
        .with_container(async |container| {
            let new_password = pg_client::config::Password::from_str("new_password_123").unwrap();
            let previous = container.session_config();

            container
                .set_superuser_password(&new_password, definition.wait_available_timeout)
                .await
                .unwrap();

            assert_eq!(
                container.libpq_config().session.password,
                Some(new_password)
            );

            assert!(matches!(
                pg_client::session::start(previous).await,
                Err(pg_client::session::StartError::Rejected { .. })
            ));

            pg_client::session::start(container.session_config())
                .await
                .unwrap()
                .send_terminate()
                .await
                .unwrap();
        })
        .await
        .unwrap();
}
