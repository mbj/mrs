pub(crate) mod ruby;

#[derive(Debug, clap::Parser)]
pub(crate) enum Command {
    /// Ruby gem management commands
    Ruby {
        #[clap(subcommand)]
        command: ruby::Command,
    },
}

impl Command {
    pub(crate) async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Self::Ruby { command } => command.run().await,
        }
    }
}

use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CpuArchitecture {
    X86_64,
    Aarch64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OperatingSystem {
    Linux,
    Darwin,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Libc {
    Musl,
    None,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Platform(CpuArchitecture, OperatingSystem, Libc);

impl Platform {
    pub(crate) const ALL: &[Platform] = &[
        Platform(CpuArchitecture::X86_64, OperatingSystem::Linux, Libc::Musl),
        Platform(CpuArchitecture::Aarch64, OperatingSystem::Linux, Libc::Musl),
        Platform(
            CpuArchitecture::Aarch64,
            OperatingSystem::Darwin,
            Libc::None,
        ),
    ];

    pub(crate) fn rust_target(self) -> &'static str {
        match self {
            Platform(CpuArchitecture::X86_64, OperatingSystem::Linux, Libc::Musl) => {
                "x86_64-unknown-linux-musl"
            }
            Platform(CpuArchitecture::Aarch64, OperatingSystem::Linux, Libc::Musl) => {
                "aarch64-unknown-linux-musl"
            }
            Platform(CpuArchitecture::Aarch64, OperatingSystem::Darwin, Libc::None) => {
                "aarch64-apple-darwin"
            }
            _ => panic!("Unsupported platform: {self:?}"),
        }
    }

    pub(crate) fn ruby_platform(self) -> &'static str {
        match self {
            Platform(CpuArchitecture::X86_64, OperatingSystem::Linux, Libc::Musl) => "x86_64-linux",
            Platform(CpuArchitecture::Aarch64, OperatingSystem::Linux, Libc::Musl) => {
                "aarch64-linux"
            }
            Platform(CpuArchitecture::Aarch64, OperatingSystem::Darwin, Libc::None) => {
                "arm64-darwin"
            }
            _ => panic!("Unsupported platform: {self:?}"),
        }
    }

    fn from_rust_target(target: &str) -> Option<Platform> {
        Platform::ALL
            .iter()
            .find(|p| p.rust_target() == target)
            .copied()
    }

    fn is_macos(self) -> bool {
        self.1 == OperatingSystem::Darwin
    }
}

pub(crate) fn verify_and_collect_file(path: PathBuf) -> PathBuf {
    if !path.exists() {
        panic!("Expected file not found: {}", path.display());
    }
    log::info!("Found: {}", path.display());
    path
}

fn detect_target_platform() -> Platform {
    let target_str = std::env::var("CARGO_BUILD_TARGET").unwrap_or_else(|_| {
        match (std::env::consts::ARCH, std::env::consts::OS) {
            ("x86_64", "linux") => "x86_64-unknown-linux-musl".to_string(),
            ("aarch64", "linux") => "aarch64-unknown-linux-musl".to_string(),
            ("x86_64", "macos") => "x86_64-apple-darwin".to_string(),
            ("aarch64", "macos") => "aarch64-apple-darwin".to_string(),
            (arch, os) => panic!("Unsupported platform: {arch}-{os}"),
        }
    });

    Platform::from_rust_target(&target_str)
        .unwrap_or_else(|| panic!("Unsupported target: {target_str}"))
}

fn copy_dir_recursive(source: &PathBuf, destination: &PathBuf) -> std::io::Result<()> {
    std::fs::create_dir_all(destination)?;
    for entry in std::fs::read_dir(source)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let source_path = entry.path();
        let destination_path = destination.join(entry.file_name());

        if file_type.is_dir() {
            copy_dir_recursive(&source_path, &destination_path)?;
        } else {
            std::fs::copy(&source_path, &destination_path)?;
        }
    }
    Ok(())
}

enum StagingItem {
    CopyFile {
        source: PathBuf,
        destination: String,
    },
    CopyDirectory {
        source: PathBuf,
        destination: String,
    },
    GenerateFile {
        destination: String,
        content: String,
    },
}

fn setup_staging_directory(staging_root: &PathBuf, items: Vec<StagingItem>) {
    log::info!(
        "Creating build staging directory: {}",
        staging_root.display()
    );
    std::fs::create_dir_all(staging_root)
        .unwrap_or_else(|error| panic!("Failed to create staging directory: {error}"));

    for item in items {
        match item {
            StagingItem::CopyFile {
                source,
                destination,
            } => {
                log::info!("Copying file: {destination}");
                let destination_path = staging_root.join(&destination);
                if let Some(parent) = destination_path.parent() {
                    std::fs::create_dir_all(parent).unwrap_or_else(|error| {
                        panic!("Failed to create directory for {destination}: {error}")
                    });
                }
                std::fs::copy(&source, &destination_path)
                    .unwrap_or_else(|error| panic!("Failed to copy {destination}: {error}"));
            }
            StagingItem::CopyDirectory {
                source,
                destination,
            } => {
                log::info!("Copying directory: {destination}");
                let destination_path = staging_root.join(&destination);
                copy_dir_recursive(&source, &destination_path).unwrap_or_else(|error| {
                    panic!("Failed to copy directory {destination}: {error}")
                });
            }
            StagingItem::GenerateFile {
                destination,
                content,
            } => {
                log::info!("Generating file: {destination}");
                let destination_path = staging_root.join(&destination);
                std::fs::write(&destination_path, content)
                    .unwrap_or_else(|error| panic!("Failed to write {destination}: {error}"));
            }
        }
    }
}
