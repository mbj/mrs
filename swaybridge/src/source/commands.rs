mod connect;
mod disconnect;
mod gc;
mod info;
mod status;
mod sink;

/// Commands to be run from the source
#[derive(Debug, clap::Parser)]
pub struct App {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, clap::Parser)]
enum Command {
    Connect(connect::App),
    Disconnect(disconnect::App),
    Gc(gc::App),
    Info(info::App),
    Status(status::App),
    Sink(sink::App),
}

impl App {
    pub fn run(self) -> Result<(), crate::Error> {
        match self.command {
            Command::Connect(command) => command.run(),
            Command::Disconnect(command) => command.run(),
            Command::Gc(command) => command.run(),
            Command::Info(command) => command.run(),
            Command::Status(command) => command.run(),
            Command::Sink(command) => command.run(),
        }
    }
}
