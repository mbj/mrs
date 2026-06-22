//! Testing utilities for container-based tests
//!
//! This module provides helpers for writing tests that interact with container runtimes.

/// Alpine latest image used by tests.
pub static ALPINE_LATEST_IMAGE: std::sync::LazyLock<crate::image::Reference> =
    std::sync::LazyLock::new(|| "alpine:latest".parse().unwrap());

/// Postgres image used by tests that bypass higher-level image abstractions.
pub static POSTGRES_IMAGE: std::sync::LazyLock<crate::image::Reference> =
    std::sync::LazyLock::new(|| "docker.io/library/postgres:17".parse().unwrap());

/// Ruby image used by integration test Dockerfiles.
pub static RUBY_IMAGE: std::sync::LazyLock<crate::image::Reference> =
    std::sync::LazyLock::new(|| "docker.io/ruby:3.4-alpine".parse().unwrap());

/// Node image used by integration test Dockerfiles.
pub static NODE_IMAGE: std::sync::LazyLock<crate::image::Reference> =
    std::sync::LazyLock::new(|| "docker.io/node:22-alpine".parse().unwrap());

/// Throwaway OCI registry image used by cache-registry round-trip tests.
pub static REGISTRY_IMAGE: std::sync::LazyLock<crate::image::Reference> =
    std::sync::LazyLock::new(|| "docker.io/library/registry:2".parse().unwrap());

/// The generic images the test suites expect to be present in the backend.
///
/// This is the shared source of truth for the pre-pull performed before the
/// test run (driven by `manager pg-ephemeral pull-test-images`, wired as a
/// nextest setup script). It deliberately includes images that not every
/// consumer uses — overpulling a few images is cheaper than each test pulling
/// on demand and racing concurrent builds on the shared layer store.
///
/// Consumers that have additional, crate-specific images (e.g. pg-ephemeral's
/// default Postgres image, which this layer cannot compute) append their own.
#[must_use]
pub fn test_images() -> Vec<crate::image::Reference> {
    vec![
        ALPINE_LATEST_IMAGE.clone(),
        POSTGRES_IMAGE.clone(),
        RUBY_IMAGE.clone(),
        NODE_IMAGE.clone(),
        REGISTRY_IMAGE.clone(),
    ]
}

#[allow(clippy::test_attr_in_doctest)]
/// Check if the current platform is not supported for container tests
///
/// Returns `true` on macOS running in GitHub Actions, where container
/// runtime is not available or reliable.
///
/// # Example
///
/// ```
/// #[test]
/// fn my_container_test() {
///     if ociman::testing::platform_not_supported() {
///         return;
///     }
///     // ... test code that requires containers
/// }
/// ```
#[must_use]
pub fn platform_not_supported() -> bool {
    crate::platform::support().is_err()
}

/// Returns the process ID for use as a run-specific prefix in test image names.
///
/// Concurrent test processes have different PIDs, preventing image-name collisions
/// when multiple test runs execute simultaneously.
#[must_use]
pub fn run_id() -> u32 {
    static RUN_ID: std::sync::LazyLock<u32> = std::sync::LazyLock::new(std::process::id);
    *RUN_ID
}

/// Create a test image reference with a run-specific prefix.
///
/// Prepends the process ID to the given base string to avoid collisions
/// between concurrent test processes.
///
/// # Panics
///
/// Panics if the resulting string is not a valid OCI image reference.
#[must_use]
pub fn test_reference(base: &str) -> crate::image::Reference {
    format!("{}-{base}", run_id()).parse().unwrap()
}

/// Create a test image name with a run-specific prefix.
///
/// Like [`test_reference`], but returns a [`Name`](crate::reference::Name) for use
/// with content-addressed build definitions.
///
/// # Panics
///
/// Panics if the resulting string is not a valid OCI image name.
#[must_use]
pub fn test_name(base: &str) -> crate::reference::Name {
    format!("{}-{base}", run_id()).parse().unwrap()
}

/// Sets up the test backend, initializing logging and checking platform support.
///
/// Returns `None` if the platform is not supported for container tests.
/// This function is async and returns an `Option<Backend>`.
/// Use the `test_backend_setup!()` macro instead which handles early return properly.
pub async fn setup_backend() -> Option<crate::Backend> {
    #[cfg(feature = "test-utils")]
    {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    if platform_not_supported() {
        return None;
    }

    Some(
        crate::backend::resolve::auto()
            .await
            .expect("No container backend detected"),
    )
}

/// Sets up the test backend for async tests.
///
/// This macro:
/// 1. Initializes env_logger for tests (if test-utils feature is enabled)
/// 2. Checks if the platform supports containers, returning early if not
/// 3. Returns the resolved backend
///
/// Must be used in an async function:
/// ```ignore
/// #[tokio::test]
/// async fn my_test() {
///     let backend = ociman::test_backend_setup!();
///     // use backend...
/// }
/// ```
#[macro_export]
macro_rules! test_backend_setup {
    () => {{
        #[cfg(feature = "test-utils")]
        {
            let _ = env_logger::builder().is_test(true).try_init();
        }

        if $crate::testing::platform_not_supported() {
            return;
        }

        $crate::backend::resolve::auto()
            .await
            .expect("No container backend detected")
    }};
}
