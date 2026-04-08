mod edge;
mod pg_ephemeral;

use git_proc::Build;
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

#[derive(Debug, clap::Parser)]
struct App {
    #[clap(subcommand)]
    command: AppCommand,
}

#[derive(Debug, clap::Parser)]
enum AppCommand {
    /// pg-ephemeral management commands
    PgEphemeral {
        #[clap(subcommand)]
        command: pg_ephemeral::Command,
    },
    /// Release management commands
    Release {
        #[clap(subcommand)]
        command: ReleaseCommand,
    },
    /// Repository lint and verification commands
    RepositoryLint {
        #[clap(subcommand)]
        command: RepositoryLintCommand,
    },
    /// Stratosphere code generation commands
    Stratosphere {
        #[clap(subcommand)]
        command: StratosphereCommand,
    },
}

#[derive(Debug, clap::Parser)]
enum ReleaseCommand {
    /// Create GitHub edge release with all built artifacts
    CreateEdge,
}

#[derive(Debug, clap::Parser)]
enum RepositoryLintCommand {
    /// Verify rust-version in Cargo.toml matches rust-toolchain.toml channel
    RustVersion,
}

#[derive(Debug, clap::Parser)]
enum StratosphereCommand {
    /// Sync stratosphere with CloudFormation resource specification (Cargo.toml features and generated source)
    Sync {
        /// Panic if git is dirty after syncing (for CI verification)
        #[clap(long)]
        reject_dirty: bool,
    },
}

impl App {
    async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        match &self.command {
            AppCommand::PgEphemeral { command } => command.run().await,
            AppCommand::Release { command } => command.run().await,
            AppCommand::RepositoryLint { command } => command.run().await,
            AppCommand::Stratosphere { command } => command.run().await,
        }
    }
}

impl ReleaseCommand {
    async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Self::CreateEdge => {
                create_edge_release().await;
                Ok(())
            }
        }
    }
}

impl RepositoryLintCommand {
    async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Self::RustVersion => lint_rust_version().await,
        }
    }
}

impl StratosphereCommand {
    async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Self::Sync { reject_dirty } => stratosphere_sync(*reject_dirty).await,
        }
    }
}

async fn lint_rust_version() -> Result<(), Box<dyn std::error::Error>> {
    let workspace_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf();

    let toolchain_path = workspace_root.join("rust-toolchain.toml");
    let cargo_path = workspace_root.join("Cargo.toml");

    log::info!("Checking rust-toolchain.toml: {}", toolchain_path.display());
    log::info!("Checking Cargo.toml: {}", cargo_path.display());

    let toolchain_content = std::fs::read_to_string(&toolchain_path)?;
    let cargo_content = std::fs::read_to_string(&cargo_path)?;

    let toolchain: toml::Value = toml::from_str(&toolchain_content)?;
    let cargo: toml::Value = toml::from_str(&cargo_content)?;

    let toolchain_channel = toolchain
        .get("toolchain")
        .and_then(|toolchain_table| toolchain_table.get("channel"))
        .and_then(|channel| channel.as_str())
        .ok_or("rust-toolchain.toml missing toolchain.channel")?;

    let cargo_rust_version = cargo
        .get("workspace")
        .and_then(|workspace| workspace.get("package"))
        .and_then(|package| package.get("rust-version"))
        .and_then(|rust_version| rust_version.as_str())
        .ok_or("Cargo.toml missing workspace.package.rust-version")?;

    log::info!("rust-toolchain.toml channel: {toolchain_channel}");
    log::info!("Cargo.toml rust-version: {cargo_rust_version}");

    if toolchain_channel != cargo_rust_version {
        return Err(format!(
            "Rust version mismatch: rust-toolchain.toml has '{toolchain_channel}' but Cargo.toml has '{cargo_rust_version}'"
        )
        .into());
    }

    log::info!("Rust versions are in sync");
    Ok(())
}

async fn create_edge_release() {
    log::info!("Creating edge release");

    let workspace_root = std::env::current_dir()
        .unwrap_or_else(|error| panic!("Failed to get current directory: {error}"));

    let edge = edge::resolve().await;

    // Get current branch name
    let branch = git_proc::rev_parse::new()
        .abbrev_ref()
        .rev("HEAD")
        .build()
        .stdout_capture()
        .string()
        .await
        .unwrap_or_else(|error| panic!("Failed to get git branch: {error}"))
        .trim()
        .to_string();

    let tag = &edge.tag;
    let title = format!("Edge Build ({branch} @ {})", edge.sha);
    let notes = format!("Automated edge build from commit {}", edge.sha);

    log::info!("Tag: {tag}");
    log::info!("Title: {title}");

    let mut release_files = Vec::new();

    // Verify and collect all expected artifacts
    for platform in pg_ephemeral::Platform::ALL {
        let paths = pg_ephemeral::ruby::platform_artifact_paths(
            &workspace_root,
            platform.rust_target(),
            platform.ruby_platform(),
        );

        release_files.push(pg_ephemeral::verify_and_collect_file(paths.gem));
        release_files.push(pg_ephemeral::verify_and_collect_file(paths.gem_sha256));
        release_files.push(pg_ephemeral::verify_and_collect_file(paths.tarball));
        release_files.push(pg_ephemeral::verify_and_collect_file(paths.tarball_sha256));

        let npm_paths = pg_ephemeral::npm::platform_artifact_paths(&workspace_root, *platform);

        release_files.push(pg_ephemeral::verify_and_collect_file(
            npm_paths.platform_tarball,
        ));
        release_files.push(pg_ephemeral::verify_and_collect_file(
            npm_paths.platform_tarball_sha256,
        ));
    }

    // npm main tarball is platform-independent, collect once from first platform
    let npm_main_paths =
        pg_ephemeral::npm::platform_artifact_paths(&workspace_root, pg_ephemeral::Platform::ALL[0]);
    release_files.push(pg_ephemeral::verify_and_collect_file(
        npm_main_paths.main_tarball,
    ));
    release_files.push(pg_ephemeral::verify_and_collect_file(
        npm_main_paths.main_tarball_sha256,
    ));

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

    cmd_proc::Command::new("gh")
        .arguments(
            arguments
                .iter()
                .map(|argument| argument.as_str())
                .collect::<Vec<_>>(),
        )
        .status()
        .await
        .unwrap_or_else(|error| panic!("Failed to create GitHub release: {error}"));

    log::info!("Successfully created edge release: {tag}");
}

fn render_token_stream(
    tokens: proc_macro2::TokenStream,
) -> Result<String, Box<dyn std::error::Error>> {
    let syntax_tree: syn::File = syn::parse2(tokens)?;
    Ok(prettyplease::unparse(&syntax_tree))
}

async fn write_token_stream(
    path: &Path,
    tokens: proc_macro2::TokenStream,
    edition: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let rendered = render_token_stream(tokens)?;
    write_rendered(path, rendered, edition)
        .await
        .map_err(|error| -> Box<dyn std::error::Error> { error })
}

async fn write_rendered(
    path: &Path,
    rendered: String,
    edition: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let formatted = cmd_proc::Command::new("rustfmt")
        .arguments(["--edition", edition])
        .stdin_bytes(rendered.into_bytes())
        .stdout_capture()
        .string()
        .await?;

    if path.exists() && std::fs::read_to_string(path)? == formatted {
        log::info!("Unchanged {}", path.display());
        return Ok(());
    }

    std::fs::write(path, &formatted)?;
    log::info!("Wrote {}", path.display());
    Ok(())
}

async fn stratosphere_sync(reject_dirty: bool) -> Result<(), Box<dyn std::error::Error>> {
    let workspace_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf();

    let workspace_toml: toml::Table =
        std::fs::read_to_string(workspace_root.join("Cargo.toml"))?.parse()?;
    let edition = workspace_toml["workspace"]["package"]["edition"]
        .as_str()
        .expect("workspace.package.edition not found in Cargo.toml");

    let spec = stratosphere_core::resource_specification::instance();
    let feature_names: Vec<_> = spec.feature_names().collect();

    // Update Cargo.toml features
    let cargo_toml_path = workspace_root.join("stratosphere/Cargo.toml");

    log::info!("Reading {}", cargo_toml_path.display());
    let content = std::fs::read_to_string(&cargo_toml_path)?;
    let mut doc: toml_edit::DocumentMut = content.parse()?;

    let mut features = toml_edit::Table::new();
    for feature_name in &feature_names {
        features.insert(
            feature_name.as_str(),
            toml_edit::value(toml_edit::Array::new()),
        );
    }

    doc["features"] = toml_edit::Item::Table(features);

    std::fs::write(&cargo_toml_path, doc.to_string())?;
    log::info!("Wrote {}", cargo_toml_path.display());

    // Create services directory
    let services_dir = workspace_root.join("stratosphere/src/services");
    std::fs::create_dir_all(&services_dir)?;

    // Build vendor map (single pass over all resources and properties)
    let vendor_map = stratosphere_core::token::build_vendor_map(spec);

    // Track all generated files for stale file cleanup.
    // Seed with files that are not generated but should be preserved.
    let mut preserve_files: BTreeSet<PathBuf> = BTreeSet::from([services_dir.join("README.md")]);

    // Generate services.rs
    let services_rs_path = workspace_root.join("stratosphere/src/services.rs");

    let vendor_modules: Vec<_> = vendor_map
        .keys()
        .map(|vendor_name| {
            let vendor_ident = quote::format_ident!("{}", vendor_name.as_ref().to_lowercase());
            quote::quote! {
                pub mod #vendor_ident;
            }
        })
        .collect();

    let services_tokens = quote::quote! {
        #![doc = include_str!("services/README.md")]

        pub mod tag;

        #(#vendor_modules)*
    };

    write_token_stream(&services_rs_path, services_tokens, edition).await?;
    preserve_files.insert(services_rs_path);

    // Generate vendor modules and create directories
    for (vendor_name, service_map) in &vendor_map {
        let vendor = vendor_name.as_ref().to_lowercase();
        let vendor_rs_path = services_dir.join(format!("{vendor}.rs"));

        let service_modules: Vec<_> = service_map
            .keys()
            .map(|service_name| {
                let service_id = stratosphere_core::resource_specification::ServiceIdentifier {
                    vendor_name: (*vendor_name).clone(),
                    service_name: (*service_name).clone(),
                };
                let feature_name = spec.feature_name(&service_id);
                let feature_str = feature_name.as_str();
                let service_ident =
                    quote::format_ident!("{}", service_name.as_ref().to_lowercase());
                quote::quote! {
                    #[cfg(feature = #feature_str)]
                    pub mod #service_ident;
                }
            })
            .collect();

        let vendor_tokens = quote::quote! {
            #(#service_modules)*
        };

        write_token_stream(&vendor_rs_path, vendor_tokens, edition).await?;
        preserve_files.insert(vendor_rs_path);

        // Create vendor directory for service files
        let vendor_dir = services_dir.join(&vendor);
        std::fs::create_dir_all(&vendor_dir)?;
    }

    // Collect all service file generation tasks
    let service_tasks: Vec<_> = vendor_map
        .iter()
        .flat_map(|(vendor_name, service_map)| {
            let vendor = vendor_name.as_ref().to_lowercase();
            let vendor_dir = services_dir.join(&vendor);

            service_map
                .iter()
                .map(move |(service_name, service_definition)| {
                    let service_id = stratosphere_core::resource_specification::ServiceIdentifier {
                        vendor_name: (*vendor_name).clone(),
                        service_name: (*service_name).clone(),
                    };
                    let service_file_name = format!("{}.rs", service_name.as_ref().to_lowercase());
                    let path = vendor_dir.join(&service_file_name);
                    (path, service_id, service_definition)
                })
        })
        .collect();

    // Track service file paths before parallel generation
    for (path, _, _) in &service_tasks {
        preserve_files.insert(path.clone());
    }

    // Render service files in parallel (CPU-bound)
    use rayon::prelude::*;

    let rendered: Vec<_> = service_tasks
        .par_iter()
        .map(|(path, service_id, service_definition)| {
            let service_tokens =
                stratosphere_core::token::service_file_token_stream(service_id, service_definition);
            let rendered = render_token_stream(service_tokens)
                .map_err(|error| format!("Failed to render {service_id}: {error}"))?;
            Ok((path.clone(), rendered))
        })
        .collect::<Result<Vec<_>, String>>()?;

    // Write service files through rustfmt in parallel (IO-bound)
    let mut tasks = tokio::task::JoinSet::new();
    let edition: String = edition.to_owned();

    for (path, rendered) in rendered {
        let edition = edition.clone();
        tasks.spawn(async move { write_rendered(&path, rendered, &edition).await });
    }

    while let Some(result) = tasks.join_next().await {
        result?.map_err(|error| -> Box<dyn std::error::Error> { error })?;
    }

    // Generate services/tag.rs
    let tag_path = services_dir.join("tag.rs");

    let tag_property_type = spec
        .property_types
        .get(&stratosphere_core::resource_specification::PropertyTypeName::Tag)
        .expect("Tag property type not found");

    let tag_definition = stratosphere_core::token::TagDefinition(tag_property_type);
    let tag_tokens = quote::quote! { #tag_definition };

    write_token_stream(&tag_path, tag_tokens, &edition).await?;
    preserve_files.insert(tag_path);

    log::info!("Generated {} services", feature_names.len());

    // Remove stale files from the services directory
    let mut removed: usize = 0;

    for entry in walkdir::WalkDir::new(&services_dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().is_file())
    {
        let path = entry.into_path();

        if !preserve_files.contains(&path) {
            std::fs::remove_file(&path)?;
            log::info!("Removed stale {}", path.display());
            removed += 1;
        }
    }

    if removed > 0 {
        log::info!("Removed {removed} stale files");
    }

    if reject_dirty {
        log::info!("Checking for uncommitted changes");
        let result = git_proc::diff::new()
            .repo_path(&workspace_root)
            .exit_code()
            .build()
            .status()
            .await;

        match result {
            Ok(()) => log::info!("Working directory is clean"),
            Err(error) => {
                if error.exit_status.is_some() {
                    return Err("Git working directory is dirty after sync.".into());
                }
                return Err(format!("Failed to run git diff: {error}").into());
            }
        }
    }

    Ok(())
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let app = <App as clap::Parser>::parse();

    if let Err(error) = app.run().await {
        log::error!("{error}");
        std::process::exit(1);
    }
}
