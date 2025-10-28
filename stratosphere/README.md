# Stratosphere

A type-safe Rust library for generating AWS CloudFormation templates programmatically.

## Overview

Stratosphere allows you to define AWS CloudFormation infrastructure as Rust code instead
of writing raw JSON or YAML templates. It provides compile-time type safety and validation
for all AWS resources based on the official CloudFormation Resource Specification.

## Features

- **Type-Safe CloudFormation** - Define infrastructure with Rust's type system, catching errors at compile time
- **9,000+ AWS Resources** - Support for all AWS CloudFormation resource types via the embedded specification
- **Intrinsic Functions** - Full support for CloudFormation functions (`Ref`, `GetAtt`, `Join`, `If`, `Base64`, etc.)
- **Pseudo Parameters** - Access AWS pseudo-parameters (`AWS::AccountId`, `AWS::Region`, `AWS::StackName`)
- **Template Parameters & Outputs** - Define parameters, outputs, and exports with type safety
- **Usage Modes** - Use via a CLI generator or as procedural macros in your Rust code

## Installation

Add stratosphere to your `Cargo.toml`:

```toml
[dependencies]
stratosphere = { path = "../stratosphere" }
```

## Usage

### Method 1: Procedural Macros (Recommended)

Generate AWS resource types at compile time using the `services!` macro:

```rust
use stratosphere::Template;

// Generate types for specific AWS services
stratosphere::generator::services!("AWS::EC2", "AWS::SecretsManager");

fn main() {
    // Use the generated types to build a template
    let template = Template::build(|template| {
        let security_group = template.resource(
            "MySecurityGroup",
            ec2::SecurityGroup! {
                group_description: "My application security group",
                vpc_id: "vpc-12345678",
                tags: [
                    Tag! {
                        key: "Environment",
                        value: "Production"
                    }
                ]
            }
        );

        template.output(
            "SecurityGroupId",
            stratosphere::Output! {
                description: "The security group ID",
                value: security_group,
            }
        );
    });

    // Render the template to JSON
    println!("{}", template.render_json_pretty());
}
```

### Method 2: CLI Generator

Generate Rust code for AWS services using the CLI tool:

```bash
# Generate code for specific services
stratosphere-generator AWS::EC2 AWS::RDS AWS::Lambda > generated.rs

# Use the generated code in your project
```

## Examples

### Basic Template with EC2 Security Group

```rust
use stratosphere::Template;

stratosphere::generator::services!("AWS::EC2");

fn main() {
    let template = Template::new()
        .resource_(
            "WebServerSecurityGroup",
            ec2::SecurityGroup_ {
                group_description: "Security group for web servers".into(),
                security_group_ingress: Some(vec![
                    ec2::SecurityGroupIngress_ {
                        ip_protocol: "tcp".into(),
                        from_port: Some(80.into()),
                        to_port: Some(80.into()),
                        cidr_ip: Some("0.0.0.0/0".into()),
                        ..Default::default()
                    },
                    ec2::SecurityGroupIngress_ {
                        ip_protocol: "tcp".into(),
                        from_port: Some(443.into()),
                        to_port: Some(443.into()),
                        cidr_ip: Some("0.0.0.0/0".into()),
                        ..Default::default()
                    }
                ].into()),
                tags: Some(vec![
                    Tag_ {
                        key: "Name".into(),
                        value: "WebServerSG".into(),
                    }
                ].into()),
                ..Default::default()
            }
        );

    println!("{}", template.render_json_pretty());
}
```

### Using CloudFormation Intrinsic Functions

```rust
use stratosphere::{Template, Value};
use stratosphere::value::{ExpString, Function};

stratosphere::generator::services!("AWS::EC2", "AWS::SecretsManager");

fn main() {
    let template = Template::build(|template| {
        // Create a secret
        let db_password = template.resource(
            "DatabasePassword",
            secretsmanager::Secret! {
                description: "Database master password",
                generate_secret_string: secretsmanager::GenerateSecretString! {
                    password_length: 32,
                    exclude_characters: "\"@/\\"
                }
            }
        );

        // Reference the secret in another resource
        let security_group = template.resource(
            "DatabaseSecurityGroup",
            ec2::SecurityGroup! {
                group_description: ExpString::from(Function::Join(
                    "-",
                    vec![
                        "Security group for".into(),
                        Function::Ref(db_password.logical_id().into()).into()
                    ]
                )),
                vpc_id: "vpc-12345678"
            }
        );

        // Output with GetAtt
        template.output(
            "SecretArn",
            stratosphere::Output! {
                description: "ARN of the database password secret",
                value: Function::GetAtt(
                    db_password.logical_id().into(),
                    "Arn".into()
                ),
            }
        );
    });

    println!("{}", template.render_json_pretty());
}
```

### Template with Parameters

```rust
use stratosphere::{Template, Parameter};

stratosphere::generator::services!("AWS::EC2");

fn main() {
    let template = Template::new()
        .parameter(
            "Environment",
            Parameter {
                parameter_type: "String".into(),
                default: Some("Development".into()),
                allowed_values: Some(vec![
                    "Development".into(),
                    "Staging".into(),
                    "Production".into()
                ]),
                description: Some("Environment name".into()),
                ..Default::default()
            }
        )
        .resource_(
            "AppSecurityGroup",
            ec2::SecurityGroup_ {
                group_description: "Application security group".into(),
                tags: Some(vec![
                    Tag_ {
                        key: "Environment".into(),
                        value: Function::Ref("Environment".into()).into(),
                    }
                ].into()),
                ..Default::default()
            }
        );

    println!("{}", template.render_json_pretty());
}
```

### Using Pseudo Parameters

```rust
use stratosphere::{Template, value::PseudoParameter};

stratosphere::generator::services!("AWS::EC2");

fn main() {
    let template = Template::build(|template| {
        template.resource(
            "MySecurityGroup",
            ec2::SecurityGroup! {
                group_description: "Security group with region tag",
                tags: [
                    Tag! {
                        key: "Region",
                        value: PseudoParameter::Region
                    },
                    Tag! {
                        key: "AccountId",
                        value: PseudoParameter::AccountId
                    }
                ]
            }
        );
    });

    println!("{}", template.render_json_pretty());
}
```

### Exporting Outputs

```rust
use stratosphere::{Template, Output, Export};

stratosphere::generator::services!("AWS::EC2");

fn main() {
    let template = Template::build(|template| {
        let vpc = template.resource(
            "MyVPC",
            ec2::VPC! {
                cidr_block: "10.0.0.0/16",
                enable_dns_support: true,
                enable_dns_hostnames: true
            }
        );

        template.output(
            "VPCId",
            Output {
                description: Some("VPC ID".into()),
                value: vpc.into(),
                export: Some(Export {
                    name: Function::Join(
                        "-",
                        vec![
                            Function::Ref("AWS::StackName".into()).into(),
                            "VPCId".into()
                        ]
                    ).into()
                }),
                ..Default::default()
            }
        );
    });

    println!("{}", template.render_json_pretty());
}
```

## Project Structure

The stratosphere project consists of three main components:

- **`stratosphere`** - The main library that re-exports public API
- **`stratosphere-core`** - Core functionality for template building and value types
- **`stratosphere-generator`** - Procedural macros for compile-time code generation

## CloudFormation Specification

Stratosphere embeds the complete AWS CloudFormation Resource Specification (270KB JSON with 9,000+ resource types) and uses it to generate type-safe Rust bindings for all AWS resources at compile time.

## Output Format

Templates are rendered as CloudFormation JSON with the following structure:

```json
{
  "AWSTemplateFormatVersion": "2010-09-09",
  "Parameters": { ... },
  "Resources": {
    "LogicalResourceId": {
      "Type": "AWS::Service::Resource",
      "Properties": { ... }
    }
  },
  "Outputs": { ... }
}
```

## License

See the workspace root for license information.
