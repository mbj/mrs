//! Testing utilities for container-based tests
//!
//! This module provides helpers for writing tests that interact with container runtimes.

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

#[macro_export]
macro_rules! test_backend_setup {
    () => {{
        let _ = env_logger::builder().is_test(true).try_init();

        if $crate::testing::platform_not_supported() {
            return;
        }
        $crate::backend::resolve::auto().expect("No container backend detected")
    }};
}
