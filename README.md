mrs
===

[![CI](https://github.com/mbj/mrs/actions/workflows/ci.yml/badge.svg)](https://github.com/mbj/mrs/actions/workflows/ci.yml)

A collection of Rust tools for cloud infrastructure, PostgreSQL, Git, and GitHub.

## Projects

### AWS & CloudFormation
- [**stratosphere**](stratosphere/) - CloudFormation template generation library
- [**stratosphere-core**](stratosphere-core/) - Core types and utilities for stratosphere
- [**stratosphere-generator**](stratosphere-generator/) - Code generator for stratosphere
- [**stack-deploy**](stack-deploy/) - CloudFormation stack deployment tool
- [**mlambda**](mlambda/) - AWS Lambda runtime and API Gateway utilities

### GCP
- [**cloud-sql-connector**](cloud-sql-connector/) - Cloud SQL Auth Proxy connector for Rust

### HTTP
- [**mhttp**](mhttp/) - Turn any type into a typed HTTP request with declarative response decoding

### Container Tools
- [**ociman**](ociman/) - OCI Manager - unified API for OCI container runtimes (Docker, Podman)

### PostgreSQL
- [**pg-client**](pg-client/) - PostgreSQL client configuration and types
- [**pg-ephemeral**](pg-ephemeral/) - Ephemeral PostgreSQL instances for testing
- [**mmigration**](mmigration/) - PostgreSQL migration management

### GitHub
- [**greenhell**](greenhell/) - GitHub status check aggregator - ensures all commits in a PR pass CI

### Git
- [**wtt**](wtt/) - Work Tree Tool - manages git worktrees using bare clones

### Utilities
- [**cmd-proc**](cmd-proc/) - `std::process::Command` wrapper with debug logging, stronger input types, and automatic exit code checking

---

By [@mbj](https://github.com/mbj)
