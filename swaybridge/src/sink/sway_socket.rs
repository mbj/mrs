use std::path::PathBuf;

/// Path to a running Sway IPC socket.
pub struct SwaySocket(PathBuf);

impl SwaySocket {
    /// Find the Sway IPC socket in `$XDG_RUNTIME_DIR`.
    ///
    /// Looks for files matching `sway-ipc.*.sock` in the runtime directory.
    pub fn find() -> Result<Self, crate::Error> {
        let runtime_dir =
            dirs::runtime_dir().ok_or(crate::Error::RuntimeDirNotFound)?;

        let sockets: Vec<PathBuf> = std::fs::read_dir(&runtime_dir)
            .map_err(crate::Error::RuntimeDirRead)?
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let name = entry.file_name();
                let name = name.to_str()?;

                if name.starts_with("sway-ipc.") && name.ends_with(".sock") {
                    Some(entry.path())
                } else {
                    None
                }
            })
            .collect();

        let mut active_sockets: Vec<PathBuf> = sockets
            .into_iter()
            .filter(|path| is_active_socket(path))
            .collect();

        match active_sockets.len() {
            0 => Err(crate::Error::SwaySocketNotFound),
            1 => Ok(Self(active_sockets.remove(0))),
            _ => {
                let list = active_sockets
                    .iter()
                    .map(|path| path.display().to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                Err(crate::Error::MultipleSwaySocketsFound(list))
            }
        }
    }

    #[must_use]
    pub fn path(&self) -> &std::path::Path {
        &self.0
    }

    /// Build a `swaymsg` command targeting this socket.
    #[must_use]
    pub fn swaymsg_command(&self) -> cmd_proc::Command {
        cmd_proc::Command::new("swaymsg")
            .argument("--socket")
            .argument(&self.0)
    }

    /// Query Sway for all outputs via `swaymsg`.
    pub fn get_outputs(&self) -> Result<Vec<Output>, crate::Error> {
        let output = self
            .swaymsg_command()
            .argument("--type")
            .argument("get_outputs")
            .argument("--raw")
            .output()?;

        if !output.success() {
            return Err(crate::Error::SwaymsgExit(output.status));
        }

        let entries: Vec<SwaymsgOutput> =
            serde_json::from_slice(&output.stdout)
                .map_err(crate::Error::SwaymsgParse)?;

        entries
            .into_iter()
            .map(|entry| {
                Ok(Output {
                    name: entry.name.parse().map_err(crate::Error::OutputName)?,
                    current_mode: entry.current_mode.map(|mode| Mode {
                        width: mode.width,
                        height: mode.height,
                        refresh: mode.refresh,
                    }),
                    transform: entry.transform,
                })
            })
            .collect()
    }
}

pub struct Output {
    pub name: crate::OutputName,
    pub current_mode: Option<Mode>,
    pub transform: Transform,
}

#[derive(Clone, Copy)]
pub struct Mode {
    pub width: u32,
    pub height: u32,
    /// Refresh rate in millihertz (e.g. 60000 = 60Hz).
    pub refresh: u32,
}

impl Mode {
    #[must_use]
    pub fn refresh_hz(&self) -> f64 {
        f64::from(self.refresh) / 1000.0
    }
}

impl std::fmt::Display for Mode {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "{}x{}@{:.3}Hz",
            self.width,
            self.height,
            self.refresh_hz(),
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize)]
pub enum Transform {
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "90")]
    Rotate90,
    #[serde(rename = "180")]
    Rotate180,
    #[serde(rename = "270")]
    Rotate270,
    #[serde(rename = "flipped")]
    Flipped,
    #[serde(rename = "flipped-90")]
    Flipped90,
    #[serde(rename = "flipped-180")]
    Flipped180,
    #[serde(rename = "flipped-270")]
    Flipped270,
}

impl std::fmt::Display for Transform {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Normal => write!(formatter, "normal"),
            Self::Rotate90 => write!(formatter, "90"),
            Self::Rotate180 => write!(formatter, "180"),
            Self::Rotate270 => write!(formatter, "270"),
            Self::Flipped => write!(formatter, "flipped"),
            Self::Flipped90 => write!(formatter, "flipped-90"),
            Self::Flipped180 => write!(formatter, "flipped-180"),
            Self::Flipped270 => write!(formatter, "flipped-270"),
        }
    }
}

#[derive(serde::Deserialize)]
struct SwaymsgOutput {
    name: String,
    current_mode: Option<SwaymsgMode>,
    transform: Transform,
}

#[derive(serde::Deserialize)]
struct SwaymsgMode {
    width: u32,
    height: u32,
    refresh: u32,
}

impl std::fmt::Display for SwaySocket {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.0.display())
    }
}

fn is_active_socket(path: &std::path::Path) -> bool {
    cmd_proc::Command::new("swaymsg")
        .argument("--socket")
        .argument(path)
        .argument("--type")
        .argument("get_version")
        .spawn()
        .stdout(cmd_proc::Stdio::Null)
        .stderr(cmd_proc::Stdio::Null)
        .run()
        .and_then(|child| child.wait())
        .map(|status| status.success())
        .unwrap_or(false)
}
