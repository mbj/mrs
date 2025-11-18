use flate2::{Compression, write::GzEncoder};
use indoc::formatdoc;
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
    /// Release management commands
    Release {
        #[clap(subcommand)]
        command: ReleaseCommand,
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

#[derive(Debug, clap::Parser)]
enum ReleaseCommand {
    /// Create GitHub edge release with all built artifacts
    CreateEdge,
}

impl App {
    fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        match &self.command {
            AppCommand::Integrations { command } => command.run(),
            AppCommand::Release { command } => command.run(),
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

impl ReleaseCommand {
    fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Self::CreateEdge => {
                create_edge_release();
                Ok(())
            }
        }
    }
}

fn verify_and_collect_file(path: PathBuf) -> PathBuf {
    if !path.exists() {
        panic!("Expected file not found: {}", path.display());
    }
    log::info!("Found: {}", path.display());
    path
}

fn create_edge_release() {
    log::info!("Creating edge release");

    let workspace_root = std::env::current_dir()
        .unwrap_or_else(|error| panic!("Failed to get current directory: {}", error));

    // Get git commit SHA
    let sha_output = cbt::Command::new("git")
        .arguments(["rev-parse", "HEAD"])
        .output_result()
        .unwrap_or_else(|error| panic!("Failed to get git SHA: {}", error));

    if !sha_output.status.success() {
        panic!("Failed to get git SHA");
    }

    let sha = String::from_utf8(sha_output.stdout)
        .unwrap_or_else(|error| panic!("Invalid UTF-8 in git SHA: {}", error))
        .trim()
        .to_string();

    // Get current branch name
    let branch_output = cbt::Command::new("git")
        .arguments(["rev-parse", "--abbrev-ref", "HEAD"])
        .output_result()
        .unwrap_or_else(|error| panic!("Failed to get git branch: {}", error));

    if !branch_output.status.success() {
        panic!("Failed to get git branch");
    }

    let branch = String::from_utf8(branch_output.stdout)
        .unwrap_or_else(|error| panic!("Invalid UTF-8 in branch name: {}", error))
        .trim()
        .to_string();

    let tag = format!("edge-{}", sha);
    let title = format!("Edge Build ({} @ {})", branch, sha);
    let notes = format!("Automated edge build from commit {}", sha);

    log::info!("Tag: {}", tag);
    log::info!("Title: {}", title);

    let version = pg_ephemeral::VERSION;

    // Expected platforms that should have built gems
    let platforms = [
        ("x86_64-unknown-linux-musl", "x86_64-linux"),
        ("aarch64-unknown-linux-musl", "aarch64-linux"),
        ("aarch64-apple-darwin", "aarch64-darwin"),
    ];

    let mut release_files = Vec::new();

    // Verify and collect all expected artifacts
    for (rust_target, ruby_platform) in &platforms {
        let artifact_base = workspace_root
            .join("artifacts")
            .join(format!("pg-ephemeral-{}", rust_target));

        let gem_base = artifact_base.join("dist").join("gems");
        let binary_base = artifact_base.join("dist").join("binaries");

        let gem_name = format!("pg-ephemeral-{}-{}.gem", version, ruby_platform);
        let tarball_name = format!("pg-ephemeral-{}.tar.gz", rust_target);

        release_files.push(verify_and_collect_file(gem_base.join(&gem_name)));
        release_files.push(verify_and_collect_file(
            gem_base.join(format!("{}.sha256", gem_name)),
        ));
        release_files.push(verify_and_collect_file(binary_base.join(&tarball_name)));
        release_files.push(verify_and_collect_file(
            binary_base.join(format!("{}.sha256", tarball_name)),
        ));
    }

    log::info!(
        "All expected artifacts found ({} files)",
        release_files.len()
    );

    // Build gh release create command
    let mut arguments = vec![
        "release".to_string(),
        "create".to_string(),
        tag.clone(),
        "--prerelease".to_string(),
        "--title".to_string(),
        title,
        "--notes".to_string(),
        notes,
    ];

    // Add all release files
    for file in &release_files {
        arguments.push(file.to_str().unwrap().to_string());
    }

    log::info!(
        "Creating GitHub release with {} artifacts",
        release_files.len()
    );

    let status = cbt::Command::new("gh")
        .arguments(
            arguments
                .iter()
                .map(|argument| argument.as_str())
                .collect::<Vec<_>>(),
        )
        .status_result()
        .unwrap_or_else(|error| panic!("Failed to execute gh command: {}", error));

    if !status.success() {
        panic!("Failed to create GitHub release");
    }

    log::info!("Successfully created edge release: {}", tag);
}

fn rust_target_to_ruby_platform(rust_target: &str) -> &str {
    match rust_target {
        "x86_64-unknown-linux-musl" => "x86_64-linux",
        "aarch64-unknown-linux-musl" => "aarch64-linux",
        "x86_64-apple-darwin" => "x86_64-darwin",
        "aarch64-apple-darwin" => "aarch64-darwin",
        _ => panic!(
            "Unsupported Rust target for Ruby platform mapping: {}",
            rust_target
        ),
    }
}

fn generate_gemspec(version: &str, ruby_platform: &str) -> String {
    formatdoc! {"
        Gem::Specification.new do |spec|
          spec.name          = 'pg-ephemeral'
          spec.version       = '{version}'
          spec.platform      = Gem::Platform.new('{ruby_platform}')
          spec.authors       = ['Markus Schirp']
          spec.email         = ['mbj@schirp-dso.com']

          spec.summary       = 'Ruby wrapper for pg-ephemeral PostgreSQL testing utility'
          spec.description   = 'Provides ephemeral PostgreSQL instances for testing, wrapping the pg-ephemeral project binary'
          spec.homepage      = 'https://github.com/mbj/mrs/tree/main/pg-ephemeral'
          spec.license       = 'MIT'
          spec.required_ruby_version = '>= 3.2'

          spec.metadata['homepage_uri'] = spec.homepage
          spec.metadata['source_code_uri'] = 'https://github.com/mbj/mrs'
          spec.metadata['changelog_uri'] = 'https://github.com/mbj/mrs/blob/main/pg-ephemeral/CHANGELOG.md'

          spec.files = Dir['lib/**/*', 'bin/*', 'README.md', 'LICENSE.txt']
          spec.require_paths = ['lib']

          spec.add_dependency 'pg', '~> 1.5'
          spec.add_dependency 'unparser', '~> 0.8.0'

          spec.add_development_dependency 'bundler', '~> 2.0'
          spec.add_development_dependency 'mutant-rspec', '~> 0.13.0'
          spec.add_development_dependency 'rspec', '~> 3.0'
        end
    "}
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
        .unwrap_or_else(|error| panic!("Failed to create staging directory: {}", error));

    for item in items {
        match item {
            StagingItem::CopyFile {
                source,
                destination,
            } => {
                log::info!("Copying file: {}", destination);
                let destination_path = staging_root.join(&destination);
                if let Some(parent) = destination_path.parent() {
                    std::fs::create_dir_all(parent).unwrap_or_else(|error| {
                        panic!("Failed to create directory for {}: {}", destination, error)
                    });
                }
                std::fs::copy(&source, &destination_path)
                    .unwrap_or_else(|error| panic!("Failed to copy {}: {}", destination, error));
            }
            StagingItem::CopyDirectory {
                source,
                destination,
            } => {
                log::info!("Copying directory: {}", destination);
                let destination_path = staging_root.join(&destination);
                copy_dir_recursive(&source, &destination_path).unwrap_or_else(|error| {
                    panic!("Failed to copy directory {}: {}", destination, error)
                });
            }
            StagingItem::GenerateFile {
                destination,
                content,
            } => {
                log::info!("Generating file: {}", destination);
                let destination_path = staging_root.join(&destination);
                std::fs::write(&destination_path, content)
                    .unwrap_or_else(|error| panic!("Failed to write {}: {}", destination, error));
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

    let ruby_platform = rust_target_to_ruby_platform(&target);
    let version = pg_ephemeral::VERSION;

    log::info!("Building pg-ephemeral binary for target: {}", target);
    log::info!("Ruby platform: {}", ruby_platform);
    log::info!("Version: {}", version);

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

    // Paths for Rust artifacts
    let binary_source = workspace_root
        .join("target")
        .join(&target)
        .join("release")
        .join("pg-ephemeral");

    // Paths for Ruby integration source
    let integration_source = workspace_root
        .join("pg-ephemeral")
        .join("integrations")
        .join("ruby");

    // Paths for build staging directory
    let build_staging = workspace_root
        .join("pg-ephemeral")
        .join("build")
        .join(&target);

    // Setup staging directory with all required files
    let gemspec_content = generate_gemspec(version, ruby_platform);

    setup_staging_directory(
        &build_staging,
        vec![
            StagingItem::CopyDirectory {
                source: integration_source.join("lib"),
                destination: "lib".to_string(),
            },
            StagingItem::CopyDirectory {
                source: integration_source.join("spec"),
                destination: "spec".to_string(),
            },
            StagingItem::CopyFile {
                source: binary_source.clone(),
                destination: "bin/pg-ephemeral".to_string(),
            },
            StagingItem::CopyFile {
                source: workspace_root.join("LICENSE.txt"),
                destination: "LICENSE.txt".to_string(),
            },
            StagingItem::GenerateFile {
                destination: "pg-ephemeral.gemspec".to_string(),
                content: gemspec_content,
            },
        ],
    );

    // Build gem
    log::info!("Building gem");
    if target.contains("darwin") {
        log::info!("Using native gem build for macOS");
        let status = cbt::Command::new("gem")
            .argument("build")
            .argument("pg-ephemeral.gemspec")
            .working_directory(&build_staging)
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
            build_staging.to_str().unwrap()
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

    // Create dist directories
    let dist_gems = workspace_root.join("dist").join("gems");
    let dist_binaries = workspace_root.join("dist").join("binaries");
    std::fs::create_dir_all(&dist_gems)
        .unwrap_or_else(|error| panic!("Failed to create dist/gems directory: {}", error));
    std::fs::create_dir_all(&dist_binaries)
        .unwrap_or_else(|error| panic!("Failed to create dist/binaries directory: {}", error));

    // Copy gem to dist
    let gem_filename = format!("pg-ephemeral-{}-{}.gem", version, ruby_platform);
    let gem_source = build_staging.join(&gem_filename);
    let gem_dest = dist_gems.join(&gem_filename);

    log::info!("Copying gem to dist: {}", gem_dest.display());
    std::fs::copy(&gem_source, &gem_dest)
        .unwrap_or_else(|error| panic!("Failed to copy gem to dist: {}", error));

    // Create SHA256 hash for gem
    log::info!("Creating SHA256 hash for gem");
    let gem_bytes =
        std::fs::read(&gem_dest).unwrap_or_else(|error| panic!("Failed to read gem: {}", error));
    let mut hasher = Sha256::new();
    hasher.update(&gem_bytes);
    let hash = hasher.finalize();
    let gem_hash_string = format!("{:x}  {}\n", hash, gem_filename);
    let gem_sha256_path = dist_gems.join(format!("{}.sha256", gem_filename));
    std::fs::write(&gem_sha256_path, gem_hash_string)
        .unwrap_or_else(|error| panic!("Failed to write gem SHA256 file: {}", error));

    log::info!("Gem SHA256 hash written to: {}", gem_sha256_path.display());

    // Create tarball
    log::info!("Creating tarball");
    let tarball_name = format!("pg-ephemeral-{}.tar.gz", target);
    let tarball_path = dist_binaries.join(&tarball_name);

    let tarball_file = std::fs::File::create(&tarball_path)
        .unwrap_or_else(|error| panic!("Failed to create tarball file: {}", error));
    let encoder = GzEncoder::new(tarball_file, Compression::default());
    let mut archive = tar::Builder::new(encoder);

    archive
        .append_path_with_name(&binary_source, "pg-ephemeral")
        .expect("Failed to add binary to archive");

    let encoder = archive.into_inner().expect("Failed to finish archive");
    encoder.finish().expect("Failed to finish gzip compression");

    log::info!("Tarball created at: {}", tarball_path.display());

    // Create SHA256 hash for tarball
    log::info!("Creating SHA256 hash for tarball");
    let tarball_bytes = std::fs::read(&tarball_path)
        .unwrap_or_else(|error| panic!("Failed to read tarball: {}", error));
    let mut hasher = Sha256::new();
    hasher.update(&tarball_bytes);
    let hash = hasher.finalize();
    let hash_string = format!("{:x}  {}\n", hash, tarball_name);
    let tarball_sha256_path = dist_binaries.join(format!("{}.sha256", tarball_name));
    std::fs::write(&tarball_sha256_path, hash_string)
        .unwrap_or_else(|error| panic!("Failed to write tarball SHA256 file: {}", error));

    log::info!(
        "Tarball SHA256 hash written to: {}",
        tarball_sha256_path.display()
    );
    log::info!("Integrations build complete");
}

fn test_local() {
    log::info!("Running local Ruby integration tests");

    let workspace_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf();

    // Determine the local target platform
    let target = match (std::env::consts::ARCH, std::env::consts::OS) {
        ("x86_64", "linux") => "x86_64-unknown-linux-musl",
        ("aarch64", "linux") => "aarch64-unknown-linux-musl",
        ("x86_64", "macos") => "x86_64-apple-darwin",
        ("aarch64", "macos") => "aarch64-apple-darwin",
        (arch, os) => panic!("Unsupported platform: {}-{}", arch, os),
    };

    let build_staging = workspace_root
        .join("pg-ephemeral")
        .join("build")
        .join(target);

    let binary_path = build_staging.join("bin").join("pg-ephemeral");
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

    // Extract pg-ephemeral binary to integration directory for mutant tests
    // Determine target platform (CI runs on Linux x86_64)
    let target = "x86_64-unknown-linux-musl";

    log::info!("Extracting pg-ephemeral binary from tarball for mutant tests");
    let tarball_path = workspace_root
        .join("dist")
        .join("binaries")
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
