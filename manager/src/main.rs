use flate2::{Compression, write::GzEncoder};
use sha2::{Digest, Sha256};
use std::path::PathBuf;

#[derive(Debug, clap::Parser)]
struct App {
    #[clap(subcommand)]
    command: AppCommand,
}

#[derive(Debug, clap::Parser)]
enum AppCommand {
    /// Integrations management commands
    Integrations {
        #[clap(subcommand)]
        command: IntegrationsCommand,
    },
}

#[derive(Debug, clap::Parser)]
enum IntegrationsCommand {
    /// Build integrations for current architecture
    Build,
    /// Test integrations locally with Ruby 3.4
    TestLocal,
    /// Test integrations in CI environment
    TestCi,
}

impl App {
    fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        match &self.command {
            AppCommand::Integrations { command } => command.run(),
        }
    }
}

impl IntegrationsCommand {
    fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Self::Build => {
                build_integrations();
                Ok(())
            }
            Self::TestLocal => {
                test_local();
                Ok(())
            }
            Self::TestCi => {
                test_ci();
                Ok(())
            }
        }
    }
}

fn build_integrations() {
    let target = std::env::var("CARGO_BUILD_TARGET").unwrap_or_else(|_| {
        match (std::env::consts::ARCH, std::env::consts::OS) {
            ("x86_64", "linux") => "x86_64-unknown-linux-musl".to_string(),
            ("aarch64", "linux") => "aarch64-unknown-linux-musl".to_string(),
            ("x86_64", "macos") => "x86_64-apple-darwin".to_string(),
            ("aarch64", "macos") => "aarch64-apple-darwin".to_string(),
            (arch, os) => panic!(
                "Unsupported platform for integration builds: {}-{}",
                arch, os
            ),
        }
    });

    log::info!("Building pg-ephemeral binary for target: {}", target);

    let status = cbt::Command::new("cargo")
        .arguments([
            "build",
            "--release",
            "--package",
            "pg-ephemeral",
            "--target",
            &target,
        ])
        .status_result()
        .unwrap_or_else(|error| panic!("Failed to execute cargo build: {}", error));

    if !status.success() {
        panic!("Failed to build pg-ephemeral binary");
    }

    let workspace_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf();
    let binary_source = workspace_root
        .join("target")
        .join(&target)
        .join("release")
        .join("pg-ephemeral");
    let integration_directory = workspace_root
        .join("pg-ephemeral")
        .join("integrations")
        .join("ruby");
    let binary_directory = integration_directory.join("bin");
    let binary_destination = binary_directory.join("pg-ephemeral");

    log::info!("Creating bin directory");
    std::fs::create_dir_all(&binary_directory).expect("Failed to create bin directory");

    log::info!("Copying binary to integration bin directory");
    std::fs::copy(&binary_source, &binary_destination).expect("Failed to copy binary");

    log::info!("Binary copied to: {}", binary_destination.display());

    let license_source = workspace_root.join("LICENSE.txt");
    let license_destination = integration_directory.join("LICENSE.txt");

    log::info!("Copying LICENSE.txt to integration directory");
    std::fs::copy(&license_source, &license_destination).expect("Failed to copy LICENSE.txt");

    log::info!("Building gem");

    // On macOS, use native gem command as Docker is not available on GitHub Actions runners
    // On Linux, use containerized build for consistency
    if target.contains("darwin") {
        log::info!("Using native gem build for macOS");
        let status = cbt::Command::new("gem")
            .argument("build")
            .argument("pg-ephemeral.gemspec")
            .working_directory(&integration_directory)
            .status_result()
            .unwrap_or_else(|error| panic!("Failed to execute gem build: {}", error));

        if !status.success() {
            panic!("Failed to build gem");
        }
    } else {
        log::info!("Using containerized gem build for Linux");
        let backend = cbt::backend::autodetect::run().expect("Failed to detect backend");

        let mount = cbt::Mount::from(format!(
            "type=bind,source={},target=/build",
            integration_directory.to_str().unwrap()
        ));

        cbt::Definition::new(
            backend,
            cbt::Image::from("docker.io/library/ruby:3.4-alpine"),
        )
        .mount(mount)
        .workdir("/build")
        .entrypoint("gem")
        .arguments(["build", "pg-ephemeral.gemspec"])
        .remove()
        .run_status_success();
    }

    // Copy gem to release directory
    log::info!("Copying gem to release artifacts");
    let release_dir = workspace_root.join("target").join(&target).join("release");
    std::fs::create_dir_all(&release_dir)
        .unwrap_or_else(|error| panic!("Failed to create release directory: {}", error));

    let gem_name = format!("pg-ephemeral-{}.gem", pg_ephemeral::VERSION);
    let gem_source = integration_directory.join(&gem_name);
    let gem_destination = release_dir.join(&gem_name);

    std::fs::copy(&gem_source, &gem_destination)
        .unwrap_or_else(|error| panic!("Failed to copy gem to release directory: {}", error));

    log::info!("Gem copied to: {}", gem_destination.display());

    // Create SHA256 hash for gem
    let gem_bytes = std::fs::read(&gem_destination)
        .unwrap_or_else(|error| panic!("Failed to read gem: {}", error));
    let mut hasher = Sha256::new();
    hasher.update(&gem_bytes);
    let hash = hasher.finalize();
    let gem_hash_string = format!("{:x}  {}\n", hash, gem_name);
    let gem_sha256_path = release_dir.join(format!("{}.sha256", gem_name));

    std::fs::write(&gem_sha256_path, gem_hash_string)
        .unwrap_or_else(|error| panic!("Failed to write gem SHA256 file: {}", error));

    log::info!("Gem SHA256 hash written to: {}", gem_sha256_path.display());

    log::info!("Creating tarball and SHA256 hash");
    let tarball_name = format!("pg-ephemeral-{}.tar.gz", target);
    let tarball_path = release_dir.join(&tarball_name);
    let sha256_path = release_dir.join(format!("{}.sha256", tarball_name));

    let tarball_file = std::fs::File::create(&tarball_path).expect("Failed to create tarball file");
    let encoder = GzEncoder::new(tarball_file, Compression::default());
    let mut archive = tar::Builder::new(encoder);

    archive
        .append_path_with_name(&binary_source, "pg-ephemeral")
        .expect("Failed to add binary to archive");

    let encoder = archive.into_inner().expect("Failed to finish archive");
    encoder.finish().expect("Failed to finish gzip compression");

    log::info!("Tarball created at: {}", tarball_path.display());

    let tarball_bytes = std::fs::read(&tarball_path).expect("Failed to read tarball");
    let mut hasher = Sha256::new();
    hasher.update(&tarball_bytes);
    let hash = hasher.finalize();
    let hash_string = format!("{:x}  {}\n", hash, tarball_name);

    std::fs::write(&sha256_path, hash_string).expect("Failed to write SHA256 file");

    log::info!("SHA256 hash written to: {}", sha256_path.display());
    log::info!("Integrations build complete");
}

fn test_local() {
    log::info!("Running local Ruby integration tests");

    let workspace_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf();

    let binary_path = workspace_root
        .join("pg-ephemeral")
        .join("integrations")
        .join("ruby")
        .join("bin")
        .join("pg-ephemeral");
    let test_script = workspace_root
        .join("pg-ephemeral")
        .join("integrations")
        .join("ruby")
        .join("test.rb");

    log::info!("Getting pg-ephemeral version");
    let version_output = cbt::Command::new(&binary_path)
        .argument("--version")
        .output_result()
        .unwrap_or_else(|error| {
            panic!("Failed to get version from pg-ephemeral binary: {}", error)
        });

    if !version_output.status.success() {
        panic!("Failed to get version from pg-ephemeral binary");
    }

    let version_string =
        String::from_utf8(version_output.stdout).expect("Invalid UTF-8 in version output");

    let version = version_string
        .strip_prefix("pg-ephemeral ")
        .and_then(|s| s.strip_suffix('\n'))
        .expect("Unexpected version format");

    log::info!("Using pg-ephemeral version: {}", version);

    let status = cbt::Command::new("zsh")
        .argument("-c")
        .argument(format!(
            "source ~/.zshrc && chruby ruby-3.4 && {}",
            test_script.display()
        ))
        .env("EXPECTED_PG_EPHEMERAL_VERSION", version)
        .status_result()
        .unwrap_or_else(|error| panic!("Failed to run test script: {}", error));

    if !status.success() {
        panic!("Integration tests failed");
    }

    log::info!("Local integration tests complete");
}

fn test_ci() {
    log::info!("Running CI Ruby acceptance tests");

    // In CI, the manager binary is run from the workspace root after checkout
    let workspace_root = std::env::current_dir()
        .unwrap_or_else(|error| panic!("Failed to get current directory: {}", error));

    let integration_directory = workspace_root
        .join("pg-ephemeral")
        .join("integrations")
        .join("ruby");
    let version = pg_ephemeral::VERSION;

    log::info!("Using pg-ephemeral version: {}", version);

    log::info!("Running bundle install with Gemfile.acceptance");
    let bundle_install = cbt::Command::new("bundle")
        .arguments(["install", "--gemfile=Gemfile.acceptance"])
        .working_directory(&integration_directory)
        .status_result()
        .unwrap_or_else(|error| panic!("Failed to run bundle install: {}", error));

    if !bundle_install.success() {
        panic!("bundle install failed");
    }

    log::info!("Running RSpec acceptance tests");
    let rspec_status = cbt::Command::new("bundle")
        .arguments([
            "exec",
            "--gemfile=Gemfile.acceptance",
            "rspec",
            "spec/integration",
        ])
        .working_directory(&integration_directory)
        .env("EXPECTED_PG_EPHEMERAL_VERSION", version)
        .status_result()
        .unwrap_or_else(|error| panic!("Failed to run RSpec tests: {}", error));

    if !rspec_status.success() {
        panic!("RSpec acceptance tests failed");
    }

    // Extract pg-ephemeral binary after acceptance tests complete
    // Determine target platform (CI runs on Linux x86_64)
    let target = "x86_64-unknown-linux-musl";

    log::info!("Extracting pg-ephemeral binary from tarball");
    let tarball_path = workspace_root
        .join("target")
        .join(target)
        .join("release")
        .join(format!("pg-ephemeral-{}.tar.gz", target));

    let bin_directory = integration_directory.join("bin");
    let binary_destination = bin_directory.join("pg-ephemeral");

    // Create bin directory
    std::fs::create_dir_all(&bin_directory)
        .unwrap_or_else(|error| panic!("Failed to create bin directory: {}", error));

    // Extract tarball
    let tarball_file = std::fs::File::open(&tarball_path).unwrap_or_else(|error| {
        panic!(
            "Failed to open tarball {}: {}",
            tarball_path.display(),
            error
        )
    });
    let decoder = flate2::read::GzDecoder::new(tarball_file);
    let mut archive = tar::Archive::new(decoder);

    for entry in archive
        .entries()
        .unwrap_or_else(|error| panic!("Failed to read tarball entries: {}", error))
    {
        let mut entry =
            entry.unwrap_or_else(|error| panic!("Failed to read tarball entry: {}", error));

        // Extract to bin directory
        entry
            .unpack(&binary_destination)
            .unwrap_or_else(|error| panic!("Failed to extract binary: {}", error));
    }

    // Make binary executable
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(&binary_destination)
            .unwrap_or_else(|error| panic!("Failed to get binary metadata: {}", error))
            .permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&binary_destination, perms)
            .unwrap_or_else(|error| panic!("Failed to set binary permissions: {}", error));
    }

    log::info!("Binary extracted to: {}", binary_destination.display());

    log::info!("Running bundle install for Mutant");
    let bundle_install_mutant = cbt::Command::new("bundle")
        .arguments(["install"])
        .working_directory(&integration_directory)
        .status_result()
        .unwrap_or_else(|error| panic!("Failed to run bundle install: {}", error));

    if !bundle_install_mutant.success() {
        panic!("bundle install failed");
    }

    log::info!("Running Mutant tests");
    let mutant_status = cbt::Command::new("bundle")
        .arguments(["exec", "mutant", "run"])
        .working_directory(&integration_directory)
        .env("EXPECTED_PG_EPHEMERAL_VERSION", version)
        .status_result()
        .unwrap_or_else(|error| panic!("Failed to run Mutant tests: {}", error));

    if !mutant_status.success() {
        panic!("Mutant tests failed");
    }

    log::info!("CI acceptance tests complete");
}

fn main() {
    env_logger::init();

    let app = <App as clap::Parser>::parse();

    if let Err(error) = app.run() {
        log::error!("{}", error);
        std::process::exit(1);
    }
}
