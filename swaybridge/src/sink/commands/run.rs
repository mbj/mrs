/// Run UDP receiver for a single output via systemd user scope.
///
/// Starts ffplay in a systemd scope, waits for the window to appear,
/// then moves it to the target output in fullscreen mode.
///
/// # Security
///
/// The SRT passphrase ends up in ffplay's command line as a URI parameter,
/// making it visible via `ps` and `/proc/<pid>/cmdline`. ffplay does not support
/// reading the passphrase from an environment variable or file.
/// Use network-level security (VPN/WireGuard) if this is a concern.
#[derive(Debug, clap::Parser)]
pub struct App {
    name: crate::OutputName,

    /// UDP FIFO size for ffplay (bytes)
    #[clap(long)]
    udp_fifo_size: Option<u32>,

    /// Enable ffplay hardware acceleration (e.g. vulkan, vaapi)
    #[clap(long)]
    hwaccel: Option<String>,

    /// Decoder to use (e.g. h264_vulkan, h264)
    #[clap(long)]
    decoder: Option<String>,

    /// Hardware accel output format (e.g. vulkan, vaapi)
    #[clap(long)]
    hwaccel_output_format: Option<String>,
}

impl App {
    pub fn run(self) -> Result<(), crate::Error> {
        let sway_socket = crate::sink::SwaySocket::find()?;
        let wayland_display = crate::sink::WaylandDisplay::from_runtime_dir()?;
        let sink_config = crate::sink::config::Sink::load_default()?;

        let output_config = sink_config
            .output
            .get(&self.name)
            .ok_or_else(|| crate::Error::OutputNotConfigured(self.name.clone()))?;

        let unit = crate::sink::SystemdUnit::new(&self.name);
        let title = format!("swaybridge:{}", self.name);
        let input_uri = build_udp_uri(
            "0.0.0.0",
            output_config.port.get(),
            self.udp_fifo_size,
        );

        let ffplay_args = build_ffplay_args(
            self.hwaccel.as_deref(),
            self.hwaccel_output_format.as_deref(),
            self.decoder.as_deref(),
        );

        // Start ffplay via systemd scope
        unit.start_ffplay(&wayland_display, &title, &ffplay_args, &input_uri)
            .status()?;

        // Wait for window to appear and move it to the correct output.
        let move_command = format!(
            "[title=\"{title}\"] move to output {}, fullscreen enable",
            self.name,
        );

        for _ in 0..50 {
            // Check if ffplay is still running
            let active = unit.is_active().output()?;

            if !active.success() {
                return Err(crate::Error::FfplayExitedUnexpectedly(active.status));
            }

            let result = sway_socket
                .swaymsg_command()
                .argument(&move_command)
                .output();

            if result.is_ok_and(|output| output.success()) {
                println!("Started {} on {}", unit, self.name);
                return Ok(());
            }

            std::thread::sleep(std::time::Duration::from_millis(100));
        }

        // Timed out waiting for window, stop the unit
        let _ = unit.stop().status();
        Err(crate::Error::FfplayWindowTimeout)
    }
}

fn build_ffplay_args(
    hwaccel: Option<&str>,
    hwaccel_output_format: Option<&str>,
    decoder: Option<&str>,
) -> Vec<String> {
    let mut args = Vec::new();

    // Low-latency defaults.
    args.extend([
        "-autoexit",
        "-fflags",
        "nobuffer",
        "-flags",
        "low_delay",
        "-framedrop",
        "-probesize",
        "32",
        "-analyzeduration",
        "0",
    ].iter().map(|value| (*value).to_string()));

    if let Some(hwaccel) = hwaccel {
        args.push("-hwaccel".to_string());
        args.push(hwaccel.to_string());
    }

    if let Some(format) = hwaccel_output_format {
        args.push("-hwaccel_output_format".to_string());
        args.push(format.to_string());
    }

    if let Some(decoder) = decoder {
        args.push("-vcodec".to_string());
        args.push(decoder.to_string());
    }

    args
}

fn build_udp_uri(host: &str, port: u16, fifo_size: Option<u32>) -> String {
    let mut uri = format!("udp://{}:{}", host, port);

    if let Some(fifo_size) = fifo_size {
        uri.push_str(&format!("?fifo_size={fifo_size}&overrun_nonfatal=1"));
    }

    uri
}
