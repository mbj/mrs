use git_proc::Build;

const GIT_COMMITTER_DATE: cmd_proc::EnvVariableName =
    cmd_proc::EnvVariableName::from_static_or_panic("GIT_COMMITTER_DATE");

/// Create a test definition with extended timeout.
///
/// CI environments may be slow, so we use 30s instead of the default 10s.
#[allow(dead_code)]
#[must_use]
pub fn test_definition(backend: ociman::Backend) -> pg_ephemeral::Definition {
    pg_ephemeral::Definition::new(
        backend,
        pg_ephemeral::Image::default(),
        "test".parse().unwrap(),
    )
    .wait_available_timeout(std::time::Duration::from_secs(30))
}

/// Run pg-ephemeral with the given arguments and assert success.
///
/// Returns the captured stdout as a String.
/// On failure, prints both stdout and stderr for debugging.
#[allow(dead_code)]
pub async fn run_pg_ephemeral(args: &[&str], current_dir: &std::path::Path) -> String {
    let pg_ephemeral_bin = env!("CARGO_BIN_EXE_pg-ephemeral");

    let output = cmd_proc::Command::new(pg_ephemeral_bin)
        .arguments(args)
        .working_directory(current_dir)
        .stdout_capture()
        .stderr_capture()
        .accept_nonzero_exit()
        .run()
        .await
        .unwrap();

    assert!(
        output.status.success(),
        "pg-ephemeral {} failed:\nstdout:\n{}\nstderr:\n{}",
        args.join(" "),
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    String::from_utf8(output.stdout).unwrap()
}

/// A temporary directory for testing.
///
/// The directory is automatically cleaned up when dropped.
#[allow(dead_code)]
pub struct TestDir {
    pub path: std::path::PathBuf,
}

#[allow(dead_code)]
impl TestDir {
    /// Create a new temporary directory with the given name suffix.
    #[must_use]
    pub fn new(name_suffix: &str) -> Self {
        let path = std::env::temp_dir().join(format!(
            "pg-ephemeral-{}-{}",
            name_suffix,
            std::process::id()
        ));
        std::fs::create_dir_all(&path).unwrap();
        Self { path }
    }

    /// Write a file to the directory.
    pub fn write_file(&self, name: &str, content: &str) {
        std::fs::write(self.path.join(name), content).unwrap();
    }
}

impl Drop for TestDir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.path);
    }
}

/// A temporary git repository for testing.
///
/// Creates a temporary directory with an initialized git repository.
/// The repository is automatically cleaned up when dropped.
#[allow(dead_code)]
pub struct TestGitRepo {
    pub path: std::path::PathBuf,
}

impl TestGitRepo {
    /// Create a new temporary git repository with the given name suffix.
    #[allow(dead_code)]
    pub async fn new(name_suffix: &str) -> Self {
        let path = std::env::temp_dir().join(format!(
            "pg-ephemeral-{}-{}",
            name_suffix,
            std::process::id()
        ));
        std::fs::create_dir_all(&path).unwrap();

        // Initialize git repository
        git_proc::init::new()
            .directory(&path)
            .status()
            .await
            .unwrap();

        // Configure git with hardcoded author (no environment reflection)
        git_proc::config::new("user.name")
            .repo_path(&path)
            .value("Test User")
            .status()
            .await
            .unwrap();

        git_proc::config::new("user.email")
            .repo_path(&path)
            .value("test@example.com")
            .status()
            .await
            .unwrap();

        Self { path }
    }

    /// Write a file to the repository.
    #[allow(dead_code)]
    pub fn write_file(&self, name: &str, content: &str) {
        std::fs::write(self.path.join(name), content).unwrap();
    }

    /// Commit all files with the given message, returning the commit hash.
    #[allow(dead_code)]
    pub async fn commit(&self, message: &str) -> String {
        git_proc::add::new()
            .repo_path(&self.path)
            .pathspec(".")
            .status()
            .await
            .unwrap();

        git_proc::commit::new()
            .repo_path(&self.path)
            .message(message)
            .author("Test User <test@example.com>")
            .date("2020-01-01T00:00:00Z")
            .env(
                GIT_COMMITTER_DATE,
                std::ffi::OsStr::new("2020-01-01T00:00:00Z"),
            )
            .status()
            .await
            .unwrap();

        git_proc::rev_parse::new()
            .repo_path(&self.path)
            .rev("HEAD")
            .build()
            .stdout_capture()
            .string()
            .await
            .unwrap()
            .trim()
            .to_string()
    }
}

impl Drop for TestGitRepo {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.path);
    }
}

#[allow(dead_code)]
pub async fn test_database_url_integration(language: &str, image_dir: &str) {
    let backend = ociman::test_backend_setup!();

    let definition = test_definition(backend.clone()).cross_container_access(true);

    definition
        .with_container(async |container| {
            let image_tag =
                ociman::testing::test_reference(&format!("pg-ephemeral-{language}-test:latest"))
                    .to_string();

            backend
                .command()
                .argument("build")
                .argument("--tag")
                .argument(&image_tag)
                .argument(image_dir)
                .stdout_capture()
                .bytes()
                .await
                .unwrap();

            let database_url = container
                .cross_container_client_config()
                .await
                .to_url_string();

            let stdout = backend
                .command()
                .argument("run")
                .argument("--rm")
                .argument("--env")
                .argument(format!("DATABASE_URL={database_url}"))
                .argument(&image_tag)
                .stdout_capture()
                .string()
                .await
                .unwrap_or_else(|error| panic!("Failed to run {language} container: {error:?}"));

            assert!(
                stdout.contains("SUCCESS: Connected to PostgreSQL successfully"),
                "Expected success message not found in output.\nOutput: {stdout}"
            );
        })
        .await
        .unwrap()
}
