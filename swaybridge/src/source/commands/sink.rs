use std::collections::BTreeMap;
use std::path::PathBuf;

mod exec;
pub(in crate::source::commands) mod seed;
mod shell;

/// Manage remote sinks from the source
#[derive(Debug, clap::Parser)]
pub struct App {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, clap::Parser)]
enum Command {
    Exec(exec::App),
    Seed(seed::App),
    Shell(shell::App),
}

impl App {
    pub fn run(self) -> Result<(), crate::Error> {
        match self.command {
            Command::Exec(command) => command.run(),
            Command::Seed(command) => command.run(),
            Command::Shell(command) => command.run(),
        }
    }
}

pub(super) fn find_sink(
    sinks: &BTreeMap<crate::SinkName, crate::source::config::Sink>,
    name: Option<crate::SinkName>,
) -> Result<&crate::source::config::Sink, crate::Error> {
    find_sink_with_name(sinks, name).map(|(_, sink)| sink)
}

pub(super) fn find_sink_with_name(
    sinks: &BTreeMap<crate::SinkName, crate::source::config::Sink>,
    name: Option<crate::SinkName>,
) -> Result<(&crate::SinkName, &crate::source::config::Sink), crate::Error> {
    match name {
        Some(name) => sinks
            .get_key_value(&name)
            .ok_or(crate::Error::SinkNotFound(name)),
        None => match sinks.len() {
            0 => Err(crate::Error::NoSinksConfigured),
            1 => Ok(sinks.iter().next().unwrap()),
            _ => Err(crate::Error::MultipleSinksRequireName),
        },
    }
}

pub(super) struct ControlSocket {
    path: PathBuf,
    host: String,
}

impl ControlSocket {
    pub(super) fn new(sink: &crate::source::config::Sink) -> Result<Self, crate::Error> {
        let host = format!("{}@{}", sink.user, sink.host);

        let path = dirs::runtime_dir()
            .ok_or(crate::Error::RuntimeDirNotFound)?
            .join(format!("swaybridge-{host}.sock"));

        Ok(Self { path, host })
    }

    #[allow(dead_code)]
    pub(super) fn host(&self) -> &str {
        &self.host
    }

    pub(super) fn swaybridge_command(&self, home: &str) -> cmd_proc::Command {
        let bin_path = format!("{}/.local/bin/swaybridge", home);
        self.ssh_command().argument(bin_path)
    }

    pub(super) fn remote_home(&self, user: &crate::UserName) -> Result<String, crate::Error> {
        let output = output_with_stderr_inherit(
            self.ssh_command()
                .argument("getent")
                .argument("passwd")
                .argument(user.as_str()),
        )?;

        if !output.success() {
            let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
            return Err(crate::Error::RemoteHomeLookupFailed {
                details: format!(
                    "getent passwd {} exited {}: {}",
                    user, output.status, stderr
                ),
            });
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let line = stdout.lines().next().ok_or_else(|| {
            crate::Error::RemoteHomeLookupFailed {
                details: format!(
                    "getent passwd {} returned empty output",
                    user
                ),
            }
        })?;

        let fields: Vec<&str> = line.split(':').collect();
        if fields.len() < 6 {
            return Err(crate::Error::RemoteHomeLookupFailed {
                details: format!(
                    "getent passwd {} returned invalid line: {}",
                    user, line
                ),
            });
        }

        Ok(fields[5].to_string())
    }

    pub(super) fn ssh_command(&self) -> cmd_proc::Command {
        cmd_proc::Command::new("ssh")
            .option("-o", "ControlMaster=auto")
            .option("-o", format!("ControlPath={}", self.path.display()))
            .option("-o", "ControlPersist=30")
            .argument(&self.host)
    }

    #[allow(dead_code)]
    pub(super) fn scp_command(&self) -> cmd_proc::Command {
        cmd_proc::Command::new("scp")
            .option("-o", format!("ControlPath={}", self.path.display()))
    }
}

pub(super) fn output_with_stderr_inherit(
    command: cmd_proc::Command,
) -> Result<cmd_proc::Output, cmd_proc::CommandError> {
    command
        .spawn()
        .stdout(cmd_proc::Stdio::Piped)
        .stderr(cmd_proc::Stdio::Inherit)
        .run()?
        .wait_with_output()
}

pub(super) fn output_with_stderr_null(
    command: cmd_proc::Command,
) -> Result<cmd_proc::Output, cmd_proc::CommandError> {
    command
        .spawn()
        .stdout(cmd_proc::Stdio::Piped)
        .stderr(cmd_proc::Stdio::Null)
        .run()?
        .wait_with_output()
}
