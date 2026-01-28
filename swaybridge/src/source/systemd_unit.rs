/// A systemd user scope unit for managing wf-recorder processes.
pub struct SystemdUnit(String);

impl SystemdUnit {
    /// Create a new unit name for the given virtual output.
    #[must_use]
    pub fn new(output_name: &crate::OutputName) -> Self {
        Self(format!("swaybridge-recorder-{output_name}"))
    }

    /// Build a command to start wf-recorder in a new scope.
    #[must_use]
    pub fn start_wf_recorder(
        &self,
        output_name: &crate::OutputName,
        output_uri: &str,
        options: &RecorderOptions<'_>,
    ) -> cmd_proc::Command {
        let mut command = cmd_proc::Command::new("systemd-run")
            .argument("--user")
            .argument("--scope")
            .option("--unit", &self.0)
            .argument("--")
            .argument("wf-recorder");

        if options.no_damage {
            command = command.argument("--no-damage");
        }

        if let Some(codec) = options.codec {
            command = command.argument("--codec").argument(codec);
        }

        if let Some(codec_options) = options.codec_options {
            command = command.argument("--codec-options").argument(codec_options);
        }

        command
            .option("--output", output_name.as_str())
            .option("--muxer", options.muxer)
            .option("--file", output_uri)
    }

    /// Build a command to stop the scope.
    #[must_use]
    pub fn stop(&self) -> cmd_proc::Command {
        cmd_proc::Command::new("systemctl")
            .argument("--user")
            .argument("stop")
            .argument(&self.0)
    }

    /// Build a command to check if the scope is active.
    #[must_use]
    pub fn is_active(&self) -> cmd_proc::Command {
        cmd_proc::Command::new("systemctl")
            .argument("--user")
            .argument("is-active")
            .argument("--quiet")
            .argument(&self.0)
    }
}

impl std::fmt::Display for SystemdUnit {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

pub struct RecorderOptions<'a> {
    pub no_damage: bool,
    pub codec: Option<&'a str>,
    pub codec_options: Option<&'a str>,
    pub muxer: &'a str,
}
