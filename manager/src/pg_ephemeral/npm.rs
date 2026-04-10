use super::{
    Platform, StagingItem, detect_target_platform, setup_staging_directory, verify_and_collect_file,
};
use cmd_proc::EnvVariableName;
use flate2::{Compression, write::GzEncoder};
use indoc::formatdoc;
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};

const ENV_EXPECTED_PG_EPHEMERAL_VERSION: EnvVariableName =
    EnvVariableName::from_static_or_panic("EXPECTED_PG_EPHEMERAL_VERSION");
const PG_DEPENDENCY_VERSION: &str = "^8.0.0";
const NODE_TYPES_DEV_DEPENDENCY_VERSION: &str = ">=20";
const PG_TYPES_DEV_DEPENDENCY_VERSION: &str = "^8.0.0";
const TYPESCRIPT_DEV_DEPENDENCY_VERSION: &str = "^5.0.0";

fn npm_version() -> String {
    let version = pg_ephemeral::version();
    let mut result = format!("{}.{}.{}", version.major, version.minor, version.patch);
    if !version.pre.is_empty() {
        result.push('-');
        result.push_str(&version.pre.to_string());
    }
    result
}

#[derive(Debug, clap::Parser)]
pub(crate) enum Command {
    /// Build npm package for current architecture
    Build {
        /// Skip compilation (use pre-built binaries)
        #[clap(long)]
        no_compile: bool,
    },
    /// Merge multi-platform npm packages
    Merge,
    /// Test npm package
    Test,
    /// Publish npm packages from GitHub artifacts
    Publish {
        /// Actually push packages to npm (default is dry-run)
        #[clap(long)]
        push: bool,
    },
    /// Sync generated files (package.json) with Rust source of truth
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
                build(*no_compile).await;
                Ok(())
            }
            Self::Merge => {
                merge().await;
                Ok(())
            }
            Self::Test => {
                test().await;
                Ok(())
            }
            Self::Publish { push } => {
                publish(*push).await;
                Ok(())
            }
            Self::Sync { reject_dirty } => sync(*reject_dirty).await,
        }
    }
}

pub(crate) struct PlatformArtifactPaths {
    pub(crate) platform_tarball: PathBuf,
    pub(crate) platform_tarball_sha256: PathBuf,
    pub(crate) main_tarball: PathBuf,
    pub(crate) main_tarball_sha256: PathBuf,
}

pub(crate) fn platform_artifact_paths(
    workspace_root: &Path,
    platform: Platform,
) -> PlatformArtifactPaths {
    let version = npm_version();
    let artifact_base = workspace_root
        .join("artifacts")
        .join(format!("pg-ephemeral-{}", platform.rust_target()));

    let npm_base = artifact_base.join("dist").join("npm");

    let platform_tarball_name = platform_tarball_name(&version, platform);
    let main_tarball_name = main_tarball_name(&version);

    PlatformArtifactPaths {
        platform_tarball: npm_base.join(&platform_tarball_name),
        platform_tarball_sha256: npm_base.join(format!("{platform_tarball_name}.sha256")),
        main_tarball: npm_base.join(&main_tarball_name),
        main_tarball_sha256: npm_base.join(format!("{main_tarball_name}.sha256")),
    }
}

fn platform_tarball_name(version: &str, platform: Platform) -> String {
    format!("pg-ephemeral-{}-{version}.tgz", platform.npm_platform())
}

fn main_tarball_name(version: &str) -> String {
    format!("pg-ephemeral-{version}.tgz")
}

pub(super) async fn sync(reject_dirty: bool) -> Result<(), Box<dyn std::error::Error>> {
    let workspace_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf();

    let version = npm_version();
    log::info!("Syncing pg-ephemeral npm generated files (version: {version})");

    let package_json_content = generate_package_json(&version);
    let package_json_path = workspace_root
        .join("pg-ephemeral")
        .join("integrations")
        .join("npm")
        .join("package.json");

    let existing = std::fs::read_to_string(&package_json_path).unwrap_or_default();

    if existing == package_json_content {
        log::info!(
            "package.json is up to date: {}",
            package_json_path.display()
        );
    } else if reject_dirty {
        let diff = similar::TextDiff::from_lines(&existing, &package_json_content);

        return Err(format!(
            "Generated package.json differs from {}. Run `manager pg-ephemeral npm sync` to update.\n\n{}",
            package_json_path.display(),
            diff.unified_diff()
                .context_radius(3)
                .header("committed", "generated")
        )
        .into());
    } else {
        log::info!("Writing package.json to: {}", package_json_path.display());
        std::fs::write(&package_json_path, &package_json_content)
            .unwrap_or_else(|error| panic!("Failed to write package.json: {error}"));
    }

    log::info!("Sync complete");
    Ok(())
}

fn generate_package_json(version: &str) -> String {
    formatdoc! {r#"
        {{
          "name": "pg-ephemeral",
          "version": "{version}",
          "description": "Provides ephemeral PostgreSQL instances for testing, wrapping the pg-ephemeral project binary",
          "license": "MIT",
          "homepage": "https://github.com/mbj/mrs/tree/main/pg-ephemeral",
          "repository": {{
            "type": "git",
            "url": "https://github.com/mbj/mrs",
            "directory": "pg-ephemeral"
          }},
          "main": "lib/index.js",
          "types": "lib/index.d.ts",
          "files": [
            "lib/"
          ],
          "engines": {{
            "node": ">=20"
          }},
          "dependencies": {{
            "pg": "{PG_DEPENDENCY_VERSION}"
          }},
          "devDependencies": {{
            "@types/node": "{NODE_TYPES_DEV_DEPENDENCY_VERSION}",
            "@types/pg": "{PG_TYPES_DEV_DEPENDENCY_VERSION}",
            "typescript": "{TYPESCRIPT_DEV_DEPENDENCY_VERSION}"
          }},
          "optionalDependencies": {{
            "@pg-ephemeral/darwin-arm64": "{version}",
            "@pg-ephemeral/linux-arm64": "{version}",
            "@pg-ephemeral/linux-x64": "{version}"
          }}
        }}
    "#}
}

fn generate_test_package_json() -> String {
    formatdoc! {r#"
        {{
          "name": "pg-ephemeral-test-consumer",
          "private": true,
          "devDependencies": {{
            "@types/node": "{NODE_TYPES_DEV_DEPENDENCY_VERSION}",
            "@types/pg": "{PG_TYPES_DEV_DEPENDENCY_VERSION}",
            "typescript": "{TYPESCRIPT_DEV_DEPENDENCY_VERSION}"
          }}
        }}
    "#}
}

fn generate_platform_package_json(version: &str, platform: Platform) -> String {
    let name = format!("@pg-ephemeral/{}", platform.npm_platform());
    let npm_platform = platform.npm_platform();
    let os = platform.npm_os();
    let cpu = platform.npm_cpu();

    formatdoc! {r#"
        {{
          "name": "{name}",
          "version": "{version}",
          "description": "pg-ephemeral binary for {npm_platform}",
          "license": "MIT",
          "os": [
            "{os}"
          ],
          "cpu": [
            "{cpu}"
          ],
          "files": [
            "bin/"
          ]
        }}
    "#}
}

fn create_npm_tarball(staging_dir: &Path, tarball_path: &Path) {
    log::info!("Creating npm tarball: {}", tarball_path.display());

    let tarball_file = std::fs::File::create(tarball_path)
        .unwrap_or_else(|error| panic!("Failed to create tarball file: {error}"));
    let encoder = GzEncoder::new(tarball_file, Compression::default());
    let mut archive = tar::Builder::new(encoder);

    archive
        .append_dir_all("package", staging_dir)
        .unwrap_or_else(|error| panic!("Failed to add files to tarball: {error}"));

    let encoder = archive.into_inner().expect("Failed to finish archive");
    encoder.finish().expect("Failed to finish gzip compression");
}

fn write_sha256(dir: &Path, filename: &str) {
    let file_path = dir.join(filename);
    let bytes = std::fs::read(&file_path)
        .unwrap_or_else(|error| panic!("Failed to read {filename}: {error}"));

    let mut hasher = Sha256::new();
    hasher.update(&bytes);
    let hash = hex::encode(hasher.finalize());
    let hash_string = format!("{hash}  {filename}\n");
    let sha256_path = dir.join(format!("{filename}.sha256"));

    std::fs::write(&sha256_path, hash_string)
        .unwrap_or_else(|error| panic!("Failed to write SHA256 file: {error}"));

    log::info!("SHA256 hash written to: {}", sha256_path.display());
}

async fn build(no_compile: bool) {
    let platform = detect_target_platform();
    let rust_target = platform.rust_target();
    let npm_platform = platform.npm_platform();
    let version = npm_version();

    log::info!("Building pg-ephemeral npm packages for target: {rust_target}");
    log::info!("npm platform: {npm_platform}");
    log::info!("Version: {version}");

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
        .join("npm");

    // Build platform package
    let platform_staging = workspace_root
        .join("pg-ephemeral")
        .join("build")
        .join(rust_target)
        .join("npm-platform");

    let mut platform_items = vec![
        StagingItem::CopyFile {
            source: binary_source.clone(),
            destination: "bin/pg-ephemeral".to_string(),
        },
        StagingItem::GenerateFile {
            destination: "package.json".to_string(),
            content: generate_platform_package_json(&version, platform),
        },
        StagingItem::CopyFile {
            source: workspace_root.join("LICENSE.txt"),
            destination: "LICENSE.txt".to_string(),
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
            platform_items.push(StagingItem::CopyDirectory {
                source: dsym_source,
                destination: "bin/pg-ephemeral.dSYM".to_string(),
            });
        } else {
            panic!(
                ".dSYM bundle not found at {}. Cannot build package without debug symbols.",
                dsym_source.display()
            );
        }
    }

    setup_staging_directory(&platform_staging, platform_items);

    // Compile TypeScript
    log::info!("Installing npm dependencies");
    cmd_proc::Command::new("npm")
        .arguments(["install"])
        .working_directory(&integration_source)
        .status()
        .await
        .unwrap_or_else(|error| panic!("Failed to install npm dependencies: {error}"));

    log::info!("Compiling TypeScript");
    cmd_proc::Command::new("npx")
        .arguments(["tsc"])
        .working_directory(&integration_source)
        .status()
        .await
        .unwrap_or_else(|error| panic!("Failed to compile TypeScript: {error}"));

    // Build main package
    let main_staging = workspace_root
        .join("pg-ephemeral")
        .join("build")
        .join(rust_target)
        .join("npm-main");

    setup_staging_directory(
        &main_staging,
        vec![
            StagingItem::CopyDirectory {
                source: integration_source.join("lib"),
                destination: "lib".to_string(),
            },
            StagingItem::CopyFile {
                source: integration_source.join("package.json"),
                destination: "package.json".to_string(),
            },
            StagingItem::CopyFile {
                source: workspace_root.join("LICENSE.txt"),
                destination: "LICENSE.txt".to_string(),
            },
        ],
    );

    // Create dist directory
    let dist_npm = workspace_root.join("dist").join("npm");
    std::fs::create_dir_all(&dist_npm)
        .unwrap_or_else(|error| panic!("Failed to create dist/npm directory: {error}"));

    // Create platform package tarball
    let platform_tarball = platform_tarball_name(&version, platform);
    create_npm_tarball(&platform_staging, &dist_npm.join(&platform_tarball));
    write_sha256(&dist_npm, &platform_tarball);

    // Create main package tarball
    let main_tarball = main_tarball_name(&version);
    create_npm_tarball(&main_staging, &dist_npm.join(&main_tarball));
    write_sha256(&dist_npm, &main_tarball);

    log::info!("npm build complete");
}

async fn merge() {
    log::info!("Merging multi-platform npm packages");

    let workspace_root = std::env::current_dir()
        .unwrap_or_else(|error| panic!("Failed to get current directory: {error}"));

    let dist_npm = workspace_root.join("dist").join("npm");
    let version = npm_version();

    log::info!("Verifying and collecting npm packages from all platforms");

    let mut staging_items = Vec::new();

    for platform in Platform::ALL {
        let paths = platform_artifact_paths(&workspace_root, *platform);

        let source = verify_and_collect_file(paths.platform_tarball);
        staging_items.push(StagingItem::CopyFile {
            destination: source.file_name().unwrap().to_str().unwrap().to_string(),
            source: source.clone(),
        });

        let sha_source = verify_and_collect_file(paths.platform_tarball_sha256);
        staging_items.push(StagingItem::CopyFile {
            destination: sha_source
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string(),
            source: sha_source.clone(),
        });
    }

    // Main package tarball (identical from any platform, take from first)
    let first_paths = platform_artifact_paths(&workspace_root, Platform::ALL[0]);

    let main_source = verify_and_collect_file(first_paths.main_tarball);
    staging_items.push(StagingItem::CopyFile {
        destination: main_tarball_name(&version),
        source: main_source,
    });

    let main_sha_source = verify_and_collect_file(first_paths.main_tarball_sha256);
    staging_items.push(StagingItem::CopyFile {
        destination: format!("{}.sha256", main_tarball_name(&version)),
        source: main_sha_source,
    });

    let collected = Platform::ALL.len();
    setup_staging_directory(&dist_npm, staging_items);

    log::info!("Collected {collected} platform packages + main package");
    log::info!("npm packages ready at: {}", dist_npm.display());
}

async fn test() {
    log::info!("Running npm integration tests");

    let workspace_root = std::env::current_dir()
        .unwrap_or_else(|error| panic!("Failed to get current directory: {error}"));

    let platform = detect_target_platform();
    let version = npm_version();
    let integration_directory = workspace_root
        .join("pg-ephemeral")
        .join("integrations")
        .join("npm");
    let test_directory = workspace_root
        .join("pg-ephemeral")
        .join("build")
        .join(platform.rust_target())
        .join("npm-test");
    let dist_npm = workspace_root.join("dist").join("npm");

    let main_tarball = dist_npm.join(main_tarball_name(&version));
    let platform_tarball = dist_npm.join(platform_tarball_name(&version, platform));

    if test_directory.exists() {
        log::info!("Removing existing npm test directory");
        std::fs::remove_dir_all(&test_directory)
            .unwrap_or_else(|error| panic!("Failed to remove npm test directory: {error}"));
    }

    setup_staging_directory(
        &test_directory,
        vec![
            StagingItem::CopyDirectory {
                source: integration_directory.join("test"),
                destination: "test".to_string(),
            },
            StagingItem::CopyFile {
                source: integration_directory.join("database.toml"),
                destination: "database.toml".to_string(),
            },
            StagingItem::GenerateFile {
                destination: "package.json".to_string(),
                content: generate_test_package_json(),
            },
        ],
    );

    log::info!("Installing dev dependencies");

    cmd_proc::Command::new("npm")
        .arguments(["install"])
        .working_directory(&test_directory)
        .status()
        .await
        .unwrap_or_else(|error| panic!("Failed to install dev dependencies: {error}"));

    log::info!("Installing from local tarballs");

    cmd_proc::Command::new("npm")
        .arguments([
            "install",
            "--install-strategy=nested",
            "--no-save",
            main_tarball.to_str().unwrap(),
            platform_tarball.to_str().unwrap(),
        ])
        .working_directory(&test_directory)
        .status()
        .await
        .unwrap_or_else(|error| panic!("Failed to install npm packages: {error}"));

    log::info!("Compiling test TypeScript");

    cmd_proc::Command::new("npx")
        .arguments(["tsc", "--project", "test/tsconfig.json"])
        .working_directory(&test_directory)
        .status()
        .await
        .unwrap_or_else(|error| panic!("Failed to compile test TypeScript: {error}"));

    log::info!("Running integration tests");

    cmd_proc::Command::new("node")
        .arguments(["--test", "test/compiled/pg_ephemeral.test.js"])
        .working_directory(&test_directory)
        .env(&ENV_EXPECTED_PG_EPHEMERAL_VERSION, &version)
        .status()
        .await
        .unwrap_or_else(|error| panic!("npm tests failed: {error}"));

    log::info!("npm integration tests complete");
}

async fn publish(push: bool) {
    if push {
        log::info!("Publishing npm packages");
    } else {
        log::info!("Running in DRY-RUN mode (use --push to actually publish)");
    }

    let workspace_root = std::env::current_dir()
        .unwrap_or_else(|error| panic!("Failed to get current directory: {error}"));

    let edge = crate::edge::resolve().await;
    let release_tag = &edge.tag;

    let dist_npm = workspace_root.join("dist").join("npm");
    if dist_npm.exists() {
        log::info!("Removing existing dist/npm directory");
        std::fs::remove_dir_all(&dist_npm)
            .unwrap_or_else(|error| panic!("Failed to remove dist/npm directory: {error}"));
    }
    std::fs::create_dir_all(&dist_npm)
        .unwrap_or_else(|error| panic!("Failed to create dist/npm directory: {error}"));

    let version = npm_version();

    // Platform packages first so they exist when the main package is installed
    let mut tarball_names: Vec<String> = Platform::ALL
        .iter()
        .map(|platform| platform_tarball_name(&version, *platform))
        .collect();

    tarball_names.push(main_tarball_name(&version));

    log::info!("Downloading npm artifacts from edge release {release_tag}");

    let mut arguments = vec![
        "release",
        "download",
        &release_tag,
        "--repo",
        "mbj/mrs",
        "--dir",
        dist_npm.to_str().unwrap(),
    ];

    for name in &tarball_names {
        arguments.push("--pattern");
        arguments.push(name);
    }

    cmd_proc::Command::new("gh")
        .arguments(arguments)
        .status()
        .await
        .unwrap_or_else(|error| {
            panic!("Failed to download npm artifacts from release {release_tag}: {error}")
        });

    let tarballs: Vec<PathBuf> = tarball_names
        .iter()
        .map(|name| dist_npm.join(name))
        .collect();

    log::info!("Collected {} packages to publish", tarballs.len());

    for tarball in &tarballs {
        let tarball_str = tarball.to_str().unwrap();

        if push {
            log::info!("Publishing: {}", tarball.display());
            cmd_proc::Command::new("npm")
                .arguments(["publish", tarball_str, "--access", "public"])
                .status()
                .await
                .unwrap_or_else(|error| panic!("Failed to publish {}: {error}", tarball.display()));
            log::info!("Successfully published: {}", tarball.display());
        } else {
            log::info!("[DRY-RUN] Would execute: npm publish {tarball_str} --access public");
        }
    }

    if push {
        log::info!("Successfully published {} npm packages", tarballs.len());
    } else {
        log::info!("[DRY-RUN] Would have published {} packages", tarballs.len());
        log::info!("Run with --push to actually publish");
    }

    log::info!("Done");
}
