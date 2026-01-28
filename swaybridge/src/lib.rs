mod config;
mod output_name;
mod sink_name;
mod user_name;

pub mod sink;
pub mod source;

pub use config::ConfigError;
pub use output_name::{OutputName, OutputNameError};
pub use sink_name::{SinkName, SinkNameError};
pub use user_name::{UserName, UserNameError};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Cannot determine config directory")]
    ConfigDirNotFound,
    #[error(transparent)]
    Config(#[from] ConfigError),
    #[error(transparent)]
    Command(#[from] cmd_proc::CommandError),
    #[error("Cannot determine current executable path")]
    CurrentExeNotFound(#[source] std::io::Error),
    #[error("Sink not found: {0}")]
    SinkNotFound(SinkName),
    #[error("No sinks configured")]
    NoSinksConfigured,
    #[error("Multiple sinks configured, specify a name")]
    MultipleSinksRequireName,
    #[error("Cannot determine runtime directory")]
    RuntimeDirNotFound,
    #[error("Failed to read runtime directory")]
    RuntimeDirRead(#[source] std::io::Error),
    #[error("Sway IPC socket not found")]
    SwaySocketNotFound,
    #[error("Multiple Sway IPC sockets found: {0}")]
    MultipleSwaySocketsFound(String),
    #[error("Output not configured: {0}")]
    OutputNotConfigured(OutputName),
    #[error(transparent)]
    OutputName(#[from] OutputNameError),
    #[error("swaymsg exited with {0}")]
    SwaymsgExit(std::process::ExitStatus),
    #[error("Failed to parse swaymsg output")]
    SwaymsgParse(#[source] serde_json::Error),
    #[error("Virtual output was not created")]
    VirtualOutputNotCreated,
    #[error("Multiple virtual outputs created unexpectedly")]
    MultipleVirtualOutputsCreated,
    #[error("Output not found: {0}")]
    OutputNotFound(OutputName),
    #[error("No outputs configured on sink")]
    NoOutputsOnSink,
    #[error("Multiple outputs on sink, specify --output")]
    MultipleOutputsRequireName,
    #[error("SSH command failed with {0}")]
    SshFailed(std::process::ExitStatus),
    #[error("Failed to parse sink info")]
    SinkInfoParse(#[source] serde_json::Error),
    #[error("ffplay exited unexpectedly with {0}")]
    FfplayExitedUnexpectedly(std::process::ExitStatus),
    #[error("ffplay window did not appear in time")]
    FfplayWindowTimeout,
    #[error("Wayland display not found")]
    WaylandDisplayNotFound,
    #[error("Multiple Wayland displays found")]
    MultipleWaylandDisplaysFound,
    #[error("systemctl stop failed for {unit} with {status}")]
    SystemdStopFailed {
        unit: String,
        status: std::process::ExitStatus,
    },
    #[error("SSH stop failed for {sink}:{output} with {status}")]
    SshStopFailed {
        sink: SinkName,
        output: OutputName,
        status: std::process::ExitStatus,
    },
    #[error("swaymsg unplug failed for {output} with {status}")]
    SwaymsgUnplugFailed {
        output: OutputName,
        status: std::process::ExitStatus,
    },
    #[error("Cleanup encountered errors: {details}")]
    CleanupFailed {
        details: String,
    },
    #[error("Status encountered errors: {details}")]
    StatusFailed {
        details: String,
    },
    #[error("Failed to resolve remote home: {details}")]
    RemoteHomeLookupFailed {
        details: String,
    },
}
