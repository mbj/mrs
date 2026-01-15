use clap::Parser;
use wtt::{Config, ConfigSource, commands};

#[derive(Debug, clap::Parser)]
#[clap(name = "wtt", version, about = "Work Tree Tool - Git worktree manager")]
struct App {
    #[clap(long, conflicts_with = "no_config_file")]
    config_file: Option<std::path::PathBuf>,

    #[clap(long, conflicts_with = "config_file")]
    no_config_file: bool,

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

    let config_source = if app.no_config_file {
        ConfigSource::None
    } else if let Some(path) = app.config_file {
        ConfigSource::File(path)
    } else {
        ConfigSource::Implicit
    };

    let config = match Config::load(&config_source) {
        Ok(config) => config,
        Err(error) => {
            log::error!("{error}");
            std::process::exit(1);
        }
    };

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
