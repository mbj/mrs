//! Dependency lints for the workspace.
//!
//! These guard invariants about the *resolved* dependency graph that the
//! type system can't express. They run as part of the normal `cargo test`.

use std::process::Command;

/// `ring` must never be compiled.
///
/// The workspace standardised on aws-lc-rs as its sole crypto backend: TLS
/// (rustls), certificate generation (rcgen) and verification (x509-parser) all
/// use aws-lc-rs. `ring` still appears in `Cargo.lock` as an *inactive*
/// optional dependency of `rustls-webpki` and `quinn-proto`, but it must not be
/// compiled. This fails if a change activates a feature that pulls `ring` back
/// into the build graph.
///
/// `cargo tree --invert` reflects the feature-resolved graph that is actually
/// compiled — unlike `Cargo.lock` / `cargo metadata`, which also list inactive
/// optional dependencies. With `ring` absent the command prints nothing to
/// stdout; when it is compiled, stdout starts with `ring v…`.
#[test]
fn ring_is_not_compiled() {
    let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_owned());

    // Default features only: `--all-features` would activate the optional ring
    // paths (e.g. `rustls-webpki/ring`) and defeat the guard.
    let output = Command::new(cargo)
        .args(["tree", "--workspace", "--invert", "ring"])
        .output()
        .expect("failed to invoke `cargo tree`");

    assert!(
        output.status.success(),
        "`cargo tree` failed:\n{}",
        String::from_utf8_lossy(&output.stderr),
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.trim().is_empty(),
        "`ring` is in the compiled dependency graph; the workspace must use \
         aws-lc-rs only.\n`cargo tree --workspace --invert ring` reported:\n{stdout}",
    );
}
