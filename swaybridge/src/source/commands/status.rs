use std::collections::BTreeSet;

/// Show source and sink status with remote validation
#[derive(Debug, clap::Parser)]
pub struct App {}

impl App {
    pub fn run(self) -> Result<(), crate::Error> {
        let config = crate::source::config::Source::load_default()?;
        let mut errors = Vec::new();
        let local_outputs = match query_local_outputs() {
            Ok(outputs) => Some(outputs),
            Err(message) => {
                errors.push(message);
                None
            }
        };

        let mut sink_rows = Vec::new();
        let mut output_rows = Vec::new();
        let mut warnings = Vec::new();

        for (sink_name, sink) in &config.sinks {
            let control = super::sink::ControlSocket::new(sink)?;
            let remote_home = match control.remote_home(&sink.user) {
                Ok(home) => home,
                Err(err) => {
                    errors.push(format!(
                        "ssh {sink_name}: failed to resolve home directory: {err}"
                    ));
                    sink_rows.push(SinkRow {
                        name: sink_name.to_string(),
                        host: sink.host.to_string(),
                        user: sink.user.to_string(),
                        reachable: "no".to_string(),
                        outputs: "-".to_string(),
                    });
                    continue;
                }
            };

            match query_sink_info(&control, &remote_home) {
                Ok(info) => {
                    let mut active_count = 0usize;
                    for output in &info.outputs {
                        let unit = format!("swaybridge-output-{}", output.name);
                        let status =
                            remote_unit_state(&control, &unit, sink_name, &mut errors);

                        if status == "active" {
                            active_count += 1;
                        }

                        output_rows.push(OutputRow {
                            sink: sink_name.to_string(),
                            output: output.name.clone(),
                            mode: format!("{}x{}", output.width, output.height),
                            port: output.port.to_string(),
                            ffplay: status,
                        });
                    }

                    sink_rows.push(SinkRow {
                        name: sink_name.to_string(),
                        host: sink.host.to_string(),
                        user: sink.user.to_string(),
                        reachable: "yes".to_string(),
                        outputs: format!("{} total / {} active", info.outputs.len(), active_count),
                    });

                    if let Some(local_outputs) = &local_outputs {
                        validate_source_outputs(
                            sink_name,
                            sink,
                            &info.outputs,
                            local_outputs,
                            &mut warnings,
                        );
                    }
                }
                Err(err) => {
                    errors.push(format!(
                        "ssh {sink_name}: failed to query sink info: {err}"
                    ));
                    sink_rows.push(SinkRow {
                        name: sink_name.to_string(),
                        host: sink.host.to_string(),
                        user: sink.user.to_string(),
                        reachable: "no".to_string(),
                        outputs: "-".to_string(),
                    });
                }
            }
        }

        print_sink_table(&sink_rows);
        print_output_table(&output_rows);
        print_warnings(&warnings);

        if errors.is_empty() {
            Ok(())
        } else {
            Err(crate::Error::StatusFailed {
                details: errors.join("; "),
            })
        }
    }
}

fn validate_source_outputs(
    sink_name: &crate::SinkName,
    sink: &crate::source::config::Sink,
    outputs: &[SinkOutput],
    local_outputs: &BTreeSet<String>,
    warnings: &mut Vec<String>,
) {
    let sink_outputs: BTreeSet<&str> = outputs.iter().map(|o| o.name.as_str()).collect();

    for (output_name, output_cfg) in &sink.outputs {
        if !sink_outputs.contains(output_name.as_str()) {
            warnings.push(format!(
                "Config: sink {sink_name} references output {output_name} not present on sink"
            ));
        }

        if !local_outputs.contains(output_cfg.relative_to.as_str()) {
            warnings.push(format!(
                "Config: sink {sink_name} output {output_name} relative_to {} not found locally",
                output_cfg.relative_to
            ));
        }
    }
}

fn print_sink_table(rows: &[SinkRow]) {
    let mut table = comfy_table::Table::new();

    table
        .load_preset(comfy_table::presets::NOTHING)
        .set_content_arrangement(comfy_table::ContentArrangement::Dynamic)
        .set_header(vec!["name", "host", "user", "reachable", "outputs"]);

    for row in rows {
        table.add_row(vec![
            row.name.as_str(),
            row.host.as_str(),
            row.user.as_str(),
            row.reachable.as_str(),
            row.outputs.as_str(),
        ]);
    }

    println!("{table}");
}

fn print_output_table(rows: &[OutputRow]) {
    if rows.is_empty() {
        return;
    }

    let mut table = comfy_table::Table::new();

    table
        .load_preset(comfy_table::presets::NOTHING)
        .set_content_arrangement(comfy_table::ContentArrangement::Dynamic)
        .set_header(vec!["sink", "output", "mode", "port", "ffplay"]);

    for row in rows {
        table.add_row(vec![
            row.sink.as_str(),
            row.output.as_str(),
            row.mode.as_str(),
            row.port.as_str(),
            row.ffplay.as_str(),
        ]);
    }

    println!("{table}");
}

fn print_warnings(warnings: &[String]) {
    if warnings.is_empty() {
        return;
    }

    println!("Warnings:");
    for warning in warnings {
        println!("  - {warning}");
    }
}

fn remote_unit_state(
    control: &super::sink::ControlSocket,
    unit: &str,
    sink_name: &crate::SinkName,
    errors: &mut Vec<String>,
) -> String {
    match super::sink::output_with_stderr_inherit(
        control
            .ssh_command()
            .argument("systemctl")
            .argument("--user")
            .argument("is-active")
            .argument("--quiet")
            .argument(unit),
    )
    {
        Ok(output) => {
            if output.success() {
                "active".to_string()
            } else {
                match output.status.code() {
                    Some(3) => "inactive".to_string(),
                    Some(4) => "not-found".to_string(),
                    Some(code) => {
                        errors.push(format!(
                            "ssh {sink_name}: systemctl --user is-active {unit} exited {code}"
                        ));
                        format!("failed({code})")
                    }
                    None => {
                        errors.push(format!(
                            "ssh {sink_name}: systemctl --user is-active {unit} terminated by signal"
                        ));
                        "failed".to_string()
                    }
                }
            }
        }
        Err(err) => {
            errors.push(format!(
                "ssh {sink_name}: systemctl --user is-active {unit} failed: {err}"
            ));
            "error".to_string()
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

#[derive(serde::Deserialize)]
struct SinkInfo {
    outputs: Vec<SinkOutput>,
}

#[derive(serde::Deserialize)]
struct SinkOutput {
    name: String,
    width: u32,
    height: u32,
    port: u16,
}

struct SinkRow {
    name: String,
    host: String,
    user: String,
    reachable: String,
    outputs: String,
}

struct OutputRow {
    sink: String,
    output: String,
    mode: String,
    port: String,
    ffplay: String,
}
