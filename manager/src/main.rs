use flate2::{Compression, write::GzEncoder};
use indoc::formatdoc;
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};

fn ruby_version() -> String {
    let version = pg_ephemeral::version();
    let mut result = format!("{}.{}.{}", version.major, version.minor, version.patch);
    if !version.pre.is_empty() {
        result.push('.');
        result.push_str(&version.pre.to_string());
    }
    result
}

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
struct Platform(CpuArchitecture, OperatingSystem, Libc);

impl Platform {
    const ALL: &[Platform] = &[
        Platform(CpuArchitecture::X86_64, OperatingSystem::Linux, Libc::Musl),
        Platform(CpuArchitecture::Aarch64, OperatingSystem::Linux, Libc::Musl),
        Platform(
            CpuArchitecture::Aarch64,
            OperatingSystem::Darwin,
            Libc::None,
        ),
    ];

    fn rust_target(self) -> &'static str {
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

    fn ruby_platform(self) -> &'static str {
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
    Build {
        /// Skip compilation (use pre-built binaries)
        #[clap(long)]
        no_compile: bool,
    },
    /// Merge multi-platform gems into unified repository
    MergeGems,
    /// Test integrations (acceptance tests + unit tests + mutant)
    Test,
    /// Publish gems to RubyGems.org from GitHub artifacts
    Publish {
        /// Actually push gems to RubyGems.org (default is dry-run)
        #[clap(long)]
        push: bool,
    },
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
            Self::Build { no_compile } => {
                build_integrations(*no_compile);
                Ok(())
            }
            Self::MergeGems => {
                merge_gems();
                Ok(())
            }
            Self::Test => {
                test();
                Ok(())
            }
            Self::Publish { push } => {
                publish_gems(*push);
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

struct PlatformArtifactPaths {
    gem: PathBuf,
    gem_sha256: PathBuf,
    tarball: PathBuf,
    tarball_sha256: PathBuf,
}

fn platform_artifact_paths(
    workspace_root: &Path,
    rust_target: &str,
    ruby_platform: &str,
) -> PlatformArtifactPaths {
    let ruby_version = ruby_version();
    let artifact_base = workspace_root
        .join("artifacts")
        .join(format!("pg-ephemeral-{rust_target}"));

    let gem_base = artifact_base.join("dist").join("gems");
    let binary_base = artifact_base.join("dist").join("binaries");

    let gem_name = format!("pg-ephemeral-{ruby_version}-{ruby_platform}.gem");
    let tarball_name = format!("pg-ephemeral-{rust_target}.tar.gz");

    PlatformArtifactPaths {
        gem: gem_base.join(&gem_name),
        gem_sha256: gem_base.join(format!("{gem_name}.sha256")),
        tarball: binary_base.join(&tarball_name),
        tarball_sha256: binary_base.join(format!("{tarball_name}.sha256")),
    }
}

fn create_edge_release() {
    log::info!("Creating edge release");

    let workspace_root = std::env::current_dir()
        .unwrap_or_else(|error| panic!("Failed to get current directory: {error}"));

    // Get git commit SHA
    let sha = ociman::Command::new("git")
        .arguments(["rev-parse", "HEAD"])
        .stdout()
        .string()
        .unwrap_or_else(|error| panic!("Failed to get git SHA: {error}"))
        .trim()
        .to_string();

    // Get current branch name
    let branch = ociman::Command::new("git")
        .arguments(["rev-parse", "--abbrev-ref", "HEAD"])
        .stdout()
        .string()
        .unwrap_or_else(|error| panic!("Failed to get git branch: {error}"))
        .trim()
        .to_string();

    let tag = format!("edge-{sha}");
    let title = format!("Edge Build ({branch} @ {sha})");
    let notes = format!("Automated edge build from commit {sha}");

    log::info!("Tag: {tag}");
    log::info!("Title: {title}");

    let mut release_files = Vec::new();

    // Verify and collect all expected artifacts
    for platform in Platform::ALL {
        let paths = platform_artifact_paths(
            &workspace_root,
            platform.rust_target(),
            platform.ruby_platform(),
        );

        release_files.push(verify_and_collect_file(paths.gem));
        release_files.push(verify_and_collect_file(paths.gem_sha256));
        release_files.push(verify_and_collect_file(paths.tarball));
        release_files.push(verify_and_collect_file(paths.tarball_sha256));
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

    ociman::Command::new("gh")
        .arguments(
            arguments
                .iter()
                .map(|argument| argument.as_str())
                .collect::<Vec<_>>(),
        )
        .status()
        .unwrap_or_else(|error| panic!("Failed to create GitHub release: {error}"));

    log::info!("Successfully created edge release: {tag}");
}

#[derive(serde::Serialize)]
struct GemspecConfig {
    version: String,
    ruby_platform: String,
    bin_files: Vec<String>,
}

fn collect_bin_files(build_staging: &Path, platform: Platform) -> Vec<String> {
    let mut bin_files = vec!["bin/pg-ephemeral".to_string()];

    if platform.is_macos() {
        // Recursively collect all files in the .dSYM bundle
        let dsym_dir = build_staging.join("bin/pg-ephemeral.dSYM");
        if dsym_dir.exists() {
            collect_files_recursive(&dsym_dir, build_staging, &mut bin_files);
        }
    }

    bin_files
}

fn collect_files_recursive(dir: &Path, base: &Path, files: &mut Vec<String>) {
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                if let Ok(relative) = path.strip_prefix(base) {
                    files.push(relative.to_string_lossy().to_string());
                }
            } else if path.is_dir() {
                collect_files_recursive(&path, base, files);
            }
        }
    }
}

fn gemspec_config(version: &str, platform: Platform, bin_files: Vec<String>) -> GemspecConfig {
    GemspecConfig {
        version: version.to_string(),
        ruby_platform: platform.ruby_platform().to_string(),
        bin_files,
    }
}

fn generate_gemspec() -> String {
    formatdoc! {"
        require 'json'

        config = JSON.parse(ENV.fetch('PG_EPHEMERAL_GEMSPEC_CONFIG'))

        Gem::Specification.new do |spec|
          spec.name          = 'pg-ephemeral'
          spec.version       = config.fetch('version')
          spec.platform      = Gem::Platform.new(config.fetch('ruby_platform'))
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

          spec.files = Dir['lib/**/*'] + config.fetch('bin_files') + ['LICENSE.txt']
          spec.require_paths = ['lib']

          spec.add_dependency 'pg', '~> 1.5'

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

fn build_integrations(no_compile: bool) {
    let platform = detect_target_platform();
    let rust_target = platform.rust_target();
    let ruby_platform = platform.ruby_platform();
    let ruby_version = ruby_version();

    log::info!("Building pg-ephemeral binary for target: {rust_target}");
    log::info!("Ruby platform: {ruby_platform}");
    log::info!("Version: {ruby_version}");

    if no_compile {
        log::info!("Skipping compilation (--no-compile flag set)");
    } else {
        ociman::Command::new("cargo")
            .arguments([
                "build",
                "--release",
                "--package",
                "pg-ephemeral",
                "--target",
                rust_target,
            ])
            .status()
            .unwrap_or_else(|error| panic!("Failed to build pg-ephemeral binary: {error}"));
    }

    let workspace_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf();

    // Paths for Rust artifacts
    let binary_source = workspace_root
        .join("target")
        .join(rust_target)
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
        .join(rust_target);

    // Setup staging directory with all required files
    let gemspec_content = generate_gemspec();

    let mut staging_items = vec![
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
    ];

    // On macOS, include the .dSYM bundle for debug symbols (required for backtraces with line numbers)
    if platform.is_macos() {
        let dsym_source = workspace_root
            .join("target")
            .join(rust_target)
            .join("release")
            .join("pg-ephemeral.dSYM");

        if dsym_source.exists() {
            log::info!("Including .dSYM debug symbols for macOS");
            staging_items.push(StagingItem::CopyDirectory {
                source: dsym_source,
                destination: "bin/pg-ephemeral.dSYM".to_string(),
            });
        } else {
            panic!(
                ".dSYM bundle not found at {}. Cannot build gem without debug symbols.",
                dsym_source.display()
            );
        }
    }

    setup_staging_directory(&build_staging, staging_items);

    // Collect actual bin files from staging directory and create gemspec config
    let bin_files = collect_bin_files(&build_staging, platform);
    let gemspec_config = gemspec_config(&ruby_version, platform, bin_files);
    let gemspec_config_json = serde_json::to_string(&gemspec_config)
        .unwrap_or_else(|error| panic!("Failed to serialize gemspec config: {error}"));

    // Build gem
    log::info!("Building gem");
    if platform.is_macos() {
        log::info!("Using native gem build for macOS");
        ociman::Command::new("gem")
            .argument("build")
            .argument("pg-ephemeral.gemspec")
            .working_directory(&build_staging)
            .env("PG_EPHEMERAL_GEMSPEC_CONFIG", &gemspec_config_json)
            .status()
            .unwrap_or_else(|error| panic!("Failed to build gem: {error}"));
    } else {
        log::info!("Using containerized gem build for Linux");
        let backend = ociman::backend::resolve::auto().expect("Failed to detect backend");

        let mount = ociman::Mount::from(format!(
            "type=bind,source={},target=/build",
            build_staging.to_str().unwrap()
        ));

        ociman::Definition::new(
            backend,
            "docker.io/library/ruby:3.4-alpine".parse().unwrap(),
        )
        .mount(mount)
        .workdir("/build")
        .environment_variable("PG_EPHEMERAL_GEMSPEC_CONFIG", &gemspec_config_json)
        .entrypoint("gem")
        .arguments(["build", "pg-ephemeral.gemspec"])
        .remove()
        .run()
        .unwrap_or_else(|error| panic!("Failed to build gem: {error}"));
    }

    // Create dist directories
    let dist_root = workspace_root.join("dist");
    let dist_gems = dist_root.join("gems");
    let dist_binaries = dist_root.join("binaries");
    std::fs::create_dir_all(&dist_gems)
        .unwrap_or_else(|error| panic!("Failed to create dist/gems directory: {error}"));
    std::fs::create_dir_all(&dist_binaries)
        .unwrap_or_else(|error| panic!("Failed to create dist/binaries directory: {error}"));

    // Prepare gem for distribution
    let gem_filename = format!("pg-ephemeral-{ruby_version}-{ruby_platform}.gem");
    let gem_source = build_staging.join(&gem_filename);

    // Create SHA256 hash for gem
    log::info!("Creating SHA256 hash for gem");
    let gem_bytes =
        std::fs::read(&gem_source).unwrap_or_else(|error| panic!("Failed to read gem: {error}"));
    let mut hasher = Sha256::new();
    hasher.update(&gem_bytes);
    let hash = hasher.finalize();
    let gem_hash_string = format!("{hash:x}  {gem_filename}\n");

    // Use staging to copy gem and write SHA256 to dist
    setup_staging_directory(
        &dist_gems,
        vec![
            StagingItem::CopyFile {
                source: gem_source,
                destination: gem_filename.clone(),
            },
            StagingItem::GenerateFile {
                destination: format!("{gem_filename}.sha256"),
                content: gem_hash_string,
            },
        ],
    );

    // Generate gem index for local gem source
    log::info!("Generating gem index in: {}", dist_root.display());
    ociman::Command::new("gem")
        .arguments(["generate_index", "--directory", dist_root.to_str().unwrap()])
        .status()
        .unwrap_or_else(|error| panic!("Failed to generate gem index: {error}"));

    log::info!("Gem index generated successfully");

    // Create tarball
    log::info!("Creating tarball");
    let tarball_name = format!("pg-ephemeral-{rust_target}.tar.gz");
    let tarball_path = dist_binaries.join(&tarball_name);

    let tarball_file = std::fs::File::create(&tarball_path)
        .unwrap_or_else(|error| panic!("Failed to create tarball file: {error}"));
    let encoder = GzEncoder::new(tarball_file, Compression::default());
    let mut archive = tar::Builder::new(encoder);

    archive
        .append_path_with_name(&binary_source, "pg-ephemeral")
        .expect("Failed to add binary to archive");

    // On macOS, include the .dSYM bundle for debug symbols
    if platform.is_macos() {
        let dsym_source = workspace_root
            .join("target")
            .join(rust_target)
            .join("release")
            .join("pg-ephemeral.dSYM");

        if dsym_source.exists() {
            log::info!("Including .dSYM debug symbols in tarball");
            archive
                .append_dir_all("pg-ephemeral.dSYM", &dsym_source)
                .expect("Failed to add .dSYM bundle to archive");
        } else {
            panic!(
                ".dSYM bundle not found at {}. Cannot create tarball without debug symbols.",
                dsym_source.display()
            );
        }
    }

    let encoder = archive.into_inner().expect("Failed to finish archive");
    encoder.finish().expect("Failed to finish gzip compression");

    log::info!("Tarball created at: {}", tarball_path.display());

    // Create SHA256 hash for tarball
    log::info!("Creating SHA256 hash for tarball");
    let tarball_bytes = std::fs::read(&tarball_path)
        .unwrap_or_else(|error| panic!("Failed to read tarball: {error}"));
    let mut hasher = Sha256::new();
    hasher.update(&tarball_bytes);
    let hash = hasher.finalize();
    let hash_string = format!("{hash:x}  {tarball_name}\n");
    let tarball_sha256_path = dist_binaries.join(format!("{tarball_name}.sha256"));
    std::fs::write(&tarball_sha256_path, hash_string)
        .unwrap_or_else(|error| panic!("Failed to write tarball SHA256 file: {error}"));

    log::info!(
        "Tarball SHA256 hash written to: {}",
        tarball_sha256_path.display()
    );
    log::info!("Integrations build complete");
}

fn merge_gems() {
    log::info!("Merging multi-platform gems into unified repository");

    let workspace_root = std::env::current_dir()
        .unwrap_or_else(|error| panic!("Failed to get current directory: {error}"));

    let dist_root = workspace_root.join("dist");
    let dist_gems = dist_root.join("gems");

    log::info!("Verifying and collecting gems from all platforms");

    let mut staging_items = Vec::new();

    // Verify and prepare staging items for all expected gem files
    for platform in Platform::ALL {
        let paths = platform_artifact_paths(
            &workspace_root,
            platform.rust_target(),
            platform.ruby_platform(),
        );

        // Verify and add gem file to staging
        let gem_source = verify_and_collect_file(paths.gem);
        staging_items.push(StagingItem::CopyFile {
            source: gem_source.clone(),
            destination: gem_source
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string(),
        });

        // Verify and add gem SHA256 file to staging
        let gem_sha_source = verify_and_collect_file(paths.gem_sha256);
        staging_items.push(StagingItem::CopyFile {
            source: gem_sha_source.clone(),
            destination: gem_sha_source
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string(),
        });
    }

    let collected_gems = Platform::ALL.len();
    setup_staging_directory(&dist_gems, staging_items);
    log::info!("Collected {collected_gems} platform gems");

    // Generate gem index for the unified repository
    log::info!("Generating gem index in: {}", dist_root.display());
    ociman::Command::new("gem")
        .arguments(["generate_index", "--directory", dist_root.to_str().unwrap()])
        .status()
        .unwrap_or_else(|error| panic!("Failed to generate gem index: {error}"));

    log::info!("Gem index generated successfully with {collected_gems} platform gems");
    log::info!(
        "Multi-platform gem repository ready at: {}",
        dist_root.display()
    );
}

fn run_ruby_tests(workspace_root: PathBuf, platform: Platform) {
    let ruby_version = ruby_version();
    let integration_directory = workspace_root
        .join("pg-ephemeral")
        .join("integrations")
        .join("ruby");

    log::info!("Using pg-ephemeral version: {ruby_version}");
    log::info!("Target platform: {}", platform.rust_target());

    // Run acceptance tests with Gemfile.acceptance
    let gem_source_directory = workspace_root.join("dist");
    let gem_source_url = url::Url::from_directory_path(&gem_source_directory)
        .unwrap_or_else(|_| {
            panic!(
                "Failed to convert path to URL: {}",
                gem_source_directory.display()
            )
        })
        .to_string();

    // Configure pg gem build options on macOS
    if std::env::consts::OS == "macos" {
        log::info!("Configuring pg gem for macOS");
        let pg_config_path = if std::env::consts::ARCH == "aarch64" {
            "/opt/homebrew/opt/libpq/bin/pg_config"
        } else {
            "/usr/local/opt/libpq/bin/pg_config"
        };

        ociman::Command::new("bundle")
            .arguments([
                "config",
                "--local",
                "build.pg",
                &format!("--with-pg-config={pg_config_path}"),
            ])
            .working_directory(&integration_directory)
            .status()
            .unwrap_or_else(|error| panic!("Failed to configure bundle: {error}"));
    }

    log::info!("Running bundle install with Gemfile.acceptance");
    ociman::Command::new("bundle")
        .arguments(["install", "--gemfile=Gemfile.acceptance"])
        .working_directory(&integration_directory)
        .env("PG_EPHEMERAL_GEM_SOURCE", &gem_source_url)
        .status()
        .unwrap_or_else(|error| panic!("Failed to run bundle install: {error}"));

    log::info!("Running RSpec acceptance tests");
    ociman::Command::new("bundle")
        .arguments([
            "exec",
            "--gemfile=Gemfile.acceptance",
            "rspec",
            "spec/integration",
        ])
        .working_directory(&integration_directory)
        .env("EXPECTED_PG_EPHEMERAL_VERSION", &ruby_version)
        .env("PG_EPHEMERAL_GEM_SOURCE", &gem_source_url)
        .status()
        .unwrap_or_else(|error| panic!("RSpec acceptance tests failed: {error}"));

    // Copy pg-ephemeral binary from installed gem for local development
    log::info!("Copying pg-ephemeral binary from installed gem");

    // Get gem path using bundler
    let gem_dir = ociman::Command::new("bundle")
        .arguments([
            "exec",
            "--gemfile=Gemfile.acceptance",
            "ruby",
            "-e",
            "puts Gem::Specification.find_by_name('pg-ephemeral').gem_dir",
        ])
        .working_directory(&integration_directory)
        .env("PG_EPHEMERAL_GEM_SOURCE", &gem_source_url)
        .stdout()
        .string()
        .unwrap_or_else(|error| panic!("Failed to get gem path: {error}"))
        .trim()
        .to_string();

    let binary_source = PathBuf::from(&gem_dir).join("bin").join("pg-ephemeral");
    let bin_directory = integration_directory.join("bin");
    let binary_destination = bin_directory.join("pg-ephemeral");

    // Create bin directory
    std::fs::create_dir_all(&bin_directory)
        .unwrap_or_else(|error| panic!("Failed to create bin directory: {error}"));

    // Copy binary from gem
    log::info!(
        "Copying {} to {}",
        binary_source.display(),
        binary_destination.display()
    );
    std::fs::copy(&binary_source, &binary_destination)
        .unwrap_or_else(|error| panic!("Failed to copy binary from gem: {error}"));

    // Make binary executable
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(&binary_destination)
            .unwrap_or_else(|error| panic!("Failed to get binary metadata: {error}"))
            .permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&binary_destination, perms)
            .unwrap_or_else(|error| panic!("Failed to set binary permissions: {error}"));
    }

    log::info!("Binary copied to: {}", binary_destination.display());

    // On macOS, also copy the .dSYM bundle for debug symbols
    if platform.is_macos() {
        let dsym_source = PathBuf::from(&gem_dir)
            .join("bin")
            .join("pg-ephemeral.dSYM");
        let dsym_destination = bin_directory.join("pg-ephemeral.dSYM");

        if dsym_source.exists() {
            log::info!(
                "Copying .dSYM bundle {} to {}",
                dsym_source.display(),
                dsym_destination.display()
            );
            copy_dir_recursive(&dsym_source, &dsym_destination)
                .unwrap_or_else(|error| panic!("Failed to copy .dSYM bundle: {error}"));
        } else {
            log::warn!(
                ".dSYM bundle not found at {}. Backtraces may not include line numbers.",
                dsym_source.display()
            );
        }
    }

    // Run bundle install
    log::info!("Running bundle install");
    ociman::Command::new("bundle")
        .arguments(["install"])
        .working_directory(&integration_directory)
        .status()
        .unwrap_or_else(|error| panic!("Failed to run bundle install: {error}"));

    // Run RSpec tests
    log::info!("Running RSpec tests");
    ociman::Command::new("bundle")
        .arguments(["exec", "rspec"])
        .working_directory(&integration_directory)
        .env("EXPECTED_PG_EPHEMERAL_VERSION", &ruby_version)
        .status()
        .unwrap_or_else(|error| panic!("RSpec tests failed: {error}"));

    // Run Mutant tests (only on supported platforms)
    match ociman::platform::support() {
        Ok(()) => {
            log::info!("Running Mutant tests");
            ociman::Command::new("bundle")
                .arguments(["exec", "mutant", "run"])
                .working_directory(&integration_directory)
                .env("EXPECTED_PG_EPHEMERAL_VERSION", &ruby_version)
                .status()
                .unwrap_or_else(|error| panic!("Mutant tests failed: {error}"));
        }
        Err(error) => {
            log::info!("Skipping Mutant tests - platform not supported: {error}");
        }
    }

    log::info!("Integration tests complete");
}

fn test() {
    log::info!("Running Ruby integration tests");

    let workspace_root = std::env::current_dir()
        .unwrap_or_else(|error| panic!("Failed to get current directory: {error}"));

    let target = detect_target_platform();

    run_ruby_tests(workspace_root, target);
}

fn publish_gems(push: bool) {
    if push {
        log::info!("Publishing gems to RubyGems.org");
    } else {
        log::info!("Running in DRY-RUN mode (use --push to actually publish)");
    }

    let workspace_root = std::env::current_dir()
        .unwrap_or_else(|error| panic!("Failed to get current directory: {error}"));

    // Get git commit SHA
    let sha = ociman::Command::new("git")
        .arguments(["rev-parse", "HEAD"])
        .stdout()
        .string()
        .unwrap_or_else(|error| panic!("Failed to get git SHA: {error}"))
        .trim()
        .to_string();

    log::info!("Current git revision: {sha}");

    // Construct edge release tag
    let release_tag = format!("edge-{sha}");
    log::info!("Looking for edge release: {release_tag}");

    // Create dist directory for downloading artifacts
    let dist_dir = workspace_root.join("dist");
    let dist_gems = dist_dir.join("gems");
    if dist_gems.exists() {
        log::info!("Removing existing dist/gems directory");
        std::fs::remove_dir_all(&dist_gems)
            .unwrap_or_else(|error| panic!("Failed to remove dist/gems directory: {error}"));
    }
    std::fs::create_dir_all(&dist_gems)
        .unwrap_or_else(|error| panic!("Failed to create dist/gems directory: {error}"));

    log::info!("Downloading artifacts from edge release {release_tag}");

    // Download all artifacts from the edge release
    ociman::Command::new("gh")
        .arguments([
            "release",
            "download",
            &release_tag,
            "--repo",
            "mbj/mrs",
            "--dir",
            dist_gems.to_str().unwrap(),
            "--pattern",
            "*.gem",
        ])
        .status()
        .unwrap_or_else(|error| {
            panic!("Failed to download artifacts from release {release_tag}: {error}")
        });

    log::info!("All artifacts downloaded successfully");

    // Collect and publish gems
    let mut gems_to_publish = Vec::new();

    for platform in Platform::ALL {
        let ruby_version = ruby_version();
        let gem_name = format!(
            "pg-ephemeral-{ruby_version}-{}.gem",
            platform.ruby_platform()
        );
        let gem_path = dist_gems.join(&gem_name);

        if !gem_path.exists() {
            panic!("Expected gem file not found: {}", gem_path.display());
        }

        log::info!("Found gem: {}", gem_path.display());
        gems_to_publish.push(gem_path);
    }

    log::info!("Collected {} gems to publish", gems_to_publish.len());

    // Publish each gem
    for gem_path in &gems_to_publish {
        let mut command = std::process::Command::new("gem");
        command.args(["push", gem_path.to_str().unwrap()]);

        if push {
            log::info!("Pushing gem: {}", gem_path.display());
            let status = command
                .status()
                .unwrap_or_else(|error| panic!("Failed to execute gem push: {error}"));

            if !status.success() {
                panic!("Failed to push gem: {}", gem_path.display());
            }
            log::info!("Successfully pushed: {}", gem_path.display());
        } else {
            log::info!("[DRY-RUN] Would execute: {command:?}");
        }
    }

    if push {
        log::info!(
            "Successfully published {} gems to RubyGems.org",
            gems_to_publish.len()
        );
    } else {
        log::info!(
            "[DRY-RUN] Would have published {} gems",
            gems_to_publish.len()
        );
        log::info!("Run with --push to actually publish");
    }

    log::info!("Done");
}

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let app = <App as clap::Parser>::parse();

    if let Err(error) = app.run() {
        log::error!("{error}");
        std::process::exit(1);
    }
}
