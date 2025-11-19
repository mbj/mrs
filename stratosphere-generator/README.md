# stratosphere-generator

Procedural macros for stratosphere - generates CloudFormation resource types.

This crate provides compile-time code generation for AWS CloudFormation resources:

- **`services!`**: Generates Rust types for specified AWS CloudFormation services

These macros are re-exported by the [`stratosphere`](https://crates.io/crates/stratosphere) crate
and should be used through that crate rather than depending on `stratosphere-generator` directly.
See the [`stratosphere`](https://crates.io/crates/stratosphere) crate for usage examples.

As a proc macro crate, this package can only export procedural macros. Shared types are provided
by the [`stratosphere-core`](https://crates.io/crates/stratosphere-core) crate.

Note: This crate also contains internal helper macros (`construct_resource_type!`, `construct_property_type!`)
that are used by generated code but are not intended for direct use.
