use clap::Parser;
use wtt::{Config, commands};

#[derive(Debug, clap::Parser)]
#[clap(name = "wtt", version, about = "Work Tree Tool - Git worktree manager")]
struct App {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, clap::Parser)]
enum Command {
    Setup(commands::Setup),
    Add(commands::Add),
    List(commands::List),
    Remove(commands::Remove),
}

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let app = App::parse();
    let config = Config::default();

    let result = match app.command {
        Command::Setup(cmd) => cmd.run(&config),
        Command::Add(cmd) => cmd.run(&config),
        Command::List(cmd) => cmd.run(&config),
        Command::Remove(cmd) => cmd.run(&config),
    };

    if let Err(error) = result {
        log::error!("{error}");
        std::process::exit(1);
    }
}
