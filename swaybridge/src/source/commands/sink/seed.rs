/// Deploy swaybridge binary to a sink
#[derive(Debug, clap::Parser)]
pub struct App {
    #[clap(long)]
    name: Option<crate::SinkName>,
}

impl App {
    pub fn run(self) -> Result<(), crate::Error> {
        let config = crate::source::config::Source::load_default()?;

        let sink = super::find_sink(&config.sinks, self.name)?;

        let socket = super::ControlSocket::new(sink)?;

        let home = socket.remote_home(&sink.user)?;
        run(&socket, &home)
    }
}

pub(in crate::source::commands) fn run(
    socket: &super::ControlSocket,
    home: &str,
) -> Result<(), crate::Error> {
    let current_exe =
        std::env::current_exe().map_err(crate::Error::CurrentExeNotFound)?;

    let bin_dir = format!("{}/.local/bin", home);
    let target_path = format!("{}/swaybridge", bin_dir);

    socket
        .ssh_command()
        .argument("mkdir")
        .argument("--parents")
        .argument(&bin_dir)
        .status()?;

    let tmp_output = super::output_with_stderr_inherit(
        socket
            .ssh_command()
            .argument("mktemp")
            .argument(format!("--tmpdir={}", bin_dir))
            .argument("swaybridge.tmp.XXXXXX"),
    )?;

    if !tmp_output.success() {
        return Err(crate::Error::SshFailed(tmp_output.status));
    }

    let tmp_path = String::from_utf8_lossy(&tmp_output.stdout)
        .trim()
        .to_string();

    socket
        .scp_command()
        .argument(&current_exe)
        .argument(format!("{}:{}", socket.host(), tmp_path))
        .status()?;

    socket
        .ssh_command()
        .argument("chmod")
        .argument("--changes")
        .argument("--")
        .argument("u+x")
        .argument(&tmp_path)
        .status()?;

    socket
        .ssh_command()
        .argument("mv")
        .argument("--force")
        .argument(&tmp_path)
        .argument(&target_path)
        .status()?;

    Ok(())
}
