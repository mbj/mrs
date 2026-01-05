mrs
===

[![CI](https://github.com/mbj/mrs/actions/workflows/ci.yml/badge.svg)](https://github.com/mbj/mrs/actions/workflows/ci.yml)

A collection of Rust tools for cloud infrastructure and PostgreSQL development.

## Projects

### AWS & CloudFormation
- [**stratosphere**](stratosphere/) - CloudFormation template generation library
- [**stratosphere-core**](stratosphere-core/) - Core types and utilities for stratosphere
- [**stratosphere-generator**](stratosphere-generator/) - Code generator for stratosphere
- [**stack-deploy**](stack-deploy/) - CloudFormation stack deployment tool
- [**mlambda**](mlambda/) - AWS Lambda runtime and API Gateway utilities

### HTTP
- [**mhttp**](mhttp/) - Turn any type into a typed HTTP request with declarative response decoding

### Container Tools
- [**ociman**](ociman/) - OCI Manager - unified API for OCI container runtimes (Docker, Podman)

### PostgreSQL
- [**pg-client**](pg-client/) - PostgreSQL client configuration and types
- [**pg-ephemeral**](pg-ephemeral/) - Ephemeral PostgreSQL instances for testing
- [**mmigration**](mmigration/) - PostgreSQL migration management

---

By [@mbj](https://github.com/mbj)
