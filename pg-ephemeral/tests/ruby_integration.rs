mod common;

#[tokio::test]
async fn test_ruby_database_url_integration() {
    if common::platform_not_supported() {
        return;
    }

    let definition = pg_ephemeral::Definition::new(pg_ephemeral::Image::default());

    definition
        .with_container(async |container| {
            // Build the Ruby test container
            let backend = pg_ephemeral::cbt::backend::autodetect::run().unwrap();
            let ruby_image_tag = "pg-ephemeral-ruby-test:latest";

            // Build the container image for the Ruby app
            let _build_output = backend
                .command()
                .argument("build")
                .argument("--tag")
                .argument(ruby_image_tag)
                .argument("tests/integration/ruby")
                .capture_only_stdout();

            // Run the Ruby container with DATABASE_URL set using cbt
            // Use --network=host so the container can access the PostgreSQL server on localhost
            let output = backend
                .command()
                .argument("run")
                .argument("--rm")
                .argument("--network=host")
                .argument("--env")
                .argument(format!("DATABASE_URL={}", container.database_url()))
                .argument(ruby_image_tag)
                .capture_only_stdout_result();

            let stdout = match output {
                Ok(bytes) => String::from_utf8_lossy(&bytes).to_string(),
                Err(e) => panic!("Failed to run Ruby container: {:?}", e),
            };

            // Check that the Ruby app successfully connected
            assert!(
                stdout.contains("SUCCESS: Connected to PostgreSQL successfully"),
                "Expected success message not found in output.\nOutput: {}",
                stdout
            );
        })
        .await
}
