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

- **Type-safe resource definitions**: Generated types for all AWS CloudFormation resources
- **Compile-time validation**: Catch errors before deployment
- **CloudFormation intrinsic functions**: Type-safe wrappers for `Fn::Sub`, `Fn::Join`, `Ref`, etc.
- **Template builder API**: Fluent interface for constructing templates
- **Helper macros**: Convenient macros for parameters, outputs, and common patterns

## Module Organization

- `generator`: Procedural macros for generating CloudFormation resource types
- `template`: Core template types and builder API
- `value`: CloudFormation values and intrinsic functions
- `resource_specification`: AWS resource type definitions
- `token`: Code generation utilities
