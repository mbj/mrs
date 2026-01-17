# Changelog

## 0.0.3

### Changed

- Bump stratosphere-core and stratosphere-generator dependencies to 0.0.3

## 0.0.2

### Changed

- Switch from proc-macro-based generation to pre-generated source files
- Feature-gated services for minimal compile times (e.g., `aws_ec2`, `aws_s3`)
- `Box` wrapping for recursive property types to handle CloudFormation type cycles

### Added

- Pre-generated resource types for all 261 AWS CloudFormation services
- `manager stratosphere sync` command for regenerating types from CloudFormation spec

## 0.0.1

Initial release.

### Added

- Type-safe CloudFormation template generation
- Template builder API with fluent interface
- CloudFormation intrinsic functions (`Fn::Sub`, `Fn::Join`, `Ref`, `Fn::GetAtt`, etc.)
- Helper macros for resources, parameters, and outputs
- Compile-time validation of resource properties
