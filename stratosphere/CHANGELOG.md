# Changelog

## 0.0.6

### Fixed

- Documentation URLs now render as clickable links in rustdoc
- Removed stale generated service files no longer in CloudFormation spec:
  `aws/iotfleethub.rs`, `aws/lookoutmetrics.rs`, `aws/opsworkscm.rs`

### Changed

- Generator only writes files when content changes
- Generator removes stale files from output directory
- Generator pipes `rustfmt` per file in parallel instead of a single `cargo fmt` pass
- Bump stratosphere-core and stratosphere-generator dependencies to 0.0.6

## 0.0.5

### Added

- `Output!` macro arms accepting `condition:` field for conditional outputs
- Updated CloudFormation resource specification

### Changed

- `Integer` properties now use `i32` (was `i64`), `Long` properties use `i64`
- Bump stratosphere-core and stratosphere-generator dependencies to 0.0.5

## 0.0.4

### Changed

- Users no longer need to add `stratosphere-generator` as a direct dependency for macros to work
- Re-export `stratosphere_generator` as `stratosphere::generator`
- Update generated macros to use `stratosphere::generator::construct_*` instead of `stratosphere_generator::construct_*`
- Bump stratosphere-core and stratosphere-generator dependencies to 0.0.4

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
