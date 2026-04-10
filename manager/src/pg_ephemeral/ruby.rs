use super::{
    Platform, StagingItem, copy_dir_recursive, detect_target_platform, setup_staging_directory,
    verify_and_collect_file,
};
use cmd_proc::EnvVariableName;
use flate2::{Compression, write::GzEncoder};
use indoc::formatdoc;
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};

const ENV_PG_EPHEMERAL_GEMSPEC_CONFIG: EnvVariableName =
    EnvVariableName::from_static_or_panic("PG_EPHEMERAL_GEMSPEC_CONFIG");
const ENV_PG_EPHEMERAL_GEM_SOURCE: EnvVariableName =
    EnvVariableName::from_static_or_panic("PG_EPHEMERAL_GEM_SOURCE");
const ENV_EXPECTED_PG_EPHEMERAL_VERSION: EnvVariableName =
    EnvVariableName::from_static_or_panic("EXPECTED_PG_EPHEMERAL_VERSION");

fn ruby_version() -> String {
    let version = pg_ephemeral::version();
    let mut result = format!("{}.{}.{}", version.major, version.minor, version.patch);
    if !version.pre.is_empty() {
        result.push('.');
        result.push_str(&version.pre.to_string());
    }
    result
}

#[derive(Debug, clap::Parser)]
pub(crate) enum Command {
    /// Build gem for current architecture
    Build {
        /// Skip compilation (use pre-built binaries)
        #[clap(long)]
        no_compile: bool,
    },
    /// Merge multi-platform gems into unified repository
    MergeGems,
    /// Test gem (acceptance tests + unit tests + mutant)
    Test,
    /// Publish gems to RubyGems.org from GitHub artifacts
    Publish {
        /// Actually push gems to RubyGems.org (default is dry-run)
        #[clap(long)]
        push: bool,
    },
    /// Sync generated files (gemspec) with Rust source of truth
    Sync {
        /// Fail if git is dirty after syncing (for CI verification)
        #[clap(long)]
        reject_dirty: bool,
    },
}

impl Command {
    pub(crate) async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Self::Build { no_compile } => {
                build_integrations(*no_compile).await;
                Ok(())
            }
            Self::MergeGems => {
                merge_gems().await;
                Ok(())
            }
            Self::Test => {
                test().await;
                Ok(())
            }
            Self::Publish { push } => {
                publish_gems(*push).await;
                Ok(())
            }
            Self::Sync { reject_dirty } => sync(*reject_dirty).await,
        }
    }
}

pub(crate) struct PlatformArtifactPaths {
    pub(crate) gem: PathBuf,
    pub(crate) gem_sha256: PathBuf,
    pub(crate) tarball: PathBuf,
    pub(crate) tarball_sha256: PathBuf,
}

pub(crate) fn platform_artifact_paths(
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

#[derive(serde::Serialize)]
struct GemspecConfig {
    version: String,
    ruby_platform: String,
    bin_files: Vec<String>,
}

fn collect_bin_files(build_staging: &Path, platform: Platform) -> Vec<String> {
    let mut bin_files = vec!["bin/pg-ephemeral".to_string()];

    if platform.is_macos() {
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

pub(super) async fn sync(reject_dirty: bool) -> Result<(), Box<dyn std::error::Error>> {
    let workspace_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf();

    let version = ruby_version();
    log::info!("Syncing pg-ephemeral generated files (version: {version})");

    let gemspec_content = generate_gemspec(&version);
    let gemspec_path = workspace_root
        .join("pg-ephemeral")
        .join("integrations")
        .join("ruby")
        .join("pg-ephemeral.gemspec");

    let existing = std::fs::read_to_string(&gemspec_path).unwrap_or_default();

    if existing == gemspec_content {
        log::info!("Gemspec is up to date: {}", gemspec_path.display());
    } else if reject_dirty {
        let diff = similar::TextDiff::from_lines(&existing, &gemspec_content);

        return Err(format!(
            "Generated gemspec differs from {}. Run `manager pg-ephemeral sync` to update.\n\n{}",
            gemspec_path.display(),
            diff.unified_diff()
                .context_radius(3)
                .header("committed", "generated")
        )
        .into());
    } else {
        log::info!("Writing gemspec to: {}", gemspec_path.display());
        std::fs::write(&gemspec_path, &gemspec_content)
            .unwrap_or_else(|error| panic!("Failed to write gemspec: {error}"));
    }

    log::info!("Sync complete");
    Ok(())
}

fn generate_gemspec(version: &str) -> String {
    formatdoc! {"
        # This file is generated by `manager pg-ephemeral sync`.
        # Do not edit manually.

        Gem::Specification.new do |spec|
          spec.name          = 'pg-ephemeral'
          spec.version       = '{version}'
          spec.authors       = ['Markus Schirp']
          spec.email         = ['mbj@schirp-dso.com']

          spec.summary       = 'Ruby wrapper for pg-ephemeral PostgreSQL testing utility'
          spec.description   = 'Provides ephemeral PostgreSQL instances for testing, wrapping the pg-ephemeral project binary'
          spec.homepage      = 'https://github.com/mbj/mrs/tree/main/pg-ephemeral'
          spec.license       = 'MIT'
          spec.required_ruby_version = '>= 3.3'

          spec.metadata['homepage_uri'] = spec.homepage
          spec.metadata['source_code_uri'] = 'https://github.com/mbj/mrs'
          spec.metadata['changelog_uri'] = 'https://github.com/mbj/mrs/blob/main/pg-ephemeral/CHANGELOG.md'

          if ENV['PG_EPHEMERAL_GEMSPEC_CONFIG']
            require 'json'
            config = JSON.parse(ENV.fetch('PG_EPHEMERAL_GEMSPEC_CONFIG'))
            spec.platform = Gem::Platform.new(config.fetch('ruby_platform'))
            spec.files    = Dir['lib/**/*'] + config.fetch('bin_files') + ['LICENSE.txt']
          else
            spec.files = Dir['lib/**/*', 'bin/**/*', 'README.md', 'LICENSE.txt']
            spec.add_development_dependency 'mutant-rspec', '~> 0.16.0'
            spec.add_development_dependency 'rspec', '~> 3.0'
          end

          spec.require_paths = ['lib']
          spec.add_dependency 'pg', '~> 1.5'
        end
    "}
}

async fn build_integrations(no_compile: bool) {
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
        cmd_proc::Command::new("cargo")
            .arguments([
                "build",
                "--release",
                "--package",
                "pg-ephemeral",
                "--target",
                rust_target,
            ])
            .status()
            .await
            .unwrap_or_else(|error| panic!("Failed to build pg-ephemeral binary: {error}"));
    }

    let workspace_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf();

    let binary_source = workspace_root
        .join("target")
        .join(rust_target)
        .join("release")
        .join("pg-ephemeral");

    let integration_source = workspace_root
        .join("pg-ephemeral")
        .join("integrations")
        .join("ruby");

    let build_staging = workspace_root
        .join("pg-ephemeral")
        .join("build")
        .join(rust_target);

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
        StagingItem::CopyFile {
            source: integration_source.join("pg-ephemeral.gemspec"),
            destination: "pg-ephemeral.gemspec".to_string(),
        },
    ];

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

    let bin_files = collect_bin_files(&build_staging, platform);
    let gemspec_config = gemspec_config(&ruby_version, platform, bin_files);
    let gemspec_config_json = serde_json::to_string(&gemspec_config)
        .unwrap_or_else(|error| panic!("Failed to serialize gemspec config: {error}"));

    log::info!("Building gem");
    if platform.is_macos() {
        log::info!("Using native gem build for macOS");
        cmd_proc::Command::new("gem")
            .argument("build")
            .argument("pg-ephemeral.gemspec")
            .working_directory(&build_staging)
            .env(&ENV_PG_EPHEMERAL_GEMSPEC_CONFIG, &gemspec_config_json)
            .status()
            .await
            .unwrap_or_else(|error| panic!("Failed to build gem: {error}"));
    } else {
        log::info!("Using containerized gem build for Linux");
        let backend = ociman::backend::resolve::auto()
            .await
            .expect("Failed to detect backend");

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
        .environment_variable(ENV_PG_EPHEMERAL_GEMSPEC_CONFIG, &gemspec_config_json)
        .entrypoint("gem")
        .arguments(["build", "pg-ephemeral.gemspec"])
        .remove()
        .run()
        .await
        .unwrap_or_else(|error| panic!("Failed to build gem: {error}"));
    }

    let dist_root = workspace_root.join("dist");
    let dist_gems = dist_root.join("gems");
    let dist_binaries = dist_root.join("binaries");
    std::fs::create_dir_all(&dist_gems)
        .unwrap_or_else(|error| panic!("Failed to create dist/gems directory: {error}"));
    std::fs::create_dir_all(&dist_binaries)
        .unwrap_or_else(|error| panic!("Failed to create dist/binaries directory: {error}"));

    let gem_filename = format!("pg-ephemeral-{ruby_version}-{ruby_platform}.gem");
    let gem_source = build_staging.join(&gem_filename);

    log::info!("Creating SHA256 hash for gem");
    let gem_bytes =
        std::fs::read(&gem_source).unwrap_or_else(|error| panic!("Failed to read gem: {error}"));
    let mut hasher = Sha256::new();
    hasher.update(&gem_bytes);
    let hash = hasher.finalize();
    let gem_hash_string = format!("{}  {gem_filename}\n", hex::encode(hash));

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

    log::info!("Generating gem index in: {}", dist_root.display());
    cmd_proc::Command::new("gem")
        .arguments(["generate_index", "--directory", dist_root.to_str().unwrap()])
        .status()
        .await
        .unwrap_or_else(|error| panic!("Failed to generate gem index: {error}"));

    log::info!("Gem index generated successfully");

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

    log::info!("Creating SHA256 hash for tarball");
    let tarball_bytes = std::fs::read(&tarball_path)
        .unwrap_or_else(|error| panic!("Failed to read tarball: {error}"));
    let mut hasher = Sha256::new();
    hasher.update(&tarball_bytes);
    let hash = hasher.finalize();
    let hash_string = format!("{}  {tarball_name}\n", hex::encode(hash));
    let tarball_sha256_path = dist_binaries.join(format!("{tarball_name}.sha256"));
    std::fs::write(&tarball_sha256_path, hash_string)
        .unwrap_or_else(|error| panic!("Failed to write tarball SHA256 file: {error}"));

    log::info!(
        "Tarball SHA256 hash written to: {}",
        tarball_sha256_path.display()
    );
    log::info!("Integrations build complete");
}

async fn merge_gems() {
    log::info!("Merging multi-platform gems into unified repository");

    let workspace_root = std::env::current_dir()
        .unwrap_or_else(|error| panic!("Failed to get current directory: {error}"));

    let dist_root = workspace_root.join("dist");
    let dist_gems = dist_root.join("gems");

    log::info!("Verifying and collecting gems from all platforms");

    let mut staging_items = Vec::new();

    for platform in Platform::ALL {
        let paths = platform_artifact_paths(
            &workspace_root,
            platform.rust_target(),
            platform.ruby_platform(),
        );

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

    log::info!("Generating gem index in: {}", dist_root.display());
    cmd_proc::Command::new("gem")
        .arguments(["generate_index", "--directory", dist_root.to_str().unwrap()])
        .status()
        .await
        .unwrap_or_else(|error| panic!("Failed to generate gem index: {error}"));

    log::info!("Gem index generated successfully with {collected_gems} platform gems");
    log::info!(
        "Multi-platform gem repository ready at: {}",
        dist_root.display()
    );
}

async fn run_ruby_tests(workspace_root: PathBuf, platform: Platform) {
    let ruby_version = ruby_version();
    let integration_directory = workspace_root
        .join("pg-ephemeral")
        .join("integrations")
        .join("ruby");

    log::info!("Using pg-ephemeral version: {ruby_version}");
    log::info!("Target platform: {}", platform.rust_target());

    let gem_source_directory = workspace_root.join("dist");
    let gem_source_url = url::Url::from_directory_path(&gem_source_directory)
        .unwrap_or_else(|_| {
            panic!(
                "Failed to convert path to URL: {}",
                gem_source_directory.display()
            )
        })
        .to_string();

    if std::env::consts::OS == "macos" {
        log::info!("Configuring pg gem for macOS");
        let pg_config_path = if std::env::consts::ARCH == "aarch64" {
            "/opt/homebrew/opt/libpq/bin/pg_config"
        } else {
            "/usr/local/opt/libpq/bin/pg_config"
        };

        cmd_proc::Command::new("bundle")
            .arguments([
                "config",
                "--local",
                "build.pg",
                &format!("--with-pg-config={pg_config_path}"),
            ])
            .working_directory(&integration_directory)
            .status()
            .await
            .unwrap_or_else(|error| panic!("Failed to configure bundle: {error}"));
    }

    log::info!("Running bundle install with Gemfile.acceptance");
    cmd_proc::Command::new("bundle")
        .arguments(["install", "--gemfile=Gemfile.acceptance"])
        .working_directory(&integration_directory)
        .env(&ENV_PG_EPHEMERAL_GEM_SOURCE, &gem_source_url)
        .status()
        .await
        .unwrap_or_else(|error| panic!("Failed to run bundle install: {error}"));

    log::info!("Running RSpec acceptance tests");
    cmd_proc::Command::new("bundle")
        .arguments([
            "exec",
            "--gemfile=Gemfile.acceptance",
            "rspec",
            "spec/integration",
        ])
        .working_directory(&integration_directory)
        .env(&ENV_EXPECTED_PG_EPHEMERAL_VERSION, &ruby_version)
        .env(&ENV_PG_EPHEMERAL_GEM_SOURCE, &gem_source_url)
        .status()
        .await
        .unwrap_or_else(|error| panic!("RSpec acceptance tests failed: {error}"));

    log::info!("Copying pg-ephemeral binary from installed gem");

    let gem_dir = cmd_proc::Command::new("bundle")
        .arguments([
            "exec",
            "--gemfile=Gemfile.acceptance",
            "ruby",
            "-e",
            "puts Gem::Specification.find_by_name('pg-ephemeral').gem_dir",
        ])
        .working_directory(&integration_directory)
        .env(&ENV_PG_EPHEMERAL_GEM_SOURCE, &gem_source_url)
        .stdout_capture()
        .string()
        .await
        .unwrap_or_else(|error| panic!("Failed to get gem path: {error}"))
        .trim()
        .to_string();

    let binary_source = PathBuf::from(&gem_dir).join("bin").join("pg-ephemeral");
    let bin_directory = integration_directory.join("bin");
    let binary_destination = bin_directory.join("pg-ephemeral");

    std::fs::create_dir_all(&bin_directory)
        .unwrap_or_else(|error| panic!("Failed to create bin directory: {error}"));

    log::info!(
        "Copying {} to {}",
        binary_source.display(),
        binary_destination.display()
    );
    std::fs::copy(&binary_source, &binary_destination)
        .unwrap_or_else(|error| panic!("Failed to copy binary from gem: {error}"));

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

    log::info!("Running bundle install");
    cmd_proc::Command::new("bundle")
        .arguments(["install"])
        .working_directory(&integration_directory)
        .status()
        .await
        .unwrap_or_else(|error| panic!("Failed to run bundle install: {error}"));

    log::info!("Running RSpec tests");
    cmd_proc::Command::new("bundle")
        .arguments(["exec", "rspec"])
        .working_directory(&integration_directory)
        .env(&ENV_EXPECTED_PG_EPHEMERAL_VERSION, &ruby_version)
        .status()
        .await
        .unwrap_or_else(|error| panic!("RSpec tests failed: {error}"));

    match ociman::platform::support() {
        Ok(()) => {
            log::info!("Running Mutant tests");
            cmd_proc::Command::new("bundle")
                .arguments(["exec", "mutant", "run"])
                .working_directory(&integration_directory)
                .env(&ENV_EXPECTED_PG_EPHEMERAL_VERSION, &ruby_version)
                .status()
                .await
                .unwrap_or_else(|error| panic!("Mutant tests failed: {error}"));
        }
        Err(error) => {
            log::info!("Skipping Mutant tests - platform not supported: {error}");
        }
    }

    log::info!("Integration tests complete");
}

async fn test() {
    log::info!("Running Ruby integration tests");

    let workspace_root = std::env::current_dir()
        .unwrap_or_else(|error| panic!("Failed to get current directory: {error}"));

    let target = detect_target_platform();

    run_ruby_tests(workspace_root, target).await;
}

async fn publish_gems(push: bool) {
    if push {
        log::info!("Publishing gems to RubyGems.org");
    } else {
        log::info!("Running in DRY-RUN mode (use --push to actually publish)");
    }

    let workspace_root = std::env::current_dir()
        .unwrap_or_else(|error| panic!("Failed to get current directory: {error}"));

    let edge = crate::edge::resolve().await;
    let release_tag = &edge.tag;

    let dist_dir = workspace_root.join("dist");
    let dist_gems = dist_dir.join("gems");
    if dist_gems.exists() {
        log::info!("Removing existing dist/gems directory");
        std::fs::remove_dir_all(&dist_gems)
            .unwrap_or_else(|error| panic!("Failed to remove dist/gems directory: {error}"));
    }
    std::fs::create_dir_all(&dist_gems)
        .unwrap_or_else(|error| panic!("Failed to create dist/gems directory: {error}"));

    let ruby_version = ruby_version();

    let gem_names: Vec<String> = Platform::ALL
        .iter()
        .map(|platform| {
            format!(
                "pg-ephemeral-{ruby_version}-{}.gem",
                platform.ruby_platform()
            )
        })
        .collect();

    log::info!("Downloading gems from edge release {release_tag}");

    let mut arguments = vec![
        "release",
        "download",
        &release_tag,
        "--repo",
        "mbj/mrs",
        "--dir",
        dist_gems.to_str().unwrap(),
    ];

    for gem_name in &gem_names {
        arguments.push("--pattern");
        arguments.push(gem_name);
    }

    cmd_proc::Command::new("gh")
        .arguments(arguments)
        .status()
        .await
        .unwrap_or_else(|error| {
            panic!("Failed to download gems from release {release_tag}: {error}")
        });

    let gems_to_publish: Vec<PathBuf> = gem_names.iter().map(|name| dist_gems.join(name)).collect();

    log::info!("Collected {} gems to publish", gems_to_publish.len());

    for gem_path in &gems_to_publish {
        let gem_path_str = gem_path.to_str().unwrap();

        if push {
            log::info!("Pushing gem: {}", gem_path.display());
            cmd_proc::Command::new("gem")
                .arguments(["push", gem_path_str])
                .status()
                .await
                .unwrap_or_else(|error| {
                    panic!("Failed to push gem {}: {error}", gem_path.display())
                });
            log::info!("Successfully pushed: {}", gem_path.display());
        } else {
            log::info!("[DRY-RUN] Would execute: gem push {gem_path_str}");
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
