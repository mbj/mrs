//! Container-level lifecycle tests (password rotation, etc.).

use std::str::FromStr;

use libtest_mimic::{Failed, Trial};

#[must_use]
pub fn trials() -> Vec<Trial> {
    vec![Trial::test(
        "set_superuser_password",
        set_superuser_password,
    )]
}

fn set_superuser_password() -> Result<(), Failed> {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    runtime.block_on(async {
        let backend = ociman::backend::resolve::auto().await.unwrap();

        let definition =
            crate::Definition::new(backend, crate::Image::default(), "test".parse().unwrap())
                .wait_available_timeout(std::time::Duration::from_secs(30));

        definition
            .with_container(async |container| {
                let new_password =
                    pg_client::config::Password::from_str("new_password_123").unwrap();
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

        Ok(())
    })
}
