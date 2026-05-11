//! Cross-language integration trials: build a per-language container image
//! with the language's PostgreSQL client, run it against the live ephemeral
//! database, and assert the integration script reports success.

use include_dir::{Dir, include_dir};
use libtest_mimic::{Failed, Trial};

use super::common::{NODE_IMAGE, RUBY_IMAGE, TestDir, materialize, test_definition};

static RUBY_INTEGRATION: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/tests/integration/ruby");
static PRISMA_INTEGRATION: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/tests/integration/prisma");

#[must_use]
pub fn trials() -> Vec<Trial> {
    vec![
        Trial::test("ruby_database_url", ruby_database_url),
        Trial::test("prisma_database_url", prisma_database_url),
    ]
}

fn ruby_database_url() -> Result<(), Failed> {
    run_database_url_integration("ruby", &RUBY_INTEGRATION, &RUBY_IMAGE)
}

fn prisma_database_url() -> Result<(), Failed> {
    run_database_url_integration("prisma", &PRISMA_INTEGRATION, &NODE_IMAGE)
}

fn run_database_url_integration(
    language: &str,
    fixture: &Dir<'_>,
    base_image: &ociman::image::Reference,
) -> Result<(), Failed> {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    runtime.block_on(async {
        let backend = ociman::backend::resolve::auto().await.unwrap();
        let definition = test_definition(backend.clone()).cross_container_access(true);

        let image_dir = TestDir::new(&format!("meta-integration-{language}"));
        materialize(fixture, &image_dir.path);

        definition
            .with_container(async |container| {
                let image_tag = ociman::testing::test_reference(&format!(
                    "pg-ephemeral-{language}-test:latest"
                ))
                .to_string();

                backend
                    .command()
                    .argument("build")
                    .argument("--build-arg")
                    .argument(format!("BASE_IMAGE={base_image}"))
                    .argument("--tag")
                    .argument(&image_tag)
                    .argument(&image_dir.path)
                    .stdout_capture()
                    .bytes()
                    .await
                    .unwrap();

                let database_url = container
                    .cross_container_client_config()
                    .await
                    .to_url_string();

                let stdout = backend
                    .command()
                    .argument("run")
                    .argument("--rm")
                    .argument("--env")
                    .argument(format!("DATABASE_URL={database_url}"))
                    .argument(&image_tag)
                    .stdout_capture()
                    .string()
                    .await
                    .unwrap();

                assert!(
                    stdout.contains("SUCCESS: Connected to PostgreSQL successfully"),
                    "Expected success message not found in output.\nOutput: {stdout}"
                );
            })
            .await
            .unwrap();

        Ok(())
    })
}
