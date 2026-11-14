# Changelog

## Unreleased

### Changed

- Raised minimum supported Rust version to 1.100.
- Bump `zip` from 8.5.0 to 9.5.1, which fixes RUSTSEC-2026-0367, -0368, -0369
  and -0429. Only the `deflate` feature is enabled now, which drops the unused
  compression codecs and encryption support from the dependency tree.

## 0.0.3

### Changed

- Bump `zip` from 7.2.0 to 8.0.0
- Bump `uuid` from 1.20.0 to 1.21.0
- Add crate metadata (authors, description, license, repository, homepage, keywords, categories)

## 0.0.2

* Add tag support to `InstanceSpec` via `TagMap`, `TagKey`, `TagValue` types.

## 0.0.1

* Initial release.
