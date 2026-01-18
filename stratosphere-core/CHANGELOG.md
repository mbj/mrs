# Changelog

## 0.0.4

### Changed

- Update generated macro paths from `stratosphere_generator::` to `stratosphere::generator::`

## 0.0.3

### Added

- All CloudFormation pseudo variables as appropriately typed expressions
- Intrinsic function support: `Fn::Cidr`, `Fn::GetAZs`, `Fn::FindInMap`, `Fn::Split`, `Fn::Select`
- Timestamp value implementation
- Full Rust keyword escaping for generated identifiers
- Missing list types for CloudFormation arrays
- Feature definition support for service-gated code generation

### Changed

- Switch to nom-based parsing for CloudFormation resource specifications
- Make `Fn::If` generic over expression type
- Improved `ToValue` implementations

## 0.0.1

Initial release.
