use pg_ephemeral::cli;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), cli::Error> {
    env_logger::init();

    use clap::Parser;

    cli::App::parse().run().await
}
