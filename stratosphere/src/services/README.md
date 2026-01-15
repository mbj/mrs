# Generated Services

This directory contains pre-generated CloudFormation resource types.

## Why Pre-generation?

We want three things that are currently incompatible in Rust:

1. **Namespaced macros** - ergonomic API like `ec2::VPC!` instead of flat `ec2_VPC!`
2. **Feature-flagged services** - users only compile services they need
3. **Canonical types** - shared libraries can reference stratosphere types

### The Problem

Rust's `macro_rules!` visibility doesn't support our use case:

- `#[macro_export]` hoists macros to the crate root, losing the namespace
- `pub(crate) use` keeps the namespace but can't cross crate boundaries
- Proc macros generating `macro_rules!` with `pub use` triggers the
  `macro_expanded_macro_exports_accessed_by_absolute_paths` lint
- `build.rs` + `include!()` also counts as macro expansion — same lint

The only way to export namespaced `macro_rules!` is from **real source files**,
not macro-expanded code.

### Current Workaround

A maintainer command generates these files from the CloudFormation spec.
The files are checked into version control and updated when AWS releases
new resource types.

### Future: Declarative Macros 2.0

The unstable `decl_macro` feature (`pub macro`) provides properly scoped macros
that can be exported with namespacing. When stabilized, we can return to
proc-macro-based generation:

```rust,ignore
// Future syntax (currently unstable)
pub macro VPC { ... }
```

Tracking issue: https://github.com/rust-lang/rust/issues/39412
