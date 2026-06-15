# Changelog

## Unreleased

## 0.0.4

### Removed

- `put_secret_value_string` is no longer public; secret writes go through the validating CLI path.

### Changed

- Raised minimum supported Rust version to 1.95.

## 0.0.3

### Changed

- Bump `zip` from 7.2.0 to 8.0.0
- Bump `uuid` from 1.20.0 to 1.21.0
- Add crate metadata (authors, description, license, repository, homepage, keywords, categories)

## 0.0.2

* Add tag support to `InstanceSpec` via `TagMap`, `TagKey`, `TagValue` types.

## 0.0.1

* Initial release.
