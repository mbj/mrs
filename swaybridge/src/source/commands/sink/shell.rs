/// Open an SSH shell on a sink
#[derive(Debug, clap::Parser)]
pub struct App {
    #[clap(long)]
    name: Option<crate::SinkName>,
    #[clap(long)]
    seed: bool,
}

impl App {
    pub fn run(self) -> Result<(), crate::Error> {
        let config = crate::source::config::Source::load_default()?;

        let sink = super::find_sink(&config.sinks, self.name)?;

        let socket = super::ControlSocket::new(sink)?;

        if self.seed {
            let home = socket.remote_home(&sink.user)?;
            super::seed::run(&socket, &home)?;
        }

        socket
            .ssh_command()
            .spawn()
            .run()?
            .wait()?;

        Ok(())
    }
}
