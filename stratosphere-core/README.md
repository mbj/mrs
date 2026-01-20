# stratosphere-core - CloudFormation Core Types

> **Status**: Pre-1.0 - exists to serve [mbj/mrs](https://github.com/mbj/mrs) monorepo, expect breaking changes without notice.

Core types and utilities for stratosphere - a type-safe CloudFormation template generation library.

This crate provides the fundamental building blocks for generating AWS CloudFormation templates:

- **Template types**: Core structures for CloudFormation templates, resources, parameters, and outputs
- **Value types**: Type-safe representations of CloudFormation intrinsic functions (`Fn::Sub`, `Fn::Join`, etc.)
- **Resource specification**: AWS CloudFormation resource type definitions
- **Code generation**: Utilities for generating Rust code from CloudFormation specifications

**Note**: Most types in this crate are re-exported by the [`stratosphere`](https://crates.io/crates/stratosphere)
crate, which provides a higher-level API. Users should typically depend on `stratosphere` rather than
using `stratosphere-core` directly.

This crate exists as a separate package to share types between the procedural macro code in
`stratosphere-generator` and the main `stratosphere` library, as proc macro crates can currently
only export procedural macros.
