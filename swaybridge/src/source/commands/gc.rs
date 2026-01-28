use std::collections::BTreeSet;

/// Garbage-collect leaked outputs and recorder/playback scopes
#[derive(Debug, clap::Parser)]
pub struct App {
    /// Sink name (required if multiple sinks configured)
    #[clap(long)]
    name: Option<crate::SinkName>,
}

impl App {
    pub fn run(self) -> Result<(), crate::Error> {
        let mut errors = Vec::new();

        gc_local(&mut errors);

        let config = crate::source::config::Source::load_default()?;
        let (sink_name, sink) = super::sink::find_sink_with_name(&config.sinks, self.name)?;
        let control = super::sink::ControlSocket::new(sink)?;

        gc_remote(&control, sink_name, &mut errors);

        if errors.is_empty() {
            println!("GC complete.");
            Ok(())
        } else {
            Err(crate::Error::CleanupFailed {
                details: errors.join("; "),
            })
        }
    }
}

fn gc_local(errors: &mut Vec<String>) {
    println!("GC: stopping local recorder scopes...");
    stop_systemd_units_local("swaybridge-recorder-*", errors);

    println!("GC: unplugging virtual outputs...");
    unplug_virtual_outputs(errors);
}

fn gc_remote(
    control: &super::sink::ControlSocket,
    sink_name: &crate::SinkName,
    errors: &mut Vec<String>,
) {
    println!("GC: stopping remote playback scopes on {sink_name}...");
    stop_systemd_units_remote(control, sink_name, "swaybridge-output-*", errors);
}

fn stop_systemd_units_local(pattern: &str, errors: &mut Vec<String>) {
    let units = match list_systemd_units_local(pattern) {
        Ok(units) => units,
        Err(message) => {
            errors.push(message);
            return;
        }
    };

    if units.is_empty() {
        println!("GC: no local units matched {pattern}");
        return;
    }

    for unit in units {
        match cmd_proc::Command::new("systemctl")
            .argument("--user")
            .argument("stop")
            .argument(&unit)
            .output()
        {
            Ok(status) => {
                if status.success() {
                    println!("GC: stopped {unit}");
                } else {
                    errors.push(format!(
                        "systemctl --user stop {unit} exited {}",
                        status.status
                    ));
                }
            }
            Err(err) => {
                errors.push(format!(
                    "systemctl --user stop {unit} failed: {err}"
                ));
            }
        }
    }
}

fn stop_systemd_units_remote(
    control: &super::sink::ControlSocket,
    sink_name: &crate::SinkName,
    pattern: &str,
    errors: &mut Vec<String>,
) {
    let units = match list_systemd_units_remote(control, sink_name, pattern) {
        Ok(units) => units,
        Err(message) => {
            errors.push(message);
            return;
        }
    };

    if units.is_empty() {
        println!("GC: no remote units matched {pattern}");
        return;
    }

    for unit in units {
        match super::sink::output_with_stderr_inherit(
            control
                .ssh_command()
                .argument("systemctl")
                .argument("--user")
                .argument("stop")
                .argument(&unit),
        )
        {
            Ok(status) => {
                if status.success() {
                    println!("GC: stopped remote {unit}");
                } else {
                    errors.push(format!(
                        "ssh {sink_name}: systemctl --user stop {unit} exited {}",
                        status.status
                    ));
                }
            }
            Err(err) => {
                errors.push(format!(
                    "ssh {sink_name}: systemctl --user stop {unit} failed: {err}"
                ));
            }
        }
    }
}

fn list_systemd_units_local(pattern: &str) -> Result<Vec<String>, String> {
    let output = cmd_proc::Command::new("systemctl")
        .argument("--user")
        .argument("list-units")
        .argument("--all")
        .argument("--no-legend")
        .argument("--no-pager")
        .argument(pattern)
        .output()
        .map_err(|err| format!("systemctl --user list-units {pattern} failed: {err}"))?;

    if !output.success() {
        return Err(format!(
            "systemctl --user list-units {pattern} exited {}",
            output.status
        ));
    }

    Ok(parse_unit_names(&output.stdout))
}

fn list_systemd_units_remote(
    control: &super::sink::ControlSocket,
    sink_name: &crate::SinkName,
    pattern: &str,
) -> Result<Vec<String>, String> {
    let output = super::sink::output_with_stderr_inherit(
        control
            .ssh_command()
            .argument("systemctl")
            .argument("--user")
            .argument("list-units")
            .argument("--all")
            .argument("--no-legend")
            .argument("--no-pager")
            .argument(pattern),
    )
    .map_err(|err| {
        format!("ssh {sink_name}: systemctl --user list-units {pattern} failed: {err}")
    })?;

    if !output.success() {
        return Err(format!(
            "ssh {sink_name}: systemctl --user list-units {pattern} exited {}",
            output.status
        ));
    }

    Ok(parse_unit_names(&output.stdout))
}

fn parse_unit_names(stdout: &[u8]) -> Vec<String> {
    let output = String::from_utf8_lossy(stdout);

    output
        .lines()
        .filter_map(|line| line.split_whitespace().next())
        .map(|unit| unit.to_string())
        .collect()
}

fn unplug_virtual_outputs(errors: &mut Vec<String>) {
    let outputs = match query_local_outputs() {
        Ok(outputs) => outputs,
        Err(message) => {
            errors.push(message);
            return;
        }
    };

    let virtual_outputs: Vec<_> = outputs
        .into_iter()
        .filter(|output| output.starts_with("HEADLESS-"))
        .collect();

    if virtual_outputs.is_empty() {
        println!("GC: no virtual outputs found");
        return;
    }

    for output in virtual_outputs {
        match cmd_proc::Command::new("swaymsg")
            .argument(format!("output {output} unplug"))
            .output()
        {
            Ok(status) => {
                if status.success() {
                    println!("GC: unplugged {output}");
                } else {
                    errors.push(format!(
                        "swaymsg output {output} unplug exited {}",
                        status.status
                    ));
                }
            }
            Err(err) => {
                errors.push(format!(
                    "swaymsg output {output} unplug failed: {err}"
                ));
            }
        }
    }
}

fn query_local_outputs() -> Result<BTreeSet<String>, String> {
    let output = cmd_proc::Command::new("swaymsg")
        .argument("--raw")
        .argument("--type")
        .argument("get_outputs")
        .output()
        .map_err(|err| format!("swaymsg get_outputs failed: {err}"))?;

    if !output.success() {
        return Err(format!("swaymsg get_outputs exited {}", output.status));
    }

    let entries: Vec<LocalOutput> =
        serde_json::from_slice(&output.stdout)
            .map_err(|err| format!("Failed to parse swaymsg output: {err}"))?;

    Ok(entries.into_iter().map(|entry| entry.name).collect())
}

#[derive(serde::Deserialize)]
struct LocalOutput {
    name: String,
}
