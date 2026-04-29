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
            container
                .set_superuser_password(&new_password)
                .await
                .unwrap();

            let mut new_client_config = container.client_config().clone();
            new_client_config.session.password = Some(new_password);

            new_client_config
                .with_sqlx_connection(async |_| {})
                .await
                .unwrap();
        })
        .await
        .unwrap();
}
