/// Run pg-ephemeral with the given arguments and assert success.
///
/// Returns the captured stdout as a String.
/// On failure, prints both stdout and stderr for debugging.
#[allow(dead_code)]
pub fn run_pg_ephemeral(args: &[&str], current_dir: &std::path::Path) -> String {
    let pg_ephemeral_bin = env!("CARGO_BIN_EXE_pg-ephemeral");

    let output = std::process::Command::new(pg_ephemeral_bin)
        .args(args)
        .current_dir(current_dir)
        .output()
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
    #[must_use]
    pub fn new(name_suffix: &str) -> Self {
        let path = std::env::temp_dir().join(format!(
            "pg-ephemeral-{}-{}",
            name_suffix,
            std::process::id()
        ));
        std::fs::create_dir_all(&path).unwrap();

        // Initialize git repository
        std::process::Command::new("git")
            .arg("init")
            .current_dir(&path)
            .output()
            .unwrap();

        // Configure git with hardcoded author (no environment reflection)
        std::process::Command::new("git")
            .args(["config", "user.name", "Test User"])
            .current_dir(&path)
            .output()
            .unwrap();

        std::process::Command::new("git")
            .args(["config", "user.email", "test@example.com"])
            .current_dir(&path)
            .output()
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
    #[must_use]
    pub fn commit(&self, message: &str) -> String {
        std::process::Command::new("git")
            .args(["add", "."])
            .current_dir(&self.path)
            .output()
            .unwrap();

        std::process::Command::new("git")
            .args([
                "commit",
                &format!("--message={message}"),
                "--author=Test User <test@example.com>",
                "--date=2020-01-01T00:00:00Z",
            ])
            .env("GIT_COMMITTER_DATE", "2020-01-01T00:00:00Z")
            .current_dir(&self.path)
            .output()
            .unwrap();

        let output = std::process::Command::new("git")
            .args(["rev-parse", "HEAD"])
            .current_dir(&self.path)
            .output()
            .unwrap();

        String::from_utf8(output.stdout).unwrap().trim().to_string()
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

    let definition = pg_ephemeral::Definition::new(backend.clone(), pg_ephemeral::Image::default())
        .cross_container_access(true);

    definition
        .with_container(async |container| {
            let image_tag = format!("pg-ephemeral-{language}-test:latest");

            backend
                .command()
                .argument("build")
                .argument("--tag")
                .argument(&image_tag)
                .argument(image_dir)
                .stdout()
                .bytes()
                .unwrap();

            let database_url = container
                .cross_container_client_config()
                .to_url()
                .to_string();

            let stdout = backend
                .command()
                .argument("run")
                .argument("--rm")
                .argument("--env")
                .argument(format!("DATABASE_URL={database_url}"))
                .argument(&image_tag)
                .stdout()
                .string()
                .unwrap_or_else(|error| panic!("Failed to run {language} container: {error:?}"));

            assert!(
                stdout.contains("SUCCESS: Connected to PostgreSQL successfully"),
                "Expected success message not found in output.\nOutput: {stdout}"
            );
        })
        .await
}
