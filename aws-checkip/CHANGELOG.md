# Changelog

## 0.0.1

- Initial release

### Added

- `read_net` async function to fetch public IP from `checkip.amazonaws.com`
- Returns `ipnet::IpNet` host address (`/32` for IPv4, `/128` for IPv6)
- `DECODER` static for response decoding via `typed-reqwest`, enabling testable decode pipelines
