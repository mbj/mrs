/// Disconnect from sink and stop streaming
#[derive(Debug, clap::Parser)]
pub struct App {
    /// Sink name (required if multiple sinks configured)
    #[clap(long)]
    name: Option<crate::SinkName>,

    /// Sink output to disconnect (required if multiple outputs configured on sink)
    #[clap(long)]
    output: Option<crate::OutputName>,

    /// Virtual output name to remove (e.g., HEADLESS-1)
    #[clap(long)]
    virtual_output: crate::OutputName,
}

impl App {
    pub fn run(self) -> Result<(), crate::Error> {
        let config = crate::source::config::Source::load_default()?;
        let (sink_name, sink) = super::sink::find_sink_with_name(&config.sinks, self.name)?;

        let control = super::sink::ControlSocket::new(sink)?;

        let remote_home = control.remote_home(&sink.user)?;

        // Determine which sink output to stop
        let sink_output_name = match self.output {
            Some(name) => name,
            None => {
                // Query sink to find the output
                let sink_info = query_sink_info(&control, &remote_home)?;
                match sink_info.outputs.len() {
                    0 => return Err(crate::Error::NoOutputsOnSink),
                    1 => sink_info.outputs[0].name.parse()?,
                    _ => return Err(crate::Error::MultipleOutputsRequireName),
                }
            }
        };

        println!("Disconnecting from {}:{}...", sink_name, sink_output_name);

        let mut errors = Vec::new();

        // Stop wf-recorder locally
        let recorder_unit = crate::source::SystemdUnit::new(&self.virtual_output);
        match recorder_unit.stop().output() {
            Ok(output) => {
                if output.success() {
                    println!("Stopped {}", recorder_unit);
                } else {
                    errors.push(format!(
                        "systemctl --user stop {} exited {}",
                        recorder_unit, output.status
                    ));
                }
            }
            Err(err) => {
                errors.push(format!(
                    "systemctl --user stop {} failed: {}",
                    recorder_unit, err
                ));
            }
        }

        // Stop ffplay on sink via SSH
        let stop_result = control
            .swaybridge_command(&remote_home)
            .argument("sink")
            .argument("stop")
            .argument(sink_output_name.as_str())
            .output();

        match stop_result {
            Ok(output) => {
                if output.success() {
                    println!("Stopped ffplay on sink");
                } else {
                    errors.push(format!(
                        "ssh {}: swaybridge sink stop {} exited {}",
                        sink_name, sink_output_name, output.status
                    ));
                }
            }
            Err(err) => {
                errors.push(format!(
                    "ssh {}: swaybridge sink stop {} failed: {}",
                    sink_name, sink_output_name, err
                ));
            }
        }

        // Remove virtual output
        let unplug_result = cmd_proc::Command::new("swaymsg")
            .argument(format!("output {} unplug", self.virtual_output))
            .output();

        match unplug_result {
            Ok(output) => {
                if output.success() {
                    println!("Removed virtual output {}", self.virtual_output);
                } else {
                    errors.push(format!(
                        "swaymsg output {} unplug exited {}",
                        self.virtual_output, output.status
                    ));
                }
            }
            Err(err) => {
                errors.push(format!(
                    "swaymsg output {} unplug failed: {}",
                    self.virtual_output, err
                ));
            }
        }

        if errors.is_empty() {
            println!("Disconnected.");
            Ok(())
        } else {
            Err(crate::Error::CleanupFailed {
                details: errors.join("; "),
            })
        }
    }
}

fn query_sink_info(
    control: &super::sink::ControlSocket,
    remote_home: &str,
) -> Result<SinkInfo, crate::Error> {
    let output = super::sink::output_with_stderr_null(
        control
            .swaybridge_command(remote_home)
            .argument("sink")
            .argument("info")
            .argument("--json"),
    )?;

    if !output.success() {
        return Err(crate::Error::SshFailed(output.status));
    }

    serde_json::from_slice(&output.stdout).map_err(crate::Error::SinkInfoParse)
}

#[derive(serde::Deserialize)]
struct SinkInfo {
    outputs: Vec<SinkOutput>,
}

#[derive(serde::Deserialize)]
struct SinkOutput {
    name: String,
}
