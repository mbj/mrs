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
