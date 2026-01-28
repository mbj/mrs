/// Execute a command on a sink over SSH
#[derive(Debug, clap::Parser)]
pub struct App {
    #[clap(long)]
    name: Option<crate::SinkName>,
    #[clap(long)]
    seed: bool,
    #[clap(trailing_var_arg = true, required = true)]
    command: Vec<String>,
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
            .arguments(&self.command)
            .spawn()
            .run()?
            .wait()?;

        Ok(())
    }
}
