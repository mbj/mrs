use clap::Parser;

#[derive(Debug, clap::Parser)]
struct App {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, clap::Parser)]
enum Command {
    Sink(swaybridge::sink::commands::App),
    Source(swaybridge::source::commands::App),
}

fn main() -> Result<(), swaybridge::Error> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let app = App::parse();

    match app.command {
        Command::Sink(command) => command.run(),
        Command::Source(command) => command.run(),
    }
}
