# Changelog

## 0.0.5

### Added

- Template `Conditions` section support with `condition()` / `condition_()` builder methods
- `ExpBool::Condition` variant for referencing named conditions in expressions
- `conditional_resource()` / `conditional_resource_()` builder methods for resources with conditions
- `condition` field on `Output` for conditional outputs
- `Serialize` impl for `ExpBool`
- `Eq`, `PartialEq` derives for `ExpPair` and `ExpBool`
- `Ord`, `PartialOrd`, `Display` derives and `From<&ConditionName>` for `ConditionName`
- `From<ConditionName>` and `From<&ConditionName>` for `ExpBool`
- `ToValue` impl for `i32`
- `Long` variant for `PrimitiveItemType`

### Changed

- `PrimitiveType::Integer` now maps to `i32` (was `i64`)
- `PrimitiveItemType::Integer` now maps to `i32` (was `i64`)

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
