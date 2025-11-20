# stratosphere

Type-safe CloudFormation template generation library for Rust.

Stratosphere provides a type-safe way to generate AWS CloudFormation templates in Rust,
with compile-time validation of resource types and properties.

## Getting Started

Generate Rust types for the AWS services you need, then build your CloudFormation template:

```rust
use stratosphere::template::*;
use cloudformation::aws::ec2;

// Generate types for AWS services
stratosphere::generator::services!(
    "AWS::EC2",
    "AWS::SecretsManager"
);

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

## Features

- **Comprehensive coverage**: Supports all AWS CloudFormation resource types
- **Type-safe resource definitions**: Generated types for all AWS CloudFormation resources
- **Compile-time validation**: Catch errors before deployment
- **On-demand type generation**: Types are dynamically created per CloudFormation service using official AWS resource specifications, keeping compiled code minimal
- **CloudFormation intrinsic functions**: Type-safe wrappers for `Fn::Sub`, `Fn::Join`, `Ref`, etc.
- **Template builder API**: Fluent interface for constructing templates
- **Helper macros**: Convenient macros for parameters, outputs, and common patterns

## Design Philosophy

Stratosphere generates CloudFormation resource types on-demand based on the AWS services you actually use. When you invoke the `services!` macro with specific AWS service names, it dynamically creates the necessary types from the official AWS CloudFormation resource specifications.

This approach ensures that your compiled binary only includes the types for the services you're actually using, resulting in minimal compiled code size. You only pay for what you use, both in terms of compilation time and binary size.

## Module Organization

- `generator`: Procedural macros for generating CloudFormation resource types
- `template`: Core template types and builder API
- `value`: CloudFormation values and intrinsic functions
- `resource_specification`: AWS resource type definitions
- `token`: Code generation utilities
