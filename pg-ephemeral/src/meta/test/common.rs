//! Shared helpers for self-contained integration trials.

use git_proc::Build;

const GIT_COMMITTER_DATE: cmd_proc::EnvVariableName =
    cmd_proc::EnvVariableName::from_static_or_panic("GIT_COMMITTER_DATE");

pub(super) static POSTGRES_IMAGE: std::sync::LazyLock<ociman::image::Reference> =
    std::sync::LazyLock::new(|| "docker.io/library/postgres:17".parse().unwrap());

pub(super) static RUBY_IMAGE: std::sync::LazyLock<ociman::image::Reference> =
    std::sync::LazyLock::new(|| "docker.io/ruby:3.4-alpine".parse().unwrap());

pub(super) static NODE_IMAGE: std::sync::LazyLock<ociman::image::Reference> =
    std::sync::LazyLock::new(|| "docker.io/node:22-alpine".parse().unwrap());

/// Create a test definition with extended timeout.
///
/// CI environments may be slow, so we use 30s instead of the default 10s.
#[must_use]
pub(super) fn test_definition(backend: ociman::Backend) -> crate::Definition {
    crate::Definition::new(backend, crate::Image::default(), "test".parse().unwrap())
        .wait_available_timeout(std::time::Duration::from_secs(30))
}

/// Run pg-ephemeral with the given arguments and assert success.
///
/// Returns the captured stdout as a String.
/// On failure, prints both stdout and stderr for debugging.
pub(super) async fn run_pg_ephemeral(args: &[&str], current_dir: &std::path::Path) -> String {
    let pg_ephemeral_bin = std::env::current_exe().unwrap();

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
        std::str::from_utf8(&output.stdout).unwrap(),
        std::str::from_utf8(&output.stderr).unwrap(),
    );

    String::from_utf8(output.stdout).unwrap()
}

/// A temporary directory for testing.
///
/// The directory is automatically cleaned up when dropped.
pub(super) struct TestDir {
    pub(super) path: std::path::PathBuf,
}

impl TestDir {
    /// Create a new temporary directory with the given name suffix.
    #[must_use]
    pub(super) fn new(name_suffix: &str) -> Self {
        let path = std::env::temp_dir().join(format!(
            "pg-ephemeral-{}-{}",
            name_suffix,
            std::process::id()
        ));
        std::fs::create_dir_all(&path).unwrap();
        Self { path }
    }

    /// Write a file to the directory.
    pub(super) fn write_file(&self, name: &str, content: &str) {
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
pub(super) struct TestGitRepo {
    pub(super) path: std::path::PathBuf,
}

impl TestGitRepo {
    /// Create a new temporary git repository with the given name suffix.
    pub(super) async fn new(name_suffix: &str) -> Self {
        let path = std::env::temp_dir().join(format!(
            "pg-ephemeral-{}-{}",
            name_suffix,
            std::process::id()
        ));
        std::fs::create_dir_all(&path).unwrap();

        git_proc::init::new()
            .directory(&path)
            .status()
            .await
            .unwrap();

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
    pub(super) fn write_file(&self, name: &str, content: &str) {
        std::fs::write(self.path.join(name), content).unwrap();
    }

    /// Commit all files with the given message, returning the commit hash.
    pub(super) async fn commit(&self, message: &str) -> String {
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

/// Materialize an [`include_dir!`]-embedded directory tree under `dest`,
/// creating intermediate directories as needed and writing each embedded
/// file's bytes to its corresponding location.
///
/// Used by trials whose original `tests/*.rs` form read fixture files from
/// `CARGO_MANIFEST_DIR`-relative paths. The fixtures travel inside the
/// production binary via `include_dir!`; this helper writes them to a
/// runtime-controlled location so the trial can `read_dir` / pass paths
/// to `pg-ephemeral` exactly as the source-tree-coupled version did.
pub(super) fn materialize(dir: &include_dir::Dir<'_>, dest: &std::path::Path) {
    std::fs::create_dir_all(dest).unwrap();

    for entry in dir.entries() {
        let target = dest.join(entry.path().file_name().unwrap());
        match entry {
            include_dir::DirEntry::Dir(subdir) => materialize(subdir, &target),
            include_dir::DirEntry::File(file) => std::fs::write(&target, file.contents()).unwrap(),
        }
    }
}
