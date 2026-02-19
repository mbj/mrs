# stratosphere - CloudFormation Template Generator

[![crates.io](https://img.shields.io/crates/v/stratosphere.svg)](https://crates.io/crates/stratosphere)
[![docs.rs](https://docs.rs/stratosphere/badge.svg)](https://docs.rs/stratosphere)

Type-safe CloudFormation template generation library for Rust.

Stratosphere provides a type-safe way to generate AWS CloudFormation templates in Rust,
with compile-time validation of resource types and properties.

## Getting Started

Install stratosphere with the AWS services you need:

```sh
cargo add stratosphere --features aws_ec2
```

Then build your CloudFormation template:

```rust
use stratosphere::template::*;
use stratosphere::services::aws::ec2;

// Build your CloudFormation template using the generated resource macros
let template = Template::build(|t| {
    let vpc_cidr = &t.parameter(
        "VpcCidr",
        stratosphere::Parameter! {
            description: "CIDR block for the VPC",
            r#type: ParameterType::String,
        },
    );

    let vpc = &t.resource(
        "Vpc",
        ec2::VPC! {
            cidr_block: vpc_cidr
        },
    );

    t.output(
        "VpcId",
        stratosphere::Output! {
            description: "ID of the VPC",
            value: vpc,
        },
    );
});

// Convert to JSON for use with CloudFormation APIs
let json = serde_json::to_string_pretty(&template).unwrap();
// This JSON can now be used with AWS CloudFormation APIs
```

## Cargo Features

Each AWS service has its own feature flag (e.g., `aws_ec2`, `aws_s3`, `aws_lambda`).
Enable only the services you need for significant compile time savings:

```sh
cargo add stratosphere --features aws_ec2 --features aws_secretsmanager
```

## Compile-Time Safety

Missing a required field is a compile-time error:

```rust,compile_fail
use stratosphere::services::aws::ec2;

ec2::SecurityGroup! {
    // error: AWS::EC2::SecurityGroup is missing required fields: group_description
}
```

Type mismatches between `ExpString` and `ExpBool` fields are caught by the compiler:

```rust,compile_fail
use stratosphere::services::aws::ec2;

ec2::SecurityGroup! {
    group_description: true
    //                 ^^^^ expected `ExpString`, found `bool`
}
```

## Features

- **Comprehensive coverage**: Supports all AWS CloudFormation resource types
- **Type-safe resource definitions**: Pre-generated types for all AWS CloudFormation resources
- **Compile-time validation**: Catch errors before deployment
- **Feature-gated services**: Only compile the services you need, keeping build times fast
- **CloudFormation intrinsic functions**: Type-safe wrappers for `Fn::Sub`, `Fn::Join`, `Ref`, etc.
- **Template builder API**: Fluent interface for constructing templates
- **Helper macros**: Convenient macros for parameters, outputs, and common patterns

## Why Stratosphere?

**vs. writing JSON/YAML by hand**: No more typos in resource type names, missing required
properties, or type mismatches discovered at deploy time. Stratosphere catches these at compile time.

**vs. AWS CDK / Pulumi**: These don't support Rust. If your infrastructure tooling is already
in Rust, stratosphere keeps everything in one language with one build system. It also generates
plain CloudFormation JSON with no runtime dependencies or state backends.

**vs. troposphere (Python)**: Similar approach (generate CloudFormation from code), but
troposphere validates at runtime. Stratosphere validates at compile time.

**vs. other Rust CloudFormation crates**: Stratosphere auto-generates types from the official
AWS CloudFormation resource specification, covering all ~260 services. Alternatives either
cover a handful of hand-written services or are abandoned. Stratosphere is also the only
Rust crate with compile-time typed CloudFormation expressions (`Ref`, `Fn::Sub`, `Fn::Join`, etc.).

Stratosphere is a Rust re-implementation of the [Haskell stratosphere](https://github.com/mbj/stratosphere)
library, which has been in production use since 2017.

## Design Philosophy

Stratosphere provides pre-generated CloudFormation resource types gated behind Cargo features.
Enable only the AWS services you need, and your compiled binary will only include those types.

This approach ensures fast compilation times and minimal binary size - you only pay for what you use.

Note: Once Rust's declarative macros 2.0 are stabilized, pre-generation can be removed in favor
of on-demand type generation via proc macros with proper namespacing support.

## Module Organization

- `services`: Pre-generated CloudFormation resource types organized by vendor and service
- `template`: Core template types and builder API
- `value`: CloudFormation values and intrinsic functions
- `resource_specification`: AWS resource type definitions
