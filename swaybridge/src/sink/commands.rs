mod info;
mod run;
mod stop;
mod sway_socket;

/// Commands to be run from the sink
#[derive(Debug, clap::Parser)]
pub struct App {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, clap::Parser)]
enum Command {
    Info(info::App),
    Run(run::App),
    Stop(stop::App),
    SwaySocket(sway_socket::App),
}

impl App {
    pub fn run(self) -> Result<(), crate::Error> {
        match self.command {
            Command::Info(command) => command.run(),
            Command::Run(command) => command.run(),
            Command::Stop(command) => command.run(),
            Command::SwaySocket(command) => command.run(),
        }
    }
}
