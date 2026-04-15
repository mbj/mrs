use pg_ephemeral::cli;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    use clap::Parser;

    if let Err(error) = cli::App::parse().run().await {
        eprintln!("Error: {error}");
        std::process::exit(1);
    }
}
