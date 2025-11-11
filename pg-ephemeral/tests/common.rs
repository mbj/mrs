#[allow(dead_code)]
pub async fn test_database_url_integration(language: &str, image_dir: &str) {
    let backend = cbt::test_backend_setup!();

    let definition = pg_ephemeral::Definition::new(backend.into(), pg_ephemeral::Image::default());

    definition
        .with_container(async |container| {
            let image_tag = format!("pg-ephemeral-{}-test:latest", language);

            let _build_output = backend
                .command()
                .argument("build")
                .argument("--tag")
                .argument(&image_tag)
                .argument(image_dir)
                .capture_only_stdout();

            let output = backend
                .command()
                .argument("run")
                .argument("--rm")
                .argument("--network=host")
                .argument("--env")
                .argument(format!("DATABASE_URL={}", container.database_url()))
                .argument(&image_tag)
                .capture_only_stdout_result();

            let stdout = match output {
                Ok(bytes) => String::from_utf8(bytes).expect("invalid utf8 in output"),
                Err(e) => panic!("Failed to run {} container: {:?}", language, e),
            };

            assert!(
                stdout.contains("SUCCESS: Connected to PostgreSQL successfully"),
                "Expected success message not found in output.\nOutput: {}",
                stdout
            );
        })
        .await
}
