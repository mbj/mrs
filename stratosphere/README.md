# stratosphere - CloudFormation Template Generator

> **Status**: Pre-1.0 - exists to serve [mbj/mrs](https://github.com/mbj/mrs) monorepo, expect breaking changes without notice.

Type-safe CloudFormation template generation library for Rust.

Stratosphere provides a type-safe way to generate AWS CloudFormation templates in Rust,
with compile-time validation of resource types and properties.

## Getting Started

Enable the AWS services you need via Cargo features, then build your CloudFormation template:

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

Enable the AWS services you need in your `Cargo.toml`:

```toml
[dependencies]
stratosphere = { version = "0.0.4", features = ["aws_ec2", "aws_secretsmanager"] }
```

Each AWS service has its own feature flag (e.g., `aws_ec2`, `aws_s3`, `aws_lambda`).

## Features

- **Comprehensive coverage**: Supports all AWS CloudFormation resource types
- **Type-safe resource definitions**: Pre-generated types for all AWS CloudFormation resources
- **Compile-time validation**: Catch errors before deployment
- **Feature-gated services**: Only compile the services you need, keeping build times fast
- **CloudFormation intrinsic functions**: Type-safe wrappers for `Fn::Sub`, `Fn::Join`, `Ref`, etc.
- **Template builder API**: Fluent interface for constructing templates
- **Helper macros**: Convenient macros for parameters, outputs, and common patterns

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
