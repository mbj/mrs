use std::path::{Path, PathBuf};

use cmd_proc::EnvVariableName;

const ENV_GITHUB_REPOSITORY: EnvVariableName =
    EnvVariableName::from_static_or_panic("GITHUB_REPOSITORY");
const ENV_GITHUB_RUN_ID: EnvVariableName = EnvVariableName::from_static_or_panic("GITHUB_RUN_ID");

#[derive(Debug, thiserror::Error)]
pub(crate) enum Error {
    #[error(
        "pg-ephemeral binary not found at {path}. Build it first with `cargo build --release --package pg-ephemeral`."
    )]
    BinaryMissing { path: PathBuf },
    #[error("unsupported host platform: {arch}-{os}")]
    UnsupportedHostPlatform {
        arch: &'static str,
        os: &'static str,
    },
    #[error("failed to prepare scratch directory at {path}")]
    Scratch {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("pg-ephemeral {args} failed")]
    PgEphemeral {
        args: String,
        #[source]
        source: cmd_proc::CommandError,
    },
    #[error("failed to parse cache status JSON")]
    StatusParse(#[source] serde_json::Error),
    #[error("cache status JSON missing required field: {path}")]
    StatusShape { path: &'static str },
    #[error(
        "cache status sanity check failed: seed {seed} expected status {expected}, got {actual}"
    )]
    StatusMismatch {
        seed: String,
        expected: &'static str,
        actual: String,
    },
    #[error("cache status returned no seeds — test configuration produced an empty chain")]
    EmptySeeds,
}

#[derive(Debug, clap::Parser)]
pub(crate) enum Command {
    /// Run the end-to-end cache registry round trip.
    ///
    /// Populates a small test cache, pushes it to ghcr.io, clears the
    /// local store, pulls it back, and verifies the tip becomes a local
    /// hit again. Assumes `docker login ghcr.io` has already been done
    /// by the caller (the CI workflow handles this via `GITHUB_TOKEN`).
    Test,
}

impl Command {
    pub(crate) async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Self::Test => Ok(test().await?),
        }
    }
}

/// Construct the ghcr.io registry path the test uses.
///
/// On GitHub Actions, `GITHUB_REPOSITORY` is set to `owner/repo` (e.g.
/// `mbj/mrs`). Locally, we fall back to `mbj/mrs` so the command is at
/// least invocable off-CI for debugging.
fn cache_registry() -> String {
    let repository =
        std::env::var(ENV_GITHUB_REPOSITORY.as_str()).unwrap_or_else(|_| "mbj/mrs".to_string());
    format!("ghcr.io/{repository}/pg-ephemeral-cache-test")
}

/// Generate a unique, validly-shaped instance name for this run.
///
/// `InstanceName` only allows `[a-z0-9-]`, no leading/trailing dash,
/// max 63 bytes. We use `GITHUB_RUN_ID` (numeric, always valid) when
/// running on CI and fall back to the local process id otherwise.
fn instance_name() -> String {
    let suffix = std::env::var(ENV_GITHUB_RUN_ID.as_str())
        .unwrap_or_else(|_| format!("local-{}", std::process::id()));
    format!("ci-{suffix}")
}

fn pg_ephemeral_binary() -> Result<PathBuf, Error> {
    let workspace_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("manager crate has a parent directory")
        .to_path_buf();

    // Match the cargo target the current manager binary was built with.
    // When manager runs from the CI release build, `CARGO_BUILD_TARGET`
    // is set via the workflow env. Locally, fall back to the native host
    // triple.
    let rust_target = match std::env::var("CARGO_BUILD_TARGET") {
        Ok(value) => value,
        Err(_) => match (std::env::consts::ARCH, std::env::consts::OS) {
            ("x86_64", "linux") => "x86_64-unknown-linux-musl".to_string(),
            ("aarch64", "linux") => "aarch64-unknown-linux-musl".to_string(),
            ("aarch64", "macos") => "aarch64-apple-darwin".to_string(),
            (arch, os) => {
                return Err(Error::UnsupportedHostPlatform { arch, os });
            }
        },
    };

    let path = workspace_root
        .join("target")
        .join(&rust_target)
        .join("release")
        .join("pg-ephemeral");

    if !path.exists() {
        return Err(Error::BinaryMissing { path });
    }

    Ok(path)
}

/// Build a fresh `database.toml` + seed files inside a scratch directory.
/// The caller is responsible for cleanup.
fn prepare_test_directory(registry: &str, instance_name: &str) -> Result<PathBuf, Error> {
    let dir = std::env::temp_dir().join(format!(
        "pg-ephemeral-ci-cache-registry-test-{}",
        std::process::id()
    ));

    let wrap = |source: std::io::Error| Error::Scratch {
        path: dir.clone(),
        source,
    };

    // Start from a clean slate in case a prior local run left crumbs.
    if dir.exists() {
        std::fs::remove_dir_all(&dir).map_err(wrap)?;
    }
    std::fs::create_dir_all(&dir).map_err(wrap)?;

    std::fs::write(
        dir.join("schema.sql"),
        "CREATE TABLE cache_registry_test (id INTEGER PRIMARY KEY);\n",
    )
    .map_err(wrap)?;

    std::fs::write(
        dir.join("data.sql"),
        "INSERT INTO cache_registry_test (id) VALUES (42);\n",
    )
    .map_err(wrap)?;

    let toml = format!(
        "cache_registry = \"{registry}\"\n\
         \n\
         [instances.{instance_name}.seeds.schema]\n\
         type = \"sql-file\"\n\
         path = \"schema.sql\"\n\
         \n\
         [instances.{instance_name}.seeds.data]\n\
         type = \"sql-file\"\n\
         path = \"data.sql\"\n"
    );
    std::fs::write(dir.join("database.toml"), toml).map_err(wrap)?;

    Ok(dir)
}

fn display_args(args: &[&str]) -> String {
    args.join(" ")
}

async fn run_pg_ephemeral(binary: &Path, working_dir: &Path, args: &[&str]) -> Result<(), Error> {
    // `cmd_proc::Command::status` returns `Err(CommandError)` on any
    // non-zero exit unless `accept_nonzero_exit()` is set, so we just
    // propagate.
    cmd_proc::Command::new(binary)
        .arguments(args)
        .working_directory(working_dir)
        .status()
        .await
        .map_err(|source| Error::PgEphemeral {
            args: display_args(args),
            source,
        })
}

async fn capture_pg_ephemeral(
    binary: &Path,
    working_dir: &Path,
    args: &[&str],
) -> Result<String, Error> {
    cmd_proc::Command::new(binary)
        .arguments(args)
        .working_directory(working_dir)
        .stdout_capture()
        .string()
        .await
        .map_err(|source| Error::PgEphemeral {
            args: display_args(args),
            source,
        })
}

fn parse_cache_status(json: &str) -> Result<serde_json::Value, Error> {
    serde_json::from_str(json).map_err(Error::StatusParse)
}

fn assert_all_stages_have_status(
    status_json: &serde_json::Value,
    expected: &'static str,
) -> Result<(), Error> {
    let seeds = status_json["seeds"].as_array().ok_or(Error::StatusShape {
        path: "seeds (expected array)",
    })?;
    if seeds.is_empty() {
        return Err(Error::EmptySeeds);
    }

    for seed in seeds {
        let name = seed["name"].as_str().ok_or(Error::StatusShape {
            path: "seeds[].name",
        })?;
        let status = seed["status"].as_str().ok_or(Error::StatusShape {
            path: "seeds[].status",
        })?;
        if status != expected {
            return Err(Error::StatusMismatch {
                seed: name.to_string(),
                expected,
                actual: status.to_string(),
            });
        }
    }
    Ok(())
}

fn assert_tip_hit(status_json: &serde_json::Value) -> Result<(), Error> {
    let seeds = status_json["seeds"].as_array().ok_or(Error::StatusShape {
        path: "seeds (expected array)",
    })?;
    let tip = seeds.last().ok_or(Error::EmptySeeds)?;
    let name = tip["name"].as_str().ok_or(Error::StatusShape {
        path: "seeds[last].name",
    })?;
    let status = tip["status"].as_str().ok_or(Error::StatusShape {
        path: "seeds[last].status",
    })?;
    if status != "hit" {
        return Err(Error::StatusMismatch {
            seed: name.to_string(),
            expected: "hit",
            actual: status.to_string(),
        });
    }
    Ok(())
}

async fn test() -> Result<(), Error> {
    let registry = cache_registry();
    let instance = instance_name();
    let binary = pg_ephemeral_binary()?;

    log::info!("Using cache_registry: {registry}");
    log::info!("Using instance name: {instance}");
    log::info!("Using pg-ephemeral: {}", binary.display());

    let dir = prepare_test_directory(&registry, &instance)?;
    let instance_arg: &[&str] = &["--instance", instance.as_str()];

    let populate_args: Vec<&str> = [&["cache", "populate"], instance_arg].concat();
    let push_args: Vec<&str> = [&["cache", "push"], instance_arg].concat();
    let reset_args: Vec<&str> = [&["cache", "reset", "--force"], instance_arg].concat();
    let pull_args: Vec<&str> = [&["cache", "pull"], instance_arg].concat();
    let status_args: Vec<&str> = [&["cache", "status", "--json"], instance_arg].concat();

    log::info!("Step 1/6: cache populate");
    run_pg_ephemeral(&binary, &dir, &populate_args).await?;

    log::info!("Step 2/6: cache push");
    run_pg_ephemeral(&binary, &dir, &push_args).await?;

    log::info!("Step 3/6: cache reset --force (clear local cache)");
    run_pg_ephemeral(&binary, &dir, &reset_args).await?;

    log::info!("Step 4/6: verify cache is empty locally");
    let status_after_reset = capture_pg_ephemeral(&binary, &dir, &status_args).await?;
    let parsed = parse_cache_status(&status_after_reset)?;
    assert_all_stages_have_status(&parsed, "miss")?;

    log::info!("Step 5/6: cache pull (should walk back and land on tip)");
    run_pg_ephemeral(&binary, &dir, &pull_args).await?;

    log::info!("Step 6/6: verify tip is now a local hit");
    let status_after_pull = capture_pg_ephemeral(&binary, &dir, &status_args).await?;
    let parsed = parse_cache_status(&status_after_pull)?;
    assert_tip_hit(&parsed)?;

    log::info!("Cleanup: remove scratch directory");
    let _ = std::fs::remove_dir_all(&dir);

    log::info!("Cache registry end-to-end test PASSED");
    Ok(())
}
