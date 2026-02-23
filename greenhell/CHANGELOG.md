# Changelog

## 0.0.1

- Initial release

### Added

- GitHub status check aggregator ensuring all commits in a PR pass CI
- `evaluate` command for evaluating status checks across PR commits
- `watch` command for continuous PR status monitoring
- `push-each` command for pushing commits individually to trigger per-commit CI
- GitHub App authentication with JWT generation and installation token exchange
- Check Runs API integration
- Token discovery from environment variables or `gh auth token`
