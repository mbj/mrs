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
///     if cbt::testing::platform_not_supported() {
///         return;
///     }
///     // ... test code that requires containers
/// }
/// ```
pub fn platform_not_supported() -> bool {
    std::env::consts::OS == "macos" && std::env::var("GITHUB_ACTIONS").is_ok()
}
