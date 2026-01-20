# mlambda - AWS Lambda Runtime

> **Status**: Pre-1.0 - exists to serve [mbj/mrs](https://github.com/mbj/mrs) monorepo, expect breaking changes without notice.

Minimal AWS Lambda runtime client with API Gateway v2 types.

## Why not lambda_runtime?

The official `lambda_runtime` crate is comprehensive but complex. mlambda takes a different approach:

- **Minimal dependencies**: Uses reqwest directly, no tower/hyper stack
- **Explicit error handling**: Decode errors are returned in the event body rather than failing the invocation, allowing handlers to produce meaningful error responses
- **Simple handler dispatch**: A `Handler` trait with `FromStr` for routing `_HANDLER` values to implementations

## Features

- **Lambda Runtime**: Event loop client for the Lambda Runtime API
- **API Gateway v2**: Request and response types for proxy integration
- **Handler Trait**: Simple trait for dispatching to registered handlers
