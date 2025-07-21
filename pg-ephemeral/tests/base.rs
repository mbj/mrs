#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn test_base_feature() {
    if platform_not_supported() {
        return;
    }
    let definition = pg_ephemeral::Definition::new(pg_ephemeral::Version::new(
        pg_ephemeral::Major::R17,
        pg_ephemeral::Minor::new(5),
    ));

    definition
        .with_container(async |container| {
            container
                .with_connection(async |connection| {
                    let row = sqlx::query("SELECT true")
                        .fetch_one(connection)
                        .await
                        .unwrap();
                    assert!(sqlx::Row::get::<bool, usize>(&row, 0))
                })
                .await
        })
        .await
}

fn platform_not_supported() -> bool {
    std::env::consts::OS == "macos" && std::env::var("GITHUB_ACTIONS").is_ok()
}
