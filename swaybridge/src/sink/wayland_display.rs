/// Wayland display name (e.g. `wayland-1`), suitable for `WAYLAND_DISPLAY`.
pub struct WaylandDisplay(String);

impl WaylandDisplay {
    /// Discover Wayland display from `$XDG_RUNTIME_DIR`.
    ///
    /// Looks for files matching `wayland-*` (excluding `.lock` files) in the runtime directory.
    pub fn from_runtime_dir() -> Result<Self, crate::Error> {
        let runtime_dir =
            dirs::runtime_dir().ok_or(crate::Error::RuntimeDirNotFound)?;

        let mut sockets: Vec<String> = std::fs::read_dir(&runtime_dir)
            .map_err(crate::Error::RuntimeDirRead)?
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let name = entry.file_name();
                let name = name.to_str()?;

                if name.starts_with("wayland-") && !name.ends_with(".lock") {
                    Some(name.to_string())
                } else {
                    None
                }
            })
            .collect();

        match sockets.len() {
            0 => Err(crate::Error::WaylandDisplayNotFound),
            1 => Ok(Self(sockets.remove(0))),
            _ => Err(crate::Error::MultipleWaylandDisplaysFound),
        }
    }

    /// The socket name, suitable for `WAYLAND_DISPLAY`.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for WaylandDisplay {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.0)
    }
}
