use pg_ephemeral::cli;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), cli::Error> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    use clap::Parser;

    cli::App::parse().run().await
}
