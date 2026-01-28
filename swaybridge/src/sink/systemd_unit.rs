/// A systemd user scope unit for managing an output's ffplay process.
pub struct SystemdUnit(String);

impl SystemdUnit {
    /// Create a new unit name for the given output.
    #[must_use]
    pub fn new(output_name: &crate::OutputName) -> Self {
        Self(format!("swaybridge-output-{output_name}"))
    }

    /// Build a command to start ffplay in a new scope.
    #[must_use]
    pub fn start_ffplay(
        &self,
        wayland_display: &crate::sink::WaylandDisplay,
        window_title: &str,
        args: &[String],
        input_uri: &str,
    ) -> cmd_proc::Command {
        cmd_proc::Command::new("systemd-run")
            .argument("--user")
            .argument("--scope")
            .option("--unit", &self.0)
            .option("--setenv", format!("WAYLAND_DISPLAY={}", wayland_display.name()))
            .argument("--")
            .argument("ffplay")
            .argument("-window_title")
            .argument(window_title)
            .arguments(args)
            .argument(input_uri)
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
