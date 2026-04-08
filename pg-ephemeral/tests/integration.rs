mod common;

#[tokio::test]
async fn test_ruby_database_url_integration() {
    common::test_database_url_integration("ruby", "tests/integration/ruby", &common::RUBY_IMAGE)
        .await
}

#[tokio::test]
async fn test_prisma_database_url_integration() {
    common::test_database_url_integration("prisma", "tests/integration/prisma", &common::NODE_IMAGE)
        .await
}
