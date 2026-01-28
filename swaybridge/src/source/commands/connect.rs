use std::collections::BTreeSet;

/// Connect to sink and start streaming
#[derive(Debug, clap::Parser)]
pub struct App {
    /// Sink name (required if multiple sinks configured)
    #[clap(long)]
    name: Option<crate::SinkName>,

    /// Sink output to stream (required if multiple outputs configured on sink)
    #[clap(long)]
    output: Option<crate::OutputName>,

    /// Deploy swaybridge binary to sink before connecting
    #[clap(long)]
    seed: bool,

    // Recorder and UDP settings are currently hardcoded.

    // Intentionally no CLI overrides for codec settings.
}

impl App {
    pub fn run(self) -> Result<(), crate::Error> {
        let config = crate::source::config::Source::load_default()?;
        let (sink_name, sink) = super::sink::find_sink_with_name(&config.sinks, self.name)?;

        let control = super::sink::ControlSocket::new(sink)?;

        let remote_home = control.remote_home(&sink.user)?;

        if self.seed {
            println!("Deploying swaybridge to sink...");
            super::sink::seed::run(&control, &remote_home)?;
        }

        // Query sink capabilities
        let sink_info = query_sink_info(&control, &remote_home)?;
        let sink_output = find_sink_output(&sink_info.outputs, self.output)?;

        println!(
            "Connecting to {}:{} ({}x{} on port {})",
            sink_name, sink_output.name, sink_output.width, sink_output.height, sink_output.port
        );

        // Get positioning config for this sink output (optional)
        let position_config = sink.outputs.get(&sink_output.name.parse()?);

        // Query existing outputs on source
        let outputs_before = query_local_outputs()?;

        // Create virtual output
        swaymsg_command().argument("create_output").status()?;

        // Find the new output (may take a moment to appear)
        let virtual_output = {
            let start = std::time::Instant::now();
            let mut last_log = start;

            loop {
                let outputs_after = query_local_outputs()?;
                let new_outputs: Vec<_> =
                    outputs_after.difference(&outputs_before).collect();

                match new_outputs.len() {
                    0 => {
                        if start.elapsed() >= std::time::Duration::from_secs(5) {
                            return Err(crate::Error::VirtualOutputNotCreated);
                        }

                        if last_log.elapsed() >= std::time::Duration::from_millis(500) {
                            println!("Waiting for virtual output to appear...");
                            last_log = std::time::Instant::now();
                        }

                        std::thread::sleep(std::time::Duration::from_millis(100));
                    }
                    1 => break new_outputs[0].clone(),
                    _ => return Err(crate::Error::MultipleVirtualOutputsCreated),
                }
            }
        };

        println!("Created virtual output: {}", virtual_output);

        // Configure resolution
        swaymsg_command()
            .argument(format!(
                "output {} resolution {}x{}",
                virtual_output, sink_output.width, sink_output.height
            ))
            .status()?;

        // Configure position if specified
        if let Some(pos_config) = position_config {
            if let Some((x, y)) = calculate_position(
                &pos_config.relative_to,
                pos_config.position,
                sink_output.width,
                sink_output.height,
            )? {
                swaymsg_command()
                    .argument(format!("output {} pos {} {}", virtual_output, x, y))
                    .status()?;

                println!(
                    "Positioned {} {:?} of {}",
                    virtual_output, pos_config.position, pos_config.relative_to
                );
            }
        }

        // Start ffplay on sink via SSH
        println!("Starting ffplay on sink...");
        let mut sink_command = control
            .swaybridge_command(&remote_home)
            .argument("sink")
            .argument("run")
            .argument(sink_output.name.as_str());

        // Use pinned FIFO size for receiver.
        sink_command = sink_command
            .argument("--udp-fifo-size")
            .argument("100000");

        sink_command.status()?;

        // Start wf-recorder locally
        let recorder_unit = crate::source::SystemdUnit::new(&virtual_output.parse()?);
        let output_uri = build_udp_uri(
            &sink.host.to_string(),
            sink_output.port,
            Some(1316),
            Some(65536),
        );

        println!("Starting wf-recorder for {}...", virtual_output);
        let recorder_options = crate::source::RecorderOptions {
            no_damage: true,
            codec: Some("h265_vaapi"),
            codec_options: Some(
                "b=20M,maxrate=20M,bufsize=20M,g=15,keyint_min=15,scenecut=0,bf=0,rc-lookahead=0",
            ),
            muxer: "mpegts",
        };

        recorder_unit
            .start_wf_recorder(&virtual_output.parse()?, &output_uri, &recorder_options)
            .status()?;

        println!("Streaming started. Use 'swaybridge source disconnect' to stop.");

        Ok(())
    }
}

fn swaymsg_command() -> cmd_proc::Command {
    cmd_proc::Command::new("swaymsg")
}

fn query_local_outputs() -> Result<BTreeSet<String>, crate::Error> {
    let output = swaymsg_command()
        .argument("--raw")
        .argument("--type")
        .argument("get_outputs")
        .output()?;

    if !output.success() {
        return Err(crate::Error::SwaymsgExit(output.status));
    }

    let entries: Vec<LocalOutput> =
        serde_json::from_slice(&output.stdout).map_err(crate::Error::SwaymsgParse)?;

    Ok(entries.into_iter().map(|e| e.name).collect())
}

#[derive(serde::Deserialize)]
struct LocalOutput {
    name: String,
    rect: LocalRect,
}

#[derive(serde::Deserialize)]
struct LocalRect {
    x: i32,
    y: i32,
    width: u32,
    height: u32,
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

fn build_udp_uri(
    host: &str,
    port: u16,
    pkt_size: Option<u32>,
    buffer_size: Option<u32>,
) -> String {
    let mut uri = format!("udp://{}:{}", host, port);
    let mut query = Vec::new();

    if let Some(pkt_size) = pkt_size {
        query.push(format!("pkt_size={pkt_size}"));
    }

    if let Some(buffer_size) = buffer_size {
        query.push(format!("buffer_size={buffer_size}"));
    }

    if !query.is_empty() {
        uri.push('?');
        uri.push_str(&query.join("&"));
    }

    uri
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

fn find_sink_output(
    outputs: &[SinkOutput],
    name: Option<crate::OutputName>,
) -> Result<&SinkOutput, crate::Error> {
    match name {
        Some(name) => outputs
            .iter()
            .find(|o| o.name == name.as_str())
            .ok_or_else(|| crate::Error::OutputNotConfigured(name)),
        None => match outputs.len() {
            0 => Err(crate::Error::NoOutputsOnSink),
            1 => Ok(&outputs[0]),
            _ => Err(crate::Error::MultipleOutputsRequireName),
        },
    }
}

fn calculate_position(
    relative_to: &crate::OutputName,
    position: crate::source::config::Position,
    width: u32,
    height: u32,
) -> Result<Option<(i32, i32)>, crate::Error> {
    let output = swaymsg_command()
        .argument("--raw")
        .argument("--type")
        .argument("get_outputs")
        .output()?;

    if !output.success() {
        return Err(crate::Error::SwaymsgExit(output.status));
    }

    let entries: Vec<LocalOutput> =
        serde_json::from_slice(&output.stdout).map_err(crate::Error::SwaymsgParse)?;

    let reference = entries
        .iter()
        .find(|o| o.name == relative_to.as_str())
        .ok_or_else(|| crate::Error::OutputNotFound(relative_to.clone()))?;

    let (x, y) = match position {
        crate::source::config::Position::Left => (reference.rect.x - width as i32, reference.rect.y),
        crate::source::config::Position::Right => {
            (reference.rect.x + reference.rect.width as i32, reference.rect.y)
        }
        crate::source::config::Position::Above => {
            (reference.rect.x, reference.rect.y - height as i32)
        }
        crate::source::config::Position::Below => (
            reference.rect.x,
            reference.rect.y + reference.rect.height as i32,
        ),
    };

    Ok(Some((x, y)))
}
